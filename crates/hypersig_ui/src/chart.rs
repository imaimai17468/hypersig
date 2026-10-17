//! Charts: bars, lines, or areas drawn from the rows of a list signal, on a
//! scale the browser fits to the values.

use std::fmt;

use hypersig::html::{El, Node, div, each, el, li, span, table, tbody, td, th, thead, tr, ul};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Display, FlexDirection, FlexWrap, FontVariantNumeric,
    FontWeight, JustifyContent, Opacity, Overflow, PointerEvents, Position, TextAlign, WhiteSpace,
    ZIndex, pct, px, raw, rem,
};
use hypersig::style::{Condition, Property, PseudoClass, Style, Token};
use hypersig::{Expr, IntoExpr, Item, Number, NumberFormat, Sig, Textual, if_else};

use crate::look;
use crate::token;

/// How a [`Chart`] draws its series.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Bar,
    Line,
    Area,
}

/// One series of a chart: its name, its color, and how a row's value reads
/// in the browser.
struct Series<S, T> {
    name: String,
    color: Token<Color>,
    value: RowValue<S, T>,
    text: RowText<S, T>,
}

/// A row's value of one series, as a number for drawing.
type RowValue<S, T> = Box<dyn Fn(Expr<S, T>) -> Expr<S, f64>>;

/// How a row's value of one series reads as text, in the format where there is one.
type RowText<S, T> = Box<dyn Fn(Item<S, T>, Option<&NumberFormat>) -> Expr<S, String>>;

/// The most grid lines a scale has. The step is at least a quarter of the
/// range of the values, and rounding both ends out to a step adds fewer
/// than two more, so a scale spans at most five steps.
const TICKS: i32 = 6;

/// A bar, line, or area chart of the rows of a list signal.
///
/// Each row is one point along the horizontal axis, labeled by the text
/// field `category`, and each [`Chart::series`] reads a number field of the
/// row. The browser fits the vertical scale to the values: it runs from a
/// round number at or below the smallest value, or 0, to a round number at
/// or above the largest, or 0, with grid lines on a step of 1, 2, 2.5, or 5
/// times a power of ten. A patch from the server or a statement in the
/// browser that changes a value moves its bar or its line and refits the
/// scale. A negative value draws its bar down from the line at 0.
///
/// Each bar, point, and grid line computes the scale from every row, so a
/// change to the rows costs time that grows with the square of their
/// number. A chart of a few hundred rows stays fast, and a longer series
/// is better summed on the server first.
///
/// The drawing, the axis labels, the legend, and the box that shows a row's
/// values under the pointer are hidden from assistive technology. A screen
/// reader reads a visually hidden table of the same rows instead, captioned
/// with `label`.
///
/// ```
/// use hypersig::{Page, Signals};
/// use hypersig_ui::{Chart, token};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Month {
///     name: String,
///     desktop: i32,
///     mobile: i32,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Traffic {
///     months: Vec<Month>,
/// }
///
/// let chart = Chart::bar("Visitors", Traffic::months(), Month::name())
///     .series("Desktop", Month::desktop(), token::chart_1())
///     .series("Mobile", Month::mobile(), token::chart_2());
/// let months = vec![
///     Month { name: "January".into(), desktop: 186, mobile: 80 },
///     Month { name: "February".into(), desktop: 305, mobile: 200 },
/// ];
/// let html = Page::new("Traffic", Traffic { months }).child(chart).render();
/// assert!(html.contains("<caption>Visitors</caption>"));
/// assert!(html.contains(">400</span>"));
/// ```
pub struct Chart<S, T> {
    kind: Kind,
    label: String,
    rows: Sig<S, Vec<T>>,
    category: Sig<T, String>,
    series: Vec<Series<S, T>>,
    format: Option<NumberFormat>,
}

impl<S: 'static, T: 'static> Chart<S, T> {
    /// A bar chart named `label` of `rows`, labeled by `category`.
    ///
    /// Each row has a group of bars, one per series.
    #[must_use]
    pub fn bar(label: impl Into<String>, rows: Sig<S, Vec<T>>, category: Sig<T, String>) -> Self {
        Self::new(Kind::Bar, label.into(), rows, category)
    }

    /// A line chart named `label` of `rows`, labeled by `category`.
    ///
    /// Each row is a point on the line of each series.
    #[must_use]
    pub fn line(label: impl Into<String>, rows: Sig<S, Vec<T>>, category: Sig<T, String>) -> Self {
        Self::new(Kind::Line, label.into(), rows, category)
    }

    /// An area chart named `label` of `rows`, labeled by `category`.
    ///
    /// It is a line chart with the area between each line and 0 filled.
    #[must_use]
    pub fn area(label: impl Into<String>, rows: Sig<S, Vec<T>>, category: Sig<T, String>) -> Self {
        Self::new(Kind::Area, label.into(), rows, category)
    }

    fn new(kind: Kind, label: String, rows: Sig<S, Vec<T>>, category: Sig<T, String>) -> Self {
        Self {
            kind,
            label,
            rows,
            category,
            series: Vec::new(),
            format: None,
        }
    }

    /// Adds a series named `name` that draws the number field `field` in `color`.
    #[must_use]
    pub fn series<U: Number + Textual + 'static>(
        mut self,
        name: impl Into<String>,
        field: Sig<T, U>,
        color: Token<Color>,
    ) -> Self {
        self.series.push(Series {
            name: name.into(),
            color,
            value: Box::new(move |row| row.at(field).to_f64()),
            text: Box::new(move |row, format| match format {
                Some(format) => row.at(field).format(format),
                None => row.at(field).to_text(),
            }),
        });
        self
    }

    /// Writes the values and the axis labels in `format`.
    ///
    /// Until set, they are written as JavaScript's `String` writes a number.
    #[must_use]
    pub fn format(mut self, format: NumberFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// The chart, its legend, and the table a screen reader reads, in a `<figure>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let count = self.rows.len().to_f64();
        let band = Expr::lit(&100.0) / count;
        let plot = el("svg")
            .attr("viewBox", "0 0 100 100")
            .attr("preserveAspectRatio", "none")
            .attr("width", "100%")
            .attr("height", "100%")
            .css(
                Style::new()
                    .display(Display::Block)
                    .overflow(Overflow::Visible),
            )
            .children((0..TICKS).map(|tick| {
                let y = self.tick(tick, |value, lo, hi| y_of(value, lo, hi).to_fixed(3));
                el("line")
                    .attr("x1", "0")
                    .attr("x2", "100")
                    .attr("vector-effect", "non-scaling-stroke")
                    .css(
                        Style::new()
                            .stroke(token::border().var())
                            .stroke_width(px(1)),
                    )
                    .show(self.tick_shown(tick))
                    .bind_attr("y1", y.clone())
                    .bind_attr("y2", y)
            }))
            .children(self.marks(&band));
        let visual = div()
            .attr("aria-hidden", "true")
            .css(
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw("auto 1fr"))
                    .grid_template_rows(raw("15rem auto"))
                    .column_gap(rem(0.5))
                    .row_gap(rem(0.5))
                    .font_variant_numeric(FontVariantNumeric::TabularNums)
                    .color(token::muted_foreground().var())
                    .font_size(rem(0.75)),
            )
            .child(self.y_labels())
            .child(
                div()
                    .css(Style::new().position(Position::Relative))
                    .child(plot)
                    .child(self.hover(&band)),
            )
            .child(div())
            .child(self.x_labels(band));
        el("figure")
            .attr("aria-label", self.label.clone())
            .css(
                Style::new()
                    .display(Display::Flex)
                    .flex_direction(FlexDirection::Column)
                    .row_gap(rem(1))
                    .margin(px(0)),
            )
            .child(visual)
            .child(self.legend())
            .child(self.data_table())
    }

    /// The expression `body` builds from the low end, the high end, and the
    /// step of the scale fitted to every value of every series.
    fn scaled<U>(
        &self,
        body: impl FnOnce(Expr<S, f64>, Expr<S, f64>, Expr<S, f64>) -> Expr<S, U>,
    ) -> Expr<S, U> {
        let mut top = Expr::lit(&0.0);
        let mut bottom = Expr::lit(&0.0);
        for series in &self.series {
            let value = &series.value;
            top = top.max(
                self.rows
                    .max_of(|row| value(row.into_expr()))
                    .unwrap_or(0.0),
            );
            bottom = bottom.min(
                self.rows
                    .min_of(|row| value(row.into_expr()))
                    .unwrap_or(0.0),
            );
        }
        bottom.let_in(|bottom| {
            // With every value 0, or no rows, the scale runs from 0 to 1.
            let top = if_else(top.clone().gt(bottom.clone()), top, bottom.clone() + 1.0);
            top.let_in(|top| {
                ((top.clone() - bottom.clone()) / 4.0).let_in(|rough| {
                    let guess = Expr::lit(&10.0).pow(rough.clone().log10().floor());
                    guess.let_in(|guess| {
                        // A last-bit error in `log10` at a power of ten puts the
                        // guess one power off, and the ratio brings it back.
                        let ratio = rough.clone() / guess.clone();
                        let power = if_else(
                            ratio.clone().ge(10.0),
                            guess.clone() * 10.0,
                            if_else(ratio.lt(1.0), guess.clone() / 10.0, guess),
                        );
                        power.let_in(|power| {
                            let n = rough / power.clone();
                            let factor = if_else(
                                n.clone().le(1.0),
                                Expr::lit(&1.0),
                                if_else(
                                    n.clone().le(2.0),
                                    Expr::lit(&2.0),
                                    if_else(
                                        n.clone().le(2.5),
                                        Expr::lit(&2.5),
                                        if_else(n.le(5.0), Expr::lit(&5.0), Expr::lit(&10.0)),
                                    ),
                                ),
                            );
                            (power * factor).let_in(|step| {
                                let lo = clean((bottom / step.clone()).floor() * step.clone());
                                let hi = clean((top / step.clone()).ceil() * step.clone());
                                lo.let_in(|lo| hi.let_in(|hi| body(lo, hi, step)))
                            })
                        })
                    })
                })
            })
        })
    }

    /// The expression `body` builds from the value of the grid line numbered
    /// `tick` from the bottom and the ends of the scale.
    fn tick<U>(
        &self,
        tick: i32,
        body: impl FnOnce(Expr<S, f64>, Expr<S, f64>, Expr<S, f64>) -> Expr<S, U>,
    ) -> Expr<S, U> {
        self.scaled(|lo, hi, step| {
            let value = clean(lo.clone() + step * f64::from(tick));
            body(value, lo, hi)
        })
    }

    /// Whether the scale reaches the grid line numbered `tick`.
    fn tick_shown(&self, tick: i32) -> Expr<S, bool> {
        self.scaled(|lo, hi, step| (lo + step.clone() * f64::from(tick)).le(hi + step / 2.0))
    }

    /// The bars, or the lines and areas, of every series.
    fn marks(&self, band: &Expr<S, f64>) -> Vec<Node<S>> {
        let count: f64 = self.series.iter().map(|_| 1.0).sum();
        let mut marks = Vec::with_capacity(self.series.len());
        let mut position = 0.0;
        for series in &self.series {
            let color = series.color.clone();
            let band = band.clone();
            let rows = self.rows;
            let kind = self.kind;
            let mark = each(rows, |row| {
                let value = || (series.value)(row.into_expr());
                let x = row.index().to_f64() * band.clone();
                match kind {
                    Kind::Bar => {
                        let width = band.clone() * (0.8 / count);
                        let left = x + band.clone() * 0.1 + width.clone() * position;
                        let top = self.scaled(|lo, hi, _| {
                            y_of(value(), lo.clone(), hi.clone()).min(y_of(Expr::lit(&0.0), lo, hi))
                        });
                        let height = self.scaled(|lo, hi, _| {
                            let y = y_of(value(), lo.clone(), hi.clone());
                            let zero = y_of(Expr::lit(&0.0), lo, hi);
                            y.clone().max(zero.clone()) - y.min(zero)
                        });
                        el("rect")
                            .css(Style::new().fill(color.var()))
                            .bind_attr("x", left.to_fixed(3))
                            .bind_attr("width", width.to_fixed(3))
                            .bind_attr("y", top.to_fixed(3))
                            .bind_attr("height", height.to_fixed(3))
                    }
                    Kind::Line | Kind::Area => {
                        let next =
                            || (series.value)(rows.nth(row.index() + 1).unwrap_or(row.value()));
                        let x1 = x + band.clone() * 0.5;
                        let x2 = x1.clone() + band.clone();
                        let y1 = self.scaled(|lo, hi, _| y_of(value(), lo, hi).to_fixed(3));
                        let y2 = self.scaled(|lo, hi, _| y_of(next(), lo, hi).to_fixed(3));
                        let has_next = rows.len().gt(row.index() + 1);
                        let line = el("line")
                            .attr("vector-effect", "non-scaling-stroke")
                            .attr("stroke-linecap", "round")
                            .css(Style::new().stroke(color.var()).stroke_width(px(2)))
                            .bind_attr("x1", x1.clone().to_fixed(3))
                            .bind_attr("x2", x2.clone().to_fixed(3))
                            .bind_attr("y1", y1.clone())
                            .bind_attr("y2", y2.clone());
                        let group = el("g").show(has_next);
                        if kind == Kind::Area {
                            let base =
                                self.scaled(|lo, hi, _| y_of(Expr::lit(&0.0), lo, hi).to_fixed(3));
                            let points = x1
                                .clone()
                                .to_fixed(3)
                                .concat(",")
                                .concat(base.clone())
                                .concat(" ")
                                .concat(x1.to_fixed(3))
                                .concat(",")
                                .concat(y1)
                                .concat(" ")
                                .concat(x2.clone().to_fixed(3))
                                .concat(",")
                                .concat(y2)
                                .concat(" ")
                                .concat(x2.to_fixed(3))
                                .concat(",")
                                .concat(base);
                            group
                                .child(
                                    el("polygon")
                                        .css(Style::new().fill(color.var()).fill_opacity(0.25))
                                        .bind_attr("points", points),
                                )
                                .child(line)
                        } else {
                            group.child(line)
                        }
                    }
                }
            });
            marks.push(mark);
            position += 1.0;
        }
        marks
    }

    /// The labels of the grid lines, each beside its line.
    fn y_labels(&self) -> El<S> {
        let format = self.format;
        div()
            .css(
                Style::new()
                    .position(Position::Relative)
                    .min_width(rem(1.5))
                    .text_align(TextAlign::End),
            )
            .children((0..TICKS).map(|tick| {
                let top = self.tick(tick, |value, lo, hi| {
                    y_of(value, lo, hi).to_fixed(3).concat("%")
                });
                let text = self.tick(tick, |value, _, _| match &format {
                    Some(format) => value.format(format),
                    None => value.to_text(),
                });
                span()
                    .css(
                        Style::new()
                            .position(Position::Absolute)
                            .right(px(0))
                            .translate(raw("0 -50%"))
                            .white_space(WhiteSpace::Nowrap),
                    )
                    .show(self.tick_shown(tick))
                    .bind_style(Property::Top, top)
                    .bind_text(text)
            }))
    }

    /// One column per row over the drawing, which shows the row's values
    /// while the pointer is on it.
    fn hover(&self, band: &Expr<S, f64>) -> El<S> {
        let category = self.category;
        let series = &self.series;
        div()
            .css(Style::new().position(Position::Absolute).inset(px(0)))
            .child(each(self.rows, |row| {
                let left = (row.index().to_f64() * band.clone())
                    .to_fixed(3)
                    .concat("%");
                let mut values = div().css(
                    Style::new()
                        .display(Display::Grid)
                        .grid_template_columns(raw("auto 1fr auto"))
                        .column_gap(rem(0.5))
                        .align_items(AlignItems::Center),
                );
                for series in series {
                    values = values
                        .child(swatch(&series.color))
                        .child(span().text(series.name.clone()))
                        .child(
                            span()
                                .css(
                                    Style::new()
                                        .font_weight(FontWeight::Number(500.0))
                                        .color(token::popover_foreground().var()),
                                )
                                .bind_text((series.text)(row, self.format.as_ref())),
                        );
                }
                div()
                    .css(
                        Style::new()
                            .position(Position::Absolute)
                            .top(px(0))
                            .bottom(px(0))
                            .group()
                            .on(
                                PseudoClass::Hover,
                                // A translucent shadow over the column, since a fill on this
                                // layer, which lies above the drawing, would hide the bars.
                                Style::new().box_shadow(raw(
                                    "inset 0 0 0 100vmax color-mix(in srgb, currentColor 8%, transparent)",
                                )),
                            ),
                    )
                    .bind_style(Property::Left, left)
                    .bind_style(Property::Width, band.clone().to_fixed(3).concat("%"))
                    .child(
                        div()
                            .css(tooltip_style())
                            .child(
                                div()
                                    .css(
                                        Style::new()
                                            .font_weight(FontWeight::Number(500.0))
                                            .color(token::popover_foreground().var()),
                                    )
                                    .bind_text(row.at(category)),
                            )
                            .child(values),
                    )
            }))
    }

    /// The category of each row, under the middle of its column.
    fn x_labels(&self, band: Expr<S, f64>) -> El<S> {
        let category = self.category;
        div()
            .css(Style::new().position(Position::Relative).height(rem(1)))
            .child(each(self.rows, |row| {
                let middle = ((row.index().to_f64() + 0.5) * band)
                    .to_fixed(3)
                    .concat("%");
                span()
                    .css(
                        Style::new()
                            .position(Position::Absolute)
                            .translate(raw("-50% 0"))
                            .white_space(WhiteSpace::Nowrap),
                    )
                    .bind_style(Property::Left, middle)
                    .bind_text(row.at(category))
            }))
    }

    /// A swatch and the name of each series.
    fn legend(&self) -> El<S> {
        ul().attr("aria-hidden", "true")
            .css(
                Style::new()
                    .display(Display::Flex)
                    .flex_wrap(FlexWrap::Wrap)
                    .justify_content(JustifyContent::Center)
                    .column_gap(rem(1))
                    .margin(px(0))
                    .padding(px(0))
                    .list_style_type(raw("none"))
                    .merge(look::small_text()),
            )
            .children(self.series.iter().map(|series| {
                li().css(
                    Style::new()
                        .display(Display::Flex)
                        .align_items(AlignItems::Center)
                        .column_gap(rem(0.5)),
                )
                .child(swatch(&series.color))
                .child(span().text(series.name.clone()))
            }))
    }

    /// The rows as a table that only a screen reader reads.
    fn data_table(&self) -> El<S> {
        let mut head = tr().child(th().attr("scope", "col"));
        for series in &self.series {
            head = head.child(th().attr("scope", "col").text(series.name.clone()));
        }
        let category = self.category;
        let series = &self.series;
        table()
            .css(visually_hidden())
            .child(el("caption").text(self.label.clone()))
            .child(thead().child(head))
            .child(tbody().child(each(self.rows, |row| {
                let mut line = tr().child(th().attr("scope", "row").bind_text(row.at(category)));
                for series in series {
                    line = line.child(td().bind_text((series.text)(row, self.format.as_ref())));
                }
                line
            })))
    }
}

impl<S, T> fmt::Debug for Chart<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chart")
            .field("kind", &self.kind)
            .field("label", &self.label)
            .field("rows", &self.rows)
            .field("category", &self.category)
            .field("format", &self.format)
            .field(
                "series",
                &self
                    .series
                    .iter()
                    .map(|series| &series.name)
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

impl<S: 'static, T: 'static> From<Chart<S, T>> for El<S> {
    fn from(chart: Chart<S, T>) -> Self {
        chart.render()
    }
}

impl<S: 'static, T: 'static> From<Chart<S, T>> for Node<S> {
    fn from(chart: Chart<S, T>) -> Self {
        chart.render().into()
    }
}

/// The distance from the top of the drawing to `value`, in hundredths of
/// its height, on the scale from `lo` to `hi`.
fn y_of<S>(value: Expr<S, f64>, lo: Expr<S, f64>, hi: Expr<S, f64>) -> Expr<S, f64> {
    (hi.clone() - value) / (hi - lo) * 100.0
}

/// `value` rounded to nine decimal places, which removes the error of a
/// multiplication such as `3 * 0.1`.
fn clean<S>(value: Expr<S, f64>) -> Expr<S, f64> {
    (value * 1e9).round() / 1e9
}

/// A small square in `color`.
fn swatch<S>(color: &Token<Color>) -> El<S> {
    span().css(
        Style::new()
            .display(Display::InlineBlock)
            .width(rem(0.625))
            .height(rem(0.625))
            .border_radius(px(2))
            .background_color(color.var()),
    )
}

/// The box of a row's values, shown while the pointer is on its column.
fn tooltip_style() -> Style {
    Style::new()
        .display(Display::None)
        .position(Position::Absolute)
        .top(rem(0.5))
        .left(pct(50))
        .translate(raw("-50% 0"))
        .z_index(ZIndex::Integer(10))
        .min_width(rem(8))
        .padding_block(rem(0.375))
        .padding_inline(rem(0.625))
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(rem(0.5))
        .background_color(token::popover().var())
        .box_shadow(raw("0 4px 12px rgb(0 0 0 / 0.1)"))
        .white_space(WhiteSpace::Nowrap)
        .pointer_events(PointerEvents::None)
        .on(
            Condition::Group(PseudoClass::Hover),
            Style::new().display(Display::Flex),
        )
        .flex_direction(FlexDirection::Column)
        .row_gap(rem(0.25))
        .opacity(Opacity::Number(1.0))
        .color(Color::CURRENT_COLOR)
}

/// Hides an element from sight and keeps it in the accessibility tree.
fn visually_hidden() -> Style {
    Style::new()
        .position(Position::Absolute)
        .width(px(1))
        .height(px(1))
        .overflow(Overflow::Hidden)
        .clip_path(raw("inset(50%)"))
        .white_space(WhiteSpace::Nowrap)
}

#[cfg(test)]
mod tests {
    use hypersig::html::span;
    use hypersig::{Page, Signals};
    use serde::{Deserialize, Serialize};

    use super::Chart;
    use crate::token;

    #[derive(Signals, Serialize, Deserialize)]
    struct Point {
        name: String,
        value: f64,
    }

    #[derive(Signals, Serialize, Deserialize)]
    struct Plot {
        points: Vec<Point>,
    }

    /// The low end, the high end, and the step of the scale the first HTML
    /// shows for `values`.
    fn fit(values: &[f64]) -> String {
        let chart = Chart::bar("Points", Plot::points(), Point::name()).series(
            "Value",
            Point::value(),
            token::chart_1(),
        );
        let text = chart.scaled(|lo, hi, step| {
            lo.to_text()
                .concat(" ")
                .concat(hi.to_text())
                .concat(" ")
                .concat(step.to_text())
        });
        let points = values
            .iter()
            .map(|&value| Point {
                name: String::new(),
                value,
            })
            .collect();
        let html = Page::new("Points", Plot { points })
            .child(span().bind_text(text))
            .render();
        let end = html.find("</span>").expect("the span is rendered");
        let start = html[..end].rfind('>').expect("the span has a start tag") + 1;
        html[start..end].to_owned()
    }

    #[test]
    fn rounds_out_to_a_step_of_one_two_two_and_a_half_or_five() {
        assert_eq!(fit(&[186.0, 305.0, 237.0]), "0 400 100");
        assert_eq!(fit(&[820.0]), "0 1000 250");
        assert_eq!(fit(&[7.0]), "0 8 2");
        assert_eq!(fit(&[400.0]), "0 400 100");
        assert_eq!(fit(&[1000.0]), "0 1000 250");
    }

    #[test]
    fn keeps_small_steps_free_of_float_error() {
        assert_eq!(fit(&[0.3]), "0 0.3 0.1");
    }

    #[test]
    fn reaches_below_zero_for_a_negative_value() {
        assert_eq!(fit(&[120.0, -45.0]), "-50 150 50");
        assert_eq!(fit(&[-7.0, -2.0]), "-8 0 2");
    }

    #[test]
    fn runs_from_zero_to_one_without_a_value_other_than_zero() {
        assert_eq!(fit(&[]), "0 1 0.25");
        assert_eq!(fit(&[0.0, 0.0]), "0 1 0.25");
    }
}

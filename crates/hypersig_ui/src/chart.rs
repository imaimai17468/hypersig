//! Charts: bars, lines, or areas drawn from the rows of a list signal, on a
//! scale the server fits to the values.

use std::fmt;

use hypersig::html::{El, Node, div, each, el, li, span, table, tbody, td, th, thead, tr, ul};
use hypersig::style::value::{
    AlignItems, BorderStyle, Color, Display, FontVariantNumeric, FontWeight, Opacity, Overflow,
    PointerEvents, Position, TextAlign, WhiteSpace, ZIndex, pct, px, raw, rem,
};
use hypersig::style::{Condition, Property, PseudoClass, Style, Token};
use hypersig::{Expr, IntoExpr, Item, Number, NumberFormat, Sig, SignalPatch, Signals, Textual};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::look;
use crate::token;

/// The vertical scale of a [`Chart`]: 0 to a round maximum, and its grid lines.
///
/// Only [`ChartScale::fit`] and [`Chart::scale`] build one, so the maximum
/// and the marks always come from the values they were fitted to. Put one in
/// the page's signals beside the rows, and patch both with [`Chart::patch`].
///
/// ```
/// use hypersig_ui::ChartScale;
///
/// let scale = ChartScale::fit([186.0, 305.0, 237.0]);
/// assert_eq!(scale, ChartScale::fit([400.0]));
/// ```
#[derive(Signals, Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ChartScale {
    max: f64,
    ticks: Vec<f64>,
}

impl ChartScale {
    /// The scale from 0 to a round number at or above the largest of `values`.
    ///
    /// The grid lines fall on a step of 1, 2, 2.5, or 5 times a power of
    /// ten, chosen to give about four of them. Values that are negative or
    /// not finite are left out, and with no positive value the scale runs
    /// from 0 to 1.
    #[must_use]
    pub fn fit(values: impl IntoIterator<Item = f64>) -> Self {
        let largest = values
            .into_iter()
            .filter(|value| value.is_finite())
            .fold(0.0, f64::max);
        let top = if largest > 0.0 { largest } else { 1.0 };
        let rough = top / 4.0;
        let magnitude = 10_f64.powf(rough.log10().floor());
        let step = magnitude
            * match rough / magnitude {
                n if n <= 1.0 => 1.0,
                n if n <= 2.0 => 2.0,
                n if n <= 2.5 => 2.5,
                n if n <= 5.0 => 5.0,
                _ => 10.0,
            };
        let max = (top / step).ceil() * step;
        let mut ticks = Vec::new();
        let mut count = 0.0;
        loop {
            let tick = count * step;
            if tick > max + step / 2.0 {
                break;
            }
            // Rounds away the error of the multiplication, as in 3 * 0.1.
            ticks.push((tick * 1e9).round() / 1e9);
            count += 1.0;
        }
        Self {
            max: (max * 1e9).round() / 1e9,
            ticks,
        }
    }
}

impl Default for ChartScale {
    fn default() -> Self {
        Self::fit([])
    }
}

/// How a [`Chart`] draws its series.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Bar,
    Line,
    Area,
}

/// One series of a chart: its name, its color, where its value lies in a
/// row, and how a row's value reads in the browser.
struct Series<S, T> {
    name: String,
    color: Token<Color>,
    path: String,
    value: RowValue<S, T>,
    text: RowText<S, T>,
}

/// A row's value of one series, as a number for drawing.
type RowValue<S, T> = Box<dyn Fn(Expr<S, T>) -> Expr<S, f64>>;

/// How a row's value of one series reads as text, in the format where there is one.
type RowText<S, T> = Box<dyn Fn(Item<S, T>, Option<&NumberFormat>) -> Expr<S, String>>;

/// A bar, line, or area chart of the rows of a list signal.
///
/// Each row is one point along the horizontal axis, labeled by the text
/// field `category`, and each [`Chart::series`] reads a number field of the
/// row. The rows and the [`ChartScale`] are signals, so a patch from the
/// server or a statement in the browser that changes a value moves its bar
/// or its line. The scale changes only with a patch from [`Chart::patch`],
/// because the browser has no way to find a round maximum. A value above
/// the scale's maximum is drawn at the maximum, and a negative one at 0.
///
/// The drawing, the axis labels, the legend, and the box that shows a row's
/// values under the pointer are hidden from assistive technology. A screen
/// reader reads a visually hidden table of the same rows instead, captioned
/// with `label`.
///
/// ```
/// use hypersig::{Page, Signals};
/// use hypersig_ui::{Chart, ChartScale, token};
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
///     scale: ChartScale,
/// }
///
/// fn chart() -> Chart<Traffic, Month> {
///     Chart::bar("Visitors", Traffic::months(), Traffic::scale(), Month::name())
///         .series("Desktop", Month::desktop(), token::chart_1())
///         .series("Mobile", Month::mobile(), token::chart_2())
/// }
///
/// let months = vec![
///     Month { name: "January".into(), desktop: 186, mobile: 80 },
///     Month { name: "February".into(), desktop: 305, mobile: 200 },
/// ];
/// let scale = chart().scale(&months);
/// assert_eq!(scale, ChartScale::fit([400.0]));
/// let html = Page::new("Traffic", Traffic { months, scale }).child(chart()).render();
/// assert!(html.contains("<caption>Visitors</caption>"));
/// ```
pub struct Chart<S, T> {
    kind: Kind,
    label: String,
    rows: Sig<S, Vec<T>>,
    scale: Sig<S, ChartScale>,
    category: Sig<T, String>,
    series: Vec<Series<S, T>>,
    format: Option<NumberFormat>,
}

impl<S: 'static, T: 'static> Chart<S, T> {
    /// A bar chart named `label` of `rows` on `scale`, labeled by `category`.
    ///
    /// Each row has a group of bars, one per series.
    #[must_use]
    pub fn bar(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>>,
        scale: Sig<S, ChartScale>,
        category: Sig<T, String>,
    ) -> Self {
        Self::new(Kind::Bar, label.into(), rows, scale, category)
    }

    /// A line chart named `label` of `rows` on `scale`, labeled by `category`.
    ///
    /// Each row is a point on the line of each series.
    #[must_use]
    pub fn line(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>>,
        scale: Sig<S, ChartScale>,
        category: Sig<T, String>,
    ) -> Self {
        Self::new(Kind::Line, label.into(), rows, scale, category)
    }

    /// An area chart named `label` of `rows` on `scale`, labeled by `category`.
    ///
    /// It is a line chart with the area under each line filled.
    #[must_use]
    pub fn area(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>>,
        scale: Sig<S, ChartScale>,
        category: Sig<T, String>,
    ) -> Self {
        Self::new(Kind::Area, label.into(), rows, scale, category)
    }

    fn new(
        kind: Kind,
        label: String,
        rows: Sig<S, Vec<T>>,
        scale: Sig<S, ChartScale>,
        category: Sig<T, String>,
    ) -> Self {
        Self {
            kind,
            label,
            rows,
            scale,
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
            path: field.path(),
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

    /// The scale fitted to the values of every series in `rows`.
    ///
    /// # Panics
    ///
    /// Panics if a row fails to serialize to JSON, which only happens for a
    /// `Serialize` impl that returns an error.
    #[must_use]
    pub fn scale(&self, rows: &[T]) -> ChartScale
    where
        T: Serialize,
    {
        let values = rows.iter().flat_map(|row| {
            let row = serde_json::to_value(row).expect("a chart row serializes to JSON");
            self.series
                .iter()
                .filter_map(|series| field(&row, &series.path).and_then(Value::as_f64))
                .collect::<Vec<_>>()
        });
        ChartScale::fit(values)
    }

    /// A patch that sets the rows to `rows` and the scale fitted to them.
    ///
    /// # Panics
    ///
    /// Panics if a row fails to serialize to JSON, as [`Chart::scale`] does.
    #[must_use]
    pub fn patch(&self, rows: Vec<T>) -> SignalPatch<S>
    where
        T: Serialize,
    {
        let scale = self.scale(&rows);
        self.rows.patch(rows).and(self.scale.patch(scale))
    }

    /// The chart, its legend, and the table a screen reader reads, in a `<figure>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let max = self.scale.at(ChartScale::max());
        let ticks = self.scale.at(ChartScale::ticks());
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
            .child(each(ticks, |tick| {
                let y = height_of(tick.value().into_expr(), max).to_fixed(3);
                el("line")
                    .attr("x1", "0")
                    .attr("x2", "100")
                    .attr("vector-effect", "non-scaling-stroke")
                    .css(
                        Style::new()
                            .stroke(token::border().var())
                            .stroke_width(px(1)),
                    )
                    .bind_attr("y1", y.clone())
                    .bind_attr("y2", y)
            }))
            .children(self.marks(max, &band));
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
            .child(y_labels(ticks, max, self.format))
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
                    .flex_direction(hypersig::style::value::FlexDirection::Column)
                    .row_gap(rem(1))
                    .margin(px(0)),
            )
            .child(visual)
            .child(self.legend())
            .child(self.data_table())
    }

    /// The bars, or the lines and areas, of every series.
    fn marks(&self, max: Sig<S, f64>, band: &Expr<S, f64>) -> Vec<Node<S>> {
        let count: f64 = self.series.iter().map(|_| 1.0).sum();
        let mut marks = Vec::with_capacity(self.series.len());
        let mut position = 0.0;
        for series in &self.series {
            let color = series.color.clone();
            let band = band.clone();
            let rows = self.rows;
            let kind = self.kind;
            let mark = each(rows, |row| {
                let value = (series.value)(row.into_expr());
                let x = row.index().to_f64() * band.clone();
                match kind {
                    Kind::Bar => {
                        let width = band.clone() * (0.8 / count);
                        let left = x + band.clone() * 0.1 + width.clone() * position;
                        let top = height_of(value, max);
                        el("rect")
                            .css(Style::new().fill(color.var()))
                            .bind_attr("x", left.to_fixed(3))
                            .bind_attr("width", width.to_fixed(3))
                            .bind_attr("y", top.clone().to_fixed(3))
                            .bind_attr("height", (Expr::lit(&100.0) - top).to_fixed(3))
                    }
                    Kind::Line | Kind::Area => {
                        let next = rows.nth(row.index() + 1).unwrap_or(row.value());
                        let next_value = (series.value)(next);
                        let x1 = x + band.clone() * 0.5;
                        let x2 = x1.clone() + band.clone();
                        let y1 = height_of(value, max).to_fixed(3);
                        let y2 = height_of(next_value, max).to_fixed(3);
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
                            let points = x1
                                .clone()
                                .to_fixed(3)
                                .concat(",100 ")
                                .concat(x1.to_fixed(3))
                                .concat(",")
                                .concat(y1)
                                .concat(" ")
                                .concat(x2.clone().to_fixed(3))
                                .concat(",")
                                .concat(y2)
                                .concat(" ")
                                .concat(x2.to_fixed(3))
                                .concat(",100");
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
                    .flex_wrap(hypersig::style::value::FlexWrap::Wrap)
                    .justify_content(hypersig::style::value::JustifyContent::Center)
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
            .field("scale", &self.scale)
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

/// The value at the dotted `path` inside `row`.
fn field<'a>(row: &'a Value, path: &str) -> Option<&'a Value> {
    path.split('.').try_fold(row, |value, key| value.get(key))
}

/// The distance from the top of the drawing to `value`, in hundredths of
/// its height, with the value held between 0 and `max`.
fn height_of<S>(value: Expr<S, f64>, max: Sig<S, f64>) -> Expr<S, f64> {
    Expr::lit(&100.0) - (value.max(0.0) / max).min(1.0) * 100.0
}

/// The labels of the grid lines, each beside its line.
fn y_labels<S>(ticks: Sig<S, Vec<f64>>, max: Sig<S, f64>, format: Option<NumberFormat>) -> El<S> {
    div()
        .css(
            Style::new()
                .position(Position::Relative)
                .min_width(rem(1.5))
                .text_align(TextAlign::End),
        )
        .child(each(ticks, |tick| {
            let top = height_of(tick.value().into_expr(), max)
                .to_fixed(3)
                .concat("%");
            span()
                .css(
                    Style::new()
                        .position(Position::Absolute)
                        .right(px(0))
                        .translate(raw("0 -50%"))
                        .white_space(WhiteSpace::Nowrap),
                )
                .bind_style(Property::Top, top)
                .bind_text(match &format {
                    Some(format) => tick.value().format(format),
                    None => tick.value().to_text(),
                })
        }))
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
        .flex_direction(hypersig::style::value::FlexDirection::Column)
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
    use super::ChartScale;

    fn fit(values: &[f64]) -> (f64, Vec<f64>) {
        let scale = ChartScale::fit(values.iter().copied());
        (scale.max, scale.ticks)
    }

    #[test]
    fn rounds_up_to_a_step_of_one_two_two_and_a_half_or_five() {
        assert_eq!(
            fit(&[305.0]),
            (400.0, vec![0.0, 100.0, 200.0, 300.0, 400.0])
        );
        assert_eq!(
            fit(&[820.0]),
            (1000.0, vec![0.0, 250.0, 500.0, 750.0, 1000.0])
        );
        assert_eq!(fit(&[7.0]), (8.0, vec![0.0, 2.0, 4.0, 6.0, 8.0]));
        assert_eq!(
            fit(&[400.0]),
            (400.0, vec![0.0, 100.0, 200.0, 300.0, 400.0])
        );
    }

    #[test]
    fn keeps_small_steps_free_of_float_error() {
        assert_eq!(fit(&[0.3]), (0.3, vec![0.0, 0.1, 0.2, 0.3]));
    }

    #[test]
    fn runs_to_one_without_a_positive_finite_value() {
        let unit = (1.0, vec![0.0, 0.25, 0.5, 0.75, 1.0]);
        assert_eq!(fit(&[]), unit);
        assert_eq!(fit(&[0.0, -5.0]), unit);
        assert_eq!(fit(&[f64::NAN, f64::INFINITY]), unit);
        assert_eq!(ChartScale::default(), ChartScale::fit([]));
    }
}

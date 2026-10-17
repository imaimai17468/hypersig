//! Donut charts: each row's share of a total, drawn as an arc of a ring.

use std::fmt;

use hypersig::html::{El, Node, div, each, el, li, span, table, tbody, td, th, thead, tr, ul};
use hypersig::style::value::{
    AlignItems, Color, Display, FlexDirection, FontVariantNumeric, FontWeight, JustifyContent,
    Position, px, raw, rem,
};
use hypersig::style::{Style, Token};
use hypersig::{Expr, Item, Number, NumberFormat, Sig, Textual, if_else};

use crate::chart::visually_hidden;
use crate::look;
use crate::token;

/// How a row's value reads, as a number for drawing.
type RowValue<S, T> = Box<dyn Fn(Item<S, T>) -> Expr<S, f64>>;

/// How a row's value reads as text, in the format where there is one.
type RowText<S, T> = Box<dyn Fn(Item<S, T>, Option<&NumberFormat>) -> Expr<S, String>>;

/// The colors of the slices, which repeat from the first after the fifth row.
fn palette() -> [Token<Color>; 5] {
    [
        token::chart_1(),
        token::chart_2(),
        token::chart_3(),
        token::chart_4(),
        token::chart_5(),
    ]
}

/// A donut chart of the rows of a list signal, each an arc as long as its share of the total.
///
/// Each row is a slice labeled by the text field `category`, and its
/// length is the number field `value` over the sum of `value` across the
/// rows. The slices run clockwise from the top in the order of the rows,
/// in the chart colors from [`token::chart_1`] to [`token::chart_5`],
/// which repeat after the fifth row. A negative value counts as 0, since a
/// share cannot be negative. The total is written in the middle of the
/// ring, and the legend names each row with its value.
///
/// Each slice sums the rows before it, so a change to the rows costs time
/// that grows with the square of their number.
///
/// The ring and the legend are hidden from assistive technology, and a
/// screen reader reads a visually hidden table of the same rows instead,
/// captioned with `label`.
///
/// ```
/// use hypersig::{Page, Signals};
/// use hypersig_ui::DonutChart;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Browser {
///     name: String,
///     visitors: i32,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Traffic {
///     browsers: Vec<Browser>,
/// }
///
/// let chart = DonutChart::new("Browsers", Traffic::browsers(), Browser::name(), Browser::visitors());
/// let browsers = vec![
///     Browser { name: "Chrome".into(), visitors: 275 },
///     Browser { name: "Safari".into(), visitors: 125 },
/// ];
/// let html = Page::new("Traffic", Traffic { browsers }).child(chart).render();
/// assert!(html.contains(">400</span>"));
/// assert!(html.contains(r#"stroke-dasharray="68.750 31.250""#));
/// ```
pub struct DonutChart<S, T> {
    label: String,
    rows: Sig<S, Vec<T>>,
    category: Sig<T, String>,
    value: RowValue<S, T>,
    text: RowText<S, T>,
    format: Option<NumberFormat>,
}

impl<S: 'static, T: 'static> DonutChart<S, T> {
    /// A donut chart named `label` of `rows`, with slices of `value` labeled by `category`.
    #[must_use]
    pub fn new<U: Number + Textual + 'static>(
        label: impl Into<String>,
        rows: Sig<S, Vec<T>>,
        category: Sig<T, String>,
        value: Sig<T, U>,
    ) -> Self {
        Self {
            label: label.into(),
            rows,
            category,
            value: Box::new(move |row| row.at(value).to_f64()),
            text: Box::new(move |row, format| match format {
                Some(format) => row.at(value).format(format),
                None => row.at(value).to_text(),
            }),
            format: None,
        }
    }

    /// Writes the values and the total in `format`.
    ///
    /// Until set, they are written as JavaScript's `String` writes a number.
    #[must_use]
    pub fn format(mut self, format: NumberFormat) -> Self {
        self.format = Some(format);
        self
    }

    /// The ring, its total, its legend, and the table a screen reader reads, in a `<figure>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let ring = el("svg")
            .attr("viewBox", "0 0 100 100")
            .attr("width", "100%")
            .attr("height", "100%")
            .css(Style::new().display(Display::Block).rotate(raw("-90deg")))
            .child(
                el("circle")
                    .attr("cx", "50")
                    .attr("cy", "50")
                    .attr("r", "40")
                    .attr("fill", "none")
                    .attr("stroke-width", "16")
                    .css(Style::new().stroke(token::muted().var())),
            )
            .child(each(self.rows, |row| self.slice(row)));
        let total = self.total();
        let total = match &self.format {
            Some(format) => total.format(format),
            None => total.to_text(),
        };
        let middle = div()
            .css(
                Style::new()
                    .position(Position::Absolute)
                    .inset(px(0))
                    .display(Display::Flex)
                    .align_items(AlignItems::Center)
                    .justify_content(JustifyContent::Center)
                    .font_size(rem(1.5))
                    .font_weight(FontWeight::Number(600.0))
                    .font_variant_numeric(FontVariantNumeric::TabularNums),
            )
            .child(span().bind_text(total));
        let visual = div()
            .attr("aria-hidden", "true")
            .css(
                Style::new()
                    .position(Position::Relative)
                    .width(rem(12))
                    .height(rem(12))
                    .margin_inline(raw("auto")),
            )
            .child(ring)
            .child(middle);
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

    /// The sum of the values, each held at 0 or above.
    fn total(&self) -> Expr<S, f64> {
        let value = &self.value;
        self.rows.sum_of(|row| value(row).max(0.0))
    }

    /// The arc of `row`: as long as its share of the total, and starting where
    /// the rows before it end.
    fn slice(&self, row: Item<S, T>) -> El<S> {
        let value = &self.value;
        let length = self
            .total()
            .let_in(|total| value(row).max(0.0) / total * 100.0)
            .let_in(|length| {
                length
                    .clone()
                    .to_fixed(3)
                    .concat(" ")
                    .concat((Expr::lit(&100.0) - length).to_fixed(3))
            });
        let before = self.rows.sum_of(|other| {
            if_else(
                other.index().lt(row.index()),
                value(other).max(0.0),
                Expr::lit(&0.0),
            )
        });
        // The ring is turned a quarter back, so the path starts at the top.
        let offset = self
            .total()
            .let_in(|total| Expr::lit(&0.0) - before / total * 100.0);
        let mut arc = el("circle")
            .attr("cx", "50")
            .attr("cy", "50")
            .attr("r", "40")
            .attr("fill", "none")
            .attr("stroke-width", "16")
            .attr("pathLength", "100")
            .show(self.total().gt(0.0))
            .bind_attr("stroke-dasharray", length)
            .bind_attr("stroke-dashoffset", offset.to_fixed(3));
        for (index, color) in (0..).zip(palette()) {
            arc = arc.css_when(
                Style::new().stroke(color.var()),
                (row.index() % 5).equals(index),
            );
        }
        arc
    }

    /// A swatch, the category, and the value of each row.
    fn legend(&self) -> El<S> {
        let category = self.category;
        ul().attr("aria-hidden", "true")
            .css(
                Style::new()
                    .display(Display::Grid)
                    .grid_template_columns(raw("auto 1fr auto"))
                    .column_gap(rem(0.5))
                    .row_gap(rem(0.25))
                    .align_items(AlignItems::Center)
                    .margin(px(0))
                    .padding(px(0))
                    .list_style_type(raw("none"))
                    .merge(look::small_text()),
            )
            .child(each(self.rows, |row| {
                li().css(Style::new().display(Display::Contents))
                    .child(swatch_for(row))
                    .child(span().bind_text(row.at(category)))
                    .child(
                        span()
                            .css(
                                Style::new()
                                    .font_variant_numeric(FontVariantNumeric::TabularNums)
                                    .color(token::muted_foreground().var()),
                            )
                            .bind_text((self.text)(row, self.format.as_ref())),
                    )
            }))
    }

    /// The rows as a table that only a screen reader reads.
    fn data_table(&self) -> El<S> {
        let category = self.category;
        table()
            .css(visually_hidden())
            .child(el("caption").text(self.label.clone()))
            .child(
                thead().child(
                    tr().child(th().attr("scope", "col"))
                        .child(th().attr("scope", "col").text(self.label.clone())),
                ),
            )
            .child(tbody().child(each(self.rows, |row| {
                tr().child(th().attr("scope", "row").bind_text(row.at(category)))
                    .child(td().bind_text((self.text)(row, self.format.as_ref())))
            })))
    }
}

/// A small square in the color of the slice of `row`.
fn swatch_for<S, T>(row: Item<S, T>) -> El<S> {
    let mut mark = span().css(
        Style::new()
            .display(Display::InlineBlock)
            .width(rem(0.625))
            .height(rem(0.625))
            .border_radius(px(2)),
    );
    for (index, color) in (0..).zip(palette()) {
        mark = mark.css_when(
            Style::new().background_color(color.var()),
            (row.index() % 5).equals(index),
        );
    }
    mark
}

impl<S, T> fmt::Debug for DonutChart<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DonutChart")
            .field("label", &self.label)
            .field("rows", &self.rows)
            .field("category", &self.category)
            .field("format", &self.format)
            .finish_non_exhaustive()
    }
}

impl<S: 'static, T: 'static> From<DonutChart<S, T>> for El<S> {
    fn from(chart: DonutChart<S, T>) -> Self {
        chart.render()
    }
}

impl<S: 'static, T: 'static> From<DonutChart<S, T>> for Node<S> {
    fn from(chart: DonutChart<S, T>) -> Self {
        chart.render().into()
    }
}

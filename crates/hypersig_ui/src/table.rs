//! Data tables: rows of a list signal under typed columns, with sorting,
//! filtering, pagination, and row selection in the browser.

use std::fmt;

use hypersig::html::{
    El, Node, button, div, each_keyed, input, p, span, table, tbody, td, th, thead, tr,
};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    AlignItems, BorderBottomStyle, BorderCollapse, BorderStyle, CaptionSide, Color, Cursor,
    Display, FontVariantNumeric, FontWeight, JustifyContent, OverflowX, TextAlign, VerticalAlign,
    WhiteSpace, pct, px, rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Comparable, Expr, IntoExpr, Item, Number, Sig, Signals, Stmt, Textual, if_else};
use serde::{Deserialize, Serialize};

use crate::button::{ButtonSize, ButtonVariant, button_style, icon_button};
use crate::choice::{check_box, check_box_checked, check_control};
use crate::field::field_style;
use crate::look::{self, icon};
use crate::token;

/// The signals a [`DataTable`] keeps: its filter text, page, and sort column.
///
/// Put one in the page's signals for each table.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::TableState;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Users {
///     table: TableState,
/// }
///
/// let users = Users { table: TableState::default() };
/// assert_eq!(users.table.page, 0);
/// ```
#[derive(Signals, Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
pub struct TableState {
    /// What the filter field holds.
    pub query: String,
    /// The page shown, counted from 0.
    pub page: i32,
    /// 0 while unsorted, `n` while sorted ascending by the `n`th column
    /// counted from 1, and `-n` while sorted descending by it.
    sort: i32,
}

/// One column of a [`DataTable`] whose rows are `T`.
///
/// A column shows a field of the row with [`Column::text`] or
/// [`Column::number`], or any element with [`Column::custom`]. A column
/// sorts only after [`Column::sort_by`] names its key, which the compiler
/// requires to be a number or a string, because the runtime orders only
/// those.
///
/// ```
/// use hypersig::Signals;
/// use hypersig_ui::Column;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct User {
///     id: i32,
///     email: String,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Users {
///     users: Vec<User>,
/// }
///
/// let email: Column<Users, User> =
///     Column::text("Email", User::email()).sort_by(|user| user.at(User::email()));
/// # let _ = email;
/// ```
pub struct Column<S, T> {
    header: String,
    numeric: bool,
    cell: Box<dyn Fn(Item<S, T>) -> El<S>>,
    sort: Option<Sorter<S, T>>,
    visible: Option<Expr<S, bool>>,
}

/// Builds the statement that sorts the rows, descending when the `bool` is true.
type Sorter<S, T> = Box<dyn Fn(Sig<S, Vec<T>>, bool) -> Stmt<S>>;

/// Renders the rows with the row template it receives.
type Each<S, T> = Box<dyn FnOnce(Box<dyn FnOnce(Item<S, T>) -> El<S>>) -> Node<S>>;

/// The text of a row that the filter searches.
type RowText<S, T> = Box<dyn Fn(Item<S, T>) -> Expr<S, String>>;

impl<S: 'static, T: 'static> Column<S, T> {
    /// A column headed `header` that shows the text field `field` of each row.
    #[must_use]
    pub fn text(header: impl Into<String>, field: Sig<T, String>) -> Self {
        Self::new(header.into(), false, move |row| {
            span().bind_text(row.at(field))
        })
    }

    /// A column headed `header` that shows the number field `field`, aligned to the end.
    #[must_use]
    pub fn number<U: Number + Textual + 'static>(
        header: impl Into<String>,
        field: Sig<T, U>,
    ) -> Self {
        Self::new(header.into(), true, move |row| {
            span().bind_text(row.at(field).to_text())
        })
    }

    /// A column headed `header` whose cells hold what `render` returns for each row.
    #[must_use]
    pub fn custom(
        header: impl Into<String>,
        render: impl Fn(Item<S, T>) -> El<S> + 'static,
    ) -> Self {
        Self::new(header.into(), false, render)
    }

    fn new(header: String, numeric: bool, cell: impl Fn(Item<S, T>) -> El<S> + 'static) -> Self {
        Self {
            header,
            numeric,
            cell: Box::new(cell),
            sort: None,
            visible: None,
        }
    }

    /// Makes the header a button that sorts the rows by `key`.
    ///
    /// The first press sorts them ascending, and a press on a column sorted
    /// ascending sorts it descending.
    /// Sorting reorders the list signal itself, so the order holds until the
    /// list changes. A list the server patches afterwards arrives in the
    /// server's order, while the header still marks the last sort.
    #[must_use]
    pub fn sort_by<K: Comparable, E: IntoExpr<S, K>>(
        mut self,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self {
        self.sort = Some(Box::new(move |rows, descending| {
            if descending {
                rows.sort_by_desc(&key)
            } else {
                rows.sort_by(&key)
            }
        }));
        self
    }

    /// Shows the column only while `when` is true.
    #[must_use]
    pub fn visible(mut self, when: impl IntoExpr<S, bool>) -> Self {
        self.visible = Some(when.into_expr());
        self
    }
}

impl<S, T> fmt::Debug for Column<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Column")
            .field("header", &self.header)
            .field("numeric", &self.numeric)
            .field("sortable", &self.sort.is_some())
            .field("visible", &self.visible)
            .finish_non_exhaustive()
    }
}

/// A table of the rows of a list signal, under typed [`Column`]s.
///
/// Each row is keyed by `key`, as in
/// [`each_keyed`](hypersig::html::each_keyed), so a row keeps its element,
/// and its checkbox its focus, when sorting moves it. Everything the table
/// does runs in the browser on the list signal and the [`TableState`]:
///
/// - A column with [`Column::sort_by`] has a button in its header that
///   sorts the rows, and the header's `aria-sort` says which way.
/// - [`DataTable::filter`] adds a text field and shows the rows whose text
///   contains what the visitor types, ignoring case.
/// - [`DataTable::paginate`] shows a fixed number of the rows that pass the
///   filter, with buttons for the previous and the next page.
/// - [`DataTable::selectable`] adds a column of checkboxes bound to a field
///   of each row, under a header checkbox that selects or clears all of
///   them. The header follows the WAI-ARIA checkbox pattern with
///   `aria-checked="mixed"` while some rows are selected.
///
/// A page past the last row, which the list reaches by shrinking from
/// outside the table, returns the table to its first page.
///
/// The header checkbox selects every row of the list, including rows the
/// filter or the page hides. Rows are counted on each change of the list,
/// the filter, or the page, and the work grows with the square of the
/// number of rows while a filter and pagination are both on, so a list of
/// thousands of rows is better filtered and paged on the server through a
/// [`Route`](hypersig::Route) with search parameters.
///
/// ```
/// use hypersig::{Page, Signals};
/// use hypersig_ui::{Column, DataTable, TableState};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct User {
///     id: i32,
///     email: String,
///     selected: bool,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Users {
///     users: Vec<User>,
///     table: TableState,
/// }
///
/// let table = DataTable::new("Users", Users::table(), Users::users(), |user| user.at(User::id()))
///     .selectable(User::selected())
///     .filter("Filter emails", |user| user.at(User::email()))
///     .paginate(10)
///     .column(Column::text("Email", User::email()).sort_by(|user| user.at(User::email())))
///     .column(Column::number("Id", User::id()));
/// let users = vec![User { id: 1, email: "ada@example.com".into(), selected: true }];
/// let html = Page::new("Users", Users { users, table: TableState::default() })
///     .child(table)
///     .render();
/// assert!(html.contains(r#"aria-sort="none""#));
/// assert!(html.contains(r#"aria-checked="true""#));
/// ```
pub struct DataTable<S, T> {
    label: String,
    state: Sig<S, TableState>,
    rows: Sig<S, Vec<T>>,
    each: Each<S, T>,
    columns: Vec<Column<S, T>>,
    selected: Option<Sig<T, bool>>,
    filter: Option<(String, RowText<S, T>)>,
    page_size: Option<i32>,
    empty: String,
}

impl<S: 'static, T: 'static> DataTable<S, T> {
    /// An empty table named `label`, listing `rows` by `key`, with its state in `state`.
    ///
    /// The name is for assistive technology, and the table has no columns
    /// until [`DataTable::column`] adds them.
    #[must_use]
    pub fn new<K: 'static, E: IntoExpr<S, K>>(
        label: impl Into<String>,
        state: Sig<S, TableState>,
        rows: Sig<S, Vec<T>>,
        key: impl FnOnce(Item<S, T>) -> E + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            state,
            rows,
            each: Box::new(move |render| each_keyed(rows, key, render)),
            columns: Vec::new(),
            selected: None,
            filter: None,
            page_size: None,
            empty: "No results.".to_owned(),
        }
    }

    /// Adds `column` after the columns added so far.
    #[must_use]
    pub fn column(mut self, column: Column<S, T>) -> Self {
        self.columns.push(column);
        self
    }

    /// Adds a first column of checkboxes bound to the field `selected` of each row.
    #[must_use]
    pub fn selectable(mut self, selected: Sig<T, bool>) -> Self {
        self.selected = Some(selected);
        self
    }

    /// Adds a field named `label` that shows the rows whose `text` contains its text.
    ///
    /// The field sits above the table. The match ignores case, and typing
    /// returns the table to its first page.
    #[must_use]
    pub fn filter<E: IntoExpr<S, String>>(
        mut self,
        label: impl Into<String>,
        text: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self {
        self.filter = Some((label.into(), Box::new(move |row| text(row).into_expr())));
        self
    }

    /// Shows `size` rows at a time, with buttons for the previous and next page.
    ///
    /// Below the table, the range of rows shown is written as `1–5 / 12`.
    ///
    /// # Panics
    ///
    /// Panics if `size` is less than 1.
    #[must_use]
    pub fn paginate(mut self, size: i32) -> Self {
        assert!(size >= 1, "a page holds at least one row");
        self.page_size = Some(size);
        self
    }

    /// Sets the text shown when no row passes the filter.
    ///
    /// It is "No results." until set.
    #[must_use]
    pub fn empty(mut self, text: impl Into<String>) -> Self {
        self.empty = text.into();
        self
    }

    /// The filter field, the table, and the page buttons, in a `<div>`.
    #[must_use]
    pub fn render(self) -> El<S> {
        let Self {
            label,
            state,
            rows,
            each,
            columns,
            selected,
            filter,
            page_size,
            empty,
        } = self;
        let query = state.at(TableState::query());
        let page = state.at(TableState::page());
        let (filter_label, text) = filter.unzip();
        let filtered = text.is_some();
        let matches = move |row: Item<S, T>| -> Expr<S, bool> {
            match &text {
                Some(text) => text(row)
                    .to_lowercase()
                    .contains(query.trim().to_lowercase()),
                None => Expr::lit(&true),
            }
        };
        let shown = if filtered {
            rows.count(&matches)
        } else {
            rows.len()
        };

        let (head, cells) = header_row(columns, rows, selected, state.at(TableState::sort()));
        let width = cells.len() + usize::from(selected.is_some());

        let row_shown = move |row: Item<S, T>| -> Expr<S, bool> {
            let Some(size) = page_size else {
                return matches(row);
            };
            let rank = if filtered {
                rows.count(|other| other.index().lt(row.index()).and(matches(other)))
            } else {
                row.index()
            };
            let start = page * size;
            matches(row)
                .and(rank.clone().ge(start.clone()))
                .and(rank.lt(start + size))
        };
        let body = each(Box::new(move |row| {
            let mut line = tr().show(row_shown(row)).css(body_row());
            if let Some(selected) = selected {
                let on = row.at(selected);
                line = line.css_when(selected_row(), on).child(
                    td().css(select_cell())
                        .child(check_control(input().attr("aria-label", "Select row"), on)),
                );
            }
            for (numeric, cell, visible) in &cells {
                let mut data = td().css(body_cell(*numeric)).child(cell(row));
                if let Some(visible) = visible {
                    data = data.show(visible.clone());
                }
                line = line.child(data);
            }
            line
        }));
        let empty_row = tr().show(shown.clone().equals(0)).child(
            td().attr("colspan", width.to_string())
                .css(empty_cell())
                .text(empty),
        );

        let mut out = div().css(stack(rem(1)));
        if let Some(label) = filter_label {
            out = out.child(filter_field(label, query, page));
        }
        out = out.child(
            div().css(frame()).child(
                table()
                    .attr("aria-label", label)
                    .css(table_style())
                    .child(thead().child(head))
                    .child(tbody().child(body).child(empty_row)),
            ),
        );
        if let Some(size) = page_size {
            // The list can shrink from outside the table, through a patch or
            // a statement of the page, and leave the page past its last row.
            let past_end = page.gt(0).and((page * size).ge(shown.clone()));
            out = out
                .effect(Stmt::when(page.lt(0).or(past_end), page.set(0)))
                .child(pager(page, shown, size));
        }
        out
    }
}

/// The header row, and for each column the parts its cells need.
fn header_row<S: 'static, T: 'static>(
    columns: Vec<Column<S, T>>,
    rows: Sig<S, Vec<T>>,
    selected: Option<Sig<T, bool>>,
    sort: Sig<S, i32>,
) -> (El<S>, Vec<Cell<S, T>>) {
    let mut head = tr();
    if let Some(selected) = selected {
        head = head.child(
            th().attr("scope", "col")
                .css(select_cell())
                .child(select_all(rows, selected)),
        );
    }
    let mut cells = Vec::with_capacity(columns.len());
    let mut code = 0;
    for column in columns {
        code += 1;
        let heading = th().attr("scope", "col").css(head_cell(column.numeric));
        let mut heading = match column.sort {
            Some(sorter) => sort_header(
                heading,
                column.header,
                column.numeric,
                |descending| sorter(rows, descending),
                sort,
                code,
            ),
            None => heading.text(column.header),
        };
        if let Some(visible) = &column.visible {
            heading = heading.show(visible.clone());
        }
        head = head.child(heading);
        cells.push((column.numeric, column.cell, column.visible));
    }
    (head, cells)
}

/// Whether a column is numeric, how it renders a row, and when it shows.
type Cell<S, T> = (
    bool,
    Box<dyn Fn(Item<S, T>) -> El<S>>,
    Option<Expr<S, bool>>,
);

/// The search field named `label` bound to `query`, which returns to the
/// first page as the visitor types.
fn filter_field<S>(label: String, query: Sig<S, String>, page: Sig<S, i32>) -> El<S> {
    input()
        .attr("type", "search")
        .attr("aria-label", label.clone())
        .attr("placeholder", label)
        .css(field_style().max_width(rem(24)))
        .bind(query)
        .on_input(page.set(0))
}

/// `heading` with a button that sorts by the column numbered `code`, which
/// `sorter` sorts ascending or descending, and the `aria-sort` that follows
/// `sort`.
fn sort_header<S>(
    heading: El<S>,
    header: String,
    numeric: bool,
    sorter: impl Fn(bool) -> Stmt<S>,
    sort: Sig<S, i32>,
    code: i32,
) -> El<S> {
    let ascending = sort.equals(code);
    let descending = sort.equals(-code);
    let toggle = Stmt::if_else(
        ascending.clone(),
        sorter(true).then(sort.set(-code)),
        sorter(false).then(sort.set(code)),
    );
    heading
        .bind_attr(
            "aria-sort",
            if_else(
                ascending.clone(),
                "ascending",
                if_else(descending.clone(), "descending", "none"),
            ),
        )
        .child(
            button()
                .attr("type", "button")
                .css(sort_button(numeric))
                .on_click(toggle)
                .child(span().text(header))
                .child(icon(icon::ARROW_UP, rem(1)).show(ascending.clone()))
                .child(icon(icon::ARROW_DOWN, rem(1)).show(descending.clone()))
                .child(
                    icon(icon::ARROW_UP_DOWN, rem(1))
                        .show(!ascending.or(descending))
                        .css(Style::new().color(token::muted_foreground().var())),
                ),
        )
}

impl<S, T> fmt::Debug for DataTable<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTable")
            .field("label", &self.label)
            .field("state", &self.state)
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .field("selected", &self.selected)
            .field("filter", &self.filter.as_ref().map(|(label, _)| label))
            .field("page_size", &self.page_size)
            .finish_non_exhaustive()
    }
}

impl<S: 'static, T: 'static> From<DataTable<S, T>> for El<S> {
    fn from(table: DataTable<S, T>) -> Self {
        table.render()
    }
}

impl<S: 'static, T: 'static> From<DataTable<S, T>> for Node<S> {
    fn from(table: DataTable<S, T>) -> Self {
        table.render().into()
    }
}

/// The header checkbox: checked while every row is selected, mixed while
/// some are, and a press selects every row unless all are selected already.
fn select_all<S, T>(rows: Sig<S, Vec<T>>, selected: Sig<T, bool>) -> El<S> {
    let count = rows.count(|row| row.at(selected));
    let all = count.clone().equals(rows.len()).and(rows.len().gt(0));
    let some = count.gt(0);
    let mark = span()
        .attr("aria-hidden", "true")
        .css(check_box())
        .css_when(check_box_checked(), some.clone())
        .child(icon(icon::CHECK, rem(0.875)).show(all.clone()))
        .child(icon(icon::MINUS, rem(0.875)).show(some.clone().and(!all.clone())));
    button()
        .attr("type", "button")
        .attr("role", "checkbox")
        .attr("aria-label", "Select all rows")
        .bind_attr(
            "aria-checked",
            if_else(all.clone(), "true", if_else(some, "mixed", "false")),
        )
        .on_click(Stmt::if_else(
            all,
            rows.set_all(selected, false),
            rows.set_all(selected, true),
        ))
        .css(
            Style::new()
                .display(Display::InlineFlex)
                .padding(px(0))
                .border_width(px(0))
                .background_color(Color::TRANSPARENT)
                .cursor(Cursor::Pointer)
                .border_radius(px(4))
                .merge(look::focus_ring()),
        )
        .child(mark)
}

/// The range of rows shown and the buttons that move between pages.
fn pager<S>(page: Sig<S, i32>, shown: Expr<S, i32>, size: i32) -> El<S> {
    let start = page * size;
    let end = (start.clone() + size).min(shown.clone());
    let first = if_else(shown.clone().equals(0), Expr::lit(&0), start.clone() + 1);
    let has_next = (start.clone() + size).lt(shown.clone());
    div()
        .css(
            Style::new()
                .display(Display::Flex)
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::End)
                .column_gap(rem(0.5)),
        )
        .child(
            p().css(
                Style::new()
                    .flex_grow(1.0)
                    .color(token::muted_foreground().var())
                    .font_variant_numeric(FontVariantNumeric::TabularNums)
                    .merge(look::small_text()),
            )
            .attr("aria-live", "polite")
            .bind_text(
                first
                    .to_text()
                    .concat("–")
                    .concat(end.to_text())
                    .concat(" / ")
                    .concat(shown.to_text()),
            ),
        )
        .child(
            icon_button(ButtonVariant::Outline, "Previous page")
                .attr_when("disabled", page.le(0))
                .on_click(page.set(page - 1))
                .child(icon(icon::CHEVRON_LEFT, rem(1))),
        )
        .child(
            icon_button(ButtonVariant::Outline, "Next page")
                .attr_when("disabled", !has_next.clone())
                .on_click(Stmt::when(has_next, page.set(page + 1)))
                .child(icon(icon::CHEVRON_RIGHT, rem(1))),
        )
}

/// The rounded border around the table, which scrolls sideways on a narrow screen.
fn frame() -> Style {
    Style::new()
        .overflow_x(OverflowX::Auto)
        .border_width(px(1))
        .border_style(BorderStyle::Solid)
        .border_color(token::border().var())
        .border_radius(token::radius().var())
}

fn table_style() -> Style {
    Style::new()
        .width(pct(100))
        .border_collapse(BorderCollapse::Collapse)
        .caption_side(CaptionSide::Bottom)
        .merge(look::small_text())
}

fn row_line() -> Style {
    Style::new()
        .border_bottom_width(px(1))
        .border_bottom_style(BorderBottomStyle::Solid)
        .border_bottom_color(token::border().var())
}

fn head_cell(numeric: bool) -> Style {
    row_line()
        .height(rem(2.5))
        .padding_inline(rem(0.5))
        .text_align(if numeric {
            TextAlign::End
        } else {
            TextAlign::Start
        })
        .vertical_align(VerticalAlign::Middle)
        .font_weight(FontWeight::Number(500.0))
        .white_space(WhiteSpace::Nowrap)
        .color(token::muted_foreground().var())
}

fn sort_button(numeric: bool) -> Style {
    button_style(ButtonVariant::Ghost, ButtonSize::Sm)
        .column_gap(rem(0.5))
        .margin_inline(rem(-0.75))
        .flex_direction(if numeric {
            hypersig::style::value::FlexDirection::RowReverse
        } else {
            hypersig::style::value::FlexDirection::Row
        })
}

fn body_row() -> Style {
    row_line().merge(look::transition()).on(
        PseudoClass::Hover,
        Style::new().background_color(token::muted().var()),
    )
}

fn selected_row() -> Style {
    Style::new().background_color(token::muted().var())
}

fn body_cell(numeric: bool) -> Style {
    Style::new()
        .padding(rem(0.5))
        .vertical_align(VerticalAlign::Middle)
        .text_align(if numeric {
            TextAlign::End
        } else {
            TextAlign::Start
        })
        .font_variant_numeric(if numeric {
            FontVariantNumeric::TabularNums
        } else {
            FontVariantNumeric::Normal
        })
}

fn select_cell() -> Style {
    Style::new()
        .width(rem(2.5))
        .padding_inline(rem(0.5))
        .vertical_align(VerticalAlign::Middle)
}

fn empty_cell() -> Style {
    Style::new()
        .height(rem(6))
        .text_align(TextAlign::Center)
        .color(token::muted_foreground().var())
}

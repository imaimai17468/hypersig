//! Data tables: rows of a list signal under typed columns, sorted,
//! filtered, and paged in the browser or on the server.

use std::fmt;
use std::rc::Rc;
use std::time::Duration;

use hypersig::html::{
    El, Event, Node, Role, attr, button, div, each_keyed, input, p, span, table, tbody, td, th,
    thead, tr,
};
use hypersig::style::pattern::stack;
use hypersig::style::value::{
    AlignItems, BorderBottomStyle, BorderCollapse, BorderStyle, CaptionSide, Color, Cursor,
    Display, FlexDirection, FontVariantNumeric, FontWeight, JustifyContent, Length, OverflowX,
    OverflowY, Position, TextAlign, VerticalAlign, WhiteSpace, ZIndex, pct, px, raw, rem,
};
use hypersig::style::{PseudoClass, Style};
use hypersig::{
    Comparable, Expr, IntoExpr, IntoStmt, Item, Number, NumberFormat, Segment, Sig, Signals, Stmt,
    Textual, if_else,
};
use serde::{Deserialize, Serialize};

use crate::button::{ButtonSize, ButtonVariant, button_style, icon_button};
use crate::choice::{check_box, check_box_checked, check_mark};
use crate::field::field_style;
use crate::look::{self, icon};
use crate::token;

/// The signals a [`DataTable`] or [`ServerTable`] keeps: filter text, page, and sort.
///
/// Put one in the page's signals for each table. A [`ServerTable`] sends it
/// to the server, which reads the sort with [`TableState::sorting`].
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
    /// The column sorted by, empty while unsorted. A [`DataTable`] writes
    /// the column's number counted from 1, and a [`ServerTable`] the
    /// column's key as a route segment writes it.
    sort: String,
    /// Whether the column is sorted descending.
    descending: bool,
}

/// The words a [`DataTable`] or [`ServerTable`] shows or names its controls with.
///
/// They are English until replaced, so a page in another language passes
/// its own:
///
/// ```
/// use hypersig_ui::TableText;
///
/// let japanese = TableText {
///     empty: "該当する行はありません。".into(),
///     select_row: "行を選ぶ".into(),
///     select_all: "すべての行を選ぶ".into(),
///     previous_page: "前のページ".into(),
///     next_page: "次のページ".into(),
///     expand_row: "行の詳細を表示".into(),
/// };
/// assert_ne!(japanese, TableText::default());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableText {
    /// Shown in place of the rows when there are none, "No results.".
    pub empty: String,
    /// The name of the checkbox of a row, "Select row".
    pub select_row: String,
    /// The name of the checkbox in the header, "Select all rows".
    pub select_all: String,
    /// The name of the button that goes back a page, "Previous page".
    pub previous_page: String,
    /// The name of the button that goes forward a page, "Next page".
    pub next_page: String,
    /// The name of the button that shows or hides a row's details, "Show row details".
    pub expand_row: String,
}

impl Default for TableText {
    fn default() -> Self {
        Self {
            empty: "No results.".to_owned(),
            select_row: "Select row".to_owned(),
            select_all: "Select all rows".to_owned(),
            previous_page: "Previous page".to_owned(),
            next_page: "Next page".to_owned(),
            expand_row: "Show row details".to_owned(),
        }
    }
}

/// The direction a [`TableState`] sorts its column in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SortOrder {
    /// Smallest first.
    Ascending,
    /// Largest first.
    Descending,
}

impl TableState {
    /// The key of the column a [`ServerTable`] is sorted by, and its order.
    ///
    /// Returns `None` while the table is unsorted, and where the sorted
    /// column is not one of `K`, such as for a state a [`DataTable`] wrote.
    ///
    /// ```
    /// use hypersig::Segment;
    /// use hypersig_ui::{SortOrder, TableState};
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Segment, Serialize, Deserialize, Debug, PartialEq)]
    /// enum By {
    ///     Email,
    ///     Amount,
    /// }
    ///
    /// let state = TableState::default().with_sorting(&By::Amount, SortOrder::Descending);
    /// assert_eq!(state.sorting::<By>(), Some((By::Amount, SortOrder::Descending)));
    /// assert_eq!(TableState::default().sorting::<By>(), None);
    /// ```
    #[must_use]
    pub fn sorting<K: Segment>(&self) -> Option<(K, SortOrder)> {
        if self.sort.is_empty() {
            return None;
        }
        let order = if self.descending {
            SortOrder::Descending
        } else {
            SortOrder::Ascending
        };
        Some((K::parse(&self.sort)?, order))
    }

    /// This state sorted by the [`ServerTable`] column whose key is `key`, in `order`.
    ///
    /// Use it for the first state of a page whose server sends the rows
    /// already sorted, so that the column's header says so.
    #[must_use]
    pub fn with_sorting<K: Segment>(mut self, key: &K, order: SortOrder) -> Self {
        self.sort = key.write();
        self.descending = order == SortOrder::Descending;
        self
    }
}

/// One column of a [`DataTable`] or a [`ServerTable`] whose rows are `T`.
///
/// A column shows a field of the row with [`Column::text`] or
/// [`Column::number`], or any element with [`Column::custom`]. The table
/// sorts by a column it receives through its `sortable_column`, which takes
/// the key the way that table sorts.
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
/// let email: Column<Users, User> = Column::text("Email", User::email());
/// # let _ = email;
/// ```
pub struct Column<S, T> {
    header: String,
    numeric: bool,
    cell: Box<dyn Fn(Item<S, T>) -> El<S>>,
    visible: Option<Expr<S, bool>>,
}

/// Renders the rows with the row template it receives.
type Each<S, T> = Box<dyn FnOnce(Box<dyn FnOnce(Item<S, T>) -> El<S>>) -> Node<S>>;

/// Whether a row has some property, such as being selected.
type RowTest<S, T> = Box<dyn Fn(Item<S, T>) -> Expr<S, bool>>;

/// A statement about a row, such as the one that selects it.
type RowStmt<S, T> = Box<dyn Fn(Item<S, T>) -> Stmt<S>>;

/// An element built from a row.
type RowEl<S, T> = Box<dyn Fn(Item<S, T>) -> El<S>>;

/// The key of a row, shared by the closures that read it.
type RowKey<S, T, K> = Rc<dyn Fn(Item<S, T>) -> Expr<S, K>>;

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

    /// A column headed `header` that shows the number field `field` written in `format`.
    ///
    /// ```
    /// use hypersig::{NumberFormat, Signals};
    /// use hypersig_ui::Column;
    /// use serde::{Deserialize, Serialize};
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Payment {
    ///     amount: f64,
    /// }
    ///
    /// #[derive(Signals, Serialize, Deserialize)]
    /// struct Ledger {
    ///     payments: Vec<Payment>,
    /// }
    ///
    /// const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");
    /// let amount: Column<Ledger, Payment> = Column::formatted("Amount", Payment::amount(), DOLLARS);
    /// # let _ = amount;
    /// ```
    #[must_use]
    pub fn formatted<U: Number + 'static>(
        header: impl Into<String>,
        field: Sig<T, U>,
        format: NumberFormat,
    ) -> Self {
        Self::new(header.into(), true, move |row| {
            span().bind_text(row.at(field).format(&format))
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
            visible: None,
        }
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
            .field("visible", &self.visible)
            .finish_non_exhaustive()
    }
}

/// A column and, for a sortable one, how its header sorts.
struct Entry<S, T> {
    column: Column<S, T>,
    sort: Option<Sorting<S>>,
}

/// The text a sortable column writes into [`TableState`], and the
/// statement that runs after the header writes it.
struct Sorting<S> {
    id: String,
    then: Box<dyn Fn(SortOrder) -> Stmt<S>>,
}

impl<S, T> fmt::Debug for Entry<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Column")
            .field("header", &self.column.header)
            .field("sort", &self.sort.as_ref().map(|sort| &sort.id))
            .finish_non_exhaustive()
    }
}

/// A table of the rows of a list signal, sorted, filtered, and paged in the browser.
///
/// Each row is keyed by `key`, as in
/// [`each_keyed`](hypersig::html::each_keyed), so a row keeps its element,
/// and its checkbox its focus, when sorting moves it. Everything the table
/// does runs in the browser on the list signal and the [`TableState`]:
///
/// - A column added with [`DataTable::sortable_column`] has a button in its
///   header that sorts the rows, and the header's `aria-sort` says which way.
/// - [`DataTable::filter`] adds a text field and shows the rows whose text
///   contains what the visitor types, ignoring case.
/// - [`DataTable::paginate`] shows a fixed number of the rows that pass the
///   filter, with buttons for the previous and the next page.
/// - [`DataTable::selectable`] adds a column of checkboxes that keep the
///   keys of the selected rows in a list signal, under a header checkbox
///   that selects or clears all of them. The header follows the WAI-ARIA
///   checkbox pattern with `aria-checked="mixed"` while some rows are
///   selected.
///
/// A page past the last row, which the list reaches by shrinking from
/// outside the table, returns the table to its first page.
///
/// The header checkbox selects every row of the list, including rows the
/// filter or the page hides. A selected key stays in the list after its
/// row leaves the table, until the header clears the selection or the
/// page removes the key. Rows are counted on each change of the list,
/// the filter, or the page, and the work grows with the square of the
/// number of rows while a filter and pagination are both on, so a list of
/// thousands of rows belongs in a [`ServerTable`], which receives one page
/// at a time from the server.
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
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Users {
///     users: Vec<User>,
///     chosen: Vec<i32>,
///     table: TableState,
/// }
///
/// let table = DataTable::new("Users", Users::table(), Users::users(), |user| user.at(User::id()))
///     .selectable(Users::chosen(), |user| user.at(User::id()))
///     .filter("Filter emails", |user| user.at(User::email()))
///     .paginate(10)
///     .sortable_column(Column::text("Email", User::email()), |user| user.at(User::email()))
///     .column(Column::number("Id", User::id()));
/// let users = vec![User { id: 1, email: "ada@example.com".into() }];
/// let html = Page::new("Users", Users { users, chosen: vec![1], table: TableState::default() })
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
    columns: Vec<Entry<S, T>>,
    selection: Option<Selection<S, T>>,
    expansion: Option<Expansion<S, T>>,
    filter: Option<(String, RowText<S, T>)>,
    page_size: Option<i32>,
    text: TableText,
    max_height: Option<Length>,
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
            selection: None,
            expansion: None,
            filter: None,
            page_size: None,
            text: TableText::default(),
            max_height: None,
        }
    }

    /// Adds `column` after the columns added so far.
    #[must_use]
    pub fn column(mut self, column: Column<S, T>) -> Self {
        self.columns.push(Entry { column, sort: None });
        self
    }

    /// Adds `column`, whose header sorts the rows by `key`, after the columns added so far.
    ///
    /// The compiler requires the key to be a number or a string, because
    /// the runtime orders only those. The first press on the header sorts
    /// the rows ascending, and a press on a column sorted ascending sorts
    /// it descending. Sorting reorders the list signal itself and keeps the
    /// order of rows with equal keys, so sorting by one column and then by
    /// another orders the rows by the second and then the first. The order
    /// holds until the list changes. A list the server patches afterwards
    /// arrives in the server's order, while the header still marks the
    /// last sort.
    #[must_use]
    pub fn sortable_column<K: Comparable, E: IntoExpr<S, K>>(
        mut self,
        column: Column<S, T>,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self {
        let rows = self.rows;
        let sort = Sorting {
            id: (self.columns.len() + 1).to_string(),
            then: Box::new(move |order| match order {
                SortOrder::Ascending => rows.sort_by(&key),
                SortOrder::Descending => rows.sort_by_desc(&key),
            }),
        };
        self.columns.push(Entry {
            column,
            sort: Some(sort),
        });
        self
    }

    /// Adds a first column of checkboxes that keep the `key` of each selected row in `selected`.
    ///
    /// The keys live in a signal of the page, apart from the rows, so a
    /// selection needs no field in the row type and outlasts a patch of the
    /// rows. The keys are compared as JSON, as `each_keyed` compares them.
    #[must_use]
    pub fn selectable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        mut self,
        selected: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self {
        self.selection = Some(Selection::new(
            self.rows,
            selected,
            Rc::new(move |row| key(row).into_expr()),
        ));
        self
    }

    /// Adds a first column of buttons that show `details` of a row under it, keeping the `key` of each open row in `expanded`.
    ///
    /// The button carries `aria-expanded`, as the WAI-ARIA disclosure
    /// pattern asks, and the details fill a row of their own under the row,
    /// across every column. Each row and its details are a `<tbody>` of
    /// their own, so the two move together when the rows are sorted.
    #[must_use]
    pub fn expandable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        mut self,
        expanded: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
        details: impl Fn(Item<S, T>) -> El<S> + 'static,
    ) -> Self {
        self.expansion = Some(Expansion {
            keys: KeySet::new(expanded, Rc::new(move |row| key(row).into_expr())),
            details: Box::new(details),
        });
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
        assert!(size >= 1, "a page holds at least one row, not {size}");
        self.page_size = Some(size);
        self
    }

    /// Replaces the words the table shows and names its controls with, English until set.
    #[must_use]
    pub fn text(mut self, text: TableText) -> Self {
        self.text = text;
        self
    }

    /// Keeps the table at most `height` high, scrolling its rows under a header that stays in place.
    #[must_use]
    pub fn max_height(mut self, height: Length) -> Self {
        self.max_height = Some(height);
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
            selection,
            expansion,
            filter,
            page_size,
            text,
            max_height,
        } = self;
        let query = state.at(TableState::query());
        let page = state.at(TableState::page());
        let (filter_label, row_text) = filter.unzip();
        let filtered = row_text.is_some();
        let matches = move |row: Item<S, T>| -> Expr<S, bool> {
            match &row_text {
                Some(row_text) => row_text(row)
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

        let lead = lead_heads(rows, selection.as_ref(), expansion.is_some(), &text);
        let (head, cells) = header_row(columns, lead, state);
        let body = Body {
            cells,
            selection,
            expansion,
            text: text.clone(),
        };
        let (width, grouped) = (body.width(), body.expansion.is_some());

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
        let body = each(Box::new(move |row| body.row(row, Some(row_shown(row)))));

        let mut out = div().css(stack(rem(1)));
        if let Some(label) = filter_label {
            out = out.child(filter_field(label, query).on_input(page.set(0)));
        }
        out = out.child(frame(
            label,
            max_height,
            grouped,
            head,
            body,
            empty_row(shown.clone().equals(0), width, text.empty.clone()),
        ));
        if let Some(size) = page_size {
            // The list can shrink from outside the table, through a patch or
            // a statement of the page, and leave the page past its last row.
            let past_end = page.gt(0).and((page * size).ge(shown.clone()));
            out = out
                .effect(Stmt::when(page.lt(0).or(past_end), page.set(0)))
                .child(pager(page, shown, size, None, &text));
        }
        out
    }
}

impl<S, T> fmt::Debug for DataTable<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DataTable")
            .field("label", &self.label)
            .field("state", &self.state)
            .field("rows", &self.rows)
            .field("columns", &self.columns)
            .field("selectable", &self.selection.is_some())
            .field("expandable", &self.expansion.is_some())
            .field("filter", &self.filter.as_ref().map(|(label, _)| label))
            .field("page_size", &self.page_size)
            .field("max_height", &self.max_height)
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

/// A table whose rows the server filters, sorts, and pages, one page at a time.
///
/// The list signal holds only the rows shown. The filter field, a sortable
/// header, and the page buttons write the [`TableState`] and then run
/// `load`, which is a call to an action that receives the state. The
/// action answers with a patch of the rows and, for a paged table, of the
/// total. The browser counts no rows, so the browser's work per change
/// grows with the rows of one page, however many the server holds.
///
/// - [`ServerTable::sortable_column`] takes a key the server reads back
///   with [`TableState::sorting`], so both sides name the column with the
///   same enum.
/// - [`ServerTable::filter`] runs `load` once typing pauses for 300 ms, and
///   returns to the first page.
/// - [`ServerTable::paginate`] shows the range of rows and moves between
///   pages, out of the total that the server writes.
///
/// What the server does with the state is the application's code, which
/// the compiler does not check against the table: it answers with the
/// rows of the page the state names, and a total that counts every row
/// that passes the filter. A page past the last row shows no rows until
/// the visitor goes back.
///
/// ```
/// use hypersig::{Action, Page, Patches, Segment, Signals};
/// use hypersig_ui::{Column, ServerTable, SortOrder, TableState};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize, Clone)]
/// struct User {
///     id: i32,
///     email: String,
/// }
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Users {
///     users: Vec<User>,
///     total: i32,
///     table: TableState,
/// }
///
/// #[derive(Segment, Serialize, Deserialize)]
/// enum By {
///     Email,
/// }
///
/// const LOAD: Action<Users, TableState> = Action::post("/users");
///
/// /// The page of users `state` names, and how many pass its filter.
/// fn query(all: &[User], state: &TableState) -> (Vec<User>, i32) {
///     let mut rows: Vec<User> = all
///         .iter()
///         .filter(|user| user.email.contains(&state.query.to_lowercase()))
///         .cloned()
///         .collect();
///     if let Some((By::Email, order)) = state.sorting::<By>() {
///         rows.sort_by(|a, b| a.email.cmp(&b.email));
///         if order == SortOrder::Descending {
///             rows.reverse();
///         }
///     }
///     let total = i32::try_from(rows.len()).unwrap_or(i32::MAX);
///     let start = usize::try_from(state.page * 10).unwrap_or(0);
///     (rows.into_iter().skip(start).take(10).collect(), total)
/// }
///
/// async fn load((): (), state: TableState) -> Patches<Users> {
///     let all = vec![User { id: 1, email: "ada@example.com".into() }];
///     let (rows, total) = query(&all, &state);
///     Users::users().patch(rows).and(Users::total().patch(total)).into()
/// }
///
/// let table = ServerTable::new(
///     "Users",
///     Users::table(),
///     Users::users(),
///     |user| user.at(User::id()),
///     LOAD.call_with(Users::table()),
/// )
/// .filter("Filter emails")
/// .paginate(10, Users::total())
/// .sortable_column(Column::text("Email", User::email()), &By::Email);
/// let all = vec![User { id: 1, email: "ada@example.com".into() }];
/// let (users, total) = query(&all, &TableState::default());
/// let html = Page::new("Users", Users { users, total, table: TableState::default() })
///     .child(table)
///     .render();
/// assert!(html.contains("1–1 / 1"));
/// # let _ = load;
/// ```
pub struct ServerTable<S, T> {
    label: String,
    state: Sig<S, TableState>,
    rows: Sig<S, Vec<T>>,
    each: Each<S, T>,
    load: Stmt<S>,
    columns: Vec<Entry<S, T>>,
    selection: Option<Selection<S, T>>,
    expansion: Option<Expansion<S, T>>,
    filter: Option<String>,
    pages: Option<(i32, Sig<S, i32>)>,
    text: TableText,
    max_height: Option<Length>,
}

impl<S: 'static, T: 'static> ServerTable<S, T> {
    /// An empty table named `label`, listing `rows` by `key`, which runs `load` to fetch rows.
    ///
    /// `state` holds the filter text, the page, and the sort, and `load`
    /// runs after the visitor changes one of them.
    #[must_use]
    pub fn new<K: 'static, E: IntoExpr<S, K>>(
        label: impl Into<String>,
        state: Sig<S, TableState>,
        rows: Sig<S, Vec<T>>,
        key: impl FnOnce(Item<S, T>) -> E + 'static,
        load: impl IntoStmt<S>,
    ) -> Self {
        Self {
            label: label.into(),
            state,
            rows,
            each: Box::new(move |render| each_keyed(rows, key, render)),
            load: load.into_stmt(),
            columns: Vec::new(),
            selection: None,
            expansion: None,
            filter: None,
            pages: None,
            text: TableText::default(),
            max_height: None,
        }
    }

    /// Adds `column` after the columns added so far.
    #[must_use]
    pub fn column(mut self, column: Column<S, T>) -> Self {
        self.columns.push(Entry { column, sort: None });
        self
    }

    /// Adds `column`, whose header asks the server to sort by `key`, after the columns added so far.
    ///
    /// The header writes `key` into the [`TableState`], returns to the
    /// first page, and runs `load`. The server reads the key back with
    /// [`TableState::sorting`]. An enum with `#[derive(Segment)]` keeps the
    /// keys of the columns and the server's `match` in one type.
    #[must_use]
    pub fn sortable_column<K: Segment>(mut self, column: Column<S, T>, key: &K) -> Self {
        let page = self.state.at(TableState::page());
        let load = self.load.clone();
        let sort = Sorting {
            id: key.write(),
            then: Box::new(move |_| page.set(0).then(load.clone())),
        };
        self.columns.push(Entry {
            column,
            sort: Some(sort),
        });
        self
    }

    /// Adds a first column of checkboxes that keep the `key` of each selected row in `selected`.
    ///
    /// The keys live in a signal of the page, so a row selected on one page
    /// stays selected while the visitor moves to another, and the server
    /// receives the keys as an argument of an action. The header checkbox
    /// selects or clears the rows of the page shown.
    #[must_use]
    pub fn selectable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        mut self,
        selected: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
    ) -> Self {
        self.selection = Some(Selection::new(
            self.rows,
            selected,
            Rc::new(move |row| key(row).into_expr()),
        ));
        self
    }

    /// Adds a first column of buttons that show `details` of a row under it, keeping the `key` of each open row in `expanded`.
    ///
    /// The button carries `aria-expanded`, as the WAI-ARIA disclosure
    /// pattern asks, and the details fill a row of their own under the row,
    /// across every column. Each row and its details are a `<tbody>` of
    /// their own, so the two move together when the rows are sorted.
    #[must_use]
    pub fn expandable<K: PartialEq + 'static, E: IntoExpr<S, K>>(
        mut self,
        expanded: Sig<S, Vec<K>>,
        key: impl Fn(Item<S, T>) -> E + 'static,
        details: impl Fn(Item<S, T>) -> El<S> + 'static,
    ) -> Self {
        self.expansion = Some(Expansion {
            keys: KeySet::new(expanded, Rc::new(move |row| key(row).into_expr())),
            details: Box::new(details),
        });
        self
    }

    /// Adds a field named `label` whose text the server filters the rows by.
    ///
    /// The field sits above the table. Once typing pauses for 300 ms, the
    /// table returns to its first page and runs `load`.
    #[must_use]
    pub fn filter(mut self, label: impl Into<String>) -> Self {
        self.filter = Some(label.into());
        self
    }

    /// Shows pages of `size` rows out of `total`, with buttons for the previous and next page.
    ///
    /// The server answers each `load` with at most `size` rows and writes
    /// `total`, the number of rows that pass the filter. Below the table,
    /// the range of rows shown is written as `11–20 / 42`.
    ///
    /// # Panics
    ///
    /// Panics if `size` is less than 1.
    #[must_use]
    pub fn paginate(mut self, size: i32, total: Sig<S, i32>) -> Self {
        assert!(size >= 1, "a page holds at least one row, not {size}");
        self.pages = Some((size, total));
        self
    }

    /// Replaces the words the table shows and names its controls with, English until set.
    #[must_use]
    pub fn text(mut self, text: TableText) -> Self {
        self.text = text;
        self
    }

    /// Keeps the table at most `height` high, scrolling its rows under a header that stays in place.
    #[must_use]
    pub fn max_height(mut self, height: Length) -> Self {
        self.max_height = Some(height);
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
            load,
            columns,
            selection,
            expansion,
            filter,
            pages,
            text,
            max_height,
        } = self;
        let query = state.at(TableState::query());
        let page = state.at(TableState::page());
        let lead = lead_heads(rows, selection.as_ref(), expansion.is_some(), &text);
        let (head, cells) = header_row(columns, lead, state);
        let body = Body {
            cells,
            selection,
            expansion,
            text: text.clone(),
        };
        let (width, grouped) = (body.width(), body.expansion.is_some());
        let body = each(Box::new(move |row| body.row(row, None)));

        let mut out = div().css(stack(rem(1)));
        if let Some(label) = filter {
            out = out.child(filter_field(label, query).on_debounced(
                Event::Input,
                Duration::from_millis(300),
                page.set(0).then(load.clone()),
            ));
        }
        out = out.child(frame(
            label,
            max_height,
            grouped,
            head,
            body,
            empty_row(rows.is_empty(), width, text.empty.clone()),
        ));
        if let Some((size, total)) = pages {
            out = out.child(pager(page, total.into_expr(), size, Some(&load), &text));
        }
        out
    }
}

impl<S, T> fmt::Debug for ServerTable<S, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerTable")
            .field("label", &self.label)
            .field("state", &self.state)
            .field("rows", &self.rows)
            .field("load", &self.load)
            .field("columns", &self.columns)
            .field("selectable", &self.selection.is_some())
            .field("expandable", &self.expansion.is_some())
            .field("filter", &self.filter)
            .field("pages", &self.pages)
            .field("max_height", &self.max_height)
            .finish_non_exhaustive()
    }
}

impl<S: 'static, T: 'static> From<ServerTable<S, T>> for El<S> {
    fn from(table: ServerTable<S, T>) -> Self {
        table.render()
    }
}

impl<S: 'static, T: 'static> From<ServerTable<S, T>> for Node<S> {
    fn from(table: ServerTable<S, T>) -> Self {
        table.render().into()
    }
}

/// Whether a column is numeric, how it renders a row, and when it shows.
type Cell<S, T> = (
    bool,
    Box<dyn Fn(Item<S, T>) -> El<S>>,
    Option<Expr<S, bool>>,
);

/// The header row, after the cells of `lead`, and for each column the
/// parts its cells need.
fn header_row<S: 'static, T: 'static>(
    columns: Vec<Entry<S, T>>,
    lead: Vec<El<S>>,
    state: Sig<S, TableState>,
) -> (El<S>, Vec<Cell<S, T>>) {
    let mut head = tr().children(lead);
    let mut cells = Vec::with_capacity(columns.len());
    for Entry { column, sort } in columns {
        let heading = th().attr(attr::SCOPE, "col").css(head_cell(column.numeric));
        let mut heading = match sort {
            Some(sort) => sort_header(heading, column.header, column.numeric, sort, state),
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

/// `line` with a cell per column for `row`.
fn row_cells<S, T>(mut line: El<S>, cells: &[Cell<S, T>], row: Item<S, T>) -> El<S> {
    for (numeric, cell, visible) in cells {
        let mut data = td().css(body_cell(*numeric)).child(cell(row));
        if let Some(visible) = visible {
            data = data.show(visible.clone());
        }
        line = line.child(data);
    }
    line
}

/// The rounded border around the `<table>` named `label`.
///
/// With a `max_height`, the border scrolls the rows and the header sticks
/// to its top.
///
/// Where the rows are `grouped`, each is a `<tbody>` of its own, and the
/// empty row gets one too.
fn frame<S>(
    label: String,
    max_height: Option<Length>,
    grouped: bool,
    head: El<S>,
    body: Node<S>,
    empty_row: El<S>,
) -> El<S> {
    let mut border = div().css(frame_style());
    let mut header = thead();
    if let Some(height) = max_height {
        border = border.css(Style::new().max_height(height).overflow_y(OverflowY::Auto));
        header = header.css(
            Style::new()
                .position(Position::Sticky)
                .top(px(0))
                .z_index(ZIndex::Integer(1))
                .background_color(token::background().var())
                // The border of a collapsed table stays with the scrolled
                // rows, so the line under the header is a shadow.
                .box_shadow(raw(format!("inset 0 -1px 0 {}", token::border().var()))),
        );
    }
    let table = table()
        .attr(attr::ARIA_LABEL, label)
        .css(table_style())
        .child(header.child(head));
    border.child(if grouped {
        table.child(body).child(tbody().child(empty_row))
    } else {
        table.child(tbody().child(body).child(empty_row))
    })
}

/// The row that spans `width` columns and says `text` while `when` is true.
fn empty_row<S>(when: Expr<S, bool>, width: usize, text: String) -> El<S> {
    tr().show(when).child(
        td().attr(attr::COLSPAN, width.to_string())
            .css(empty_cell())
            .text(text),
    )
}

/// The search field named `label` bound to `query`.
fn filter_field<S>(label: String, query: Sig<S, String>) -> El<S> {
    input()
        .attr(attr::TYPE, "search")
        .attr(attr::ARIA_LABEL, label.clone())
        .attr(attr::PLACEHOLDER, label)
        .css(field_style().max_width(rem(24)))
        .bind(query)
}

/// `heading` with a button that writes `sort` into `state` and runs its
/// statement, and the `aria-sort` that follows `state`.
fn sort_header<S>(
    heading: El<S>,
    header: String,
    numeric: bool,
    sort: Sorting<S>,
    state: Sig<S, TableState>,
) -> El<S> {
    let column = state.at(TableState::sort());
    let descending = state.at(TableState::descending());
    let on = column.equals(sort.id.clone());
    let up = on.clone().and(!descending);
    let down = on.and(descending);
    let toggle = Stmt::if_else(
        up.clone(),
        descending
            .set(true)
            .then((sort.then)(SortOrder::Descending)),
        column
            .set(sort.id)
            .then(descending.set(false))
            .then((sort.then)(SortOrder::Ascending)),
    );
    heading
        .bind_attr(
            attr::ARIA_SORT,
            if_else(
                up.clone(),
                "ascending",
                if_else(down.clone(), "descending", "none"),
            ),
        )
        .child(
            button()
                .attr(attr::TYPE, "button")
                .css(sort_button(numeric))
                .on_click(toggle)
                .child(span().text(header))
                .child(icon(icon::ARROW_UP, rem(1)).show(up.clone()))
                .child(icon(icon::ARROW_DOWN, rem(1)).show(down.clone()))
                .child(
                    icon(icon::ARROW_UP_DOWN, rem(1))
                        .show(!up.or(down))
                        .css(Style::new().color(token::muted_foreground().var())),
                ),
        )
}

/// A list of row keys in a signal: whether a row's key is in it, and the
/// statement that puts the key in or takes it out.
struct KeySet<S, T> {
    has: RowTest<S, T>,
    toggle: RowStmt<S, T>,
}

impl<S: 'static, T: 'static> KeySet<S, T> {
    fn new<K: PartialEq + 'static>(list: Sig<S, Vec<K>>, key: RowKey<S, T, K>) -> Self {
        let has = {
            let key = Rc::clone(&key);
            move |row: Item<S, T>| list.count(|kept| kept.value().equals(key(row))).gt(0)
        };
        let toggle = {
            let has = has.clone();
            move |row: Item<S, T>| {
                Stmt::if_else(
                    has(row),
                    list.retain(|kept| kept.value().not_equals(key(row))),
                    list.push(key(row)),
                )
            }
        };
        Self {
            has: Box::new(has),
            toggle: Box::new(toggle),
        }
    }
}

/// Which rows are selected, and the statements of the header checkbox.
struct Selection<S, T> {
    keys: KeySet<S, T>,
    select_all: Stmt<S>,
    clear_all: Stmt<S>,
}

impl<S: 'static, T: 'static> Selection<S, T> {
    /// The selection of `rows` by `key`, kept in `selected`.
    fn new<K: PartialEq + 'static>(
        rows: Sig<S, Vec<T>>,
        selected: Sig<S, Vec<K>>,
        key: RowKey<S, T, K>,
    ) -> Self {
        // Keeps the keys of rows outside `rows`, such as another page's.
        let clear_all =
            selected.retain(|chosen| rows.count(|row| key(row).equals(chosen.value())).equals(0));
        let select_all = clear_all
            .clone()
            .then(selected.append(rows.map(|row| key(row))));
        Self {
            keys: KeySet::new(selected, key),
            select_all,
            clear_all,
        }
    }
}

/// Which rows show their details, and how the details of a row look.
struct Expansion<S, T> {
    keys: KeySet<S, T>,
    details: RowEl<S, T>,
}

/// The parts a row of the body is built from, which both tables share.
struct Body<S, T> {
    cells: Vec<Cell<S, T>>,
    selection: Option<Selection<S, T>>,
    expansion: Option<Expansion<S, T>>,
    text: TableText,
}

impl<S, T> Body<S, T> {
    /// The number of columns, those of the buttons and the checkboxes included.
    fn width(&self) -> usize {
        self.cells.len()
            + usize::from(self.selection.is_some())
            + usize::from(self.expansion.is_some())
    }

    /// The row of `row`, shown while `shown` is true where there is one,
    /// with its details under it where the table expands rows.
    fn row(&self, row: Item<S, T>, shown: Option<Expr<S, bool>>) -> El<S> {
        let mut line = tr().css(body_row());
        if let Some(expansion) = &self.expansion {
            let open = (expansion.keys.has)(row);
            line = line.child(
                td().css(select_cell()).child(
                    icon_button(ButtonVariant::Ghost, self.text.expand_row.clone())
                        .bind_attr(attr::ARIA_EXPANDED, if_else(open.clone(), "true", "false"))
                        .on_click((expansion.keys.toggle)(row))
                        .child(
                            icon(icon::CHEVRON_RIGHT, rem(1))
                                .css(look::transition())
                                .css_when(Style::new().rotate(raw("90deg")), open),
                        ),
                ),
            );
        }
        if let Some(selection) = &self.selection {
            let checked = (selection.keys.has)(row);
            line = line.css_when(selected_row(), checked.clone()).child(
                td().css(select_cell()).child(check_mark(
                    input()
                        .attr(attr::ARIA_LABEL, self.text.select_row.clone())
                        .bind_checked(checked)
                        .on_change((selection.keys.toggle)(row)),
                )),
            );
        }
        let line = row_cells(line, &self.cells, row);
        let Some(expansion) = &self.expansion else {
            return match shown {
                Some(shown) => line.show(shown),
                None => line,
            };
        };
        let details = tr().show((expansion.keys.has)(row)).child(
            td().attr(attr::COLSPAN, self.width().to_string())
                .css(details_cell())
                .child((expansion.details)(row)),
        );
        let group = tbody().child(line).child(details);
        match shown {
            Some(shown) => group.show(shown),
            None => group,
        }
    }
}

/// The header cells before the columns: an empty one over the buttons
/// that expand rows, and the checkbox over those that select them.
fn lead_heads<S, T>(
    rows: Sig<S, Vec<T>>,
    selection: Option<&Selection<S, T>>,
    expandable: bool,
    text: &TableText,
) -> Vec<El<S>> {
    let mut lead = Vec::new();
    if expandable {
        lead.push(td().css(select_cell()));
    }
    if let Some(selection) = selection {
        lead.push(
            th().attr(attr::SCOPE, "col")
                .css(select_cell())
                .child(select_all(rows, selection, &text.select_all)),
        );
    }
    lead
}

/// The header checkbox: checked while every row of `rows` is selected,
/// mixed while some are, and a press selects every row unless all are
/// selected already.
fn select_all<S, T>(rows: Sig<S, Vec<T>>, selection: &Selection<S, T>, label: &str) -> El<S> {
    let count = rows.count(|row| (selection.keys.has)(row));
    let all = count.clone().equals(rows.len()).and(rows.len().gt(0));
    let some = count.gt(0);
    let mark = span()
        .attr(attr::ARIA_HIDDEN, "true")
        .css(check_box())
        .css_when(check_box_checked(), some.clone())
        .child(icon(icon::CHECK, rem(0.875)).show(all.clone()))
        .child(icon(icon::MINUS, rem(0.875)).show(some.clone().and(!all.clone())));
    button()
        .attr(attr::TYPE, "button")
        .role(Role::Checkbox)
        .attr(attr::ARIA_LABEL, label)
        .bind_attr(
            attr::ARIA_CHECKED,
            if_else(all.clone(), "true", if_else(some, "mixed", "false")),
        )
        .on_click(Stmt::if_else(
            all,
            selection.clear_all.clone(),
            selection.select_all.clone(),
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

/// The range of rows shown out of `shown`, and the buttons that move
/// between pages and then run `then`.
fn pager<S>(
    page: Sig<S, i32>,
    shown: Expr<S, i32>,
    size: i32,
    then: Option<&Stmt<S>>,
    text: &TableText,
) -> El<S> {
    let start = page * size;
    let end = (start.clone() + size).min(shown.clone());
    let first = if_else(shown.clone().equals(0), Expr::lit(&0), start.clone() + 1);
    let has_next = (start.clone() + size).lt(shown.clone());
    let go = |to: Expr<S, i32>| match then {
        Some(then) => page.set(to).then(then.clone()),
        None => page.set(to),
    };
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
            .attr(attr::ARIA_LIVE, "polite")
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
            icon_button(ButtonVariant::Outline, text.previous_page.clone())
                .disabled(page.le(0))
                .on_click(Stmt::when(page.gt(0), go(page - 1)))
                .child(icon(icon::CHEVRON_LEFT, rem(1))),
        )
        .child(
            icon_button(ButtonVariant::Outline, text.next_page.clone())
                .disabled(!has_next.clone())
                .on_click(Stmt::when(has_next, go(page + 1)))
                .child(icon(icon::CHEVRON_RIGHT, rem(1))),
        )
}

/// The rounded border around the table, which scrolls sideways on a narrow screen.
fn frame_style() -> Style {
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
            FlexDirection::RowReverse
        } else {
            FlexDirection::Row
        })
}

fn body_row() -> Style {
    row_line().merge(look::transition()).on(
        PseudoClass::Hover,
        Style::new().background_color(token::muted().var()),
    )
}

/// The cell that holds a row's details, under the row.
fn details_cell() -> Style {
    row_line()
        .padding(rem(1))
        .background_color(token::muted().var().opacity(50))
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

//! The components of `hypersig_ui` on one page: a form whose email the
//! server checks, toggles, pickers, tabs, an accordion, dialogs, a menu,
//! tooltips, toasts, a data table, a table the server pages, charts, and
//! the static pieces. Every response carries a Content-Security-Policy that
//! allows styles only by nonce, which the progress bar, the slider, and the
//! charts run under.
//!
//! Run with `cargo run -p example-ui` and open <http://localhost:3005>.

use axum::extract::Request;
use axum::http::{HeaderValue, header};
use axum::middleware::{self, Next};
use axum::response::Response;
use axum::{Extension, Router};
use hypersig::html::{
    self, El, Orientation, attr, div, each, form, h1, h2, header, li, p, section, span, ul,
};

use hypersig::style::Media;
use hypersig::style::Style;
use hypersig::style::pattern::{container, grid, hstack, stack, wrap};
use hypersig::style::value::{FontWeight, JustifyContent, raw, rem};
use hypersig::{
    Action, Expr, NumberFormat, Page, Patches, Route, RouterExt, Segment, Signals, Url, if_else,
};
use hypersig_ui::{
    Accordion, AlertVariant, Autocomplete, BadgeVariant, Breadcrumb, ButtonSize, ButtonVariant,
    Chart, Column, Combobox, ComboboxState, DataTable, Dialog, DonutChart, InputKind, Level,
    Listbox, Menu, MenuItem, NavList, NumberField, PaginationText, ServerTable, Side, Slider,
    SortOrder, TableState, Tabs, TextField, Toast, ToastVariant, Toaster, Tooltip, alert,
    alert_description, alert_title, badge, button, card, card_content, card_description,
    card_footer, card_header, card_title, checkbox, dialog_footer, pagination, progress,
    radio_group, select_field, separator, skeleton, submit_button, switch, token,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Scheme {
    System,
    Light,
    Dark,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Plan {
    Hobby,
    Pro,
    Team,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Region {
    Tokyo,
    Frankfurt,
    Virginia,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Font {
    Sans,
    Serif,
    Mono,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Section {
    Account,
    Password,
    Billing,
}

/// One month of the charts.
#[derive(Signals, Serialize, Deserialize)]
struct Month {
    name: String,
    desktop: i32,
    mobile: i32,
    /// Desktop visitors less mobile ones, negative where mobile led.
    lead: i32,
}

/// One slice of the donut chart.
#[derive(Signals, Serialize, Deserialize)]
struct Browser {
    name: String,
    visitors: i32,
}

/// One row of the data table.
#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    email: String,
    status: String,
    amount: f64,
}

/// One row of the table the server pages.
#[derive(Signals, Serialize, Deserialize, Clone)]
struct Order {
    id: i32,
    email: String,
    amount: f64,
}

/// The columns the server sorts the orders by.
#[derive(Segment, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OrderBy {
    Email,
    Amount,
}

/// The two toggles, kept together so that both are saved as one value.
#[derive(Signals, Serialize, Deserialize)]
struct Preferences {
    notifications: bool,
    newsletter: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Gallery {
    scheme: Scheme,
    name: String,
    nickname: String,
    owner: String,
    email: String,
    bio: String,
    seats: i32,
    email_error: String,
    saving: bool,
    saved: String,
    preferences: Preferences,
    plan: Plan,
    region: Region,
    section: Section,
    progress: i32,
    archived: bool,
    status: String,
    font: Font,
    country: ComboboxState,
    volume: i32,
    toasts: Vec<Toast>,
    payments: Vec<Payment>,
    payments_table: TableState,
    chosen_payments: Vec<i32>,
    open_payments: Vec<i32>,
    show_status: bool,
    orders: Vec<Order>,
    orders_total: i32,
    orders_table: TableState,
    chosen_orders: Vec<i32>,
    months: Vec<Month>,
    browsers: Vec<Browser>,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
    const CATALOG_HOME: Route = "/catalog" => catalog_home;
    const CATALOG: Route<u32> = "/catalog/{page}" => catalog;
}

/// The items of the catalog page.
#[derive(Signals, Serialize, Deserialize)]
struct Catalog {
    items: Vec<String>,
}

/// The number of pages of the catalog.
const CATALOG_PAGES: u32 = 12;

/// The first page of the catalog, at the address the sections link to.
async fn catalog_home((): (), Extension(nonce): Extension<Nonce>, (): ()) -> Option<Page<Catalog>> {
    Some(catalog_page(1, CATALOG_HOME.url(())).csp_nonce(nonce.0))
}

async fn catalog((): (), Extension(nonce): Extension<Nonce>, page: u32) -> Option<Page<Catalog>> {
    (1..=CATALOG_PAGES)
        .contains(&page)
        .then(|| catalog_page(page, CATALOG.url(page)).csp_nonce(nonce.0))
}

/// Page `page` of a catalog of 12 pages, served at `here`, with a list of
/// the sections, a breadcrumb, and links to the other pages.
fn catalog_page(page: u32, here: Url) -> Page<Catalog> {
    let items = (1..=3)
        .map(|n| format!("Item {}", (page - 1) * 3 + n))
        .collect();
    Page::new("Catalog", Catalog { items })
        .global(hypersig_ui::global())
        .theme(hypersig_ui::theme())
        .child(
            div()
                .css(container(rem(48), rem(2)).padding_block(rem(2)))
                .child(
                    div()
                        .css(grid(1, rem(2)).on(
                            Media::Md,
                            Style::new().grid_template_columns(raw("12rem 1fr")),
                        ))
                        .child(
                            NavList::new("Sections", here)
                                .item("Gallery", INDEX, ())
                                .item("Catalog", CATALOG_HOME, ()),
                        )
                        .child(
                            div()
                                .css(stack(rem(1.5)))
                                .child(
                                    Breadcrumb::new("Breadcrumb")
                                        .link("Gallery", INDEX.link())
                                        .link("Catalog", CATALOG_HOME.link())
                                        .current(format!("Page {page}")),
                                )
                                .child(ul().child(each(Catalog::items(), |item| {
                                    li().bind_text(item.value())
                                })))
                                .child(pagination(
                                    page,
                                    CATALOG_PAGES,
                                    |page| CATALOG.link_with(&page),
                                    &PaginationText::default(),
                                )),
                        ),
                ),
        )
}

const SAVE: Action<Gallery, (String, String)> = Action::post("/save");
const NEXT_HALF: Action<Gallery> = Action::post("/next-half");
const ORDERS: Action<Gallery, TableState> = Action::post("/orders");
const DOLLARS: NumberFormat = NumberFormat::new().digits(2).prefix("$");
const EDIT: Dialog<Gallery> = Dialog::new("edit-profile");
const DELETE: Dialog<Gallery> = Dialog::new("delete-project");
const FILTERS: Dialog<Gallery> = Dialog::new("filters");
const ACTIONS: Menu<Gallery> = Menu::new("project-actions");
const ADD_TIP: Tooltip<Gallery> = Tooltip::new("add-item-tip");
const COUNTRY: Combobox<Gallery> = Combobox::new("country", "country-options");

/// Checks on the server what the browser cannot: whether the address is
/// taken. The browser has already checked that it is an email address.
async fn save((): (), (name, email): (String, String)) -> Patches<Gallery> {
    let error = if email.ends_with("@taken.example") {
        "That email is already in use.".to_owned()
    } else {
        String::new()
    };
    let saved = if error.is_empty() {
        format!(
            "Saved {}.",
            if name.is_empty() {
                "your profile"
            } else {
                &name
            }
        )
    } else {
        String::new()
    };
    Gallery::email_error()
        .patch(error)
        .and(Gallery::saved().patch(saved))
        .into()
}

fn panel(title: &str) -> El<Gallery> {
    section()
        .css(stack(rem(1)))
        .child(h2().css(heading()).text(title))
}

fn heading() -> Style {
    Style::new()
        .font_size(rem(1.25))
        .font_weight(FontWeight::Number(600.0))
}

fn top() -> El<Gallery> {
    header()
        .css(hstack(rem(1)).justify_content(JustifyContent::SpaceBetween))
        .child(
            div()
                .child(h1().css(heading().font_size(rem(1.75))).text("hypersig_ui"))
                .child(
                    p().css(Style::new().color(token::muted_foreground().var()))
                        .text("shadcn/ui's look on native elements and the WAI-ARIA patterns."),
                ),
        )
        .child(div().id("scheme").child(select_field(
            "Theme",
            Gallery::scheme(),
            [
                (Scheme::System, "System"),
                (Scheme::Light, "Light"),
                (Scheme::Dark, "Dark"),
            ],
        )))
}

fn buttons() -> El<Gallery> {
    let variants = [
        (ButtonVariant::Default, "Default"),
        (ButtonVariant::Secondary, "Secondary"),
        (ButtonVariant::Outline, "Outline"),
        (ButtonVariant::Ghost, "Ghost"),
        (ButtonVariant::Destructive, "Destructive"),
        (ButtonVariant::Link, "Link"),
    ];
    panel("Buttons and badges")
        .child(
            div().css(wrap(rem(0.5))).children(
                variants
                    .into_iter()
                    .map(|(variant, text)| button(variant, ButtonSize::Default).text(text)),
            ),
        )
        .child(
            div()
                .css(wrap(rem(0.5)))
                .child(button(ButtonVariant::Default, ButtonSize::Sm).text("Small"))
                .child(button(ButtonVariant::Default, ButtonSize::Lg).text("Large"))
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Default)
                        .attr(attr::DISABLED, "")
                        .text("Disabled"),
                )
                .child(
                    ADD_TIP
                        .icon_trigger(ButtonVariant::Outline, "Add item")
                        .text("+"),
                )
                .child(ADD_TIP.tooltip("Adds an empty row at the end")),
        )
        .child(
            div()
                .css(wrap(rem(0.5)))
                .child(badge(BadgeVariant::Default).text("Default"))
                .child(badge(BadgeVariant::Secondary).text("Secondary"))
                .child(badge(BadgeVariant::Outline).text("Outline"))
                .child(badge(BadgeVariant::Destructive).text("Failed")),
        )
}

fn profile() -> El<Gallery> {
    let name = Gallery::name();
    let email = Gallery::email();
    let saving = Gallery::saving();
    let submit = SAVE
        .call_with((name, email))
        .indicator(saving)
        .then(Gallery::email_error().set(""));
    card().id("profile").child(
        form()
            .on_submit(submit)
            .css(stack(rem(1.5)))
            .child(
                card_header()
                    .child(card_title(Level::H3).text("Profile"))
                    .child(
                        card_description()
                            .text("The server checks that the email is free when you save."),
                    ),
            )
            .child(
                card_content()
                    .css(stack(rem(1)))
                    .child(
                        TextField::new("Name", name)
                            .autocomplete(Autocomplete::Name)
                            .placeholder("Ada Lovelace"),
                    )
                    .child(
                        TextField::new("Email", email)
                            .kind(InputKind::Email)
                            .autocomplete(Autocomplete::Email)
                            .description("We send receipts here.")
                            .error(Gallery::email_error())
                            .required(),
                    )
                    .child(
                        TextField::new("Bio", Gallery::bio())
                            .multiline()
                            .description("A sentence or two for your profile page."),
                    )
                    .child(
                        NumberField::new("Seats", Gallery::seats())
                            .min(1)
                            .max(8)
                            .description("Up to 8 seats."),
                    ),
            )
            .child(
                card_footer()
                    .child(
                        submit_button(ButtonVariant::Default, ButtonSize::Default)
                            .disabled(saving)
                            .bind_text(if_else(saving, "Saving…", "Save")),
                    )
                    .child(
                        span()
                            .id("saved")
                            .attr(attr::ROLE, "status")
                            .css(Style::new().color(token::muted_foreground().var()))
                            .bind_text(Gallery::saved()),
                    ),
            ),
    )
}

fn choices() -> El<Gallery> {
    let preferences = Gallery::preferences();
    card().id("choices").child(
        card_content()
            .css(stack(rem(1.25)))
            .child(checkbox(
                "Email me about new releases",
                preferences.at(Preferences::newsletter()),
            ))
            .child(switch(
                "Notifications",
                preferences.at(Preferences::notifications()),
            ))
            .child(separator(Orientation::Horizontal))
            .child(radio_group(
                "Plan",
                Gallery::plan(),
                [
                    (Plan::Hobby, "Hobby"),
                    (Plan::Pro, "Pro"),
                    (Plan::Team, "Team"),
                ],
            ))
            .child(select_field(
                "Region",
                Gallery::region(),
                [
                    (Region::Tokyo, "Tokyo"),
                    (Region::Frankfurt, "Frankfurt"),
                    (Region::Virginia, "Virginia"),
                ],
            )),
    )
}

fn pickers() -> El<Gallery> {
    let volume = Gallery::volume();
    panel("Pickers").child(
        div()
            .css(grid(1, rem(1.5)).on(Media::Md, grid(2, rem(1.5))))
            .child(
                div().id("font").child(
                    Listbox::new("Font", Gallery::font())
                        .option(Font::Sans, "Sans serif")
                        .option(Font::Serif, "Serif")
                        .option(Font::Mono, "Monospace"),
                ),
            )
            .child(
                div()
                    .css(stack(rem(1.5)))
                    .child(COUNTRY.render(
                        "Country",
                        Gallery::country(),
                        [
                            "Brazil", "Canada", "France", "Germany", "India", "Japan", "Kenya",
                            "Mexico", "Norway", "Spain",
                        ],
                    ))
                    .child(
                        Slider::new("Volume", volume)
                            .step(5)
                            .value_text(volume.to_text().concat(" %")),
                    ),
            ),
    )
}

fn settings() -> El<Gallery> {
    panel("Tabs").child(
        Tabs::new("Settings", Gallery::section())
            .tab(
                Section::Account,
                "Account",
                p().text("Change your name and email in the profile above."),
            )
            .tab(
                Section::Password,
                "Password",
                p().text("Passwords are at least twelve characters."),
            )
            .tab(
                Section::Billing,
                "Billing",
                p().text("Invoices arrive on the first of each month."),
            ),
    )
}

fn faq() -> El<Gallery> {
    panel("Accordion").child(
        Accordion::exclusive("faq")
            .item(
                "Does it need JavaScript?",
                p().text("The accordion is <details>, which opens without it."),
            )
            .item(
                "Can I restyle it?",
                p().text("Set the tokens in a Theme, or add a Style to any element."),
            )
            .item(
                "Is it accessible?",
                p().text("Each component follows the WAI-ARIA Authoring Practices."),
            ),
    )
}

/// A sheet along the right edge, opened by the Filters button.
fn filters() -> El<Gallery> {
    FILTERS.sheet(
        Side::Right,
        "Filters",
        "Narrow the projects down, then press Apply.",
        div()
            .css(stack(rem(1)))
            .child(TextField::new("Owner", Gallery::owner()))
            .child(
                dialog_footer().child(
                    FILTERS
                        .close(ButtonVariant::Default, ButtonSize::Default)
                        .text("Apply"),
                ),
            ),
    )
}

fn overlays() -> El<Gallery> {
    let status = Gallery::status();
    let archived = Gallery::archived();
    panel("Dialogs and menus")
        .child(
            div()
                .css(wrap(rem(0.5)))
                .child(
                    EDIT.trigger(ButtonVariant::Outline, ButtonSize::Default)
                        .id("open-edit")
                        .text("Edit profile"),
                )
                .child(
                    DELETE
                        .trigger(ButtonVariant::Destructive, ButtonSize::Default)
                        .id("open-delete")
                        .text("Delete project"),
                )
                .child(
                    ACTIONS
                        .trigger(ButtonVariant::Outline, ButtonSize::Default)
                        .text("Project actions"),
                )
                .child(
                    FILTERS
                        .trigger(ButtonVariant::Outline, ButtonSize::Default)
                        .id("open-filters")
                        .text("Filters"),
                )
                .child(
                    ACTIONS.menu([
                        MenuItem::new("Rename", status.set("Renamed the project.")),
                        MenuItem::new("Duplicate", status.set("Duplicated the project.")),
                        MenuItem::new(
                            "Archive",
                            archived.set(true).then(status.set("Archived the project.")),
                        )
                        .disabled(archived),
                        MenuItem::new("Delete", status.set("Deleted the project.")).destructive(),
                    ]),
                ),
        )
        .child(
            p().id("status")
                .attr(attr::ROLE, "status")
                .css(Style::new().color(token::muted_foreground().var()))
                .bind_text(status),
        )
        .child(
            EDIT.dialog(
                "Edit profile",
                "Make changes to your profile, then press Done.",
                div()
                    .css(stack(rem(1)))
                    .child(TextField::new("Display name", Gallery::nickname()))
                    .child(
                        dialog_footer()
                            .child(
                                EDIT.close(ButtonVariant::Outline, ButtonSize::Default)
                                    .text("Cancel"),
                            )
                            .child(
                                EDIT.close(ButtonVariant::Default, ButtonSize::Default)
                                    .text("Done"),
                            ),
                    ),
            ),
        )
        .child(filters())
        .child(
            DELETE.alert_dialog(
                "Delete this project?",
                "The project and its deployments are removed. This cannot be undone.",
                dialog_footer()
                    .child(
                        DELETE
                            .close(ButtonVariant::Outline, ButtonSize::Default)
                            .text("Cancel"),
                    )
                    .child(
                        DELETE
                            .close(ButtonVariant::Destructive, ButtonSize::Default)
                            .on_click(status.set("Deleted the project."))
                            .text("Delete"),
                    ),
            ),
        )
}

fn feedback() -> El<Gallery> {
    let percent = Gallery::progress();
    panel("Feedback")
        .child(
            alert(AlertVariant::Default)
                .child(alert_title().text("Deployments are paused"))
                .child(alert_description().text("They resume at 18:00 UTC.")),
        )
        .child(
            alert(AlertVariant::Destructive)
                .child(alert_title().text("Build failed"))
                .child(alert_description().text("The last commit does not compile.")),
        )
        .child(progress("Upload", percent))
        .child(toasts())
        .child(
            div()
                .css(wrap(rem(0.5)))
                .child(
                    button(ButtonVariant::Secondary, ButtonSize::Sm)
                        .id("less")
                        .on_click(percent.set((percent - 10).max(0)))
                        .text("−10%"),
                )
                .child(
                    button(ButtonVariant::Secondary, ButtonSize::Sm)
                        .id("more")
                        .on_click(percent.set((percent + 10).min(100)))
                        .text("+10%"),
                ),
        )
        .child(
            div()
                .css(stack(rem(0.5)))
                .child(skeleton().css(Style::new().height(rem(1)).width(rem(16))))
                .child(skeleton().css(Style::new().height(rem(1)).width(rem(12)))),
        )
}

fn toasts() -> El<Gallery> {
    let toasts = Gallery::toasts();
    div()
        .css(wrap(rem(0.5)))
        .child(
            button(ButtonVariant::Outline, ButtonSize::Sm)
                .id("toast")
                .on_click(
                    Toast::new("Event created")
                        .description("Sunday, December 3 at 9:00")
                        .push(toasts),
                )
                .text("Show a toast"),
        )
        .child(
            button(ButtonVariant::Outline, ButtonSize::Sm)
                .id("toast-error")
                .on_click(
                    Toast::new("Upload failed")
                        .description("The file is larger than 10 MB.")
                        .variant(ToastVariant::Destructive)
                        .push(toasts),
                )
                .text("Show an error"),
        )
        .child(
            button(ButtonVariant::Outline, ButtonSize::Sm)
                .id("toast-name")
                .on_click(Toast::push_expr(
                    toasts,
                    Expr::lit(&Toast::new("Name")).with(
                        Toast::body(),
                        Expr::text("Your name is ").concat(Gallery::name()),
                    ),
                ))
                .text("Show the name"),
        )
        .child(Toaster::new(toasts))
}

/// The desktop and mobile visitors of each month, on `chart`.
fn visitors(chart: Chart<Gallery, Month>) -> Chart<Gallery, Month> {
    chart
        .series("Desktop", Month::desktop(), token::chart_1())
        .series("Mobile", Month::mobile(), token::chart_2())
        .format(NumberFormat::new())
}

fn first_half() -> Vec<Month> {
    months([
        ("January", 186, 80),
        ("February", 305, 200),
        ("March", 237, 120),
        ("April", 73, 190),
        ("May", 209, 130),
        ("June", 214, 140),
    ])
}

fn months<const N: usize>(rows: [(&str, i32, i32); N]) -> Vec<Month> {
    rows.into_iter()
        .map(|(name, desktop, mobile)| Month {
            name: name.to_owned(),
            desktop,
            mobile,
            lead: desktop - mobile,
        })
        .collect()
}

/// Replaces the months with the second half of the year. The charts fit
/// their scales to the new rows.
async fn next_half((): (), (): ()) -> Patches<Gallery> {
    let rows = months([
        ("July", 520, 310),
        ("August", 610, 420),
        ("September", 480, 390),
        ("October", 700, 450),
        ("November", 655, 500),
        ("December", 820, 610),
    ]);
    Gallery::months().patch(rows).into()
}

fn charts() -> El<Gallery> {
    let first_desktop = Gallery::months().update_where(
        |month| month.index().equals(0),
        Month::desktop(),
        |month| month.at(Month::desktop()) + 50,
    );
    panel("Charts")
        .child(
            div()
                .css(wrap(rem(0.5)))
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(first_desktop)
                        .text("Add 50 desktop visitors in the first month"),
                )
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(NEXT_HALF.call())
                        .text("Show the second half"),
                ),
        )
        .child(
            div()
                .css(grid(1, rem(1.5)).on(Media::Md, grid(2, rem(1.5))))
                .child(div().id("bar-chart").child(visitors(Chart::bar(
                    "Visitors (bars)",
                    Gallery::months(),
                    Month::name(),
                ))))
                .child(div().id("line-chart").child(visitors(Chart::line(
                    "Visitors (line)",
                    Gallery::months(),
                    Month::name(),
                ))))
                .child(div().id("area-chart").child(visitors(Chart::area(
                    "Visitors (area)",
                    Gallery::months(),
                    Month::name(),
                ))))
                .child(
                    div().id("stacked-chart").child(
                        visitors(Chart::stacked_bar(
                            "Visitors (stacked)",
                            Gallery::months(),
                            Month::name(),
                        ))
                        .max_labels(3),
                    ),
                )
                .child(
                    div()
                        .id("horizontal-chart")
                        .child(visitors(Chart::horizontal_bar(
                            "Visitors (horizontal)",
                            Gallery::months(),
                            Month::name(),
                        ))),
                )
                .child(div().id("donut-chart").child(DonutChart::new(
                    "Visitors by browser",
                    Gallery::browsers(),
                    Browser::name(),
                    Browser::visitors(),
                )))
                .child(
                    div().id("lead-chart").child(
                        Chart::bar("Desktop lead", Gallery::months(), Month::name())
                            .series("Lead", Month::lead(), token::chart_3())
                            .format(NumberFormat::new()),
                    ),
                ),
        )
}

fn payments() -> El<Gallery> {
    let table = DataTable::new(
        "Payments",
        Gallery::payments_table(),
        Gallery::payments(),
        |payment| payment.at(Payment::id()),
    )
    .selectable(Gallery::chosen_payments(), |payment| {
        payment.at(Payment::id())
    })
    .expandable(
        Gallery::open_payments(),
        |payment| payment.at(Payment::id()),
        |payment| {
            p().text("Payment ")
                .child(span().bind_text(payment.at(Payment::id()).to_text()))
                .text(" from ")
                .child(span().bind_text(payment.at(Payment::email())))
                .text(" is ")
                .child(span().bind_text(payment.at(Payment::status())))
                .text(".")
        },
    )
    .filter("Filter payments", |payment| {
        payment
            .at(Payment::email())
            .concat(" ")
            .concat(payment.at(Payment::status()))
    })
    .paginate(5)
    .sortable_column(
        Column::text("Status", Payment::status()).visible(Gallery::show_status()),
        |payment| payment.at(Payment::status()),
    )
    .sortable_column(Column::text("Email", Payment::email()), |payment| {
        payment.at(Payment::email())
    })
    .sortable_column(
        Column::formatted("Amount", Payment::amount(), DOLLARS),
        |payment| payment.at(Payment::amount()),
    );
    panel("Data table")
        .child(
            div()
                .css(wrap(rem(1)))
                .child(switch("Show the status column", Gallery::show_status()))
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(
                            Gallery::payments()
                                .retain(|payment| {
                                    Gallery::chosen_payments()
                                        .count(|id| id.value().equals(payment.at(Payment::id())))
                                        .equals(0)
                                })
                                .then(Gallery::chosen_payments().clear()),
                        )
                        .text("Delete selected"),
                ),
        )
        .child(table)
}

fn server_table() -> El<Gallery> {
    panel("Server table")
        .child(
            p().id("chosen-orders")
                .bind_text(Gallery::chosen_orders().len().to_text().concat(" selected")),
        )
        .child(
            ServerTable::new(
                "Orders",
                Gallery::orders_table(),
                Gallery::orders(),
                |order| order.at(Order::id()),
                ORDERS.call_with(Gallery::orders_table()),
            )
            .selectable(Gallery::chosen_orders(), |order| order.at(Order::id()))
            .filter("Filter orders")
            .max_height(rem(11))
            .paginate(5, Gallery::orders_total())
            .column(Column::number("Id", Order::id()))
            .sortable_column(Column::text("Email", Order::email()), &OrderBy::Email)
            .sortable_column(
                Column::formatted("Amount", Order::amount(), DOLLARS),
                &OrderBy::Amount,
            ),
        )
}

/// The 42 orders the server holds. The table receives five at a time.
fn all_orders() -> Vec<Order> {
    (1..=42)
        .map(|id| Order {
            id,
            email: format!("customer{id:02}@example.com"),
            amount: f64::from((id * 37) % 100 * 10) + 0.5,
        })
        .collect()
}

/// The page of orders `state` names, and how many pass its filter.
fn query_orders(state: &TableState) -> (Vec<Order>, i32) {
    let query = state.query.trim().to_lowercase();
    let mut rows: Vec<Order> = all_orders()
        .into_iter()
        .filter(|order| order.email.contains(&query))
        .collect();
    if let Some((by, order)) = state.sorting::<OrderBy>() {
        match by {
            OrderBy::Email => rows.sort_by(|a, b| a.email.cmp(&b.email)),
            OrderBy::Amount => rows.sort_by(|a, b| a.amount.total_cmp(&b.amount)),
        }
        if order == SortOrder::Descending {
            rows.reverse();
        }
    }
    let total = i32::try_from(rows.len()).unwrap_or(i32::MAX);
    let start = usize::try_from(state.page * 5).unwrap_or(0);
    (rows.into_iter().skip(start).take(5).collect(), total)
}

async fn load_orders((): (), state: TableState) -> Patches<Gallery> {
    let (rows, total) = query_orders(&state);
    Gallery::orders()
        .patch(rows)
        .and(Gallery::orders_total().patch(total))
        .into()
}

/// Twelve payments, enough for three pages of five.
fn sample_payments() -> Vec<Payment> {
    let people = [
        ("ken99@example.com", "success", 316.0),
        ("abe45@example.com", "success", 242.0),
        ("monserrat44@example.com", "processing", 837.0),
        ("silas22@example.com", "success", 874.0),
        ("carmella@example.com", "failed", 721.0),
        ("jason78@example.com", "pending", 450.0),
        ("sarah23@example.com", "processing", 120.5),
        ("olivia56@example.com", "success", 980.0),
        ("liam34@example.com", "failed", 65.25),
        ("emma12@example.com", "pending", 310.0),
        ("noah90@example.com", "success", 555.0),
        ("ava67@example.com", "processing", 199.99),
    ];
    people
        .into_iter()
        .zip(1..)
        .map(|((email, status, amount), id)| Payment {
            id,
            email: email.to_owned(),
            status: status.to_owned(),
            amount,
        })
        .collect()
}

fn browsers() -> Vec<Browser> {
    [
        ("Chrome", 275),
        ("Safari", 200),
        ("Firefox", 187),
        ("Edge", 173),
        ("Other", 90),
    ]
    .into_iter()
    .map(|(name, visitors)| Browser {
        name: name.to_owned(),
        visitors,
    })
    .collect()
}

async fn index((): (), Extension(nonce): Extension<Nonce>, (): ()) -> Page<Gallery> {
    gallery().csp_nonce(nonce.0)
}

/// The page, before the nonce of its response.
fn gallery() -> Page<Gallery> {
    let (orders, orders_total) = query_orders(&TableState::default());
    Page::new(
        "hypersig_ui",
        Gallery {
            scheme: Scheme::System,
            name: String::new(),
            nickname: String::new(),
            owner: String::new(),
            email: String::new(),
            bio: String::new(),
            seats: 2,
            email_error: String::new(),
            saving: false,
            saved: String::new(),
            preferences: Preferences {
                notifications: true,
                newsletter: false,
            },
            plan: Plan::Pro,
            region: Region::Tokyo,
            section: Section::Account,
            progress: 40,
            archived: false,
            status: String::new(),
            font: Font::Sans,
            country: ComboboxState::default(),
            volume: 40,
            toasts: Vec::new(),
            payments: sample_payments(),
            payments_table: TableState::default(),
            chosen_payments: Vec::new(),
            open_payments: Vec::new(),
            show_status: true,
            orders,
            orders_total,
            orders_table: TableState::default(),
            chosen_orders: Vec::new(),
            months: first_half(),
            browsers: browsers(),
        },
    )
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
    // Command+K on a Mac and Ctrl+K elsewhere opens the profile dialog from anywhere.
    .on_key(html::Key::Char('k').primary(), EDIT.open())
    .child(
        div()
            .id("app")
            .css(
                Style::new()
                    .min_height(raw("100vh"))
                    .background_color(token::background().var())
                    .color(token::foreground().var()),
            )
            .css_match(
                Gallery::scheme(),
                [
                    (Scheme::Light, hypersig_ui::light()),
                    (Scheme::Dark, hypersig_ui::dark()),
                ],
            )
            .child(
                html::main()
                    .css(
                        container(rem(56), rem(1.5))
                            .padding_block(rem(2.5))
                            .merge(stack(rem(2.5))),
                    )
                    .child(top())
                    .child(buttons())
                    .child(
                        panel("Forms").child(
                            div()
                                .css(grid(1, rem(1.5)).on(Media::Md, grid(2, rem(1.5))))
                                .child(profile())
                                .child(choices()),
                        ),
                    )
                    .child(pickers())
                    .child(settings())
                    .child(faq())
                    .child(overlays())
                    .child(payments())
                    .child(server_table())
                    .child(charts())
                    .child(feedback()),
            ),
    )
}

/// The nonce of one response, which its `<style>` elements carry.
#[derive(Clone)]
struct Nonce(String);

/// Gives each request a fresh nonce and answers with a Content-Security-Policy
/// that allows a style only with that nonce and a script only from this origin.
async fn content_security_policy(mut request: Request, next: Next) -> Response {
    let mut bytes = [0_u8; 16];
    getrandom::fill(&mut bytes).expect("the operating system provides random bytes");
    // Hex digits are a subset of the base64 alphabet a nonce is written in.
    let nonce = format!("{:032x}", u128::from_ne_bytes(bytes));
    let policy = format!(
        "default-src 'self'; script-src 'self'; style-src 'nonce-{nonce}'; img-src 'self' data:; frame-ancestors 'self'"
    );
    request.extensions_mut().insert(Nonce(nonce));
    let mut response = next.run(request).await;
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_str(&policy).expect("the policy is ASCII"),
    );
    response
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(SAVE, save)
        .action(NEXT_HALF, next_half)
        .action(ORDERS, load_orders)
        .layer(middleware::from_fn(content_security_policy));
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3005".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}

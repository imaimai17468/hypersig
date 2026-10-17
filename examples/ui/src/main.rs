//! The components of `hypersig_ui` on one page: a form whose email the
//! server checks, toggles, pickers, tabs, an accordion, dialogs, a menu,
//! tooltips, toasts, a data table, and the static pieces.
//!
//! Run with `cargo run -p example-ui` and open <http://localhost:3005>.

use axum::Router;
use hypersig::html::{self, El, Orientation, div, form, h1, h2, header, p, section, span};
use hypersig::style::Media;
use hypersig::style::Style;
use hypersig::style::pattern::{container, grid, hstack, stack, wrap};
use hypersig::style::value::{FontWeight, JustifyContent, raw, rem};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals, if_else};
use hypersig_ui::{
    Accordion, AlertVariant, Autocomplete, BadgeVariant, ButtonSize, ButtonVariant, Column,
    Combobox, ComboboxState, DataTable, Dialog, InputKind, Level, Listbox, Menu, MenuItem, Slider,
    TableState, Tabs, TextField, Toast, ToastVariant, Toaster, Tooltip, alert, alert_description,
    alert_title, badge, button, card, card_content, card_description, card_footer, card_header,
    card_title, checkbox, dialog_footer, progress, radio_group, select_field, separator, skeleton,
    submit_button, switch, token,
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

/// One row of the data table.
#[derive(Signals, Serialize, Deserialize)]
struct Payment {
    id: i32,
    email: String,
    status: String,
    amount: f64,
    selected: bool,
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
    email: String,
    bio: String,
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
    next_toast: i32,
    payments: Vec<Payment>,
    payments_table: TableState,
    show_status: bool,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

const SAVE: Action<Gallery, (String, String)> = Action::post("/save");
const EDIT: Dialog<Gallery> = Dialog::new("edit-profile");
const DELETE: Dialog<Gallery> = Dialog::new("delete-project");
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
                        .attr("disabled", "")
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
                    ),
            )
            .child(
                card_footer()
                    .child(
                        submit_button(ButtonVariant::Default, ButtonSize::Default)
                            .attr_when("disabled", saving)
                            .bind_text(if_else(saving, "Saving…", "Save")),
                    )
                    .child(
                        span()
                            .id("saved")
                            .attr("role", "status")
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
                .attr("role", "status")
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
    let next = Gallery::next_toast();
    div()
        .css(wrap(rem(0.5)))
        .child(
            button(ButtonVariant::Outline, ButtonSize::Sm)
                .id("toast")
                .on_click(
                    Toast::new("Event created")
                        .description("Sunday, December 3 at 9:00")
                        .push(toasts, next)
                        .then(next.set(next + 1)),
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
                        .push(toasts, next)
                        .then(next.set(next + 1)),
                )
                .text("Show an error"),
        )
        .child(Toaster::new(toasts))
}

fn payments() -> El<Gallery> {
    let table = DataTable::new(
        "Payments",
        Gallery::payments_table(),
        Gallery::payments(),
        |payment| payment.at(Payment::id()),
    )
    .selectable(Payment::selected())
    .filter("Filter payments", |payment| {
        payment
            .at(Payment::email())
            .concat(" ")
            .concat(payment.at(Payment::status()))
    })
    .paginate(5)
    .column(
        Column::text("Status", Payment::status())
            .sort_by(|payment| payment.at(Payment::status()))
            .visible(Gallery::show_status()),
    )
    .column(Column::text("Email", Payment::email()).sort_by(|payment| payment.at(Payment::email())))
    .column(
        Column::number("Amount", Payment::amount())
            .sort_by(|payment| payment.at(Payment::amount())),
    );
    panel("Data table")
        .child(
            div()
                .css(wrap(rem(1)))
                .child(switch("Show the status column", Gallery::show_status()))
                .child(
                    button(ButtonVariant::Outline, ButtonSize::Sm)
                        .on_click(
                            Gallery::payments().retain(|payment| !payment.at(Payment::selected())),
                        )
                        .text("Delete selected"),
                ),
        )
        .child(table)
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
            selected: false,
        })
        .collect()
}

async fn index((): (), (): ()) -> Page<Gallery> {
    Page::new(
        "hypersig_ui",
        Gallery {
            scheme: Scheme::System,
            name: String::new(),
            nickname: String::new(),
            email: String::new(),
            bio: String::new(),
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
            next_toast: 1,
            payments: sample_payments(),
            payments_table: TableState::default(),
            show_status: true,
        },
    )
    .global(hypersig_ui::global())
    .theme(hypersig_ui::theme())
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
                    .child(feedback()),
            ),
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new()).hypersig_runtime().action(SAVE, save);
    let addr = std::env::var("ADDR").unwrap_or_else(|_| "127.0.0.1:3005".to_owned());
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("listening on http://{addr}");
    axum::serve(listener, app).await
}

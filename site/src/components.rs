//! The Components section: setting up `hypersig_ui`, theming it, and one
//! page per component.
//!
//! A component page shows the component running with its source, says what
//! it is built on, lists the keys it answers to, and links its items in the
//! rustdoc output under `api/`, as the Reference section does.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

/// One item: its name, its rustdoc page and anchor under `api/`, and its
/// summary.
type Row = (&'static str, &'static str, &'static str);

/// What a component page holds.
struct Page {
    lead: &'static str,
    demo: Demo,
    /// What the component is built on and what assistive technology hears.
    built_on: &'static [&'static str],
    /// Each key and what it does, empty where the component takes no keys.
    keys: &'static [(&'static str, &'static str)],
    items: &'static [Row],
}

/// The article of `page`.
pub fn content(site: Site, page: Component) -> Part {
    match page {
        Component::Installation => installation(site),
        Component::Theming => theming(site),
        _ => component(site, page, &component_page(page)),
    }
}

fn component(site: Site, page: Component, content: &Page) -> Part {
    let mut children = vec![
        title(Doc::Components(page).title()),
        lead(site, content.lead),
        demo::show(site, content.demo),
        heading("Accessibility"),
    ];
    children.extend(content.built_on.iter().map(|text| para(site, text)));
    if !content.keys.is_empty() {
        let rows: Vec<[&str; 2]> = content
            .keys
            .iter()
            .map(|&(key, action)| [key, action])
            .collect();
        let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
        children.push(table_of(site, &["Key", "What it does"], &rows));
    }
    children.push(heading("API"));
    let cells: Vec<[String; 2]> = content
        .items
        .iter()
        .map(|(name, target, summary)| [format!("[`{name}`](api:{target})"), (*summary).to_owned()])
        .collect();
    let rows: Vec<Vec<&str>> = cells
        .iter()
        .map(|[name, summary]| vec![name.as_str(), summary.as_str()])
        .collect();
    let rows: Vec<&[&str]> = rows.iter().map(Vec::as_slice).collect();
    children.push(table_of(site, &["Item", "Summary"], &rows));
    parts(children)
}

fn installation(site: Site) -> Part {
    parts([
        title(Doc::Components(Component::Installation).title()),
        lead(
            site,
            "`hypersig_ui` is a set of components that look like shadcn/ui and take their behavior from the browser's native elements and the WAI-ARIA Authoring Practices.",
        ),
        heading("Add the crate"),
        para(
            site,
            "It lives in the hypersig repository and is not on crates.io yet. Add it next to `hypersig`:",
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = { git = "https://github.com/imaimai17468/hypersig" }
hypersig_ui = { git = "https://github.com/imaimai17468/hypersig" }"#,
        ),
        heading("Add the theme to the page"),
        para(
            site,
            "Add `hypersig_ui::global()` and `hypersig_ui::theme()` to the page once. `global` is a small reset with the font and the page colors, and `theme` switches the color tokens to their dark values for a visitor whose system prefers dark.",
        ),
        show(Snippet::UiSetup),
        heading("How the components are built"),
        bullets(
            site,
            &[
                "Each component is a function or a small builder that returns an `El<S>`, so it takes handlers, styles, and children like any other element.",
                "The behavior a screen reader and a keyboard depend on comes from the browser where the browser has it: a native `<button>`, `<input>`, `<select>`, `<details>`, `<dialog>`, or `popover`. Where it does not, the component follows the WAI-ARIA Authoring Practices, with keyboard handling from `El::roving_focus` and `El::active_descendant`.",
                "The ids that tie a label to its control, or a tab to its panel, are derived from the signal the component is bound to. A dialog, a menu, a tooltip, and a combobox take their names in a `const` handle that their parts share, and a name that an id cannot hold is a compile error.",
                "The colors follow shadcn/ui's zinc theme. Three values differ, to meet WCAG 2.2 AA: a control's border has 3:1 contrast or more, secondary text keeps 4.5:1 on the muted fill, and focus is a solid outline, which forced-colors mode still draws.",
            ],
        ),
        para(
            site,
            "The demos on these pages run in frames on this static site, so they use signals alone. The [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) example in the repository puts every component on one page with a server, and its end-to-end test drives them with the keyboard.",
        ),
    ])
}

fn theming(site: Site) -> Part {
    parts([
        title(Doc::Components(Component::Theming).title()),
        lead(
            site,
            "The components read their colors and radius from design tokens, which a theme sets for the page and a style sets for one element.",
        ),
        heading("Tokens"),
        para(
            site,
            "The [`token`](api:hypersig_ui/token/index.html) module has one function per token, named as in shadcn/ui: `background`, `foreground`, `primary`, `muted`, `border`, `ring`, `radius`, and the rest. Each returns a `Token`, which is a CSS custom property with a typed default. A page that sets nothing gets the light values.",
        ),
        show(Snippet::UiTheme),
        heading("Light and dark"),
        para(
            site,
            "`hypersig_ui::theme()` sets the dark values under `prefers-color-scheme: dark`. To let a signal pick the scheme instead, put `hypersig_ui::dark()` or `hypersig_ui::light()` on an element with `css_when` or `css_match`. Each sets every color token for the element and those inside it, and `color-scheme` with them, so native controls such as a scrollbar follow.",
        ),
        demo::show(site, Demo::Theme),
        note(
            site,
            "The tokens are plain values of `hypersig::style`. [Tokens and themes](style/tokens/) covers how they are written to the page.",
        ),
    ])
}

/// The content of a component page.
#[expect(clippy::too_many_lines, reason = "one block of data per component")]
fn component_page(page: Component) -> Page {
    match page {
        Component::Installation | Component::Theming => unreachable!("written by hand"),
        Component::Button => Page {
            lead: "A button in one of six variants and three sizes.",
            demo: Demo::Button,
            built_on: &[
                "Each is a native `<button>`, so Enter and Space press it and a screen reader announces it as a button. `button` has `type=\"button\"`, so it does not submit a form it sits in, and `submit_button` is the one that does. Disable either with `.attr_when(\"disabled\", ...)`.",
                "An icon button has no text, so `icon_button` takes the label that names it as an argument. `button_style` gives the same look to an element that is not a button, such as a link.",
            ],
            keys: &[("Enter, Space", "Presses the button.")],
            items: &[
                (
                    "button",
                    "hypersig_ui/fn.button.html",
                    "A `<button type=\"button\">` of `variant` and `size`.",
                ),
                (
                    "submit_button",
                    "hypersig_ui/fn.submit_button.html",
                    "A `<button type=\"submit\">` of `variant` and `size`, which submits the form it is in.",
                ),
                (
                    "icon_button",
                    "hypersig_ui/fn.icon_button.html",
                    "A square button that shows only an icon, named by `label`.",
                ),
                (
                    "button_style",
                    "hypersig_ui/fn.button_style.html",
                    "The style of a button of `variant` and `size`, for an element that is not a `<button>`.",
                ),
                (
                    "ButtonVariant",
                    "hypersig_ui/enum.ButtonVariant.html",
                    "The look of a button.",
                ),
                (
                    "ButtonSize",
                    "hypersig_ui/enum.ButtonSize.html",
                    "The height and padding of a button.",
                ),
            ],
        },
        Component::TextField => Page {
            lead: "A text field with its label, and where given, a description and an error message tied to it.",
            demo: Demo::TextField,
            built_on: &[
                "It is a `<label>` and an `<input>`, or a `<textarea>` with `multiline`. The ids that tie them together are derived from the signal's path, so a label always names its own field. `aria-describedby` points at the description and the error, and `aria-invalid` is `true` while the error is not empty.",
                "The error sits in an `aria-live=\"polite\"` region, so a screen reader reads a new message when it appears. A required field shows an asterisk that a screen reader skips, since it reads \"required\" from the field. `autocomplete` tells the browser what the field holds, for WCAG 1.3.5.",
            ],
            keys: &[],
            items: &[
                (
                    "TextField::new",
                    "hypersig_ui/struct.TextField.html#method.new",
                    "A single-line text field labeled `label`, bound to `value`.",
                ),
                (
                    "kind",
                    "hypersig_ui/struct.TextField.html#method.kind",
                    "Sets the `type` of the input. A multiline field ignores it.",
                ),
                (
                    "multiline",
                    "hypersig_ui/struct.TextField.html#method.multiline",
                    "Makes the field a `<textarea>` that the visitor can resize vertically.",
                ),
                (
                    "autocomplete",
                    "hypersig_ui/struct.TextField.html#method.autocomplete",
                    "Tells the browser what the field holds, so that it can fill it in.",
                ),
                (
                    "placeholder",
                    "hypersig_ui/struct.TextField.html#method.placeholder",
                    "Shows `text` in the empty field.",
                ),
                (
                    "description",
                    "hypersig_ui/struct.TextField.html#method.description",
                    "Adds a line under the field that `aria-describedby` points at.",
                ),
                (
                    "error",
                    "hypersig_ui/struct.TextField.html#method.error",
                    "Shows `message` under the field and marks the field invalid while `message` is not empty.",
                ),
                (
                    "required",
                    "hypersig_ui/struct.TextField.html#method.required",
                    "Marks the field `required`, which the label shows with an asterisk.",
                ),
                (
                    "disabled",
                    "hypersig_ui/struct.TextField.html#method.disabled",
                    "Disables the field while `when` is true.",
                ),
                (
                    "InputKind",
                    "hypersig_ui/enum.InputKind.html",
                    "The `type` of a single-line `TextField`.",
                ),
                (
                    "Autocomplete",
                    "hypersig_ui/enum.Autocomplete.html",
                    "What a `TextField` holds, as `autocomplete` names it.",
                ),
            ],
        },
        Component::Checkbox => Page {
            lead: "A checkbox and an on and off switch, each bound to a `bool` signal.",
            demo: Demo::Checkbox,
            built_on: &[
                "Both are a native `<input type=\"checkbox\">` inside the `<label>` that holds the text, so a click on the text toggles them and they need no id. That also lets them sit on a field of a list item.",
                "The switch has `role=\"switch\"`, so a screen reader says \"switch, on\" rather than \"checkbox, checked\".",
            ],
            keys: &[("Space", "Toggles the checkbox or the switch.")],
            items: &[
                (
                    "checkbox",
                    "hypersig_ui/fn.checkbox.html",
                    "A checkbox labeled `text`, bound to `checked`.",
                ),
                (
                    "switch",
                    "hypersig_ui/fn.switch.html",
                    "An on and off switch labeled `text`, bound to `on`.",
                ),
            ],
        },
        Component::RadioGroup => Page {
            lead: "A group of radio buttons that sets a signal to the chosen option's value.",
            demo: Demo::RadioGroup,
            built_on: &[
                "The radios share a `name` derived from the signal's path, so the browser gives the group one Tab stop and moves between its radios with the arrow keys. The `<fieldset>` and `<legend>` name the group for a screen reader.",
            ],
            keys: &[
                ("Tab", "Moves focus into the group, to the checked radio."),
                (
                    "Arrow keys",
                    "Move to the next or previous radio and check it.",
                ),
            ],
            items: &[(
                "radio_group",
                "hypersig_ui/fn.radio_group.html",
                "A group of radio buttons under the legend `title`, one per option, that sets `selected` to the option's value.",
            )],
        },
        Component::Select => Page {
            lead: "A select that sets a signal to the chosen option's value.",
            demo: Demo::Select,
            built_on: &[
                "It is a native `<select>`, which a screen reader, a keyboard, and a phone's picker wheel already handle. The closed control is drawn like a text field with a chevron.",
                "Where the browser supports `appearance: base-select` and `::picker(select)`, which browser-compat-data 8.1.6 lists for Chrome 135 and Safari 27, the open list is drawn as a popover card too. Elsewhere it is the browser's own picker.",
            ],
            keys: &[(
                "The browser's keys",
                "A native `<select>` takes the keys of the platform, which differ between systems.",
            )],
            items: &[(
                "select_field",
                "hypersig_ui/fn.select_field.html",
                "A select labeled `title`, offering `options`, that sets `selected` to the chosen option's value.",
            )],
        },
        Component::Slider => Page {
            lead: "A slider that sets an `i32` signal, with a value text for a unit.",
            demo: Demo::Slider,
            built_on: &[
                "It is a native `<input type=\"range\">`, so the browser gives it `role=\"slider\"`, its keys, and dragging with a pointer or a finger. The input is transparent and lies over a track and a thumb drawn from the value, so the slider looks the same in every browser.",
                "`value_text` shows a value such as \"40 %\" beside the label and gives it to assistive technology as `aria-valuetext`. The drawn thumb can sit a few pixels off the browser's own transparent thumb near either end.",
            ],
            keys: &[
                ("Arrow keys", "Move the value by one step."),
                ("Page Up, Page Down", "Move the value by a larger step."),
                ("Home, End", "Move to the lowest or the highest value."),
            ],
            items: &[
                (
                    "Slider::new",
                    "hypersig_ui/struct.Slider.html#method.new",
                    "A slider from 0 to 100 in steps of 1, labeled `label`, bound to `value`.",
                ),
                (
                    "range",
                    "hypersig_ui/struct.Slider.html#method.range",
                    "Sets the lowest and the highest value.",
                ),
                (
                    "step",
                    "hypersig_ui/struct.Slider.html#method.step",
                    "Sets how far one arrow key press moves the value.",
                ),
                (
                    "value_text",
                    "hypersig_ui/struct.Slider.html#method.value_text",
                    "Shows `text` beside the label and gives it to assistive technology as `aria-valuetext`.",
                ),
            ],
        },
        Component::Listbox => Page {
            lead: "A list of options of which one is selected, kept in a signal.",
            demo: Demo::Listbox,
            built_on: &[
                "It follows the WAI-ARIA listbox pattern for a single selection. The list has `role=\"listbox\"` and each option `role=\"option\"` with `aria-selected`. Focus moves between the options through `El::roving_focus`, and each option is selected as it gains focus. The ids are derived from the signal's path and the option's position.",
            ],
            keys: &[
                (
                    "Tab",
                    "Moves focus into the list, to the selected option, or to the first where none is selected.",
                ),
                (
                    "Up, Down",
                    "Move to the previous or next option and select it.",
                ),
                (
                    "Home, End",
                    "Move to the first or last option and select it.",
                ),
            ],
            items: &[
                (
                    "Listbox::new",
                    "hypersig_ui/struct.Listbox.html#method.new",
                    "An empty list named `label` for assistive technology.",
                ),
                (
                    "option",
                    "hypersig_ui/struct.Listbox.html#method.option",
                    "Adds an option labeled `title` that selects `value`.",
                ),
            ],
        },
        Component::Combobox => Page {
            lead: "A text field that suggests options from a list as the visitor types.",
            demo: Demo::Combobox,
            built_on: &[
                "It follows the WAI-ARIA combobox pattern with list autocomplete. The text field has `role=\"combobox\"`, `aria-expanded`, `aria-controls`, and `aria-activedescendant`, and focus stays in it while the arrows move through the options shown, through `El::active_descendant`. Typing shows the options that contain the text, ignoring case.",
                "Declare the combobox as a `const` naming its text field and its list, and keep its state in a `ComboboxState` field of the page's signals. The chosen text is `ComboboxState::text`. The visitor can leave text that matches no option, which the page checks where it needs one of the options.",
            ],
            keys: &[
                (
                    "Typing",
                    "Opens the list and shows the options that contain the text.",
                ),
                (
                    "Down",
                    "Opens the list, then moves to the next option shown.",
                ),
                ("Up", "Moves to the previous option shown."),
                (
                    "Enter",
                    "Chooses the option the arrows reached and closes the list.",
                ),
                (
                    "Escape",
                    "Closes the list, or clears the text when it is closed. A dialog around the combobox does not close on it from the text field.",
                ),
            ],
            items: &[
                (
                    "Combobox::new",
                    "hypersig_ui/struct.Combobox.html#method.new",
                    "A combobox whose text field has the id `field` and whose list has the id `list`.",
                ),
                (
                    "render",
                    "hypersig_ui/struct.Combobox.html#method.render",
                    "The label, the text field, and the list of `options`, keeping their state in `state`.",
                ),
                (
                    "ComboboxState",
                    "hypersig_ui/struct.ComboboxState.html",
                    "The signals a `Combobox` keeps.",
                ),
                (
                    "ComboboxState::new",
                    "hypersig_ui/struct.ComboboxState.html#method.new",
                    "A closed combobox whose text field holds `text`.",
                ),
            ],
        },
        Component::Tabs => Page {
            lead: "A tab list whose selection is a signal, with one panel per tab.",
            demo: Demo::Tabs,
            built_on: &[
                "It follows the WAI-ARIA tabs pattern. The list has `role=\"tablist\"`, each tab `role=\"tab\"` with `aria-selected` and `aria-controls`, and each panel `role=\"tabpanel\"` labeled by its tab. A tab is selected as it gains focus, through `El::roving_focus`. The ids are derived from the signal's path and the tab's position.",
            ],
            keys: &[
                (
                    "Tab",
                    "Moves focus into the list at the selected tab, then on to the panel.",
                ),
                (
                    "Left, Right",
                    "Move to the previous or next tab and select it.",
                ),
                ("Home, End", "Move to the first or last tab and select it."),
            ],
            items: &[
                (
                    "Tabs::new",
                    "hypersig_ui/struct.Tabs.html#method.new",
                    "An empty tab list named `label` for assistive technology.",
                ),
                (
                    "tab",
                    "hypersig_ui/struct.Tabs.html#method.tab",
                    "Adds a tab titled `title` that selects `value` and shows `panel`.",
                ),
            ],
        },
        Component::Accordion => Page {
            lead: "A stack of sections that open and close, one at a time or each on its own.",
            demo: Demo::Accordion,
            built_on: &[
                "Each section is a native `<details>` whose `<summary>` opens and closes it. The browser handles the keyboard and announces each summary as a button that is expanded or collapsed, and find-in-page opens a closed section that holds a match.",
                "An exclusive accordion gives its sections one `name`, so the browser closes the open one when another opens.",
            ],
            keys: &[("Enter, Space", "Opens or closes the focused section.")],
            items: &[
                (
                    "Accordion::new",
                    "hypersig_ui/struct.Accordion.html#method.new",
                    "An accordion whose sections open and close independently.",
                ),
                (
                    "Accordion::exclusive",
                    "hypersig_ui/struct.Accordion.html#method.exclusive",
                    "An accordion that keeps at most one section open, grouped by `name`.",
                ),
                (
                    "item",
                    "hypersig_ui/struct.Accordion.html#method.item",
                    "Adds a section titled `title` whose content is `content`.",
                ),
            ],
        },
        Component::Card => Page {
            lead: "A bordered surface with a header, content, and a footer.",
            demo: Demo::Card,
            built_on: &[
                "A card is a `<div>` and takes no role. Its title is a heading whose level you choose with `Level`, so that it sits one level below the section around it and a screen reader's list of headings stays in order.",
            ],
            keys: &[],
            items: &[
                (
                    "card",
                    "hypersig_ui/fn.card.html",
                    "A `<div>` with a border, a shadow, and `token::card` behind it.",
                ),
                (
                    "card_header",
                    "hypersig_ui/fn.card_header.html",
                    "The top of a `card`, holding its title and description.",
                ),
                (
                    "card_title",
                    "hypersig_ui/fn.card_title.html",
                    "The title of a `card`, as a heading of `level`.",
                ),
                (
                    "card_description",
                    "hypersig_ui/fn.card_description.html",
                    "A line under the title of a `card`, in `token::muted_foreground`.",
                ),
                (
                    "card_content",
                    "hypersig_ui/fn.card_content.html",
                    "The body of a `card`.",
                ),
                (
                    "card_footer",
                    "hypersig_ui/fn.card_footer.html",
                    "The bottom of a `card`, laying its children out in a row.",
                ),
                (
                    "Level",
                    "hypersig_ui/enum.Level.html",
                    "The level of a heading, which follows the headings around it.",
                ),
            ],
        },
        Component::Separator => Page {
            lead: "A one-pixel line between groups of content, across or along.",
            demo: Demo::Separator,
            built_on: &[
                "A horizontal separator is an `<hr>`, and a vertical one is a `<div>` with `role=\"separator\"` and `aria-orientation=\"vertical\"`, so a screen reader announces both as separators.",
            ],
            keys: &[],
            items: &[(
                "separator",
                "hypersig_ui/fn.separator.html",
                "A one-pixel line in `token::border` that runs along `orientation`.",
            )],
        },
        Component::Dialog => Page {
            lead: "A modal dialog, and an alert dialog that asks the visitor to confirm.",
            demo: Demo::Dialog,
            built_on: &[
                "Declare the dialog as a `const` and use it for the dialog and for the buttons that open and close it. The buttons carry `commandfor` and `command`, and the browser opens the `<dialog>` with `showModal()`: the rest of the page becomes inert, Escape closes it, and focus returns to the button that opened it. Invoker commands ship in Chrome 135, Firefox 144, and Safari 26.2.",
                "The dialog is labeled by its title and described by its description through ids derived from the name. An alert dialog has `role=\"alertdialog\"` and no close button in its corner, so the visitor answers with one of its buttons.",
            ],
            keys: &[
                (
                    "Tab",
                    "Moves between the dialog's controls and never reaches the page behind.",
                ),
                (
                    "Escape",
                    "Closes the dialog and returns focus to the button that opened it.",
                ),
            ],
            items: &[
                (
                    "Dialog::new",
                    "hypersig_ui/struct.Dialog.html#method.new",
                    "A dialog whose id is `name`.",
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Dialog.html#method.trigger",
                    "A button that opens the dialog.",
                ),
                (
                    "close",
                    "hypersig_ui/struct.Dialog.html#method.close",
                    "A button that closes the dialog. Put it inside the dialog.",
                ),
                (
                    "dialog",
                    "hypersig_ui/struct.Dialog.html#method.dialog",
                    "The dialog, with `title`, `description`, `content`, and a close button in its corner.",
                ),
                (
                    "alert_dialog",
                    "hypersig_ui/struct.Dialog.html#method.alert_dialog",
                    "An alert dialog, which asks the visitor to confirm or cancel before going on.",
                ),
                (
                    "dialog_footer",
                    "hypersig_ui/fn.dialog_footer.html",
                    "The row of buttons at the bottom of a dialog, aligned to the end.",
                ),
            ],
        },
        Component::Menu => Page {
            lead: "A menu of actions opened by a button.",
            demo: Demo::Menu,
            built_on: &[
                "It follows the WAI-ARIA menu button pattern: the button has `aria-haspopup=\"menu\"`, the menu `role=\"menu\"`, and each action `role=\"menuitem\"`. The menu is a `popover`, so the browser puts it above the page and closes it on Escape or on a click outside. It is placed under the button with CSS anchor positioning and flips above it where there is no room below.",
                "A disabled action keeps its place, and the arrow keys skip it.",
            ],
            keys: &[
                (
                    "Enter, Space",
                    "On the button, opens the menu and moves focus to the first action. On an action, runs it and closes the menu.",
                ),
                ("Up, Down", "Move to the previous or next action."),
                ("Home, End", "Move to the first or last action."),
                ("Escape", "Closes the menu and returns focus to the button."),
                ("Tab", "Closes the menu."),
            ],
            items: &[
                (
                    "Menu::new",
                    "hypersig_ui/struct.Menu.html#method.new",
                    "A menu whose id is `name`.",
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Menu.html#method.trigger",
                    "The button that opens and closes the menu.",
                ),
                (
                    "menu",
                    "hypersig_ui/struct.Menu.html#method.menu",
                    "The menu, holding `items` in order.",
                ),
                (
                    "MenuItem::new",
                    "hypersig_ui/struct.MenuItem.html#method.new",
                    "An action labeled `label` that runs `run` when chosen.",
                ),
                (
                    "destructive",
                    "hypersig_ui/struct.MenuItem.html#method.destructive",
                    "Draws the action in `token::destructive`, for one that deletes.",
                ),
                (
                    "disabled",
                    "hypersig_ui/struct.MenuItem.html#method.disabled",
                    "Disables the action while `when` is true, which the arrow keys then skip.",
                ),
            ],
        },
        Component::Tooltip => Page {
            lead: "A short description shown while a button is hovered or focused.",
            demo: Demo::Tooltip,
            built_on: &[
                "It follows the WAI-ARIA tooltip pattern: the tooltip has `role=\"tooltip\"` and the button `aria-describedby` pointing at it, so a screen reader reads the text after the button's name. The tooltip builds its own button, which carries the attributes the tooltip needs.",
                "The tooltip is a `popover=\"hint\"`, which browser-compat-data 8.1.6 lists for Chrome 151 and Firefox 153, with an older partial version before them and Safari only in its preview. A browser without it treats the value as `manual`, and there Escape closes the tooltip only while focus is on its button.",
            ],
            keys: &[
                (
                    "Tab",
                    "Focusing the button opens the tooltip, and leaving it closes the tooltip.",
                ),
                (
                    "Any key",
                    "Pressed on the button, closes the tooltip. Escape among them still reaches a dialog around it.",
                ),
            ],
            items: &[
                (
                    "Tooltip::new",
                    "hypersig_ui/struct.Tooltip.html#method.new",
                    "A tooltip whose id is `name`.",
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Tooltip.html#method.trigger",
                    "A button described by the tooltip.",
                ),
                (
                    "icon_trigger",
                    "hypersig_ui/struct.Tooltip.html#method.icon_trigger",
                    "An icon button named `label` and described by the tooltip.",
                ),
                (
                    "tooltip",
                    "hypersig_ui/struct.Tooltip.html#method.tooltip",
                    "The tooltip, holding `text`.",
                ),
            ],
        },
        Component::Toast => Page {
            lead: "Short notices stacked in a corner of the page, added from the browser or by the server.",
            demo: Demo::Toast,
            built_on: &[
                "The toaster is a landmark region named \"Notifications\" holding an `aria-live=\"polite\"` list, so a screen reader reads a toast when it is added, after what it is reading. Each toast has a button that removes it.",
                "A toast stays until it is dismissed, unless `Toaster::dismiss_after` sets a time, because a visitor who reads slowly or uses a magnifier can miss a notice that leaves by itself (WCAG 2.2.1). The region sits in the page rather than in the top layer, so an open modal dialog makes it inert until the dialog closes.",
            ],
            keys: &[("Tab, Enter", "Reach a toast's Dismiss button and press it.")],
            items: &[
                (
                    "Toaster::new",
                    "hypersig_ui/struct.Toaster.html#method.new",
                    "A toaster that shows `toasts`, newest last.",
                ),
                (
                    "label",
                    "hypersig_ui/struct.Toaster.html#method.label",
                    "Names the region for assistive technology, in place of \"Notifications\".",
                ),
                (
                    "dismiss_after",
                    "hypersig_ui/struct.Toaster.html#method.dismiss_after",
                    "Removes each toast `delay` after it appears.",
                ),
                (
                    "Toast::new",
                    "hypersig_ui/struct.Toast.html#method.new",
                    "A toast titled `title`, with the id 0.",
                ),
                (
                    "description",
                    "hypersig_ui/struct.Toast.html#method.description",
                    "Adds a line of text under the title.",
                ),
                (
                    "variant",
                    "hypersig_ui/struct.Toast.html#method.variant",
                    "Sets how the toast is drawn.",
                ),
                (
                    "id",
                    "hypersig_ui/struct.Toast.html#method.id",
                    "Sets the id that tells this toast apart from the others in its list.",
                ),
                (
                    "push",
                    "hypersig_ui/struct.Toast.html#method.push",
                    "A statement that adds this toast to `toasts` with the id `id`.",
                ),
            ],
        },
        Component::Alert => Page {
            lead: "A bordered box for a message, with a title and a description.",
            demo: Demo::Alert,
            built_on: &[
                "Both variants are live regions. A destructive alert has `role=\"alert\"`, which a screen reader reads when it is put on the page, as the demo does with `when`. A default alert has `role=\"status\"`, which is read when its text changes. Some screen readers skip a status that arrives with its text already in it, so keep a status box on the page and change its text with `bind_text`.",
            ],
            keys: &[],
            items: &[
                (
                    "alert",
                    "hypersig_ui/fn.alert.html",
                    "A bordered box for a message, holding an `alert_title` and an `alert_description`.",
                ),
                (
                    "alert_title",
                    "hypersig_ui/fn.alert_title.html",
                    "The first line of an `alert`.",
                ),
                (
                    "alert_description",
                    "hypersig_ui/fn.alert_description.html",
                    "The text under the title of an `alert`.",
                ),
                (
                    "AlertVariant",
                    "hypersig_ui/enum.AlertVariant.html",
                    "The look and the urgency of an alert.",
                ),
            ],
        },
        Component::Badge => Page {
            lead: "A short label, such as a status, drawn as a badge.",
            demo: Demo::Badge,
            built_on: &[
                "A badge is a `<span>` and takes no role, so a screen reader reads it where it stands. Where the color alone carries the meaning, put the meaning in the text too, such as \"Failed\" rather than a red dot.",
            ],
            keys: &[],
            items: &[
                (
                    "badge",
                    "hypersig_ui/fn.badge.html",
                    "A `<span>` that shows a short label, such as a status, as a badge.",
                ),
                (
                    "BadgeVariant",
                    "hypersig_ui/enum.BadgeVariant.html",
                    "The look of a badge.",
                ),
            ],
        },
        Component::Progress => Page {
            lead: "A bar that fills to a percentage, and a block that stands in for content still loading.",
            demo: Demo::Progress,
            built_on: &[
                "The bar is a `role=\"progressbar\"` with `aria-valuenow` kept equal to the value, so a screen reader reads the label and the percentage. A value below 0 or above 100 is drawn and announced as 0 or 100.",
                "A skeleton is hidden from assistive technology. Tell a screen reader that the content is loading elsewhere, as the demo does with `aria-busy` on the region it fills.",
            ],
            keys: &[],
            items: &[
                (
                    "progress",
                    "hypersig_ui/fn.progress.html",
                    "A bar that fills to `percent`, a value from 0 to 100, named by `label`.",
                ),
                (
                    "skeleton",
                    "hypersig_ui/fn.skeleton.html",
                    "A pulsing block in `token::accent` that stands in for content still loading.",
                ),
            ],
        },
    }
}

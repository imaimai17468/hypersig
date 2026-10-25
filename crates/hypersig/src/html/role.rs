//! The roles of WAI-ARIA 1.2 that [`El::role`](super::El::role) writes.

/// A WAI-ARIA role, which [`El::role`](super::El::role) writes into `role`.
///
/// The variants are the concrete roles of WAI-ARIA 1.2, as the npm package
/// `aria-query` 5.3.2 lists them. A screen reader announces an element by
/// its role, so give the element the keyboard behavior the role promises,
/// as the WAI-ARIA Authoring Practices describe for each pattern.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Role {
    /// `alert`.
    Alert,
    /// `alertdialog`.
    AlertDialog,
    /// `application`.
    Application,
    /// `article`.
    Article,
    /// `banner`.
    Banner,
    /// `blockquote`.
    Blockquote,
    /// `button`.
    Button,
    /// `caption`.
    Caption,
    /// `cell`.
    Cell,
    /// `checkbox`.
    Checkbox,
    /// `code`.
    Code,
    /// `columnheader`.
    ColumnHeader,
    /// `combobox`.
    Combobox,
    /// `complementary`.
    Complementary,
    /// `contentinfo`.
    ContentInfo,
    /// `definition`.
    Definition,
    /// `deletion`.
    Deletion,
    /// `dialog`.
    Dialog,
    /// `directory`.
    Directory,
    /// `document`.
    Document,
    /// `emphasis`.
    Emphasis,
    /// `feed`.
    Feed,
    /// `figure`.
    Figure,
    /// `form`.
    Form,
    /// `generic`.
    Generic,
    /// `grid`.
    Grid,
    /// `gridcell`.
    GridCell,
    /// `group`.
    Group,
    /// `heading`.
    Heading,
    /// `img`.
    Img,
    /// `insertion`.
    Insertion,
    /// `link`.
    Link,
    /// `list`.
    List,
    /// `listbox`.
    ListBox,
    /// `listitem`.
    ListItem,
    /// `log`.
    Log,
    /// `main`.
    Main,
    /// `mark`.
    Mark,
    /// `marquee`.
    Marquee,
    /// `math`.
    Math,
    /// `menu`.
    Menu,
    /// `menubar`.
    MenuBar,
    /// `menuitem`.
    MenuItem,
    /// `menuitemcheckbox`.
    MenuItemCheckbox,
    /// `menuitemradio`.
    MenuItemRadio,
    /// `meter`.
    Meter,
    /// `navigation`.
    Navigation,
    /// `none`.
    None,
    /// `note`.
    Note,
    /// `option`.
    Option,
    /// `paragraph`.
    Paragraph,
    /// `presentation`.
    Presentation,
    /// `progressbar`.
    ProgressBar,
    /// `radio`.
    Radio,
    /// `radiogroup`.
    RadioGroup,
    /// `region`.
    Region,
    /// `row`.
    Row,
    /// `rowgroup`.
    RowGroup,
    /// `rowheader`.
    RowHeader,
    /// `scrollbar`.
    ScrollBar,
    /// `search`.
    Search,
    /// `searchbox`.
    SearchBox,
    /// `separator`.
    Separator,
    /// `slider`.
    Slider,
    /// `spinbutton`.
    SpinButton,
    /// `status`.
    Status,
    /// `strong`.
    Strong,
    /// `subscript`.
    Subscript,
    /// `superscript`.
    Superscript,
    /// `switch`.
    Switch,
    /// `tab`.
    Tab,
    /// `table`.
    Table,
    /// `tablist`.
    TabList,
    /// `tabpanel`.
    TabPanel,
    /// `term`.
    Term,
    /// `textbox`.
    TextBox,
    /// `time`.
    Time,
    /// `timer`.
    Timer,
    /// `toolbar`.
    Toolbar,
    /// `tooltip`.
    Tooltip,
    /// `tree`.
    Tree,
    /// `treegrid`.
    TreeGrid,
    /// `treeitem`.
    TreeItem,
}

impl Role {
    /// The role's name, as `role` holds it.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::Alert => "alert",
            Self::AlertDialog => "alertdialog",
            Self::Application => "application",
            Self::Article => "article",
            Self::Banner => "banner",
            Self::Blockquote => "blockquote",
            Self::Button => "button",
            Self::Caption => "caption",
            Self::Cell => "cell",
            Self::Checkbox => "checkbox",
            Self::Code => "code",
            Self::ColumnHeader => "columnheader",
            Self::Combobox => "combobox",
            Self::Complementary => "complementary",
            Self::ContentInfo => "contentinfo",
            Self::Definition => "definition",
            Self::Deletion => "deletion",
            Self::Dialog => "dialog",
            Self::Directory => "directory",
            Self::Document => "document",
            Self::Emphasis => "emphasis",
            Self::Feed => "feed",
            Self::Figure => "figure",
            Self::Form => "form",
            Self::Generic => "generic",
            Self::Grid => "grid",
            Self::GridCell => "gridcell",
            Self::Group => "group",
            Self::Heading => "heading",
            Self::Img => "img",
            Self::Insertion => "insertion",
            Self::Link => "link",
            Self::List => "list",
            Self::ListBox => "listbox",
            Self::ListItem => "listitem",
            Self::Log => "log",
            Self::Main => "main",
            Self::Mark => "mark",
            Self::Marquee => "marquee",
            Self::Math => "math",
            Self::Menu => "menu",
            Self::MenuBar => "menubar",
            Self::MenuItem => "menuitem",
            Self::MenuItemCheckbox => "menuitemcheckbox",
            Self::MenuItemRadio => "menuitemradio",
            Self::Meter => "meter",
            Self::Navigation => "navigation",
            Self::None => "none",
            Self::Note => "note",
            Self::Option => "option",
            Self::Paragraph => "paragraph",
            Self::Presentation => "presentation",
            Self::ProgressBar => "progressbar",
            Self::Radio => "radio",
            Self::RadioGroup => "radiogroup",
            Self::Region => "region",
            Self::Row => "row",
            Self::RowGroup => "rowgroup",
            Self::RowHeader => "rowheader",
            Self::ScrollBar => "scrollbar",
            Self::Search => "search",
            Self::SearchBox => "searchbox",
            Self::Separator => "separator",
            Self::Slider => "slider",
            Self::SpinButton => "spinbutton",
            Self::Status => "status",
            Self::Strong => "strong",
            Self::Subscript => "subscript",
            Self::Superscript => "superscript",
            Self::Switch => "switch",
            Self::Tab => "tab",
            Self::Table => "table",
            Self::TabList => "tablist",
            Self::TabPanel => "tabpanel",
            Self::Term => "term",
            Self::TextBox => "textbox",
            Self::Time => "time",
            Self::Timer => "timer",
            Self::Toolbar => "toolbar",
            Self::Tooltip => "tooltip",
            Self::Tree => "tree",
            Self::TreeGrid => "treegrid",
            Self::TreeItem => "treeitem",
        }
    }
}

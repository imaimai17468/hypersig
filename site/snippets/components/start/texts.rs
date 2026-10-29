use hypersig::{DialogRef, NodeRef, Signals};
use hypersig_ui::{
    ButtonSize, ButtonVariant, Combobox, ComboboxState, ComboboxText, Dialog, DialogText,
    PaginationText, TableText, Toast, Toaster, dialog_footer,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
enum Lang {
    En,
    Ja,
}

#[derive(Signals, Serialize, Deserialize)]
struct Shipping {
    country: ComboboxState,
    toasts: Vec<Toast>,
}

hypersig::node_ref!(const COUNTRY_FIELD: NodeRef<Shipping>);
hypersig::node_ref!(const COUNTRY_OPTIONS: NodeRef<Shipping>);
const COUNTRY: Combobox<Shipping> = Combobox::new(COUNTRY_FIELD, COUNTRY_OPTIONS);

hypersig::dialog_ref!(const HELP_DIALOG: DialogRef<Shipping>);
const HELP: Dialog<Shipping> = Dialog::new(HELP_DIALOG);

// Each struct has a field per word, so a translation that leaves one out
// does not compile.
fn combobox_text(lang: Lang) -> ComboboxText {
    match lang {
        Lang::En => ComboboxText::default(),
        Lang::Ja => ComboboxText {
            list: "候補".into(),
            empty: "一致する候補はありません".into(),
        },
    }
}

fn dialog_text(lang: Lang) -> DialogText {
    match lang {
        Lang::En => DialogText::default(),
        Lang::Ja => DialogText { close: "閉じる".into() },
    }
}

fn pagination_text(lang: Lang) -> PaginationText {
    match lang {
        Lang::En => PaginationText::default(),
        Lang::Ja => PaginationText {
            label: "ページ送り".into(),
            previous: "前へ".into(),
            next: "次へ".into(),
        },
    }
}

fn table_text(lang: Lang) -> TableText {
    match lang {
        Lang::En => TableText::default(),
        Lang::Ja => TableText {
            empty: "該当する行はありません。".into(),
            select_row: "行を選ぶ".into(),
            select_all: "すべての行を選ぶ".into(),
            previous_page: "前のページ".into(),
            next_page: "次のページ".into(),
            expand_row: "行の詳細を表示".into(),
        },
    }
}
# fn main() {

let lang = Lang::Ja;
let country = COUNTRY.render(
    "国",
    Shipping::country(),
    ["フランス", "日本", "ケニア"],
    &combobox_text(lang),
);
let help = HELP.dialog(
    "配送について",
    "3 営業日以内に発送します。",
    dialog_footer().child(HELP.close(ButtonVariant::Default, ButtonSize::Default).text("OK")),
    &dialog_text(lang),
);
// The toast region takes its two names through methods.
let toasts = Toaster::new(Shipping::toasts())
    .label("通知")
    .dismiss_label("閉じる");
# let _ = (pagination_text(lang), table_text(lang));
# assert!(country.render().contains(r#"aria-label="候補""#));
# assert!(help.render().contains(r#"aria-label="閉じる""#));
# assert!(toasts.render().render().contains(r#"aria-label="通知""#));
# }

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
use crate::lang::{L, Text};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

/// One item: its name, its rustdoc page and anchor under `api/`, and its
/// summary.
type Row = (&'static str, &'static str, L<&'static str>);

/// What a component page holds.
struct Page {
    lead: L<&'static str>,
    demo: Demo,
    /// What the component is built on and what assistive technology hears.
    built_on: &'static [L<&'static str>],
    /// Each key and what it does, empty where the component takes no keys.
    keys: &'static [(L<&'static str>, L<&'static str>)],
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
        title(Doc::Components(page).title(site)),
        lead(site, content.lead),
        demo::show(site, content.demo),
    ];
    if matches!(page, Component::Toast) {
        children.extend(toast_from_the_server(site));
    }
    children.push(heading(
        site,
        L {
            en: "Accessibility",
            ja: "アクセシビリティ",
        },
    ));
    children.extend(content.built_on.iter().map(|&text| para(site, text)));
    if !content.keys.is_empty() {
        let rows: Vec<[&str; 2]> = content
            .keys
            .iter()
            .map(|&(key, action)| [site.t(key), site.t(action)])
            .collect();
        let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
        children.push(table_of(
            site,
            L {
                en: &["Key", "What it does"],
                ja: &["キー", "動作"],
            },
            Text::Chosen(rows.as_slice()),
        ));
    }
    children.push(heading(
        site,
        L {
            en: "API",
            ja: "API",
        },
    ));
    let cells: Vec<[String; 2]> = content
        .items
        .iter()
        .map(|&(name, target, summary)| {
            [
                format!("[`{name}`](api:{target})"),
                site.t(summary).to_owned(),
            ]
        })
        .collect();
    let rows: Vec<[&str; 2]> = cells
        .iter()
        .map(|[name, summary]| [name.as_str(), summary.as_str()])
        .collect();
    let rows: Vec<&[&str]> = rows.iter().map(<[&str; 2]>::as_slice).collect();
    children.push(table_of(
        site,
        L {
            en: &["Item", "Summary"],
            ja: &["項目", "概要"],
        },
        Text::Chosen(rows.as_slice()),
    ));
    parts(children)
}

/// The section of the toast page on toasts a handler adds, which the demo
/// cannot show because the site has no server.
fn toast_from_the_server(site: Site) -> [Part; 4] {
    [
        heading(
            site,
            L {
                en: "Add a toast from the server",
                ja: "サーバーからトーストを追加する",
            },
        ),
        para(
            site,
            L {
                en: "A handler adds a toast with `patch_append`, which puts it after the toasts the browser shows, where `patch` would replace them. A failed call shows the server's message when `on_error` passes it to `Toast::push_expr` as `Toast::body()`.",
                ja: "ハンドラは `patch_append` でトーストを追加します。`patch_append` はブラウザが表示中のトーストの後ろに追加し、`patch` はそれらを置き換えます。呼び出しが失敗したときにサーバーのメッセージを表示するには、`on_error` が渡すメッセージを `Toast::push_expr` で `Toast::body()` に入れます。",
            },
        ),
        show(Snippet::ToastServer),
        para(
            site,
            L {
                en: "A toast from `Toast::new` has an id below 0, and a toast the browser pushes has an id above 0, so a toast from the server and one from the browser never share an id.",
                ja: "`Toast::new` で作ったトーストの id は 0 未満で、ブラウザが追加したトーストの id は 0 より大きいので、サーバーのトーストとブラウザのトーストの id は重なりません。",
            },
        ),
    ]
}

fn installation(site: Site) -> Part {
    parts([
        title(Doc::Components(Component::Installation).title(site)),
        lead(site, Doc::Components(Component::Installation).leads()),
        heading(
            site,
            L {
                en: "Add the crate",
                ja: "クレートを追加する",
            },
        ),
        para(
            site,
            L {
                en: "Add it next to `hypersig`:",
                ja: "次のように `hypersig` と並べて追加します。",
            },
        ),
        code::block(
            Lang::Toml,
            Some("Cargo.toml"),
            r#"[dependencies]
hypersig = "0.8"
hypersig_ui = "0.8""#,
        ),
        heading(
            site,
            L {
                en: "Add the theme to the page",
                ja: "ページにテーマを入れる",
            },
        ),
        para(
            site,
            L {
                en: "Add `hypersig_ui::global()` and `hypersig_ui::theme()` to the page once. `global` is a small reset with the font and the page colors, and `theme` switches the color tokens to their dark values for a visitor whose system prefers dark.",
                ja: "`hypersig_ui::global()` と `hypersig_ui::theme()` を、ページに 1 回ずつ追加します。`global` はフォントとページの色を含む小さなリセットです。`theme` は、システムの設定がダークモードの訪問者に対して、色のトークンをダークの値に切り替えます。",
            },
        ),
        show(Snippet::UiSetup),
        heading(
            site,
            L {
                en: "How the components are built",
                ja: "コンポーネントの作り",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Each component is a function or a small builder that returns an `El<S>`, so it takes handlers, styles, and children like any other element.",
                    ja: "どのコンポーネントも `El<S>` を返す関数か小さなビルダーなので、ほかの要素と同じようにハンドラ、スタイル、子要素を受け取ります。",
                },
                L {
                    en: "The behavior a screen reader and a keyboard depend on comes from the browser where the browser has it: a native `<button>`, `<input>`, `<select>`, `<details>`, `<dialog>`, or `popover`. Where it does not, the component follows the WAI-ARIA Authoring Practices, with keyboard handling from `El::roving_focus` and `El::active_descendant`.",
                    ja: "スクリーンリーダーやキーボードでの操作に必要な振る舞いは、ブラウザに備わっていればブラウザのものを使います。ネイティブの `<button>`、`<input>`、`<select>`、`<details>`、`<dialog>`、`popover` です。ブラウザに備わっていない振る舞いは、コンポーネントが WAI-ARIA Authoring Practices に従って作り、キーボード操作は `El::roving_focus` と `El::active_descendant` で扱います。",
                },
                L {
                    en: "The ids that tie a label to its control, or a tab to its panel, are derived from the signal the component is bound to. A dialog, a menu, a tooltip, and a combobox are `const` handles built from a `NodeRef` that `node_ref!` declares, or a `DialogRef` from `dialog_ref!`, and their parts share the handle. The macro names the reference after its module and constant, so two handles never share an id.",
                    ja: "ラベルとコントロール、タブとパネルを結び付ける id は、コンポーネントが束縛するシグナルから作られます。ダイアログ、メニュー、ツールチップ、コンボボックスは、`node_ref!` で宣言した `NodeRef` か `dialog_ref!` で宣言した `DialogRef` から作る `const` のハンドルで、部品どうしでそのハンドルを共有します。マクロはモジュールと定数の名前から参照の名前を作るので、2 つのハンドルが同じ id になることはありません。",
                },
                L {
                    en: "The colors follow shadcn/ui's zinc theme. Three values differ, to meet WCAG 2.2 AA: a control's border has 3:1 contrast or more, secondary text keeps 4.5:1 on the muted background, and focus is a solid outline, which forced-colors mode still draws.",
                    ja: "色は shadcn/ui の zinc テーマに合わせ、WCAG 2.2 AA を満たすために 3 つの値を変えています。コントロールの境界線は 3:1 以上のコントラストを持ち、補足のテキストは muted の背景色の上でも 4.5:1 を保ち、フォーカスは forced-colors モードでも描かれる実線の outline です。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The demos on these pages run in frames on this static site, so they use signals alone. The [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) example in the repository puts every component on one page with a server, and its end-to-end test drives them with the keyboard.",
                ja: "各ページのデモはこの静的サイトの中のフレームで動くので、シグナルだけを使います。リポジトリにある例 [`ui`](https://github.com/imaimai17468/hypersig/tree/main/examples/ui) は全コンポーネントをサーバー付きで 1 ページに並べたもので、その E2E テストはキーボードでコンポーネントを操作します。",
            },
        ),
    ])
}

fn theming(site: Site) -> Part {
    parts([
        title(Doc::Components(Component::Theming).title(site)),
        lead(site, Doc::Components(Component::Theming).leads()),
        heading(
            site,
            L {
                en: "Tokens",
                ja: "トークン",
            },
        ),
        para(
            site,
            L {
                en: "The [`token`](api:hypersig_ui/token/index.html) module has one function per token, named as in shadcn/ui: `background`, `foreground`, `primary`, `muted`, `border`, `ring`, `radius`, and the rest. Each returns a `Token`, which is a CSS custom property with a typed default. A page that sets nothing gets the light values.",
                ja: "[`token`](api:hypersig_ui/token/index.html) モジュールにはトークンごとに 1 つの関数があり、名前は shadcn/ui に合わせています。`background`、`foreground`、`primary`、`muted`、`border`、`ring`、`radius` などです。どの関数も `Token` を返します。`Token` は型の付いた既定値を持つ CSS のカスタムプロパティです。何も設定しないページには、ライトの値が入ります。",
            },
        ),
        show(Snippet::UiTheme),
        heading(
            site,
            L {
                en: "Light and dark",
                ja: "ライトとダーク",
            },
        ),
        para(
            site,
            L {
                en: "`hypersig_ui::theme()` sets the dark values under `prefers-color-scheme: dark`. To let a signal pick the scheme instead, put `hypersig_ui::dark()` or `hypersig_ui::light()` on an element with `css_when` or `css_match`. Each sets every color token and `color-scheme` on the element and those inside it, so native controls such as a scrollbar follow the scheme.",
                ja: "`hypersig_ui::theme()` は `prefers-color-scheme: dark` のときにダークの値を設定します。代わりにシグナルで配色を選ぶには、`hypersig_ui::dark()` か `hypersig_ui::light()` を `css_when` や `css_match` で要素に付けます。どちらもその要素と内側の要素に色のトークンをすべて設定し、`color-scheme` も一緒に設定するので、スクロールバーのようなネイティブのコントロールも配色に従います。",
            },
        ),
        demo::show(site, Demo::Theme),
        note(
            site,
            L {
                en: "The tokens are plain values of `hypersig::style`. [Tokens and themes](style/tokens/) covers how they are written to the page.",
                ja: "トークンは `hypersig::style` の普通の値です。トークンをページに書き出す方法は、[トークンとテーマ](style/tokens/) で説明しています。",
            },
        ),
    ])
}

/// The lead of the page of `page`, a component rather than Installation or
/// Theming.
pub(crate) fn component_lead(page: Component) -> L<&'static str> {
    component_page(page).lead
}

/// The content of a component page.
#[expect(clippy::too_many_lines, reason = "one block of data per component")]
fn component_page(page: Component) -> Page {
    match page {
        Component::Installation | Component::Theming => unreachable!("written by hand"),
        Component::Button => Page {
            lead: L {
                en: "A button in one of six variants and three sizes.",
                ja: "6 つのバリアントと 3 つのサイズから選べるボタンです。",
            },
            demo: Demo::Button,
            built_on: &[
                L {
                    en: "Each is a native `<button>`, so Enter and Space press it and a screen reader announces it as a button. `button` has `type=\"button\"`, so it does not submit the form around it, and `submit_button` is the one that does. Disable either with `El::disabled`, which dims it.",
                    ja: "どれもネイティブの `<button>` です。そのため Enter と Space で押せて、スクリーンリーダーもボタンとして読み上げます。`button` は `type=\"button\"` を持つので、囲んでいるフォームを送信しません。フォームを送信するのは `submit_button` です。どちらも `El::disabled` で無効にでき、無効の間は薄く表示されます。",
                },
                L {
                    en: "An icon button has no text, so `icon_button` takes the label that names it as an argument. `button_style` gives the same look to an element that is not a button, such as a link.",
                    ja: "アイコンボタンにはテキストがないので、`icon_button` はボタンの名前になるラベルを引数に取ります。`button_style` はリンクのようなボタン以外の要素に同じ見た目を与えます。",
                },
            ],
            keys: &[(
                L {
                    en: "Enter, Space",
                    ja: "Enter、Space",
                },
                L {
                    en: "Presses the button.",
                    ja: "ボタンを押します。",
                },
            )],
            items: &[
                (
                    "button",
                    "hypersig_ui/fn.button.html",
                    L {
                        en: "A `<button type=\"button\">` of `variant` and `size`.",
                        ja: "`variant` と `size` を指定した `<button type=\"button\">`",
                    },
                ),
                (
                    "submit_button",
                    "hypersig_ui/fn.submit_button.html",
                    L {
                        en: "A `<button type=\"submit\">` of `variant` and `size`, which submits the form around it.",
                        ja: "囲んでいるフォームを送信する、`variant` と `size` を指定した `<button type=\"submit\">`",
                    },
                ),
                (
                    "icon_button",
                    "hypersig_ui/fn.icon_button.html",
                    L {
                        en: "A square button that shows only an icon, named by `label`.",
                        ja: "アイコンだけを表示し、`label` を名前に持つ正方形のボタン",
                    },
                ),
                (
                    "button_style",
                    "hypersig_ui/fn.button_style.html",
                    L {
                        en: "The style of a button of `variant` and `size`, for an element that is not a `<button>`.",
                        ja: "`<button>` 以外の要素に使う、`variant` と `size` を指定したボタンのスタイル",
                    },
                ),
                (
                    "ButtonVariant",
                    "hypersig_ui/enum.ButtonVariant.html",
                    L {
                        en: "The look of a button.",
                        ja: "ボタンの見た目",
                    },
                ),
                (
                    "ButtonSize",
                    "hypersig_ui/enum.ButtonSize.html",
                    L {
                        en: "The height and padding of a button.",
                        ja: "ボタンの高さと余白",
                    },
                ),
            ],
        },
        Component::TextField => Page {
            lead: L {
                en: "A text field with its label, and where given, a description and an error message tied to it.",
                ja: "ラベルの付いたテキストフィールドです。指定すると、説明とエラーメッセージもフィールドに結び付きます。",
            },
            demo: Demo::TextField,
            built_on: &[
                L {
                    en: "A text field is a `<label>` and an `<input>`, or a `<textarea>` in place of the `<input>` with `multiline`. The ids that tie the label to the input are derived from the signal's path, so a label always names its own field. `aria-describedby` points at the description and the error, and `aria-invalid` is `true` while the error is not empty.",
                    ja: "テキストフィールドは `<label>` と `<input>` からできており、`multiline` を指定すると `<input>` の代わりに `<textarea>` になります。ラベルと入力欄を結び付ける id はシグナルのパスから作られるので、ラベルは必ず自分のフィールドを指します。`aria-describedby` は説明とエラーを指し、`aria-invalid` はエラーが空でない間 `true` になります。",
                },
                L {
                    en: "The error sits in an `aria-live=\"polite\"` region, so a screen reader reads a new message when it appears. A screen reader reads \"required\" from the field itself, so it skips the asterisk that a required field shows. `autocomplete` tells the browser what the field holds, to meet WCAG 1.3.5.",
                    ja: "エラーは `aria-live=\"polite\"` の領域に入るので、新しいメッセージが出るとスクリーンリーダーが読み上げます。スクリーンリーダーは「必須」であることをフィールド自体から読み取るので、必須のフィールドに付くアスタリスクは読み飛ばします。`autocomplete` は WCAG 1.3.5 を満たすために、フィールドに何が入るかをブラウザに伝えます。",
                },
                L {
                    en: "`NumberField` is an `<input type=\"number\">` bound to an `i32` or `f64` signal, with the same label, ids, and messages. Its `min`, `max`, and `step` take the signal's own type. A field whose minimum is 0 or more asks for a keyboard of digits.",
                    ja: "`NumberField` は `i32` か `f64` のシグナルに束縛した `<input type=\"number\">` で、ラベル、id、メッセージはテキストフィールドと同じです。`min`、`max`、`step` はシグナルと同じ型の値を取ります。最小値が 0 以上のフィールドは、数字のキーボードを求めます。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "TextField::new",
                    "hypersig_ui/struct.TextField.html#method.new",
                    L {
                        en: "A single-line text field labeled `label`, bound to `value`.",
                        ja: "`value` に束縛し、`label` をラベルに持つ 1 行のテキストフィールド",
                    },
                ),
                (
                    "TextField::in_form",
                    "hypersig_ui/struct.TextField.html#method.in_form",
                    L {
                        en: "A single-line text field labeled `label`, bound to `field` of `form`.",
                        ja: "`form` の `field` に束縛し、`label` をラベルに持つ 1 行のテキストフィールド",
                    },
                ),
                (
                    "kind",
                    "hypersig_ui/struct.TextField.html#method.kind",
                    L {
                        en: "Sets the `type` of the input. A multiline field ignores it.",
                        ja: "入力欄の `type` を設定します。複数行のフィールドでは無視されます。",
                    },
                ),
                (
                    "multiline",
                    "hypersig_ui/struct.TextField.html#method.multiline",
                    L {
                        en: "Makes the field a `<textarea>` that the visitor can resize vertically.",
                        ja: "フィールドを、訪問者が縦にサイズを変えられる `<textarea>` にします。",
                    },
                ),
                (
                    "autocomplete",
                    "hypersig_ui/struct.TextField.html#method.autocomplete",
                    L {
                        en: "Tells the browser what the field holds, so that it can fill it in.",
                        ja: "フィールドに何が入るかをブラウザに伝え、ブラウザが自動で入力できるようにします。",
                    },
                ),
                (
                    "input_mode",
                    "hypersig_ui/struct.TextField.html#method.input_mode",
                    L {
                        en: "Asks for the on-screen keyboard `mode`, as `inputmode` does.",
                        ja: "`inputmode` と同じく、画面上のキーボードを `mode` にするよう求めます。",
                    },
                ),
                (
                    "max_length",
                    "hypersig_ui/struct.TextField.html#method.max_length",
                    L {
                        en: "Stops the visitor from typing more than `count` characters, as `maxlength` does.",
                        ja: "`maxlength` と同じく、`count` 文字より多く入力できないようにします。",
                    },
                ),
                (
                    "min_length",
                    "hypersig_ui/struct.TextField.html#method.min_length",
                    L {
                        en: "Makes the browser refuse to submit fewer than `count` characters, as `minlength` does.",
                        ja: "`minlength` と同じく、`count` 文字に満たない値をブラウザが送信しないようにします。",
                    },
                ),
                (
                    "placeholder",
                    "hypersig_ui/struct.TextField.html#method.placeholder",
                    L {
                        en: "Shows `text` in the empty field.",
                        ja: "空のフィールドに `text` を表示します。",
                    },
                ),
                (
                    "description",
                    "hypersig_ui/struct.TextField.html#method.description",
                    L {
                        en: "Adds a line under the field that `aria-describedby` points at.",
                        ja: "フィールドの下に、`aria-describedby` が指す 1 行を追加します。",
                    },
                ),
                (
                    "error",
                    "hypersig_ui/struct.TextField.html#method.error",
                    L {
                        en: "Shows `message` under the field and marks the field invalid while `message` is not empty.",
                        ja: "フィールドの下に `message` を表示し、`message` が空でない間はフィールドを無効な入力として示します。",
                    },
                ),
                (
                    "required",
                    "hypersig_ui/struct.TextField.html#method.required",
                    L {
                        en: "Marks the field `required`, which the label shows with an asterisk.",
                        ja: "フィールドを `required` にします。ラベルにはアスタリスクが付きます。",
                    },
                ),
                (
                    "disabled",
                    "hypersig_ui/struct.TextField.html#method.disabled",
                    L {
                        en: "Disables the field while `when` is true.",
                        ja: "`when` が true の間、フィールドを無効にします。",
                    },
                ),
                (
                    "InputKind",
                    "hypersig_ui/enum.InputKind.html",
                    L {
                        en: "The `type` of a single-line `TextField`.",
                        ja: "1 行の `TextField` の `type`",
                    },
                ),
                (
                    "InputMode",
                    "hypersig_ui/enum.InputMode.html",
                    L {
                        en: "The on-screen keyboard a `TextField` asks for, as `inputmode` names it.",
                        ja: "`TextField` が求める画面上のキーボードを、`inputmode` の値で表したもの",
                    },
                ),
                (
                    "NumberField::new",
                    "hypersig_ui/struct.NumberField.html#method.new",
                    L {
                        en: "A number field labeled `label`, bound to `value`.",
                        ja: "`value` に束縛し、`label` をラベルに持つ数値フィールド",
                    },
                ),
                (
                    "NumberField::in_form",
                    "hypersig_ui/struct.NumberField.html#method.in_form",
                    L {
                        en: "A number field labeled `label`, bound to `field` of `form`.",
                        ja: "`form` の `field` に束縛し、`label` をラベルに持つ数値フィールド",
                    },
                ),
                (
                    "min",
                    "hypersig_ui/struct.NumberField.html#method.min",
                    L {
                        en: "Sets the lowest value the browser accepts.",
                        ja: "ブラウザが受け付ける最小値を設定します。",
                    },
                ),
                (
                    "max",
                    "hypersig_ui/struct.NumberField.html#method.max",
                    L {
                        en: "Sets the highest value the browser accepts.",
                        ja: "ブラウザが受け付ける最大値を設定します。",
                    },
                ),
                (
                    "step",
                    "hypersig_ui/struct.NumberField.html#method.step",
                    L {
                        en: "Sets how far the arrow keys move the value, and which values the browser accepts.",
                        ja: "矢印キーで値が動く幅と、ブラウザが受け付ける値を設定します。",
                    },
                ),
                (
                    "Numeric",
                    "hypersig_ui/trait.Numeric.html",
                    L {
                        en: "A number type a `NumberField` can be bound to: `i32` or `f64`.",
                        ja: "`NumberField` を束縛できる数値の型（`i32` か `f64`）",
                    },
                ),
                (
                    "Autocomplete",
                    "hypersig_ui/enum.Autocomplete.html",
                    L {
                        en: "What a `TextField` holds, as `autocomplete` names it.",
                        ja: "`TextField` に入る内容を、`autocomplete` の値で表したもの",
                    },
                ),
            ],
        },
        Component::Checkbox => Page {
            lead: L {
                en: "A checkbox and an on and off switch, each bound to a `bool` signal.",
                ja: "チェックボックスと、オンとオフを切り替えるスイッチです。どちらも `bool` のシグナルに束縛します。",
            },
            demo: Demo::Checkbox,
            built_on: &[
                L {
                    en: "Both are a native `<input type=\"checkbox\">` inside the `<label>` that holds the text, so a click on the text toggles them and they need no id. Because they need no id, they can also be bound to a field of a list item.",
                    ja: "どちらもテキストを含む `<label>` の中に置いたネイティブの `<input type=\"checkbox\">` です。そのためテキストをクリックしても切り替わり、id も不要です。id が不要なので、リストの各項目のフィールドにも束縛できます。",
                },
                L {
                    en: "The switch has `role=\"switch\"`, so a screen reader says \"switch, on\" rather than \"checkbox, checked\".",
                    ja: "スイッチは `role=\"switch\"` を持つので、スクリーンリーダーは「チェックボックス、チェックあり」ではなく「スイッチ、オン」と読み上げます。",
                },
            ],
            keys: &[(
                L {
                    en: "Space",
                    ja: "Space",
                },
                L {
                    en: "Toggles the checkbox or the switch.",
                    ja: "チェックボックスやスイッチを切り替えます。",
                },
            )],
            items: &[
                (
                    "checkbox",
                    "hypersig_ui/fn.checkbox.html",
                    L {
                        en: "A checkbox labeled `text`, bound to `checked`.",
                        ja: "`checked` に束縛し、`text` をラベルに持つチェックボックス",
                    },
                ),
                (
                    "checkbox_in_form",
                    "hypersig_ui/fn.checkbox_in_form.html",
                    L {
                        en: "A checkbox labeled `text`, bound to `field` of `form`.",
                        ja: "`form` の `field` に束縛し、`text` をラベルに持つチェックボックス",
                    },
                ),
                (
                    "switch",
                    "hypersig_ui/fn.switch.html",
                    L {
                        en: "An on and off switch labeled `text`, bound to `on`.",
                        ja: "`on` に束縛し、`text` をラベルに持つ、オンとオフを切り替えるスイッチ",
                    },
                ),
                (
                    "switch_in_form",
                    "hypersig_ui/fn.switch_in_form.html",
                    L {
                        en: "An on and off switch labeled `text`, bound to `field` of `form`.",
                        ja: "`form` の `field` に束縛し、`text` をラベルに持つ、オンとオフを切り替えるスイッチ",
                    },
                ),
            ],
        },
        Component::RadioGroup => Page {
            lead: L {
                en: "A group of radio buttons that sets a signal to the chosen option's value.",
                ja: "選んだ選択肢の値をシグナルに入れる、ラジオボタンのグループです。",
            },
            demo: Demo::RadioGroup,
            built_on: &[L {
                en: "The radios share a `name` derived from the signal's path, so the browser gives the group one Tab stop and moves between its radios with the arrow keys. The `<fieldset>` and `<legend>` name the group for a screen reader.",
                ja: "ラジオボタンはシグナルのパスから作った `name` を共有するので、ブラウザはグループ全体で Tab の止まる場所を 1 つにし、グループの中のラジオボタンの間を矢印キーで移れるようにします。`<fieldset>` と `<legend>` がスクリーンリーダーに向けてグループに名前を付けます。",
            }],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Moves focus into the group, to the checked radio.",
                        ja: "グループの中のチェックされたラジオボタンにフォーカスを移します。",
                    },
                ),
                (
                    L {
                        en: "Arrow keys",
                        ja: "矢印キー",
                    },
                    L {
                        en: "Move to the next or previous radio and check it.",
                        ja: "次か前のラジオボタンに移り、それをチェックします。",
                    },
                ),
            ],
            items: &[
                (
                    "radio_group",
                    "hypersig_ui/fn.radio_group.html",
                    L {
                        en: "A group of radio buttons with `title` in its `<legend>`, one per option, that sets `selected` to the option's value.",
                        ja: "`title` を `<legend>` に表示し、選択肢ごとに 1 つのラジオボタンを並べて、選んだ選択肢の値を `selected` に入れるグループ",
                    },
                ),
                (
                    "radio_group_in_form",
                    "hypersig_ui/fn.radio_group_in_form.html",
                    L {
                        en: "A group of radio buttons with `title` in its `<legend>`, bound to `field` of `form`.",
                        ja: "`title` を `<legend>` に表示し、`form` の `field` に束縛したラジオボタンのグループ",
                    },
                ),
            ],
        },
        Component::Select => Page {
            lead: L {
                en: "A select that sets a signal to the chosen option's value.",
                ja: "選んだ選択肢の値をシグナルに入れるセレクトです。",
            },
            demo: Demo::Select,
            built_on: &[
                L {
                    en: "It is a native `<select>`, which a screen reader, a keyboard, and a phone's picker wheel already handle. The closed control is drawn like a text field with a chevron.",
                    ja: "セレクトはネイティブの `<select>` なので、スクリーンリーダー、キーボード、スマートフォンのピッカーホイールがもともと対応しています。閉じた状態のコントロールは、山形の矢印が付いたテキストフィールドのように描かれます。",
                },
                L {
                    en: "Where the browser supports `appearance: base-select` and `::picker(select)`, which browser-compat-data 8.1.6 lists for Chrome 135 and Safari 27, the open list is drawn as a popover card too. Elsewhere it is the browser's own picker.",
                    ja: "ブラウザが `appearance: base-select` と `::picker(select)` に対応していれば、開いた一覧も popover のカードとして描かれます。browser-compat-data 8.1.6 では、対応しているのは Chrome 135 と Safari 27 です。それ以外のブラウザでは、ブラウザ自身のピッカーが開きます。",
                },
            ],
            keys: &[(
                L {
                    en: "The browser's keys",
                    ja: "ブラウザのキー",
                },
                L {
                    en: "A native `<select>` takes the keys of the platform, which differ between systems.",
                    ja: "ネイティブの `<select>` はプラットフォームのキー操作に従うので、使えるキーはシステムごとに異なります。",
                },
            )],
            items: &[
                (
                    "select_field",
                    "hypersig_ui/fn.select_field.html",
                    L {
                        en: "A select labeled `title`, offering `options`, that sets `selected` to the chosen option's value.",
                        ja: "`title` をラベルに持ち、`options` を選択肢として出して、選んだ選択肢の値を `selected` に入れるセレクト",
                    },
                ),
                (
                    "select_field_in_form",
                    "hypersig_ui/fn.select_field_in_form.html",
                    L {
                        en: "A select labeled `title`, offering `options`, bound to `field` of `form`.",
                        ja: "`title` をラベルに持ち、`options` を選択肢として出す、`form` の `field` に束縛したセレクト",
                    },
                ),
            ],
        },
        Component::Slider => Page {
            lead: L {
                en: "A slider that sets an `i32` signal, with a value text for a unit.",
                ja: "`i32` のシグナルを設定するスライダーです。単位を表す値のテキストも付けられます。",
            },
            demo: Demo::Slider,
            built_on: &[
                L {
                    en: "It is a native `<input type=\"range\">`, so the browser gives it `role=\"slider\"`, its keys, and dragging with a pointer or a finger. The input is transparent and lies over a track and a thumb drawn from the value, so the slider looks the same in every browser.",
                    ja: "スライダーはネイティブの `<input type=\"range\">` なので、`role=\"slider\"`、キー操作、ポインタや指でのドラッグはブラウザが用意します。入力欄は透明で、値から描いたトラックとつまみの上に重なっているので、スライダーはどのブラウザでも同じ見た目になります。",
                },
                L {
                    en: "`value_text` shows a value such as \"40 %\" beside the label and gives it to assistive technology as `aria-valuetext`. The drawn thumb can sit a few pixels off the browser's own transparent thumb near either end.",
                    ja: "`value_text` は「40 %」のような値をラベルの横に表示し、支援技術には `aria-valuetext` として渡します。両端の近くでは、描いたつまみがブラウザ自身の透明なつまみから数ピクセルずれることがあります。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Arrow keys",
                        ja: "矢印キー",
                    },
                    L {
                        en: "Move the value by one step.",
                        ja: "値を 1 ステップ動かします。",
                    },
                ),
                (
                    L {
                        en: "Page Up, Page Down",
                        ja: "Page Up、Page Down",
                    },
                    L {
                        en: "Move the value by a larger step.",
                        ja: "値を大きめのステップで動かします。",
                    },
                ),
                (
                    L {
                        en: "Home, End",
                        ja: "Home、End",
                    },
                    L {
                        en: "Move to the lowest or the highest value.",
                        ja: "最小値か最大値に移ります。",
                    },
                ),
            ],
            items: &[
                (
                    "Slider::new",
                    "hypersig_ui/struct.Slider.html#method.new",
                    L {
                        en: "A slider from 0 to 100 in steps of 1, labeled `label`, bound to `value`.",
                        ja: "`value` に束縛し、`label` をラベルに持つ、0 から 100 まで 1 刻みのスライダー",
                    },
                ),
                (
                    "range",
                    "hypersig_ui/struct.Slider.html#method.range",
                    L {
                        en: "Sets the lowest and the highest value.",
                        ja: "最小値と最大値を設定します。",
                    },
                ),
                (
                    "step",
                    "hypersig_ui/struct.Slider.html#method.step",
                    L {
                        en: "Sets how far one arrow key press moves the value.",
                        ja: "矢印キーを 1 回押したときに値が動く幅を設定します。",
                    },
                ),
                (
                    "value_text",
                    "hypersig_ui/struct.Slider.html#method.value_text",
                    L {
                        en: "Shows `text` beside the label and gives it to assistive technology as `aria-valuetext`.",
                        ja: "ラベルの横に `text` を表示し、支援技術には `aria-valuetext` として渡します。",
                    },
                ),
                (
                    "disabled",
                    "hypersig_ui/struct.Slider.html#method.disabled",
                    L {
                        en: "Disables the slider while `when` is true, and dims it.",
                        ja: "`when` が true の間、スライダーを無効にして薄く表示します。",
                    },
                ),
            ],
        },
        Component::Listbox => Page {
            lead: L {
                en: "A list of options of which one is selected, kept in a signal.",
                ja: "選択肢のうち 1 つを選ぶ一覧です。選ばれた選択肢はシグナルに入ります。",
            },
            demo: Demo::Listbox,
            built_on: &[L {
                en: "It follows the WAI-ARIA listbox pattern for a single selection. The list has `role=\"listbox\"` and each option `role=\"option\"` with `aria-selected`. Focus moves between the options through `El::roving_focus`, and each option is selected as it gains focus. The ids are derived from the signal's path and the option's position. The signal holds an enum with `#[derive(Segment)]`, and the options are its variants, so a `match` that leaves a variant without a title fails to compile.",
                ja: "単一選択の WAI-ARIA listbox パターンに従います。一覧は `role=\"listbox\"` を持ち、各選択肢は `role=\"option\"` と `aria-selected` を持ちます。フォーカスは `El::roving_focus` で選択肢の間を移り、選択肢はフォーカスを受けると同時に選ばれます。id はシグナルのパスと選択肢の位置から作られます。シグナルの値は `#[derive(Segment)]` を付けた列挙型で、選択肢はそのバリアントです。そのため、タイトルのないバリアントを残した `match` はコンパイルできません。",
            }],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Moves focus into the list, to the selected option.",
                        ja: "一覧の中の選ばれた選択肢にフォーカスを移します。",
                    },
                ),
                (
                    L {
                        en: "Up, Down",
                        ja: "上矢印、下矢印",
                    },
                    L {
                        en: "Move to the previous or next option and select it.",
                        ja: "前か次の選択肢に移り、それを選びます。",
                    },
                ),
                (
                    L {
                        en: "Home, End",
                        ja: "Home、End",
                    },
                    L {
                        en: "Move to the first or last option and select it.",
                        ja: "最初か最後の選択肢に移り、それを選びます。",
                    },
                ),
            ],
            items: &[(
                "Listbox::new",
                "hypersig_ui/struct.Listbox.html#method.new",
                L {
                    en: "A list named `label`, with the title `option` gives each variant.",
                    ja: "`label` を名前に持ち、`option` が各バリアントに返すタイトルを選択肢にする一覧",
                },
            )],
        },
        Component::Combobox => Page {
            lead: L {
                en: "A text field that suggests options from a list as the visitor types.",
                ja: "訪問者が入力するのに合わせて、一覧から選択肢を候補に出すテキストフィールドです。",
            },
            demo: Demo::Combobox,
            built_on: &[
                L {
                    en: "It follows the WAI-ARIA combobox pattern with list autocomplete. The text field has `role=\"combobox\"`, `aria-expanded`, `aria-controls`, and `aria-activedescendant`, and focus stays in it while the arrows move through the options shown, through `El::active_descendant`. Typing shows the options that contain the text, ignoring case.",
                    ja: "WAI-ARIA combobox パターンのうち、list autocomplete の形に従います。テキストフィールドは `role=\"combobox\"`、`aria-expanded`、`aria-controls`、`aria-activedescendant` を持ちます。`El::active_descendant` を使うので、矢印キーで表示中の選択肢をたどる間も、フォーカスはテキストフィールドに残ります。文字を入力すると、大文字と小文字を区別せずに、その文字列を含む選択肢が表示されます。",
                },
                L {
                    en: "Declare the combobox as a `const` from two node refs, one for its text field and one for its list, and keep its state in a `ComboboxState` field of the page's signals. The chosen text is `ComboboxState::text`. The visitor can leave text that matches no option, which the page checks where it needs one of the options.",
                    ja: "コンボボックスはテキストフィールド用と一覧用の 2 つのノード参照から `const` として宣言し、状態はページのシグナルにある `ComboboxState` のフィールドに持たせます。選ばれたテキストは `ComboboxState::text` です。訪問者はどの選択肢にも合わないテキストを残せるので、選択肢のどれかが必要なところでは、ページの側で確かめます。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Typing",
                        ja: "文字の入力",
                    },
                    L {
                        en: "Opens the list and shows the options that contain the text.",
                        ja: "一覧を開き、その文字列を含む選択肢を表示します。",
                    },
                ),
                (
                    L {
                        en: "Down",
                        ja: "下矢印",
                    },
                    L {
                        en: "Opens the list, then moves to the next option shown.",
                        ja: "一覧を開きます。開いたあとは、表示中の次の選択肢に移ります。",
                    },
                ),
                (
                    L {
                        en: "Up",
                        ja: "上矢印",
                    },
                    L {
                        en: "Moves to the previous option shown.",
                        ja: "表示中の前の選択肢に移ります。",
                    },
                ),
                (
                    L {
                        en: "Enter",
                        ja: "Enter",
                    },
                    L {
                        en: "Chooses the option the arrow keys moved to and closes the list.",
                        ja: "矢印キーで移った先の選択肢を選び、一覧を閉じます。",
                    },
                ),
                (
                    L {
                        en: "Escape",
                        ja: "Escape",
                    },
                    L {
                        en: "Closes the list, or clears the text when the list is already closed. Escape pressed in the text field does not close a dialog around the combobox.",
                        ja: "一覧を閉じます。一覧が閉じているときは、テキストを消します。コンボボックスを囲むダイアログは、テキストフィールドで押された Escape では閉じません。",
                    },
                ),
            ],
            items: &[
                (
                    "Combobox::new",
                    "hypersig_ui/struct.Combobox.html#method.new",
                    L {
                        en: "A combobox whose text field `field` names and whose list `list` names.",
                        ja: "テキストフィールドに `field`、一覧に `list` の名前を付けたコンボボックス",
                    },
                ),
                (
                    "render",
                    "hypersig_ui/struct.Combobox.html#method.render",
                    L {
                        en: "The label, the text field, and the list of `options`, keeping their state in `state`.",
                        ja: "状態を `state` に持つ、ラベルとテキストフィールドと `options` の一覧",
                    },
                ),
                (
                    "ComboboxState",
                    "hypersig_ui/struct.ComboboxState.html",
                    L {
                        en: "The signals a `Combobox` keeps.",
                        ja: "`Combobox` が持つシグナル",
                    },
                ),
                (
                    "ComboboxState::new",
                    "hypersig_ui/struct.ComboboxState.html#method.new",
                    L {
                        en: "A closed combobox whose text field holds `text`.",
                        ja: "テキストフィールドに `text` が入った、閉じたコンボボックス",
                    },
                ),
            ],
        },
        Component::Tabs => Page {
            lead: L {
                en: "A tab list that keeps the selected tab in a signal, with one panel per tab.",
                ja: "選ばれたタブをシグナルに持つタブの一覧です。タブごとに 1 つのパネルがあります。",
            },
            demo: Demo::Tabs,
            built_on: &[L {
                en: "It follows the WAI-ARIA tabs pattern. The list has `role=\"tablist\"`, each tab `role=\"tab\"` with `aria-selected` and `aria-controls`, and each panel `role=\"tabpanel\"` labeled by its tab. A tab is selected as it gains focus, through `El::roving_focus`. The ids are derived from the signal's path and the tab's position. The signal holds an enum with `#[derive(Segment)]`, and the tabs are its variants, so the selected value always has a tab that Tab reaches, and a `match` that leaves a variant without a tab fails to compile.",
                ja: "WAI-ARIA tabs パターンに従います。一覧は `role=\"tablist\"` を持ち、各タブは `role=\"tab\"` と `aria-selected`、`aria-controls` を持ちます。各パネルは `role=\"tabpanel\"` を持ち、対応するタブがパネルのラベルになります。タブは `El::roving_focus` でフォーカスを受けると同時に選ばれます。id はシグナルのパスとタブの位置から作られます。シグナルの値は `#[derive(Segment)]` を付けた列挙型で、タブはそのバリアントです。そのため、選ばれた値には必ず Tab キーで移れるタブがあり、タブのないバリアントを残した `match` はコンパイルできません。",
            }],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Moves focus into the list at the selected tab, then on to the panel.",
                        ja: "一覧の中の選ばれたタブにフォーカスを移し、もう一度押すとパネルに移します。",
                    },
                ),
                (
                    L {
                        en: "Left, Right",
                        ja: "左矢印、右矢印",
                    },
                    L {
                        en: "Move to the previous or next tab and select it.",
                        ja: "前か次のタブに移り、それを選びます。",
                    },
                ),
                (
                    L {
                        en: "Home, End",
                        ja: "Home、End",
                    },
                    L {
                        en: "Move to the first or last tab and select it.",
                        ja: "最初か最後のタブに移り、それを選びます。",
                    },
                ),
            ],
            items: &[(
                "Tabs::new",
                "hypersig_ui/struct.Tabs.html#method.new",
                L {
                    en: "A tab list named `label`, with the title and panel `tab` gives each variant.",
                    ja: "`label` を名前に持ち、`tab` が各バリアントに返すタイトルとパネルを並べるタブの一覧",
                },
            )],
        },
        Component::Accordion => Page {
            lead: L {
                en: "A stack of sections that open and close, one at a time or each on its own.",
                ja: "開閉するセクションを縦に並べたものです。一度に 1 つだけ開く形と、それぞれが独立に開く形があります。",
            },
            demo: Demo::Accordion,
            built_on: &[
                L {
                    en: "Each section is a native `<details>` whose `<summary>` opens and closes it. The browser handles the keyboard, and each summary is announced as a button that is expanded or collapsed, and find-in-page opens a closed section that holds a match.",
                    ja: "各セクションはネイティブの `<details>` で、その `<summary>` がセクションを開閉します。キーボード操作はブラウザが扱い、各 summary は展開か折りたたみの状態を持つボタンとして読み上げられます。ページ内検索で一致した箇所が閉じたセクションにあれば、ブラウザがそのセクションを開きます。",
                },
                L {
                    en: "An exclusive accordion gives its sections one `name`, so the browser closes the open one when another opens. The name is an `Id` that the accordion's `<div>` also carries, so two accordions given one `Id::new` make the page panic when it renders, where they would otherwise close each other's sections.",
                    ja: "排他的なアコーディオンはセクションに同じ `name` を付けるので、別のセクションが開くと、ブラウザが開いていたセクションを閉じます。この名前は `Id` で、アコーディオンの `<div>` も同じ id を持ちます。そのため、1 つの `Id::new` を 2 つのアコーディオンに渡すと、互いのセクションを閉じ合う代わりに、ページの描画がパニックします。",
                },
            ],
            keys: &[(
                L {
                    en: "Enter, Space",
                    ja: "Enter、Space",
                },
                L {
                    en: "Opens or closes the focused section.",
                    ja: "フォーカスのあるセクションを開くか閉じます。",
                },
            )],
            items: &[
                (
                    "Accordion::new",
                    "hypersig_ui/struct.Accordion.html#method.new",
                    L {
                        en: "An accordion whose sections open and close independently.",
                        ja: "セクションがそれぞれ独立に開閉するアコーディオン",
                    },
                ),
                (
                    "Accordion::exclusive",
                    "hypersig_ui/struct.Accordion.html#method.exclusive",
                    L {
                        en: "An accordion that keeps at most one section open, its `<div>` carrying the id `group`.",
                        ja: "開いているセクションを多くても 1 つにし、`<div>` に id `group` を持つアコーディオン",
                    },
                ),
                (
                    "item",
                    "hypersig_ui/struct.Accordion.html#method.item",
                    L {
                        en: "Adds a section titled `title` whose content is `content`.",
                        ja: "`title` をタイトルに持ち、中身が `content` のセクションを追加します。",
                    },
                ),
            ],
        },
        Component::Card => Page {
            lead: L {
                en: "A bordered box with a header, content, and a footer.",
                ja: "ヘッダー、中身、フッターを持つ、枠線付きのボックスです。",
            },
            demo: Demo::Card,
            built_on: &[L {
                en: "A card is a `<div>` and takes no role. Its title is a heading whose level you choose with `Level`, so that it sits one level below the section around it and a screen reader's list of headings stays in order.",
                ja: "カードは `<div>` で、ロールを持ちません。タイトルは見出しで、そのレベルは `Level` で選びます。周りのセクションより 1 つ下のレベルにすると、スクリーンリーダーの見出しの一覧が順番どおりに並びます。",
            }],
            keys: &[],
            items: &[
                (
                    "card",
                    "hypersig_ui/fn.card.html",
                    L {
                        en: "A `<div>` with a border, a shadow, and `token::card` behind it.",
                        ja: "枠線と影を持ち、背景が `token::card` の `<div>`",
                    },
                ),
                (
                    "card_header",
                    "hypersig_ui/fn.card_header.html",
                    L {
                        en: "The top of a `card`, holding its title and description.",
                        ja: "タイトルと説明を入れる、`card` の上部",
                    },
                ),
                (
                    "card_title",
                    "hypersig_ui/fn.card_title.html",
                    L {
                        en: "The title of a `card`, as a heading of `level`.",
                        ja: "`level` の見出しとして描く、`card` のタイトル",
                    },
                ),
                (
                    "card_description",
                    "hypersig_ui/fn.card_description.html",
                    L {
                        en: "A line under the title of a `card`, in `token::muted_foreground`.",
                        ja: "`card` のタイトルの下に `token::muted_foreground` の色で書く 1 行",
                    },
                ),
                (
                    "card_content",
                    "hypersig_ui/fn.card_content.html",
                    L {
                        en: "The body of a `card`.",
                        ja: "`card` の本体",
                    },
                ),
                (
                    "card_footer",
                    "hypersig_ui/fn.card_footer.html",
                    L {
                        en: "The bottom of a `card`, laying its children out in a row.",
                        ja: "子要素を横に並べる、`card` の下部",
                    },
                ),
                (
                    "Level",
                    "hypersig_ui/enum.Level.html",
                    L {
                        en: "The level of a heading, which follows the headings around it.",
                        ja: "周りの見出しに合わせて決める、見出しのレベル",
                    },
                ),
            ],
        },
        Component::Separator => Page {
            lead: L {
                en: "A one-pixel line between groups of content, across or along.",
                ja: "内容のまとまりの間に引く、横向きか縦向きの 1 ピクセルの線です。",
            },
            demo: Demo::Separator,
            built_on: &[L {
                en: "A horizontal separator is an `<hr>`, and a vertical one is a `<div>` with `role=\"separator\"` and `aria-orientation=\"vertical\"`, so a screen reader announces both as separators.",
                ja: "横向きの区切り線は `<hr>` で、縦向きの区切り線は `role=\"separator\"` と `aria-orientation=\"vertical\"` を持つ `<div>` です。そのため、スクリーンリーダーはどちらも区切り線として読み上げます。",
            }],
            keys: &[],
            items: &[(
                "separator",
                "hypersig_ui/fn.separator.html",
                L {
                    en: "A one-pixel line in `token::border` that runs along `orientation`.",
                    ja: "`orientation` の向きに引く、`token::border` の色の 1 ピクセルの線",
                },
            )],
        },
        Component::Navigation => Page {
            lead: L {
                en: "Links between the pages of an application: a list of its sections, a breadcrumb trail, and the pages of a long list.",
                ja: "アプリケーションのページの間を移るリンクです。セクションの一覧、パンくずリスト、長い一覧のページ送りがあります。",
            },
            demo: Demo::Navigation,
            built_on: &[
                L {
                    en: "Each is a `<nav>` named by the label it takes, holding a list of links, so a screen reader lists it among the page's landmarks. The links are typed `Link` values from routes, and the one to the page the visitor is on has `aria-current=\"page\"`.",
                    ja: "どれも受け取ったラベルを名前に持つ `<nav>` で、中にリンクの一覧を持ちます。そのため、スクリーンリーダーはページのランドマークの 1 つとして一覧に出します。リンクはルートから作った型付きの `Link` で、訪問者がいるページへのリンクは `aria-current=\"page\"` を持ちます。",
                },
                L {
                    en: "`NavList` decides the mark on the server, from the page's own `Url` that `Route::url` writes. A link to the same path has `aria-current=\"page\"`, and a link to a path that the page's path begins with has `aria-current=\"true\"`. `Breadcrumb::current` ends the trail with the current page as text and returns the element, so no link can follow it. `pagination` shows the first and the last page and the pages next to the current one, and marks the previous or next link `aria-disabled` where that page does not exist. The links in the demo lead to pages of this site.",
                    ja: "`NavList` は `Route::url` で書いたページ自身の `Url` をもとに、どのリンクに印を付けるかをサーバーで決めます。同じパスへのリンクは `aria-current=\"page\"` を持ち、ページのパスがリンク先のパスの続きになっているリンクは `aria-current=\"true\"` を持ちます。`Breadcrumb::current` は今のページをテキストとして最後に置き、要素を返すので、そのあとにリンクは続けられません。`pagination` は最初と最後のページ、今のページの前後のページを表示し、前や次のページがないときはそのリンクを `aria-disabled` にします。デモのリンクは、このサイトのページにつながります。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "NavList::new",
                    "hypersig_ui/struct.NavList.html#method.new",
                    L {
                        en: "An empty list whose `<nav>` is named `label`, on the page at `current`.",
                        ja: "`<nav>` の名前が `label` で、`current` のページに置く空の一覧",
                    },
                ),
                (
                    "item",
                    "hypersig_ui/struct.NavList.html#method.item",
                    L {
                        en: "Adds a link to `route` with the arguments `args`, as `Route::url` takes them.",
                        ja: "`Route::url` と同じ形の引数 `args` を付けた、`route` へのリンクを追加します。",
                    },
                ),
                (
                    "Breadcrumb::new",
                    "hypersig_ui/struct.Breadcrumb.html#method.new",
                    L {
                        en: "An empty trail whose `<nav>` is named `label`, such as \"Breadcrumb\".",
                        ja: "`<nav>` の名前が `label`（たとえば「Breadcrumb」）の空のパンくずリスト",
                    },
                ),
                (
                    "link",
                    "hypersig_ui/struct.Breadcrumb.html#method.link",
                    L {
                        en: "Adds a link to a page above the current one, after the links before it.",
                        ja: "今のページより上のページへのリンクを、それまでのリンクのあとに追加します。",
                    },
                ),
                (
                    "current",
                    "hypersig_ui/struct.Breadcrumb.html#method.current",
                    L {
                        en: "The trail, ending at the current page `text`.",
                        ja: "今のページ `text` で終わるパンくずリスト",
                    },
                ),
                (
                    "pagination",
                    "hypersig_ui/fn.pagination.html",
                    L {
                        en: "Links to the pages of a list of `count` pages, on page `current`.",
                        ja: "`count` ページある一覧の各ページへのリンク。今のページは `current`",
                    },
                ),
                (
                    "PaginationText",
                    "hypersig_ui/struct.PaginationText.html",
                    L {
                        en: "The words `pagination` names its parts with.",
                        ja: "`pagination` が各部分の名前に使う言葉",
                    },
                ),
            ],
        },
        Component::DataTable => Page {
            lead: L {
                en: "A table of the rows of a list signal under typed columns, which sorts, filters, pages, and selects rows in the browser, or asks the server for one page at a time.",
                ja: "リストのシグナルの行を、型の付いた列の下に並べる表です。並べ替え、絞り込み、ページ送り、行の選択をブラウザで行うほか、1 ページずつサーバーに求めることもできます。",
            },
            demo: Demo::DataTable,
            built_on: &[
                L {
                    en: "It is a `<table>` whose headers have `scope=\"col\"`. A sortable column's header holds a button and carries `aria-sort`, which says whether the rows are sorted by that column in ascending order, in descending order, or not at all. The header checkbox is a button with `role=\"checkbox\"` whose `aria-checked` is `mixed` while some rows are selected, because a native checkbox's indeterminate state is a property that no attribute can set.",
                    ja: "見出しに `scope=\"col\"` を付けた `<table>` です。並べ替えられる列の見出しはボタンを持ち、その列で昇順か降順に並んでいるか、並んでいないかを `aria-sort` で伝えます。見出しのチェックボックスは `role=\"checkbox\"` のボタンで、一部の行が選ばれている間は `aria-checked` が `mixed` になります。ネイティブのチェックボックスの中間状態はプロパティで、属性からは設定できないためです。",
                },
                L {
                    en: "A column is built from a field of the row type, so a field of another struct does not compile, and the key that `sortable_column` sorts by must be a number or a string. The table keeps its filter text, page, and sort column in a `TableState` field of the page's signals, and sorting reorders the list signal itself. The selection is a list of the selected rows' keys in another signal of the page, so the row type holds no field for it and a `ServerTable` keeps it across pages.",
                    ja: "列は行の型のフィールドから作るので、別の構造体のフィールドを渡すとコンパイルエラーになります。`sortable_column` で並べ替えるキーは数値か文字列に限られます。表は絞り込みの文字列、ページ、並べ替えた列を、ページのシグナルにある `TableState` のフィールドに持ちます。並べ替えはリストのシグナルそのものの順序を変えます。選択は、選んだ行のキーのリストとしてページの別のシグナルに持つので、行の型に選択用のフィールドは不要です。`ServerTable` ではページを移っても選択が残ります。",
                },
                L {
                    en: "The browser counts the rows on each change, and with a filter and pagination both on, the work grows with the square of the number of rows. A list of thousands of rows belongs in a `ServerTable`. Its filter field, sortable headers, and page buttons write the `TableState` and then call an action with it, and the action answers with a patch of the page's rows and the total. A sortable column takes an enum with `#[derive(Segment)]` as its key, and the server reads it back from the state with `TableState::sorting`. The demo below runs in the browser alone, since this site has no server, and `examples/ui` has a `ServerTable` beside its `DataTable`.",
                    ja: "ブラウザは変更のたびに行を数え直し、絞り込みとページ送りを両方使うと、その手間は行数の 2 乗で増えます。数千行のリストには `ServerTable` を使ってください。絞り込みの欄、並べ替えの見出し、ページのボタンが `TableState` を書き換えてから、それを引数にアクションを呼びます。アクションは、そのページの行と総数をパッチで返します。並べ替えられる列は `#[derive(Segment)]` を付けた列挙型をキーに取り、サーバーは `TableState::sorting` で状態からキーを読み戻します。このサイトにはサーバーがないので下のデモはブラウザだけで動き、`examples/ui` には `DataTable` と並べて `ServerTable` を置いています。",
                },
                L {
                    en: "To keep a `ServerTable`'s state in the URL, give the page a route whose search parameters are the application's own struct with `#[derive(Search)]` and `Option` fields, so the address without a query string still loads. The page handler builds the `TableState` from it with `TableState::with_sorting`, and the action answers with `Patches::replace_url` of the URL its state names, so a reload or a shared link shows the same page of rows. `TableState` itself is not a search struct, because a URL without one of its fields would get a 400.",
                    ja: "`ServerTable` の状態を URL に残すには、`Option` のフィールドを持ち `#[derive(Search)]` を付けたアプリケーション自身の構造体を、ページのルートのクエリパラメータにします。こうするとクエリ文字列のないアドレスも読み込めます。ページのハンドラはそこから `TableState::with_sorting` で `TableState` を作り、アクションは状態が指す URL を `Patches::replace_url` で応答に含めます。再読み込みしても、共有したリンクを開いても、同じページの行が出ます。`TableState` そのものをクエリパラメータにしないのは、フィールドが 1 つでも欠けた URL に 400 が返るためです。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Space",
                        ja: "Space",
                    },
                    L {
                        en: "Toggles the focused checkbox. On the header checkbox, selects every row, or clears them once all are selected.",
                        ja: "フォーカスのあるチェックボックスを切り替えます。見出しのチェックボックスでは、すべての行を選び、すべて選ばれていれば選択を外します。",
                    },
                ),
                (
                    L {
                        en: "Enter, Space",
                        ja: "Enter、Space",
                    },
                    L {
                        en: "On a column header, sorts the rows by that column. On the button at the start of a row, shows or hides the row's details.",
                        ja: "列の見出しで押すと、その列で行を並べ替えます。行の先頭のボタンで押すと、その行の詳細を開閉します。",
                    },
                ),
            ],
            items: &[
                (
                    "DataTable::new",
                    "hypersig_ui/struct.DataTable.html#method.new",
                    L {
                        en: "An empty table named `label`, listing `rows` by `key`, with its state in `state`.",
                        ja: "`label` を名前に持ち、`rows` を `key` で並べ、状態を `state` に持つ空の表",
                    },
                ),
                (
                    "column",
                    "hypersig_ui/struct.DataTable.html#method.column",
                    L {
                        en: "Adds `column` after the columns added so far.",
                        ja: "それまでに追加した列の後ろに `column` を追加します。",
                    },
                ),
                (
                    "sortable_column",
                    "hypersig_ui/struct.DataTable.html#method.sortable_column",
                    L {
                        en: "Adds `column`, whose header sorts the rows by `key`, after the columns added so far.",
                        ja: "見出しで行を `key` の順に並べ替える列 `column` を、それまでに追加した列の後ろに追加します。",
                    },
                ),
                (
                    "selectable",
                    "hypersig_ui/struct.DataTable.html#method.selectable",
                    L {
                        en: "Adds a first column of checkboxes that keep the `key` of each selected row in `selected`.",
                        ja: "選んだ行の `key` を `selected` に持つチェックボックスの列を先頭に追加します。",
                    },
                ),
                (
                    "expandable",
                    "hypersig_ui/struct.DataTable.html#method.expandable",
                    L {
                        en: "Adds a first column of buttons that show `details` of a row under it, keeping the `key` of each open row in `expanded`.",
                        ja: "行の下に `details` を開閉するボタンの列を先頭に追加し、開いている行の `key` を `expanded` に持ちます。",
                    },
                ),
                (
                    "filter",
                    "hypersig_ui/struct.DataTable.html#method.filter",
                    L {
                        en: "Adds a field named `label` that shows the rows whose `text` contains its text.",
                        ja: "`label` を名前に持つ欄を追加し、`text` がその欄の文字列を含む行だけを表示します。",
                    },
                ),
                (
                    "paginate",
                    "hypersig_ui/struct.DataTable.html#method.paginate",
                    L {
                        en: "Shows `size` rows at a time, with buttons for the previous and next page.",
                        ja: "一度に `size` 行を表示し、前と次のページに移るボタンを置きます。",
                    },
                ),
                (
                    "max_height",
                    "hypersig_ui/struct.DataTable.html#method.max_height",
                    L {
                        en: "Keeps the table at most `height` high, scrolling its rows under a header that stays in place.",
                        ja: "表の高さを `height` までに抑え、見出しを固定したまま行をスクロールさせます。",
                    },
                ),
                (
                    "Column::text",
                    "hypersig_ui/struct.Column.html#method.text",
                    L {
                        en: "A column headed `header` that shows the text field `field` of each row.",
                        ja: "見出しが `header` で、各行の文字列のフィールド `field` を表示する列",
                    },
                ),
                (
                    "Column::number",
                    "hypersig_ui/struct.Column.html#method.number",
                    L {
                        en: "A column headed `header` that shows the number field `field`, aligned to the end.",
                        ja: "見出しが `header` で、数値のフィールド `field` を行末に揃えて表示する列",
                    },
                ),
                (
                    "Column::formatted",
                    "hypersig_ui/struct.Column.html#method.formatted",
                    L {
                        en: "A column headed `header` that shows the number field `field` written in `format`.",
                        ja: "見出しが `header` で、数値のフィールド `field` を `format` の形で表示する列",
                    },
                ),
                (
                    "Column::custom",
                    "hypersig_ui/struct.Column.html#method.custom",
                    L {
                        en: "A column headed `header` whose cells hold what `render` returns for each row.",
                        ja: "見出しが `header` で、各行に対して `render` が返す要素をセルに入れる列",
                    },
                ),
                (
                    "visible",
                    "hypersig_ui/struct.Column.html#method.visible",
                    L {
                        en: "Shows the column only while `when` is true.",
                        ja: "`when` が真の間だけ列を表示します。",
                    },
                ),
                (
                    "TableState",
                    "hypersig_ui/struct.TableState.html",
                    L {
                        en: "The signals a `DataTable` or `ServerTable` keeps: filter text, page, and sort.",
                        ja: "`DataTable` と `ServerTable` が持つシグナル。絞り込みの文字列、ページ、並べ替え",
                    },
                ),
                (
                    "TableText",
                    "hypersig_ui/struct.TableText.html",
                    L {
                        en: "The words a `DataTable` or `ServerTable` shows or names its controls with.",
                        ja: "`DataTable` と `ServerTable` が表示する言葉と、操作部品に付ける名前",
                    },
                ),
                (
                    "ServerTable::new",
                    "hypersig_ui/struct.ServerTable.html#method.new",
                    L {
                        en: "An empty table named `label`, listing `rows` by `key`, which runs `load` to fetch rows.",
                        ja: "`label` を名前に持ち、`rows` を `key` で並べ、行を取りに `load` を実行する空の表",
                    },
                ),
                (
                    "ServerTable::sortable_column",
                    "hypersig_ui/struct.ServerTable.html#method.sortable_column",
                    L {
                        en: "Adds `column`, whose header asks the server to sort by `key`, after the columns added so far.",
                        ja: "見出しでサーバーに `key` での並べ替えを求める列 `column` を、それまでに追加した列の後ろに追加します。",
                    },
                ),
                (
                    "ServerTable::selectable",
                    "hypersig_ui/struct.ServerTable.html#method.selectable",
                    L {
                        en: "Adds a first column of checkboxes that keep the `key` of each selected row in `selected`.",
                        ja: "選んだ行の `key` を `selected` に持つチェックボックスの列を先頭に追加します。",
                    },
                ),
                (
                    "ServerTable::filter",
                    "hypersig_ui/struct.ServerTable.html#method.filter",
                    L {
                        en: "Adds a field named `label` whose text the server filters the rows by.",
                        ja: "`label` を名前に持つ欄を追加し、サーバーがその文字列で行を絞り込みます。",
                    },
                ),
                (
                    "ServerTable::paginate",
                    "hypersig_ui/struct.ServerTable.html#method.paginate",
                    L {
                        en: "Shows pages of `size` rows out of `total`, with buttons for the previous and next page.",
                        ja: "`total` 行のうち `size` 行ずつのページを表示し、前と次のページに移るボタンを置きます。",
                    },
                ),
                (
                    "TableState::sorting",
                    "hypersig_ui/struct.TableState.html#method.sorting",
                    L {
                        en: "The key of the column a `ServerTable` is sorted by, and its order.",
                        ja: "`ServerTable` が並べ替えている列のキーと、その向き",
                    },
                ),
            ],
        },
        Component::Chart => Page {
            lead: L {
                en: "Bar, stacked bar, horizontal bar, line, area, and donut charts of the rows of a list signal, on a scale the browser fits to the values.",
                ja: "リストのシグナルの行を、棒、積み上げ棒、横棒、折れ線、面、ドーナツで描くチャートです。値の軸の目盛りはブラウザが値に合わせて決めます。",
            },
            demo: Demo::Chart,
            built_on: &[
                L {
                    en: "The drawing is an `<svg>` with `viewBox=\"0 0 100 100\"` and `preserveAspectRatio=\"none\"`, so it stretches to its box, and its lines keep their width with `vector-effect=\"non-scaling-stroke\"`. The axis labels, the legend, and the box that shows a row's values under the pointer are HTML, which does not stretch. A `DonutChart` draws each row as a circle with `pathLength=\"100\"`, whose `stroke-dasharray` is the row's share of the total in hundredths.",
                    ja: "描画は `viewBox=\"0 0 100 100\"` と `preserveAspectRatio=\"none\"` を持つ `<svg>` で、枠に合わせて伸び縮みします。線は `vector-effect=\"non-scaling-stroke\"` で太さを保ちます。軸のラベル、凡例、ポインタの下に出る行の値は、伸び縮みしない HTML で置きます。`DonutChart` は各行を `pathLength=\"100\"` の円で描き、その `stroke-dasharray` に合計に占める行の割合を百分率で書きます。",
                },
                L {
                    en: "The drawing is hidden from assistive technology. A screen reader reads a visually hidden `<table>` of the same rows, captioned with the chart's name, and the values under the pointer have no keyboard equivalent beyond that table.",
                    ja: "描画は支援技術から隠します。スクリーンリーダーは、チャートの名前をキャプションに持つ、視覚的に隠した同じ行の `<table>` を読みます。ポインタの下に出る値をキーボードで見る手段は、この表のほかにありません。",
                },
                L {
                    en: "A series reads a number field of the row type, so a field of another struct or a text field does not compile. The scale runs from a round number at or below the smallest value, or 0, to one at or above the largest, with grid lines on a step of 1, 2, 2.5, or 5 times a power of ten. The browser computes it with `max_of`, `min_of`, and `let_in`, so a statement or a patch that changes the rows refits it, and a negative value draws its bar down from the line at 0. Each bar and grid line computes the scale from every row, so the work of a change grows with the square of the number of rows.",
                    ja: "系列は行の型の数値のフィールドを読むので、別の構造体のフィールドや文字列のフィールドを渡すとコンパイルエラーになります。目盛りは、最小値か 0 の小さいほう以下の切りのよい数から、最大値以上の切りのよい数までで、刻み幅は 10 の累乗の 1、2、2.5、5 倍です。ブラウザが `max_of`、`min_of`、`let_in` で計算するので、文やパッチで行が変わると目盛りも合わせ直され、負の値の棒は 0 の線から下に伸びます。棒や目盛りの線はそれぞれ全行から目盛りを計算するので、変更 1 回の手間は行数の 2 乗で増えます。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "Chart::bar",
                    "hypersig_ui/struct.Chart.html#method.bar",
                    L {
                        en: "A bar chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける棒グラフ",
                    },
                ),
                (
                    "Chart::stacked_bar",
                    "hypersig_ui/struct.Chart.html#method.stacked_bar",
                    L {
                        en: "A stacked bar chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける積み上げ棒グラフ",
                    },
                ),
                (
                    "Chart::horizontal_bar",
                    "hypersig_ui/struct.Chart.html#method.horizontal_bar",
                    L {
                        en: "A horizontal bar chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける横棒グラフ",
                    },
                ),
                (
                    "Chart::line",
                    "hypersig_ui/struct.Chart.html#method.line",
                    L {
                        en: "A line chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける折れ線グラフ",
                    },
                ),
                (
                    "Chart::area",
                    "hypersig_ui/struct.Chart.html#method.area",
                    L {
                        en: "An area chart named `label` of `rows`, labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` を描き、`category` でラベルを付ける面グラフ",
                    },
                ),
                (
                    "series",
                    "hypersig_ui/struct.Chart.html#method.series",
                    L {
                        en: "Adds a series named `name` that draws the number field `field` in `color`.",
                        ja: "数値のフィールド `field` を `color` で描く、`name` という名前の系列を追加します。",
                    },
                ),
                (
                    "max_labels",
                    "hypersig_ui/struct.Chart.html#method.max_labels",
                    L {
                        en: "Labels at most `count` rows along the axis of the categories, 12 until set.",
                        ja: "カテゴリの軸にラベルを付ける行を最大 `count` 個にします。設定しなければ 12 個です。",
                    },
                ),
                (
                    "format",
                    "hypersig_ui/struct.Chart.html#method.format",
                    L {
                        en: "Writes the values and the axis labels in `format`.",
                        ja: "値と軸のラベルを `format` の形で書きます。",
                    },
                ),
                (
                    "DonutChart::new",
                    "hypersig_ui/struct.DonutChart.html#method.new",
                    L {
                        en: "A donut chart named `label` of `rows`, with slices of `value` labeled by `category`.",
                        ja: "`label` を名前に持ち、`rows` の `value` を `category` でラベルを付けた弧で描くドーナツグラフ",
                    },
                ),
                (
                    "token::chart_1",
                    "hypersig_ui/token/fn.chart_1.html",
                    L {
                        en: "The first series of a chart.",
                        ja: "チャートの 1 つめの系列の色",
                    },
                ),
            ],
        },
        Component::Dialog => Page {
            lead: L {
                en: "A modal dialog, a sheet along an edge of the viewport, and an alert dialog that asks the visitor to confirm.",
                ja: "モーダルダイアログ、ビューポートの端に沿って出るシート、訪問者に確認を求めるアラートダイアログです。",
            },
            demo: Demo::Dialog,
            built_on: &[
                L {
                    en: "Declare the dialog as a `const` and use it for the dialog and for the buttons that open and close it. The buttons carry `commandfor` and `command`, and the browser opens the `<dialog>` with `showModal()`. While it is open, the rest of the page is inert, Escape closes the dialog, and on closing, focus returns to the button that opened it. Invoker commands are available from Chrome 135, Firefox 144, and Safari 26.2.",
                    ja: "ダイアログは `const` として宣言し、ダイアログ本体と、それを開くボタン、閉じるボタンに使います。ボタンには `commandfor` と `command` が付き、ブラウザが `<dialog>` を `showModal()` で開きます。開いている間はページのほかの部分が inert になって操作できず、Escape でダイアログが閉じ、閉じるとフォーカスはダイアログを開いたボタンに戻ります。invoker commands は Chrome 135、Firefox 144、Safari 26.2 から使えます。",
                },
                L {
                    en: "The dialog is labeled by its title and described by its description through ids derived from the name. An alert dialog has `role=\"alertdialog\"` and no close button in its corner, so the visitor answers with one of its buttons.",
                    ja: "ダイアログには名前から作った id を通して、タイトルがラベルとして、説明が説明文として結び付きます。アラートダイアログは `role=\"alertdialog\"` を持ち、角に閉じるボタンがないので、訪問者はダイアログのボタンのどれかで答えます。",
                },
                L {
                    en: "A sheet is the same modal `<dialog>`, placed along one edge of the viewport by `Dialog::sheet`, so it keeps the focus, Escape, and backdrop behavior of a dialog.",
                    ja: "シートは同じモーダルの `<dialog>` を `Dialog::sheet` でビューポートの端に沿って置いたものなので、フォーカス、Escape、背景のクリックの振る舞いはダイアログと同じです。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Moves between the dialog's controls and never reaches the page behind.",
                        ja: "ダイアログのコントロールの間を移り、後ろのページには移りません。",
                    },
                ),
                (
                    L {
                        en: "Escape",
                        ja: "Escape",
                    },
                    L {
                        en: "Closes the dialog and returns focus to the button that opened it.",
                        ja: "ダイアログを閉じ、ダイアログを開いたボタンにフォーカスを戻します。",
                    },
                ),
            ],
            items: &[
                (
                    "Dialog::new",
                    "hypersig_ui/struct.Dialog.html#method.new",
                    L {
                        en: "A dialog that `dialog` names, whose id is the name of `dialog`.",
                        ja: "`dialog` が名前を付け、`dialog` の名前を id とするダイアログ",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Dialog.html#method.trigger",
                    L {
                        en: "A button that opens the dialog.",
                        ja: "ダイアログを開くボタン",
                    },
                ),
                (
                    "open",
                    "hypersig_ui/struct.Dialog.html#method.open",
                    L {
                        en: "A statement that opens the dialog, as its trigger button does.",
                        ja: "トリガーのボタンと同じようにダイアログを開く文",
                    },
                ),
                (
                    "dismiss",
                    "hypersig_ui/struct.Dialog.html#method.dismiss",
                    L {
                        en: "A statement that closes the dialog, as its close button does.",
                        ja: "閉じるボタンと同じようにダイアログを閉じる文",
                    },
                ),
                (
                    "close",
                    "hypersig_ui/struct.Dialog.html#method.close",
                    L {
                        en: "A button that closes the dialog. Put it inside the dialog.",
                        ja: "ダイアログの中に置く、ダイアログを閉じるボタン",
                    },
                ),
                (
                    "dialog",
                    "hypersig_ui/struct.Dialog.html#method.dialog",
                    L {
                        en: "The dialog, with `title`, `description`, `content`, and a close button in its corner.",
                        ja: "`title`、`description`、`content` と、角の閉じるボタンを持つダイアログ",
                    },
                ),
                (
                    "alert_dialog",
                    "hypersig_ui/struct.Dialog.html#method.alert_dialog",
                    L {
                        en: "An alert dialog, which asks the visitor to confirm or cancel before going on.",
                        ja: "先に進む前に、訪問者に確認か取り消しを求めるアラートダイアログ",
                    },
                ),
                (
                    "sheet",
                    "hypersig_ui/struct.Dialog.html#method.sheet",
                    L {
                        en: "A sheet: a dialog along the `side` edge of the viewport.",
                        ja: "シート。ビューポートの `side` の端に沿って出るダイアログ",
                    },
                ),
                (
                    "Side",
                    "hypersig_ui/enum.Side.html",
                    L {
                        en: "The edge of the viewport a `Dialog::sheet` sits along.",
                        ja: "`Dialog::sheet` が沿うビューポートの端",
                    },
                ),
                (
                    "dialog_footer",
                    "hypersig_ui/fn.dialog_footer.html",
                    L {
                        en: "The row of buttons at the bottom of a dialog, aligned to the end.",
                        ja: "ダイアログの下部に、末尾に寄せて並べるボタンの列",
                    },
                ),
            ],
        },
        Component::Menu => Page {
            lead: L {
                en: "A menu of actions opened by a button.",
                ja: "ボタンで開く、操作のメニューです。",
            },
            demo: Demo::Menu,
            built_on: &[
                L {
                    en: "It follows the WAI-ARIA menu button pattern. The button has `aria-haspopup=\"menu\"`, the menu `role=\"menu\"`, and each action `role=\"menuitem\"`. The menu is a `popover`, so the browser shows it over the page and closes it on Escape or on a click outside. It is placed under the button with CSS anchor positioning and moves above it where there is no room below.",
                    ja: "WAI-ARIA menu button パターンに従います。ボタンは `aria-haspopup=\"menu\"` を、メニューは `role=\"menu\"` を、各操作は `role=\"menuitem\"` を持ちます。メニューは `popover` なので、ブラウザがページの上に重ねて表示し、Escape か外側のクリックで閉じます。メニューは CSS の anchor positioning でボタンの下に置かれ、下に場所がなければボタンの上に移ります。",
                },
                L {
                    en: "A disabled action keeps its place, and the arrow keys skip it.",
                    ja: "無効な操作はその位置に残り、矢印キーでは飛ばされます。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Enter, Space",
                        ja: "Enter、Space",
                    },
                    L {
                        en: "With focus on the button, opens the menu and moves focus to the first action. With focus on an action, runs it and closes the menu.",
                        ja: "ボタンにフォーカスがあるときに押すと、メニューを開いて最初の操作にフォーカスを移します。操作にフォーカスがあるときに押すと、その操作を実行してメニューを閉じます。",
                    },
                ),
                (
                    L {
                        en: "Up, Down",
                        ja: "上矢印、下矢印",
                    },
                    L {
                        en: "Move to the previous or next action.",
                        ja: "前か次の操作に移ります。",
                    },
                ),
                (
                    L {
                        en: "Home, End",
                        ja: "Home、End",
                    },
                    L {
                        en: "Move to the first or last action.",
                        ja: "最初か最後の操作に移ります。",
                    },
                ),
                (
                    L {
                        en: "Escape",
                        ja: "Escape",
                    },
                    L {
                        en: "Closes the menu and returns focus to the button.",
                        ja: "メニューを閉じ、ボタンにフォーカスを戻します。",
                    },
                ),
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Closes the menu.",
                        ja: "メニューを閉じます。",
                    },
                ),
            ],
            items: &[
                (
                    "Menu::new",
                    "hypersig_ui/struct.Menu.html#method.new",
                    L {
                        en: "A menu that `menu` names, whose id is the name of `menu`.",
                        ja: "`menu` が名前を付け、`menu` の名前を id とするメニュー",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Menu.html#method.trigger",
                    L {
                        en: "The button that opens and closes the menu.",
                        ja: "メニューを開閉するボタン",
                    },
                ),
                (
                    "menu",
                    "hypersig_ui/struct.Menu.html#method.menu",
                    L {
                        en: "The menu, holding `items` in order.",
                        ja: "`items` を順に並べたメニュー",
                    },
                ),
                (
                    "MenuItem::new",
                    "hypersig_ui/struct.MenuItem.html#method.new",
                    L {
                        en: "An action labeled `label` that runs `run` when chosen.",
                        ja: "`label` をラベルに持ち、選ばれると `run` を実行する操作",
                    },
                ),
                (
                    "destructive",
                    "hypersig_ui/struct.MenuItem.html#method.destructive",
                    L {
                        en: "Draws the action in `token::destructive`. It is for an action that deletes.",
                        ja: "操作を `token::destructive` の色で描きます。削除する操作に使います。",
                    },
                ),
                (
                    "disabled",
                    "hypersig_ui/struct.MenuItem.html#method.disabled",
                    L {
                        en: "Disables the action while `when` is true, which the arrow keys then skip.",
                        ja: "`when` が true の間、操作を無効にします。無効な操作は矢印キーで飛ばされます。",
                    },
                ),
            ],
        },
        Component::Tooltip => Page {
            lead: L {
                en: "A short description shown while a button is hovered or focused.",
                ja: "ボタンにポインタが重なっている間か、フォーカスがある間に表示する短い説明です。",
            },
            demo: Demo::Tooltip,
            built_on: &[
                L {
                    en: "It follows the WAI-ARIA tooltip pattern. The tooltip has `role=\"tooltip\"` and the button `aria-describedby` pointing at it, so a screen reader reads the text after the button's name. `Tooltip` builds the button and gives it the attributes the tooltip needs.",
                    ja: "WAI-ARIA tooltip パターンに従います。ツールチップは `role=\"tooltip\"` を持ち、ボタンの `aria-describedby` がツールチップを指すので、スクリーンリーダーはボタンの名前のあとにツールチップのテキストを読み上げます。ボタンは `Tooltip` が作り、ツールチップに必要な属性をそのボタンに付けます。",
                },
                L {
                    en: "The tooltip is a `popover=\"hint\"`, which browser-compat-data 8.1.6 lists for Chrome 151 and Firefox 153, with partial older implementations in earlier versions, and for Safari only in its preview. A browser without it treats the value as `manual`, and there Escape closes the tooltip only while focus is on its button.",
                    ja: "ツールチップは `popover=\"hint\"` です。browser-compat-data 8.1.6 では、Chrome 151 と Firefox 153 が対応しており、それより前のバージョンには部分的な古い実装があり、Safari はプレビュー版だけが対応しています。対応していないブラウザはこの値を `manual` として扱うので、Escape でツールチップが閉じるのは、フォーカスがそのボタンにある間だけです。",
                },
            ],
            keys: &[
                (
                    L {
                        en: "Tab",
                        ja: "Tab",
                    },
                    L {
                        en: "Focusing the button opens the tooltip, and leaving it closes the tooltip.",
                        ja: "ボタンにフォーカスが移るとツールチップが開き、フォーカスが離れると閉じます。",
                    },
                ),
                (
                    L {
                        en: "Any key",
                        ja: "任意のキー",
                    },
                    L {
                        en: "With focus on the button, closes the tooltip. Escape also reaches a dialog around the tooltip.",
                        ja: "ボタンにフォーカスがあるときに押すと、ツールチップが閉じます。Escape の場合は、ツールチップを囲むダイアログにもキーが届きます。",
                    },
                ),
            ],
            items: &[
                (
                    "Tooltip::new",
                    "hypersig_ui/struct.Tooltip.html#method.new",
                    L {
                        en: "A tooltip that `tip` names, whose id is the name of `tip`.",
                        ja: "`tip` が名前を付け、`tip` の名前を id とするツールチップ",
                    },
                ),
                (
                    "trigger",
                    "hypersig_ui/struct.Tooltip.html#method.trigger",
                    L {
                        en: "A button described by the tooltip.",
                        ja: "ツールチップが説明するボタン",
                    },
                ),
                (
                    "icon_trigger",
                    "hypersig_ui/struct.Tooltip.html#method.icon_trigger",
                    L {
                        en: "An icon button named `label` and described by the tooltip.",
                        ja: "`label` を名前に持ち、ツールチップが説明するアイコンボタン",
                    },
                ),
                (
                    "tooltip",
                    "hypersig_ui/struct.Tooltip.html#method.tooltip",
                    L {
                        en: "The tooltip, holding `text`.",
                        ja: "`text` を入れたツールチップ",
                    },
                ),
            ],
        },
        Component::Toast => Page {
            lead: L {
                en: "Short notices shown together in a corner of the page, added from the browser or by the server.",
                ja: "ページの隅に並べて表示する短いお知らせです。ブラウザからもサーバーからも追加できます。",
            },
            demo: Demo::Toast,
            built_on: &[
                L {
                    en: "The toaster is a landmark region named \"Notifications\" holding an `aria-live=\"polite\"` list, so a screen reader reads a toast when it is added, after what it is reading. Each toast has a button that removes it.",
                    ja: "トーストを並べる領域（toaster）は「Notifications」という名前のランドマークで、`aria-live=\"polite\"` の一覧を持ちます。そのため、トーストが追加されると、スクリーンリーダーは読み上げ中の内容のあとにそのトーストを読み上げます。各トーストには、そのトーストを消すボタンがあります。",
                },
                L {
                    en: "A toast stays until it is dismissed, unless `Toaster::dismiss_after` sets a time, because a visitor who reads slowly or uses a magnifier can miss a notice that is removed automatically (WCAG 2.2.1). The region is in the page rather than in the top layer, so an open modal dialog makes it inert until the dialog closes.",
                    ja: "トーストは `Toaster::dismiss_after` で時間を設定しない限り、消されるまで残ります。ゆっくり読む訪問者や拡大鏡を使う訪問者は、自動で消えるお知らせを見逃すことがあるからです（WCAG 2.2.1）。この領域は top layer ではなくページの中にあるので、モーダルダイアログが開いている間は inert になって操作できず、ダイアログが閉じると操作できるようになります。",
                },
            ],
            keys: &[(
                L {
                    en: "Tab, Enter",
                    ja: "Tab、Enter",
                },
                L {
                    en: "Reach a toast's Dismiss button and press it.",
                    ja: "トーストの Dismiss ボタンに移り、それを押します。",
                },
            )],
            items: &[
                (
                    "Toaster::new",
                    "hypersig_ui/struct.Toaster.html#method.new",
                    L {
                        en: "A region that shows `toasts`, newest last.",
                        ja: "`toasts` を新しいものほど後ろに並べて表示する領域",
                    },
                ),
                (
                    "label",
                    "hypersig_ui/struct.Toaster.html#method.label",
                    L {
                        en: "Names the region for assistive technology, in place of \"Notifications\".",
                        ja: "「Notifications」の代わりに、支援技術に向けた領域の名前を付けます。",
                    },
                ),
                (
                    "dismiss_after",
                    "hypersig_ui/struct.Toaster.html#method.dismiss_after",
                    L {
                        en: "Removes each toast `delay` after it appears.",
                        ja: "各トーストを表示してから `delay` が経ったら取り除きます。",
                    },
                ),
                (
                    "Toast::new",
                    "hypersig_ui/struct.Toast.html#method.new",
                    L {
                        en: "A toast titled `title`, with an id no other toast of the process has.",
                        ja: "`title` をタイトルに持ち、プロセスのほかのどのトーストとも違う id を持つトースト",
                    },
                ),
                (
                    "description",
                    "hypersig_ui/struct.Toast.html#method.description",
                    L {
                        en: "Adds a line of text under the title.",
                        ja: "タイトルの下に 1 行のテキストを追加します。",
                    },
                ),
                (
                    "variant",
                    "hypersig_ui/struct.Toast.html#method.variant",
                    L {
                        en: "Sets the look of the toast.",
                        ja: "トーストの見た目を設定します。",
                    },
                ),
                (
                    "push",
                    "hypersig_ui/struct.Toast.html#method.push",
                    L {
                        en: "A statement that adds this toast to `toasts` in the browser.",
                        ja: "ブラウザでこのトーストを `toasts` に追加する文",
                    },
                ),
                (
                    "Toast::push_expr",
                    "hypersig_ui/struct.Toast.html#method.push_expr",
                    L {
                        en: "A statement that adds `toast`, which the browser evaluates, to `toasts`.",
                        ja: "ブラウザが評価する `toast` を `toasts` に追加する文",
                    },
                ),
                (
                    "Toast::body",
                    "hypersig_ui/struct.Toast.html#method.body",
                    L {
                        en: "Typed reference to the `body` signal.",
                        ja: "`body` シグナルへの型付きの参照",
                    },
                ),
            ],
        },
        Component::Alert => Page {
            lead: L {
                en: "A bordered box for a message, with a title and a description.",
                ja: "タイトルと説明を持つ、メッセージ用の枠線付きのボックスです。",
            },
            demo: Demo::Alert,
            built_on: &[L {
                en: "Both variants are live regions. A destructive alert has `role=\"alert\"`, which a screen reader reads when it is put on the page, as the demo does with `when`. A default alert has `role=\"status\"`, which is read when its text changes. Some screen readers skip a status that arrives with its text already in it, so keep a status box on the page and change its text with `bind_text`.",
                ja: "どちらのバリアントもライブリージョンです。destructive のアラートは `role=\"alert\"` を持ち、ページに置かれたときにスクリーンリーダーが読み上げます。デモでは `when` でアラートをページに置いています。default のアラートは `role=\"status\"` を持ち、テキストが変わったときに読み上げられます。最初からテキストが入った状態で現れた status を読み飛ばすスクリーンリーダーもあるので、status のボックスはページに置いたままにして、テキストを `bind_text` で変えてください。",
            }],
            keys: &[],
            items: &[
                (
                    "alert",
                    "hypersig_ui/fn.alert.html",
                    L {
                        en: "A bordered box for a message, holding an `alert_title` and an `alert_description`.",
                        ja: "`alert_title` と `alert_description` を入れる、メッセージ用の枠線付きのボックス",
                    },
                ),
                (
                    "alert_title",
                    "hypersig_ui/fn.alert_title.html",
                    L {
                        en: "The first line of an `alert`.",
                        ja: "`alert` の最初の行",
                    },
                ),
                (
                    "alert_description",
                    "hypersig_ui/fn.alert_description.html",
                    L {
                        en: "The text under the title of an `alert`.",
                        ja: "`alert` のタイトルの下のテキスト",
                    },
                ),
                (
                    "AlertVariant",
                    "hypersig_ui/enum.AlertVariant.html",
                    L {
                        en: "The look and the urgency of an alert.",
                        ja: "アラートの見た目と緊急度",
                    },
                ),
            ],
        },
        Component::Badge => Page {
            lead: L {
                en: "A badge that shows a short label, such as a status.",
                ja: "状態などの短いラベルを表示するバッジです。",
            },
            demo: Demo::Badge,
            built_on: &[L {
                en: "A badge is a `<span>` and takes no role, so a screen reader reads it where it stands. Where the color alone carries the meaning, put the meaning in the text too, such as \"Failed\" rather than a red dot.",
                ja: "バッジは `<span>` でロールを持たないので、スクリーンリーダーは置かれた位置でそのまま読み上げます。色だけで意味を伝えている場合は、赤い点だけで済ませずに、「失敗」のように意味をテキストにも書いてください。",
            }],
            keys: &[],
            items: &[
                (
                    "badge",
                    "hypersig_ui/fn.badge.html",
                    L {
                        en: "A `<span>` that shows a short label, such as a status, as a badge.",
                        ja: "状態のような短いラベルをバッジとして表示する `<span>`",
                    },
                ),
                (
                    "BadgeVariant",
                    "hypersig_ui/enum.BadgeVariant.html",
                    L {
                        en: "The look of a badge.",
                        ja: "バッジの見た目",
                    },
                ),
            ],
        },
        Component::Progress => Page {
            lead: L {
                en: "A bar filled up to a percentage, and a block that stands in for content still loading.",
                ja: "割合の分だけ塗りつぶされるバーと、読み込み中の内容の代わりに置くブロックです。",
            },
            demo: Demo::Progress,
            built_on: &[
                L {
                    en: "The bar is a `role=\"progressbar\"` with `aria-valuenow` kept equal to the value, so a screen reader reads the label and the percentage. A value below 0 or above 100 is drawn and announced as 0 or 100.",
                    ja: "バーは `role=\"progressbar\"` を持ち、`aria-valuenow` が常に値と同じになるので、スクリーンリーダーはラベルと割合を読み上げます。0 未満や 100 を超える値は、0 や 100 として描かれ、読み上げられます。",
                },
                L {
                    en: "A skeleton is hidden from assistive technology. Tell a screen reader that the content is loading elsewhere, as the demo does with `aria-busy` on the region that holds it.",
                    ja: "スケルトンは支援技術から隠してあります。内容を読み込み中であることは、別の場所でスクリーンリーダーに伝えてください。デモではスケルトンを置いた領域に `aria-busy` を付けています。",
                },
            ],
            keys: &[],
            items: &[
                (
                    "progress",
                    "hypersig_ui/fn.progress.html",
                    L {
                        en: "A bar filled up to `percent`, a value from 0 to 100, named by `label`.",
                        ja: "`label` を名前に持ち、0 から 100 の値 `percent` の分だけ塗りつぶされるバー",
                    },
                ),
                (
                    "skeleton",
                    "hypersig_ui/fn.skeleton.html",
                    L {
                        en: "A pulsing block in `token::accent` that stands in for content still loading.",
                        ja: "読み込み中の内容の代わりに置く、`token::accent` の色で明滅するブロック",
                    },
                ),
            ],
        },
    }
}

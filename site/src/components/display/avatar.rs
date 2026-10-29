//! The Components page on `Avatar`: a round picture of a person, with their
//! initials in its place.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Avatar).leads()),
        demo::show(site, Demo::Avatar),
        accessibility(site),
        building(site),
        html(site),
        compile_errors(site),
        unchecked(site),
        example(site),
    ])
}

fn accessibility(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Accessibility",
                ja: "アクセシビリティ",
            },
        ),
        para(
            site,
            L {
                en: "The avatar is a `role=\"img\"` named after the person through `aria-label`, so a screen reader reads the name once, as an image, and nothing inside it: neither the initials nor the picture's `alt`. The avatar takes no keys.",
                ja: "アバターは `aria-label` で人の名前を付けた `role=\"img\"` なので、スクリーンリーダーは名前を画像として 1 回だけ読み、中身（イニシャルも画像の `alt` も）は読みません。アバターはキー操作を受けません。",
            },
        ),
        para(
            site,
            L {
                en: "Without a picture it shows the initials. With one, it shows the picture, and where the picture fails to load, the broken `<img>` draws the initials over itself through its `::after`. Chrome draws that `::after`. A browser that draws no `::after` on a broken image shows its own marker for one instead.",
                ja: "画像がなければイニシャルを表示します。画像があれば画像を表示し、画像の読み込みに失敗したときは、壊れた `<img>` が `::after` でイニシャルを上に描きます。Chrome はこの `::after` を描きます。壊れた画像に `::after` を描かないブラウザでは、代わりにそのブラウザ自身の壊れた画像の印が表示されます。",
            },
        ),
    ])
}

fn building(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Building an avatar",
                ja: "アバターを作る",
            },
        ),
        para(
            site,
            L {
                en: "[`Avatar::new`](api:hypersig_ui/struct.Avatar.html#method.new) takes the person's name, which a screen reader reads, and the initials shown in place of a picture. [`picture`](api:hypersig_ui/struct.Avatar.html#method.picture) adds a picture from a checked URL:",
                ja: "[`Avatar::new`](api:hypersig_ui/struct.Avatar.html#method.new) は、スクリーンリーダーが読む人の名前と、画像の代わりに表示するイニシャルを受け取ります。[`picture`](api:hypersig_ui/struct.Avatar.html#method.picture) は検査済みの URL から画像を加えます。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl<S> Avatar<S> {
    pub fn new(name: impl Into<String>, initials: impl Into<String>) -> Self;
    pub fn picture(self, url: impl IntoHref<S>) -> Self;
    pub fn render(self) -> El<S>;
}

impl<S> From<Avatar<S>> for El<S>;
impl<S> From<Avatar<S>> for Node<S>;",
        ),
        table_of(
            site,
            L {
                en: &["`url`", "Where the picture comes from"],
                ja: &["`url`", "画像の出どころ"],
            },
            L {
                en: &[
                    &[
                        "A `SafeUrl` constant, `SafeUrl::new(\"https://…\")`",
                        "A URL checked when the program is compiled",
                    ],
                    &[
                        "A `Sig<S, SafeUrl>`",
                        "A signal, so the browser swaps the picture when a statement or a patch changes it",
                    ],
                    &["A route's link", "A page or a file the server serves"],
                ],
                ja: &[
                    &[
                        "`SafeUrl` の定数（`SafeUrl::new(\"https://…\")`）",
                        "プログラムのコンパイル時に検査した URL",
                    ],
                    &[
                        "`Sig<S, SafeUrl>`",
                        "シグナル。文や差分でシグナルが変わると、ブラウザが画像を差し替えます",
                    ],
                    &["ルートのリンク", "サーバーが返すページやファイル"],
                ],
            },
        ),
        para(
            site,
            L {
                en: "`El::child` takes the `Avatar` itself, and `render` returns the `<span>` as an `El<S>` for a method of `El`. [Values the types do not check](learn/unchecked-values/) covers `SafeUrl` and the URLs it checks.",
                ja: "`El::child` には `Avatar` をそのまま渡せます。`El` のメソッドを使うときは、`render` で `<span>` を `El<S>` として取り出します。`SafeUrl` と、それが検査する URL は [型で検査しない値](learn/unchecked-values/) で説明しています。",
            },
        ),
    ])
}

fn html(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The HTML",
                ja: "出力される HTML",
            },
        ),
        para(
            site,
            L {
                en: "The example at the end of this page renders these two avatars, with the classes left out. The initials of the first are the `alt` of its picture:",
                ja: "このページの最後の例は次の 2 つのアバターを出力します（クラスは省いています）。1 つめのイニシャルは画像の `alt` になります。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            r#"<span role="img" aria-label="Ada Lovelace">
  <img src="https://example.com/ada.png" alt="AL">
</span>
<span role="img" aria-label="Grace Hopper">GH</span>"#,
        ),
    ])
}

fn compile_errors(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What fails to compile",
                ja: "コンパイルエラーになるもの",
            },
        ),
        para(
            site,
            L {
                en: "`picture` takes `impl IntoHref<S>`, which a `&str` or a `String` does not implement, so a URL from a visitor's input cannot reach `src` unchecked. It is E0277, with a message that says the string is not a URL:",
                ja: "`picture` は `impl IntoHref<S>` を受け取り、`&str` や `String` はこれを実装していないので、訪問者が入力した URL が検査されずに `src` に入ることはありません。E0277 になり、エラーメッセージは文字列が URL ではないことを示します。",
            },
        ),
        show(Snippet::UiAvatarUrl),
        para(
            site,
            L {
                en: "The name and the initials are both required, so an avatar with no name for a screen reader is E0061.",
                ja: "名前とイニシャルはどちらも必須なので、スクリーンリーダーが読む名前のないアバターは E0061 になります。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What is not checked",
                ja: "検査されないこと",
            },
        ),
        para(
            site,
            L {
                en: "Nothing checks that the initials match the name, or that they fit: the avatar is `2rem` across, and more than two or three letters overflow it and are cut off. Nothing checks that the picture exists either. A missing one shows the initials in Chrome and the browser's own marker for a broken image elsewhere, as the demo's first button shows.",
                ja: "イニシャルが名前と合っているか、収まるかは検査されません。アバターの幅は `2rem` なので、2〜3 文字を超えるとはみ出し、切り取られます。画像が存在するかどうかも検査されません。画像がないとき、Chrome ではイニシャルが、ほかのブラウザではそのブラウザの壊れた画像の印が表示されます。デモの 1 つめのボタンで確かめられます。",
            },
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example: a team",
                ja: "例: チーム",
            },
        ),
        para(
            site,
            L {
                en: "The first avatar's picture comes from a signal, and the second shows initials alone:",
                ja: "1 つめのアバターの画像はシグナルから取り、2 つめはイニシャルだけを表示します。",
            },
        ),
        show(Snippet::UiAvatar),
        para(
            site,
            L {
                en: "[Badge](components/badge/) puts a short label, such as a role, next to the avatar, and [Card](components/card/) holds a person's details around it.",
                ja: "アバターの横に役割のような短いラベルを置くには [バッジ](components/badge/) を、アバターの周りに人の詳細をまとめるには [カード](components/card/) を使います。",
            },
        ),
    ])
}

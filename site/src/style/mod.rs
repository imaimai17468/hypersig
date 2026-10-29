//! The Style section: typed atomic CSS from `hypersig::style`.

mod animation;
mod layout;
mod unchecked;

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::{L, Text};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

/// The article of `page`.
pub fn content(site: Site, page: StylePage) -> Part {
    let head = title(Doc::Style(page).title(site));
    match page {
        // Get started
        StylePage::Overview | StylePage::Applying => overview(site, head),
        // Values
        StylePage::Values
        | StylePage::Lengths
        | StylePage::Colors
        | StylePage::AnglesAndTimes
        | StylePage::UrlsAndStrings
        | StylePage::Keywords => values(site, head),
        // Conditions
        StylePage::PseudoClasses
        | StylePage::PseudoElements
        | StylePage::Media
        | StylePage::Containers
        | StylePage::GroupAndPeer
        | StylePage::Toggled
        | StylePage::Conditions => conditions(site, head),
        // Tokens and themes
        StylePage::Tokens | StylePage::Themes | StylePage::DarkMode => tokens(site, head),
        // Global styles
        StylePage::Global | StylePage::Fonts => global(site, head),
        // Layout
        StylePage::Stacks => layout::stacks::page(site),
        StylePage::Grids => layout::grids::page(site),
        StylePage::Patterns => layout::patterns::page(site),
        // Animation
        StylePage::Animation => animation::keyframes::page(site),
        StylePage::Transitions => animation::transitions::page(site),
        StylePage::PageTransitions => animation::page_transitions::page(site),
        StylePage::ReducedMotion => animation::reduced_motion::page(site),
        // Opting out of type checks
        StylePage::Unchecked => unchecked::values::page(site),
    }
}

fn overview(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Overview).leads()),
        heading(
            site,
            L {
                en: "A style is a Rust value",
                ja: "スタイルは Rust の値",
            },
        ),
        para(
            site,
            L {
                en: "[`Style`](api:hypersig_style/struct.Style.html) has one method per standard CSS property, and per anchor positioning property. The property list, each property's keywords, the pseudo-classes, the pseudo-elements, and the named colors are generated from MDN's CSS data (`mdn-data` 2.37.2). `El::css` adds a style to an element.",
                ja: "[`Style`](api:hypersig_style/struct.Style.html) には、標準の CSS プロパティとアンカー配置のプロパティごとに 1 つずつメソッドがあります。プロパティの一覧、各プロパティのキーワード、擬似クラス、擬似要素、名前付きの色は、MDN の CSS データ（`mdn-data` 2.37.2）から生成しています。要素にスタイルを付けるには `El::css` を使います。",
            },
        ),
        show(Snippet::StyleBasics),
        para(
            site,
            L {
                en: "A second call to the same property's method under the same conditions replaces the earlier value, as a later declaration does in a CSS rule. `merge` adds the declarations of another style, and the added declarations override the original ones.",
                ja: "同じ条件の下で同じプロパティのメソッドをもう一度呼ぶと、CSS のルールで後に書いた宣言と同じく、後の値が前の値を置き換えます。`merge` は別のスタイルの宣言を追加し、追加した側の宣言が元の宣言を上書きします。",
            },
        ),
        heading(
            site,
            L {
                en: "One class per declaration",
                ja: "宣言ごとに 1 つのクラス",
            },
        ),
        para(
            site,
            L {
                en: "Each declaration becomes one class, whose name comes from the declaration, such as `cursor_pointer` or `hover:color_crimson`. Panda CSS and Tailwind CSS do the same in their atomic CSS. For a value with characters other than letters, digits, `.`, `#`, `%`, and `-`, the class name uses a hash of the value. Two elements with the same declaration use the same class, so the stylesheet grows with the number of distinct declarations rather than the number of elements.",
                ja: "宣言はそれぞれ 1 つのクラスになり、クラス名は `cursor_pointer` や `hover:color_crimson` のように宣言から決まります。Panda CSS や Tailwind CSS の atomic CSS と同じ方式です。英字、数字、`.`、`#`、`%`、`-` 以外の文字を含む値では、値のハッシュをクラス名に使います。同じ宣言を持つ 2 つの要素は同じクラスを使うので、スタイルシートは要素の数ではなく異なる宣言の数に応じて大きくなります。",
            },
        ),
        para(
            site,
            L {
                en: "The page collects the styles its elements use and writes each rule once into `<head>`. A patch that contains new elements also contains their rules, inserted where a breakpoint value still overrides the base value.",
                ja: "ページは要素が使うスタイルを集め、各ルールを 1 回だけ `<head>` に書きます。新しい要素を含むパッチにはその要素のルールも含まれ、ルールはブレークポイントの値が引き続き基本の値を上書きする位置に入ります。",
            },
        ),
        heading(
            site,
            L {
                en: "Styles switched by a signal",
                ja: "シグナルで切り替わるスタイル",
            },
        ),
        para(
            site,
            L {
                en: "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal, like the variants of a Panda CSS recipe. Their rules are in a later cascade layer than those of `css`, so while switched on they override the element's own declarations for the same property.",
                ja: "`css_when(style, cond)` はシグナルの条件が成り立つ間だけスタイルを追加し、`css_match(value, cases)` はシグナルの値ごとにスタイルを 1 つ選びます（Panda CSS の recipe の variants に当たります）。これらのルールは `css` のルールより後ろのカスケードレイヤーに入るので、有効な間は同じプロパティについて要素自身の宣言を上書きします。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Cascade layer", "Holds"],
                ja: &["カスケードレイヤー", "入るもの"],
            },
            L {
                en: &[
                    &["`hypersig-tokens`", "The values a `Theme` sets on `:root`"],
                    &[
                        "`hypersig-base`",
                        "The rules of a `Global`, such as the reset",
                    ],
                    &["`hypersig-atoms`", "The classes of `El::css`"],
                    &[
                        "`hypersig-toggled`",
                        "The classes of `El::css_when` and `El::css_match`",
                    ],
                ],
                ja: &[
                    &["`hypersig-tokens`", "`Theme` が `:root` に設定する値"],
                    &["`hypersig-base`", "リセットなど、`Global` のルール"],
                    &["`hypersig-atoms`", "`El::css` のクラス"],
                    &[
                        "`hypersig-toggled`",
                        "`El::css_when` と `El::css_match` のクラス",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The layers come in that order, so an element's classes override the global rules, and a switched-on style overrides both. A stylesheet added with `Page::style` is in no layer and overrides all four layers.",
                ja: "レイヤーはこの順に並ぶので、要素のクラスはグローバルなルールを上書きし、有効になったスタイルはその両方を上書きします。`Page::style` で追加したスタイルシートはどのレイヤーにも入らず、4 つのレイヤーすべてを上書きします。",
            },
        ),
    ])
}

fn values(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Values).leads()),
        heading(
            site,
            L {
                en: "Typed values",
                ja: "型の付いた値",
            },
        ),
        para(
            site,
            L {
                en: "Where a property accepts one keyword or one value of a type in [`value`](api:hypersig_style/value/index.html), that value is typed. `display` takes a `Display`, whose variants are its keywords. `width` takes a `Width`, which converts from a length or a percentage.",
                ja: "プロパティがキーワード 1 つか、[`value`](api:hypersig_style/value/index.html) にある型の値 1 つを受け取る場合、その値には型が付きます。`display` は `Display` を受け取り、そのバリアントが `display` のキーワードです。`width` は `Width` を受け取り、`Width` は長さかパーセンテージから変換できます。",
            },
        ),
        show(Snippet::StyleValues),
        table_of(
            site,
            L {
                en: &["Value", "Written"],
                ja: &["値", "書き方"],
            },
            L {
                en: &[
                    &["Length", "`px(1)`, `rem(1.5)`, and the other units"],
                    &["Percentage", "`pct(50)`"],
                    &[
                        "Time and angle",
                        "`ms(150)`, `secs(1)`, `deg(90)`, `turn(0.5)`",
                    ],
                    &[
                        "Color",
                        "`Color::CRIMSON` and the other named colors, `Color::hex`, `Color::rgb`, `opacity`, `mix`, `Color::CURRENT_COLOR`, `Color::TRANSPARENT`",
                    ],
                    &[
                        "Text and URLs",
                        "`quoted(\"→\")` escapes the text, and `url(\"/a.png\")` writes `url(\"...\")` for a path or an `http` or `https` URL, so `url(\"javascript:...\")` in a `const` fails to compile",
                    ],
                    &[
                        "Every property",
                        "`keyword(Property::Color, CssWideKeyword::Inherit)` for `inherit`, `initial`, `unset`, `revert`, and `revert-layer`",
                    ],
                ],
                ja: &[
                    &["長さ", "`px(1)`、`rem(1.5)` などの単位"],
                    &["パーセンテージ", "`pct(50)`"],
                    &["時間と角度", "`ms(150)`、`secs(1)`、`deg(90)`、`turn(0.5)`"],
                    &[
                        "色",
                        "`Color::CRIMSON` などの名前付きの色、`Color::hex`、`Color::rgb`、`opacity`、`mix`、`Color::CURRENT_COLOR`、`Color::TRANSPARENT`",
                    ],
                    &[
                        "テキストと URL",
                        "`quoted(\"→\")` はテキストをエスケープします。`url(\"/a.png\")` はパスか `http`、`https` の URL から `url(\"...\")` を書くので、`const` の中の `url(\"javascript:...\")` はコンパイルエラーになります",
                    ],
                    &[
                        "どのプロパティでも",
                        "`inherit`、`initial`、`unset`、`revert`、`revert-layer` には `keyword(Property::Color, CssWideKeyword::Inherit)`",
                    ],
                ],
            },
        ),
        heading(
            site,
            L {
                en: "What does not compile",
                ja: "コンパイルできないもの",
            },
        ),
        para(
            site,
            L {
                en: "A misspelled property has no method, a misspelled keyword has no variant, and a value of the wrong type has no conversion:",
                ja: "綴りを間違えたプロパティにはメソッドがなく、綴りを間違えたキーワードにはバリアントがなく、型の違う値には変換がありません。",
            },
        ),
        show(Snippet::WrongValue),
        note(
            site,
            L {
                en: "Ranges are not checked, so a negative width compiles and the browser ignores the declaration. A value with several parts, such as `margin: 0 auto`, is written with `raw`, which [Unchecked values](style/unchecked/) covers.",
                ja: "値の範囲は検査しないので、負の幅もコンパイルが通り、ブラウザはその宣言を無視します。`margin: 0 auto` のように複数の部分からなる値は `raw` で書きます。`raw` は[検査しない値](style/unchecked/)で説明しています。",
            },
        ),
    ])
}

fn conditions(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Conditions).leads()),
        heading(
            site,
            L {
                en: "Pseudo-classes, media, and containers",
                ja: "擬似クラス、メディア、コンテナ",
            },
        ),
        show(Snippet::Conditions),
        bullets(
            site,
            &[
                L {
                    en: "[`PseudoClass`](api:hypersig_style/enum.PseudoClass.html) and [`PseudoElement`](api:hypersig_style/enum.PseudoElement.html) are generated from MDN's data. `pseudo` puts declarations on a pseudo-element. A pseudo-class written inside it applies to the element itself, so `pseudo(Before, Style::new().on(Hover, ..))` becomes `:hover::before`.",
                    ja: "[`PseudoClass`](api:hypersig_style/enum.PseudoClass.html) と [`PseudoElement`](api:hypersig_style/enum.PseudoElement.html) は MDN のデータから生成しています。`pseudo` は擬似要素に宣言を付けます。その中に書いた擬似クラスは元の要素に対して適用されるので、`pseudo(Before, Style::new().on(Hover, ..))` は `:hover::before` になります。",
                },
                L {
                    en: "The user-action pseudo-classes are written to the stylesheet in a fixed order. As a result, `:active` takes precedence over `:hover` while a button is pressed, and `:disabled` takes precedence over all of them.",
                    ja: "ユーザー操作の擬似クラスは決まった順でスタイルシートに書かれます。そのため、ボタンを押している間は `:active` が `:hover` より優先され、`:disabled` はそのどれよりも優先されます。",
                },
                L {
                    en: "[`Media`](api:hypersig_style/enum.Media.html) has the breakpoints `Sm`, `Md`, `Lg`, `Xl`, and `Xxl`, min-width queries at 40, 48, 64, 80, and 96rem, the widths Tailwind CSS and Panda CSS use. It also has `Dark`, `Light`, `ReducedMotion`, `Print`, `Landscape`, `Portrait`, `CanHover`, `CoarsePointer`, `ForcedColors`, and `MoreContrast`.",
                    ja: "[`Media`](api:hypersig_style/enum.Media.html) にはブレークポイント `Sm`、`Md`、`Lg`、`Xl`、`Xxl` があり、それぞれ 40、48、64、80、96rem の min-width のメディアクエリです。幅は Tailwind CSS と Panda CSS が使うものと同じです。ほかに `Dark`、`Light`、`ReducedMotion`、`Print`、`Landscape`、`Portrait`、`CanHover`、`CoarsePointer`、`ForcedColors`、`MoreContrast` があります。",
                },
                L {
                    en: "[`Container`](api:hypersig_style/enum.Container.html) has Panda CSS's container sizes from 16rem to 90rem. An ancestor becomes a query container with `container_type`, and without one the condition never matches.",
                    ja: "[`Container`](api:hypersig_style/enum.Container.html) には、Panda CSS と同じ 16rem から 90rem までのコンテナの大きさがあります。祖先は `container_type` でクエリコンテナになります。そうした祖先がなければ、条件は一度も当てはまりません。",
                },
                L {
                    en: "Conditions can be nested. A style passed to `on` can use `on` itself, and the inner declarations then apply when both conditions hold.",
                    ja: "条件は入れ子にできます。`on` に渡すスタイルの中でも `on` を使えます。その場合、内側の宣言は両方の条件が成り立つときに適用されます。",
                },
            ],
        ),
        heading(
            site,
            L {
                en: "Group and peer",
                ja: "group と peer",
            },
        ),
        para(
            site,
            L {
                en: "`Style::group` marks an ancestor, and `Condition::Group(PseudoClass::Hover)` applies a style only while that ancestor is hovered. `Style::peer` marks an earlier sibling, and `Condition::Peer(PseudoClass::Checked)` applies a style only while that sibling is checked. Both work like the `group` and `peer` of Tailwind CSS and Panda CSS.",
                ja: "`Style::group` は祖先に印を付け、`Condition::Group(PseudoClass::Hover)` はその祖先がホバーされている間だけスタイルを適用します。`Style::peer` は前にある兄弟に印を付け、`Condition::Peer(PseudoClass::Checked)` はその兄弟がチェックされている間だけスタイルを適用します。どちらも Tailwind CSS や Panda CSS の `group` と `peer` と同じように動きます。",
            },
        ),
        show(Snippet::GroupPeer),
        demo::show(site, Demo::Hover),
    ])
}

fn tokens(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Tokens).leads()),
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
                en: "[`Token::new(name, default)`](api:hypersig_style/struct.Token.html) creates a token of a value type: `Color`, `Length`, `Percentage`, `Time`, or `Angle`. `var()` refers to the token as `var(--name, default)`, so a page that sets no value uses the default. The result of `var()` can be passed anywhere a value of that type is accepted.",
                ja: "[`Token::new(name, default)`](api:hypersig_style/struct.Token.html) は、`Color`、`Length`、`Percentage`、`Time`、`Angle` のどれかの型のトークンを作ります。`var()` はトークンを `var(--name, default)` の形で参照するので、値を何も設定しないページでは既定値が使われます。`var()` の結果はその型の値を受け取る場所ならどこにでも渡せます。",
            },
        ),
        show(Snippet::Tokens),
        bullets(
            site,
            &[
                L {
                    en: "`Page::theme` takes a [`Theme`](api:hypersig_style/struct.Theme.html), whose values are written on `:root`. `Theme::on(Media::Dark, ..)` sets values under a media condition, and a later `theme` call adds to the earlier one.",
                    ja: "`Page::theme` は [`Theme`](api:hypersig_style/struct.Theme.html) を受け取り、その値を `:root` に書きます。`Theme::on(Media::Dark, ..)` はメディアの条件の下で値を設定します。`theme` を後からもう一度呼ぶと、前の呼び出しの値に追加されます。",
                },
                L {
                    en: "`Style::token` sets a token on one element and those inside it, in place of the default and the page's theme.",
                    ja: "`Style::token` は 1 つの要素とその内側の要素にトークンを設定し、既定値とページのテーマの値を置き換えます。",
                },
                L {
                    en: "`El::bind_token` keeps a token's value equal to the value of a string expression over the signals. As with `El::bind_style`, the value is not checked.",
                    ja: "`El::bind_token` は、トークンの値をシグナルから作る文字列の式の値と常に同じにします。`El::bind_style` と同じく、値は検査しません。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The swatches and the button below read the same token. `bind_token` sets that token's value from the radio group's signal.",
                ja: "下の色見本とボタンは同じトークンを読みます。このトークンの値は、`bind_token` がラジオグループのシグナルから設定します。",
            },
        ),
        demo::show(site, Demo::Accent),
        note(
            site,
            L {
                en: "Two tokens with the same name are one custom property to the browser. Duplicate names are not checked. The components of `hypersig_ui` read the tokens in the same crate's [`token`](api:hypersig_ui/token/index.html) module, which [Theming](components/theming/) covers.",
                ja: "同じ名前の 2 つのトークンは、ブラウザにとっては 1 つのカスタムプロパティです。名前が重複していないかは検査されません。`hypersig_ui` のコンポーネントは同じクレートの [`token`](api:hypersig_ui/token/index.html) モジュールにあるトークンを読みます。このモジュールは[テーマ](components/theming/)で説明しています。",
            },
        ),
    ])
}

fn global(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(site, Doc::Style(StylePage::Global).leads()),
        heading(
            site,
            L {
                en: "Rules by tag",
                ja: "タグごとのルール",
            },
        ),
        para(
            site,
            L {
                en: "[`Global::rule`](api:hypersig_style/struct.Global.html#method.rule) adds a style to every element a selector matches. The selector can be an `Element`, which selects by tag, `Selector::Root`, or `Selector::Everything`. The rules are in the base cascade layer, before the classes of `El::css`, so an element's own style overrides them. Conditions in a rule's style apply to the selector as they do to a class.",
                ja: "[`Global::rule`](api:hypersig_style/struct.Global.html#method.rule) はセレクタに当てはまるすべての要素にスタイルを追加します。セレクタにはタグで選ぶ `Element`、`Selector::Root`、`Selector::Everything` を使えます。これらのルールは `El::css` のクラスより前の base のカスケードレイヤーに入るので、要素自身のスタイルがこれらのルールを上書きします。ルールのスタイルにある条件は、クラスの場合と同じようにセレクタに対して適用されます。",
            },
        ),
        show(Snippet::GlobalRules),
        bullets(
            site,
            &[
                L {
                    en: "A `Global` made with `Global::preflight()` starts with a small reset. The reset sizes every element with `border-box`, removes default margins, makes form controls use the page's font, and shrinks images and videos to fit their box.",
                    ja: "`Global::preflight()` で作った `Global` には、最初から小さなリセットが入っています。このリセットは、すべての要素の大きさを `border-box` で測り、既定のマージンを消し、フォームのコントロールにページのフォントを使わせ、画像や動画を枠に収まるまで縮めます。",
                },
                L {
                    en: "`FontFace::new(family, url(..))` loads a font as `@font-face`, with `weights` for a variable font and `italic` for an italic face. `family()` returns the value `font-family` takes.",
                    ja: "`FontFace::new(family, url(..))` はフォントを `@font-face` として読み込みます。可変フォントには `weights` を、イタリック体には `italic` を使います。`family()` は `font-family` に渡す値を返します。",
                },
                L {
                    en: "`Page::global` adds the rules to the page. A later call adds to the earlier rules. `hypersig_ui::global()` is a `Global` built this way.",
                    ja: "`Page::global` はルールをページに追加します。後からもう一度呼ぶと、前のルールに追加されます。`hypersig_ui::global()` は、このようにして組み立てた `Global` です。",
                },
            ],
        ),
    ])
}

/// The styling benchmark of `bench/style`, with the numbers of its
/// `results/latest.md`, for the landing page of the Style docs.
pub fn benchmarks(site: Site) -> [Part; 4] {
    [
        para(
            site,
            L {
                en: "These figures come from the 2000-item TodoMVC page of [hypersig's framework benchmark](https://github.com/imaimai17468/hypersig/tree/main/bench/compare), styled six ways and rendered by hypersig with the same markup, so only the class names and the CSS differ. They were measured in Chrome 155 on an Apple M4 Max with the CPU slowed 4x. Browser values are medians of 5 runs, and the server render is the median of 50.",
                ja: "[hypersig のフレームワークのベンチマーク](https://github.com/imaimai17468/hypersig/tree/main/bench/compare) にある 2000 件の TodoMVC のページを、6 通りの方法でスタイルを付け、同じマークアップのまま hypersig で描画したものです。違うのはクラス名と CSS だけです。Apple M4 Max の Chrome 155 で、CPU を 4 倍に減速して測りました。ブラウザの値は 5 回、サーバーでの描画時間は 50 回の中央値です。",
            },
        ),
        table_of(
            site,
            L {
                en: &[
                    "",
                    "hypersig",
                    "Tailwind CSS",
                    "Panda CSS",
                    "StyleX",
                    "vanilla-extract",
                    "Hand-written CSS",
                ],
                ja: &[
                    "",
                    "hypersig",
                    "Tailwind CSS",
                    "Panda CSS",
                    "StyleX",
                    "vanilla-extract",
                    "手書きの CSS",
                ],
            },
            Text::Chosen::<[&[&str; 7]]>(&[
                &[
                    site.t(L {
                        en: "CSS (gzip)",
                        ja: "CSS（gzip）",
                    }),
                    "1.0 KB",
                    "1.6 KB",
                    "1.0 KB",
                    "1.0 KB",
                    "0.6 KB",
                    "0.8 KB",
                ],
                &[
                    site.t(L {
                        en: "HTML without CSS (gzip)",
                        ja: "CSS を除いた HTML（gzip）",
                    }),
                    "30.2 KB",
                    "29.1 KB",
                    "26.9 KB",
                    "26.8 KB",
                    "27.6 KB",
                    "27.3 KB",
                ],
                &[
                    site.t(L {
                        en: "Sent in all (gzip)",
                        ja: "送る合計（gzip）",
                    }),
                    "31.5 KB",
                    "30.7 KB",
                    "27.8 KB",
                    "27.8 KB",
                    "28.2 KB",
                    "28.1 KB",
                ],
                &[
                    site.t(L {
                        en: "Build of the CSS",
                        ja: "CSS のビルド",
                    }),
                    site.t(L {
                        en: "None",
                        ja: "なし",
                    }),
                    "268 ms",
                    "574 ms",
                    "182 ms",
                    "74 ms",
                    "3.1 ms",
                ],
                &[
                    site.t(L {
                        en: "Server render",
                        ja: "サーバーでの描画時間",
                    }),
                    "1.7 ms",
                    "1.5 ms",
                    "1.4 ms",
                    "1.4 ms",
                    "0.8 ms",
                    "0.8 ms",
                ],
                &[
                    "First Contentful Paint",
                    "136 ms",
                    "140 ms",
                    "136 ms",
                    "144 ms",
                    "140 ms",
                    "140 ms",
                ],
                &[
                    site.t(L {
                        en: "Style and layout on load",
                        ja: "読み込み時のスタイル計算とレイアウト",
                    }),
                    "76 ms",
                    "74 ms",
                    "73 ms",
                    "81 ms",
                    "67 ms",
                    "68 ms",
                ],
                &[
                    site.t(L {
                        en: "Marking 2000 items done",
                        ja: "2000 件を完了にする時間",
                    }),
                    "27 ms",
                    "37 ms",
                    "26 ms",
                    "27 ms",
                    "24 ms",
                    "24 ms",
                ],
            ]),
        ),
        table_of(
            site,
            L {
                en: &["Where hypersig falls behind", "Why"],
                ja: &["hypersig が不利な点", "理由"],
            },
            L {
                en: &[
                    &[
                        "HTML size",
                        "Each class name is made from its declaration alone: the conditions, the property's full name, and the value, as in `font-size_24px`. A value with a space or another character outside letters, digits, `.`, `#`, `%`, and `-` becomes a 16-digit hash. A server patch renders new elements without knowing the page's classes, so one declaration has to get the same name every time it is rendered",
                    ],
                    &[
                        "Server render",
                        "`Page::render` writes those names into every item. The HTML before gzip is 1.21 MB for hypersig and 0.88 MB for Panda CSS, and the render times grow in the same order",
                    ],
                ],
                ja: &[
                    &[
                        "HTML の大きさ",
                        "クラス名を宣言だけから作るので長くなります。名前は条件、プロパティの正式な名前、値をつないだ `font-size_24px` のような形で、空白など英数字と `.`、`#`、`%`、`-` 以外の文字を含む値は 16 桁のハッシュになります。サーバーのパッチはページにあるクラスを知らずに要素を描画するので、同じ宣言はいつ描画しても同じ名前になる必要があります",
                    ],
                    &[
                        "サーバーでの描画時間",
                        "`Page::render` がその長い名前を項目ごとに書き出します。gzip 前の HTML は hypersig が 1.21 MB、Panda CSS が 0.88 MB で、描画時間も同じ順に長くなります",
                    ],
                ],
            },
        ),
        para(
            site,
            L {
                en: "The pages and the harness are in [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style).",
                ja: "ページと計測の仕組みは [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style) にあります。",
            },
        ),
    ]
}

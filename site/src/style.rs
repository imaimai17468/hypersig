//! The Style section: typed atomic CSS from `hypersig::style`.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

/// The article of `page`.
pub fn content(site: Site, page: StylePage) -> Part {
    let head = title(Doc::Style(page).title(site));
    match page {
        StylePage::Overview => overview(site, head),
        StylePage::Values => values(site, head),
        StylePage::Conditions => conditions(site, head),
        StylePage::Tokens => tokens(site, head),
        StylePage::Global => global(site, head),
        StylePage::Patterns => patterns(site, head),
        StylePage::Animation => animation(site, head),
        StylePage::Unchecked => unchecked(site, head),
    }
}

fn overview(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "`hypersig::style` is typed atomic CSS. A misspelled property or keyword, or a value of the wrong type, is a compile error, and each declaration becomes one class.",
                ja: "`hypersig::style` は型の付いた atomic CSS です。プロパティやキーワードの綴り違いと型の違う値はコンパイルエラーになり、宣言 1 つがクラス 1 つになります。",
            },
        ),
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
                en: "A later call for the same property under the same conditions replaces the earlier value, as it would later in a rule. `merge` adds the declarations of another style, which win over the first one's.",
                ja: "同じ条件で同じプロパティをもう一度呼ぶと、ルールの後ろに書いた宣言と同じく、後の値が前の値を置き換えます。`merge` は別のスタイルの宣言を足し、足した側の宣言が元の宣言に勝ちます。",
            },
        ),
        heading(
            site,
            L {
                en: "One class per declaration",
                ja: "宣言 1 つにクラス 1 つ",
            },
        ),
        para(
            site,
            L {
                en: "Each declaration becomes one class named after it, such as `cursor_pointer` or `hover:color_crimson`, as in the atomic CSS of Panda CSS and Tailwind CSS. A value with characters other than letters, digits, `.`, `#`, `%`, and `-` is named by a hash of it. Two elements with the same declaration share the class, so the stylesheet grows with the number of distinct declarations, not with the number of elements.",
                ja: "宣言はそれぞれ、`cursor_pointer` や `hover:color_crimson` のように宣言から名前の決まるクラス 1 つになります。Panda CSS や Tailwind CSS の atomic CSS と同じ作りです。英字、数字、`.`、`#`、`%`、`-` 以外の文字を含む値には、値のハッシュから名前を付けます。同じ宣言を持つ 2 つの要素は同じクラスを使うので、スタイルシートの大きさは要素の数ではなく、異なる宣言の数に比例します。",
            },
        ),
        para(
            site,
            L {
                en: "The page collects the styles its elements use and writes each rule once into `<head>`. A patch that brings new elements brings their rules, placed so that a breakpoint still overrides the base value.",
                ja: "ページは要素が使うスタイルを集め、ルールを 1 回ずつ `<head>` に書きます。新しい要素を運ぶパッチはそのルールも運び、ブレークポイントの値が引き続き基本の値に勝つ位置に置きます。",
            },
        ),
        heading(
            site,
            L {
                en: "Styles that follow a signal",
                ja: "シグナルに従うスタイル",
            },
        ),
        para(
            site,
            L {
                en: "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal, the variants of a Panda CSS recipe. Their rules sit in a later cascade layer than those of `css`, so while switched on they override the element's own declarations for the same property.",
                ja: "`css_when(style, cond)` はシグナルの条件が成り立つ間だけスタイルを足し、`css_match(value, cases)` はシグナルの値ごとにスタイルを 1 つ選びます（Panda CSS の recipe の variants に当たります）。これらのルールは `css` のルールより後ろのカスケードレイヤーに入るので、有効な間は、同じプロパティについて要素自身の宣言に勝ちます。",
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
                en: "The layers come in that order, so an element's classes override the global rules, and a switched-on style overrides both. A stylesheet added with `Page::style` sits outside any layer and overrides all four.",
                ja: "レイヤーはこの順に並ぶので、要素のクラスはグローバルなルールに勝ち、有効になったスタイルはその両方に勝ちます。`Page::style` で足したスタイルシートはどのレイヤーにも入らず、4 つすべてに勝ちます。",
            },
        ),
    ])
}

fn values(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "Each property method takes its own value type, which lists the property's keywords and accepts the units it allows.",
                ja: "プロパティのメソッドはそれぞれ専用の値の型を受け取ります。その型はプロパティのキーワードを並べ、プロパティが認める単位を受け付けます。",
            },
        ),
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
                        "`quoted(\"→\")` escapes the text, and `url(\"/a.png\")` writes `url(\"...\")`",
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
                        "`quoted(\"→\")` はテキストをエスケープし、`url(\"/a.png\")` は `url(\"...\")` を書きます",
                    ],
                    &[
                        "どのプロパティにも",
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
                en: "Ranges are not checked, so a negative width compiles and the browser drops it. A value with several parts, such as `margin: 0 auto`, goes through `raw`, which [Unchecked values](style/unchecked/) covers.",
                ja: "値の範囲は検査しないので、負の幅もコンパイルでき、ブラウザがその宣言を捨てます。`margin: 0 auto` のように複数の部分からなる値は `raw` を通します。`raw` は[検査しない値](style/unchecked/)で説明しています。",
            },
        ),
    ])
}

fn conditions(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "`Style::on` applies part of a style under a condition: a pseudo-class, a media feature, a container width, or the state of a marked ancestor or sibling.",
                ja: "`Style::on` は、スタイルの一部を条件の下でだけ適用します。条件には、擬似クラス、メディア特性、コンテナの幅、印を付けた祖先や兄弟の状態を使えます。",
            },
        ),
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
                    en: "[`PseudoClass`](api:hypersig_style/enum.PseudoClass.html) and [`PseudoElement`](api:hypersig_style/enum.PseudoElement.html) are generated from MDN's data. `pseudo` puts declarations on a pseudo-element, and a pseudo-class inside it applies to the element, so `pseudo(Before, Style::new().on(Hover, ..))` is written `:hover::before`.",
                    ja: "[`PseudoClass`](api:hypersig_style/enum.PseudoClass.html) と [`PseudoElement`](api:hypersig_style/enum.PseudoElement.html) は MDN のデータから生成しています。`pseudo` は擬似要素に宣言を置きます。その中の擬似クラスは要素の側に掛かるので、`pseudo(Before, Style::new().on(Hover, ..))` は `:hover::before` と書かれます。",
                },
                L {
                    en: "The user-action pseudo-classes are written in a fixed order, so `:active` wins over `:hover` while a button is pressed, and `:disabled` wins over all of them.",
                    ja: "ユーザー操作の擬似クラスは決まった順に書かれるので、ボタンを押している間は `:active` が `:hover` に勝ち、`:disabled` はそのどれにも勝ちます。",
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
                    en: "Conditions nest: a style passed to `on` may use `on` itself, and its declarations then apply under both.",
                    ja: "条件は入れ子にできます。`on` に渡すスタイルの中でも `on` を使え、その宣言は両方の条件の下で適用されます。",
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
                en: "`Style::group` marks an ancestor, and `Condition::Group(PseudoClass::Hover)` applies a style while that ancestor is hovered. `Style::peer` marks an earlier sibling, and `Condition::Peer(PseudoClass::Checked)` applies a style while that sibling is checked. They work like the `group` and `peer` of Tailwind CSS and Panda CSS.",
                ja: "`Style::group` は祖先に印を付け、`Condition::Group(PseudoClass::Hover)` はその祖先にポインターが乗っている間だけスタイルを適用します。`Style::peer` は前にある兄弟に印を付け、`Condition::Peer(PseudoClass::Checked)` はその兄弟がチェックされている間だけスタイルを適用します。どちらも Tailwind CSS や Panda CSS の `group` と `peer` と同じように動きます。",
            },
        ),
        show(Snippet::GroupPeer),
        demo::show(site, Demo::Hover),
    ])
}

fn tokens(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "A token is a named value written as a CSS custom property, which a theme sets for the page and a style sets for one element.",
                ja: "トークンは、CSS のカスタムプロパティとして書かれる名前付きの値です。テーマはページ全体に、スタイルは 1 つの要素にトークンの値を設定します。",
            },
        ),
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
                en: "[`Token::new(name, default)`](api:hypersig_style/struct.Token.html) creates a token of a value type: `Color`, `Length`, `Percentage`, `Time`, or `Angle`. `var()` reads it as `var(--name, default)`, so a page that sets nothing shows the default, and it is accepted wherever a value of that type is.",
                ja: "[`Token::new(name, default)`](api:hypersig_style/struct.Token.html) は、`Color`、`Length`、`Percentage`、`Time`、`Angle` のどれかの型のトークンを作ります。`var()` はトークンを `var(--name, default)` として読むので、何も設定しないページには既定値が表示されます。`var()` の結果は、その型の値を受け取る場所ならどこにでも渡せます。",
            },
        ),
        show(Snippet::Tokens),
        bullets(
            site,
            &[
                L {
                    en: "`Page::theme` takes a [`Theme`](api:hypersig_style/struct.Theme.html), whose values are written on `:root`. `Theme::on(Media::Dark, ..)` sets values under a media condition, and a later `theme` call adds to the earlier one.",
                    ja: "`Page::theme` は [`Theme`](api:hypersig_style/struct.Theme.html) を受け取り、その値を `:root` に書きます。`Theme::on(Media::Dark, ..)` はメディアの条件の下で値を設定します。`theme` を後からもう一度呼ぶと、前の呼び出しの値に足されます。",
                },
                L {
                    en: "`Style::token` sets a token on one element and those inside it, in place of the default and the page's theme.",
                    ja: "`Style::token` は 1 つの要素とその内側の要素にトークンを設定し、既定値とページのテーマの値を置き換えます。",
                },
                L {
                    en: "`El::bind_token` keeps a token equal to a string expression over the signals. The value is not checked, as for `El::bind_style`.",
                    ja: "`El::bind_token` は、シグナルから作る文字列の式にトークンの値を合わせ続けます。`El::bind_style` と同じく、値は検査しません。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The swatches and the button below read one token, which `bind_token` sets from the radio group's signal.",
                ja: "下の色見本とボタンは同じトークンを読み、そのトークンは `bind_token` がラジオグループのシグナルから設定します。",
            },
        ),
        demo::show(site, Demo::Accent),
        note(
            site,
            L {
                en: "Two tokens with the same name are one custom property to the browser, and nothing checks that each name is used once. The components of `hypersig_ui` read the tokens in its [`token`](api:hypersig_ui/token/index.html) module, which [Theming](components/theming/) covers.",
                ja: "同じ名前の 2 つのトークンは、ブラウザにとっては 1 つのカスタムプロパティです。名前が 1 回ずつしか使われていないかは、どこも検査しません。`hypersig_ui` のコンポーネントは、その [`token`](api:hypersig_ui/token/index.html) モジュールにあるトークンを読みます。このモジュールは[テーマ](components/theming/)で説明しています。",
            },
        ),
    ])
}

fn global(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "A `Global` holds rules for elements by their tag, a small reset, and the fonts the page loads.",
                ja: "`Global` には、タグごとの要素へのルール、小さなリセット、ページが読み込むフォントが入ります。",
            },
        ),
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
                en: "[`Global::rule`](api:hypersig_style/struct.Global.html#method.rule) adds a style for every element a selector matches: an `Element` by its tag, `Selector::Root`, or `Selector::Everything`. The rules sit in the base cascade layer, before the classes of `El::css`, so an element's own style overrides them. Conditions in a rule's style apply to the selector as they do to a class.",
                ja: "[`Global::rule`](api:hypersig_style/struct.Global.html#method.rule) は、セレクタに当てはまるすべての要素にスタイルを足します。セレクタには、タグで選ぶ `Element`、`Selector::Root`、`Selector::Everything` を使えます。これらのルールは `El::css` のクラスより前の base のカスケードレイヤーに入るので、要素自身のスタイルがルールに勝ちます。ルールのスタイルにある条件は、クラスの場合と同じようにセレクタに掛かります。",
            },
        ),
        show(Snippet::GlobalRules),
        bullets(
            site,
            &[
                L {
                    en: "`Global::preflight()` starts from a small reset: `border-box` sizing everywhere, no default margins, form controls in the page's font, and media that shrinks to its box.",
                    ja: "`Global::preflight()` は小さなリセットから始めます。すべての要素の大きさを `border-box` で測り、既定のマージンを消し、フォームのコントロールにページのフォントを使わせ、画像や動画を枠に収まるまで縮めます。",
                },
                L {
                    en: "`FontFace::new(family, url(..))` loads a font as `@font-face`, with `weights` for a variable font and `italic` for an italic face. `family()` returns the value `font-family` takes.",
                    ja: "`FontFace::new(family, url(..))` はフォントを `@font-face` として読み込みます。可変フォントには `weights` を、イタリック体には `italic` を使います。`family()` は `font-family` に渡す値を返します。",
                },
                L {
                    en: "`Page::global` adds the rules, and a later call adds to the earlier ones. `hypersig_ui::global()` is a `Global` built this way.",
                    ja: "`Page::global` はルールを足し、後から呼ぶと前のルールに足されます。`hypersig_ui::global()` は、このようにして組み立てた `Global` です。",
                },
            ],
        ),
    ])
}

fn patterns(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "[`hypersig::style::pattern`](api:hypersig_style/pattern/index.html) has functions that return the styles of common layouts, which you merge with your own.",
                ja: "[`hypersig::style::pattern`](api:hypersig_style/pattern/index.html) には、よく使うレイアウトのスタイルを返す関数があります。返ったスタイルは自分のスタイルと `merge` して使います。",
            },
        ),
        show(Snippet::Patterns),
        table_of(
            site,
            L {
                en: &["Pattern", "Lays out"],
                ja: &["パターン", "並べ方"],
            },
            L {
                en: &[
                    &[
                        "`stack(gap)`, `vstack(gap)`",
                        "Children in a column, the second centered across it",
                    ],
                    &["`hstack(gap)`", "Children in a row, centered on its line"],
                    &["`wrap(gap)`", "Children in rows that wrap onto new lines"],
                    &[
                        "`grid(columns, gap)`, `grid_item(columns)`",
                        "A grid of equal columns, and a child spanning several",
                    ],
                    &[
                        "`container(max, padding)`",
                        "A block no wider than `max`, centered, with padding on both sides",
                    ],
                    &[
                        "`center()`, `spacer()`",
                        "Children centered on both axes, and an empty flex child that takes the free space",
                    ],
                    &[
                        "`square(size)`, `circle(size)`, `aspect_ratio(ratio)`",
                        "Boxes of a fixed shape",
                    ],
                    &["`divider(width, color)`", "A line across the box"],
                    &[
                        "`visually_hidden()`",
                        "Hidden from sight but read by screen readers",
                    ],
                    &[
                        "`link_overlay()`",
                        "A link that covers its positioned ancestor, so a whole card is clickable",
                    ],
                ],
                ja: &[
                    &[
                        "`stack(gap)`、`vstack(gap)`",
                        "子を縦 1 列に並べ、`vstack` は横方向の中央に揃えます",
                    ],
                    &[
                        "`hstack(gap)`",
                        "子を横 1 行に並べ、行の中で縦の中央に揃えます",
                    ],
                    &[
                        "`wrap(gap)`",
                        "子を横に並べ、収まらない分を次の行に折り返します",
                    ],
                    &[
                        "`grid(columns, gap)`、`grid_item(columns)`",
                        "等しい幅の列のグリッドと、複数の列にまたがる子",
                    ],
                    &[
                        "`container(max, padding)`",
                        "幅が `max` を超えない中央寄せのブロックで、左右に余白があります",
                    ],
                    &[
                        "`center()`、`spacer()`",
                        "子を縦横の中央に置くボックスと、空いた場所を埋める空の flex の子",
                    ],
                    &[
                        "`square(size)`、`circle(size)`、`aspect_ratio(ratio)`",
                        "形の決まったボックス",
                    ],
                    &["`divider(width, color)`", "ボックスを横切る線"],
                    &[
                        "`visually_hidden()`",
                        "画面には見えず、スクリーンリーダーには読まれる要素",
                    ],
                    &[
                        "`link_overlay()`",
                        "位置指定された祖先を覆うリンクで、カード全体をクリックできるようにします",
                    ],
                ],
            },
        ),
        demo::show(site, Demo::Layout),
    ])
}

fn animation(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "Keyframes are Rust values too, and an element that uses them carries their rule to the page.",
                ja: "キーフレームも Rust の値です。キーフレームを使う要素は、そのルールをページまで運びます。",
            },
        ),
        heading(
            site,
            L {
                en: "Keyframes",
                ja: "キーフレーム",
            },
        ),
        para(
            site,
            L {
                en: "[`Keyframes::new(name)`](api:hypersig_style/struct.Keyframes.html) starts an animation, and `at(percent, style)` adds a step. `Style::keyframes` sets an element's `animation-name` and carries the `@keyframes` rule with it, so the page or the patch that brings the element also brings the rule. A keyframe writes only the unconditioned declarations of its style, because a keyframe takes no selector or media query.",
                ja: "[`Keyframes::new(name)`](api:hypersig_style/struct.Keyframes.html) でアニメーションを作り始め、`at(percent, style)` で段階を 1 つ足します。`Style::keyframes` は要素の `animation-name` を設定し、`@keyframes` のルールも一緒に運びます。そのため、要素を運ぶページやパッチがルールも運びます。キーフレームにはセレクタもメディアクエリも書けないので、キーフレームに書かれるのはスタイルのうち条件の付かない宣言だけです。",
            },
        ),
        show(Snippet::Animation),
        demo::show(site, Demo::Spinner),
        heading(
            site,
            L {
                en: "Motion and the visitor",
                ja: "動きと訪問者",
            },
        ),
        para(
            site,
            L {
                en: "`Media::ReducedMotion` applies a style for a visitor who asks for reduced motion. The components of `hypersig_ui` make their color transitions instant there.",
                ja: "`Media::ReducedMotion` は、動きを減らすよう求める訪問者にだけスタイルを適用します。`hypersig_ui` のコンポーネントは、その場合に色のトランジションを一瞬で終わらせます。",
            },
        ),
        note(
            site,
            L {
                en: "Two `Keyframes` with the same name are one rule to the browser, and the later one in the stylesheet wins. Nothing checks that each name is used once.",
                ja: "同じ名前の 2 つの `Keyframes` は、ブラウザにとっては 1 つのルールで、スタイルシートで後にある方が勝ちます。名前が 1 回ずつしか使われていないかは、どこも検査しません。",
            },
        ),
    ])
}

fn unchecked(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            L {
                en: "A few forms go to the browser without a check. Each is named, so a search for it finds every place that relies on one.",
                ja: "いくつかの書き方は、検査を通らずにブラウザへ渡ります。どれにも名前があるので、その名前で検索すれば、検査なしの値に頼る箇所をすべて見つけられます。",
            },
        ),
        show(Snippet::Unchecked),
        bullets(
            site,
            &[
                L {
                    en: "`raw(css)` writes a value as given, for the forms a property's generated type leaves out, such as `margin: 0 auto`, a shadow, or a `calc()`. A value the browser rejects drops the declaration. `raw` panics on `;`, `{`, `}`, and `<`, which would end the declaration or the rule, or close the `<style>` element.",
                    ja: "`raw(css)` は値を渡されたとおりに書きます。`margin: 0 auto`、影、`calc()` のように、プロパティの生成された型に含まれない書き方に使います。ブラウザが受け付けない値なら、その宣言は捨てられます。`;`、`{`、`}`、`<` は宣言やルールを終わらせたり `<style>` 要素を閉じたりするので、`raw` はこれらを受け取るとパニックします。",
                },
                L {
                    en: "`Stylesheet::new(css)` wraps CSS you already have, and `Page::style` adds it to the page. `Stylesheet::class(\"name\")` checks at compile time that the CSS has a selector for the class, and `Class::new` names a class without that check.",
                    ja: "`Stylesheet::new(css)` は手元にある CSS を包み、`Page::style` がそれをページに足します。`Stylesheet::class(\"name\")` は、その CSS にクラスのセレクタがあるかをコンパイル時に確かめます。`Class::new` はその確認なしにクラスを名指しします。",
                },
                L {
                    en: "`El::bind_style` and `El::bind_token` set a property from a string expression over the signals, which nothing checks.",
                    ja: "`El::bind_style` と `El::bind_token` は、シグナルから作る文字列の式でプロパティを設定します。この式はどこも検査しません。",
                },
            ],
        ),
    ])
}

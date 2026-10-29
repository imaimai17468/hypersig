//! The page on the `token` module: every token, its values, the contrast
//! choices, and setting a token for a page or for one element.

use crate::Site;
use crate::code::{self, Lang};
use crate::demo::{self, Demo};
use crate::doc::{Component, Doc};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts, table_of};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::Components(Component::Tokens).leads()),
        demo::show(site, Demo::Theme),
        token_type(site),
        list(site),
        contrast(site),
        setting(site),
        unchecked(site),
        next(site),
    ])
}

fn token_type(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A token",
                ja: "トークン",
            },
        ),
        para(
            site,
            L {
                en: "Each function of the [`token`](api:hypersig_ui/token/index.html) module returns a [`Token`](api:hypersig/style/struct.Token.html): a CSS custom property with a name and a default of a type. The colors are `Token<Color>` and the radius is `Token<Length>`:",
                ja: "[`token`](api:hypersig_ui/token/index.html) モジュールの関数はどれも [`Token`](api:hypersig/style/struct.Token.html) を返します。`Token` は名前と型の付いた既定値を持つ CSS のカスタムプロパティです。色は `Token<Color>`、角の丸みは `Token<Length>` です。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "pub fn primary() -> Token<Color>\npub fn radius() -> Token<Length>\n\n\
             impl<T> Token<T> {\n    pub fn var(&self) -> T\n}",
        ),
        para(
            site,
            L {
                en: "`var()` returns a value of the token's own type, so a `Token<Color>` goes wherever a color goes and a `Token<Length>` wherever a length goes. It writes `var()` with the light value as the fallback, so a page that sets no token still shows the light theme:",
                ja: "`var()` はトークンと同じ型の値を返すので、`Token<Color>` は色を受け取る場所に、`Token<Length>` は長さを受け取る場所に渡せます。`var()` はライトの値をフォールバックにした `var()` を書き出すので、トークンを設定しないページにもライトのテーマが表示されます。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            "background-color: var(--muted, #f4f4f5);\nborder-radius: var(--radius, 0.625rem);",
        ),
    ])
}

/// One row of the token table: the function, the light value, the dark
/// value, and what reads it.
type Row = [&'static str; 4];

const ROWS_EN: [Row; 25] = [
    [
        "`background`",
        "`#ffffff`",
        "`#09090b`",
        "The page background",
    ],
    [
        "`foreground`",
        "`#09090b`",
        "`#fafafa`",
        "Text on `background`",
    ],
    ["`card`", "`#ffffff`", "`#18181b`", "The surface of a card"],
    [
        "`card_foreground`",
        "`#09090b`",
        "`#fafafa`",
        "Text on `card`",
    ],
    [
        "`popover`",
        "`#ffffff`",
        "`#18181b`",
        "The surface of a menu, a popover, a listbox, and the open list of a select",
    ],
    [
        "`popover_foreground`",
        "`#09090b`",
        "`#fafafa`",
        "Text on `popover`",
    ],
    [
        "`primary`",
        "`#18181b`",
        "`#e4e4e7`",
        "A default button, a checked box, an on switch, the slider's fill",
    ],
    [
        "`primary_foreground`",
        "`#fafafa`",
        "`#18181b`",
        "Text and marks on `primary`",
    ],
    [
        "`secondary`",
        "`#f4f4f5`",
        "`#27272a`",
        "A secondary button",
    ],
    [
        "`secondary_foreground`",
        "`#18181b`",
        "`#fafafa`",
        "Text on `secondary`",
    ],
    [
        "`muted`",
        "`#f4f4f5`",
        "`#27272a`",
        "A quiet fill, such as the slider's track or a tab list",
    ],
    [
        "`muted_foreground`",
        "`#6b6b74`",
        "`#a1a1aa`",
        "A description, a placeholder, secondary text",
    ],
    [
        "`accent`",
        "`#f4f4f5`",
        "`#3f3f46`",
        "The fill under the pointer, and a toggle that is on",
    ],
    [
        "`accent_foreground`",
        "`#18181b`",
        "`#fafafa`",
        "Text on `accent`",
    ],
    [
        "`destructive`",
        "`#e7000b`",
        "`#ff6467`",
        "A destructive button, an error message, an invalid field's border",
    ],
    [
        "`destructive_foreground`",
        "`#ffffff`",
        "`#09090b`",
        "Text on `destructive`",
    ],
    [
        "`border`",
        "`#e4e4e7`",
        "`#27272a`",
        "The border of a card, a menu, and a separator",
    ],
    [
        "`input`",
        "`#8b8b94`",
        "`#70707a`",
        "The border of a text field, a checkbox, a radio, a toggle, and an off switch",
    ],
    ["`ring`", "`#18181b`", "`#d4d4d8`", "The focus outline"],
    [
        "`chart_1`",
        "`#155dfc`",
        "`#51a2ff`",
        "The first series of a chart",
    ],
    ["`chart_2`", "`#009689`", "`#00d5be`", "The second series"],
    ["`chart_3`", "`#f54900`", "`#ff8904`", "The third series"],
    ["`chart_4`", "`#9810fa`", "`#c27aff`", "The fourth series"],
    ["`chart_5`", "`#bb4d00`", "`#ffd230`", "The fifth series"],
    [
        "`radius`",
        "`0.625rem`",
        "`0.625rem`",
        "The corners of a card, a dialog, a menu, and a button",
    ],
];

const ROWS_JA: [Row; 25] = [
    ["`background`", "`#ffffff`", "`#09090b`", "ページの背景"],
    [
        "`foreground`",
        "`#09090b`",
        "`#fafafa`",
        "`background` の上の文字",
    ],
    ["`card`", "`#ffffff`", "`#18181b`", "カードの面"],
    [
        "`card_foreground`",
        "`#09090b`",
        "`#fafafa`",
        "`card` の上の文字",
    ],
    [
        "`popover`",
        "`#ffffff`",
        "`#18181b`",
        "メニュー、ポップオーバー、リストボックス、セレクトの開いた一覧の面",
    ],
    [
        "`popover_foreground`",
        "`#09090b`",
        "`#fafafa`",
        "`popover` の上の文字",
    ],
    [
        "`primary`",
        "`#18181b`",
        "`#e4e4e7`",
        "既定のボタン、チェックしたボックス、オンのスイッチ、スライダーの塗り",
    ],
    [
        "`primary_foreground`",
        "`#fafafa`",
        "`#18181b`",
        "`primary` の上の文字と印",
    ],
    [
        "`secondary`",
        "`#f4f4f5`",
        "`#27272a`",
        "セカンダリのボタン",
    ],
    [
        "`secondary_foreground`",
        "`#18181b`",
        "`#fafafa`",
        "`secondary` の上の文字",
    ],
    [
        "`muted`",
        "`#f4f4f5`",
        "`#27272a`",
        "スライダーのトラックやタブの一覧のような控えめな塗り",
    ],
    [
        "`muted_foreground`",
        "`#6b6b74`",
        "`#a1a1aa`",
        "説明、プレースホルダー、補足の文字",
    ],
    [
        "`accent`",
        "`#f4f4f5`",
        "`#3f3f46`",
        "ポインターの下の塗りと、オンのトグル",
    ],
    [
        "`accent_foreground`",
        "`#18181b`",
        "`#fafafa`",
        "`accent` の上の文字",
    ],
    [
        "`destructive`",
        "`#e7000b`",
        "`#ff6467`",
        "破壊的な操作のボタン、エラーメッセージ、不正な入力欄の境界線",
    ],
    [
        "`destructive_foreground`",
        "`#ffffff`",
        "`#09090b`",
        "`destructive` の上の文字",
    ],
    [
        "`border`",
        "`#e4e4e7`",
        "`#27272a`",
        "カード、メニュー、区切り線の境界線",
    ],
    [
        "`input`",
        "`#8b8b94`",
        "`#70707a`",
        "テキストフィールド、チェックボックス、ラジオ、トグル、オフのスイッチの境界線",
    ],
    ["`ring`", "`#18181b`", "`#d4d4d8`", "フォーカスの outline"],
    [
        "`chart_1`",
        "`#155dfc`",
        "`#51a2ff`",
        "チャートの 1 つ目の系列",
    ],
    ["`chart_2`", "`#009689`", "`#00d5be`", "2 つ目の系列"],
    ["`chart_3`", "`#f54900`", "`#ff8904`", "3 つ目の系列"],
    ["`chart_4`", "`#9810fa`", "`#c27aff`", "4 つ目の系列"],
    ["`chart_5`", "`#bb4d00`", "`#ffd230`", "5 つ目の系列"],
    [
        "`radius`",
        "`0.625rem`",
        "`0.625rem`",
        "カード、ダイアログ、メニュー、ボタンの角",
    ],
];

fn list(site: Site) -> Part {
    let en: Vec<&[&str]> = ROWS_EN.iter().map(<[&str; 4]>::as_slice).collect();
    let ja: Vec<&[&str]> = ROWS_JA.iter().map(<[&str; 4]>::as_slice).collect();
    parts([
        heading(
            site,
            L {
                en: "Every token",
                ja: "トークンの一覧",
            },
        ),
        para(
            site,
            L {
                en: "The names and the light values follow shadcn/ui's zinc theme. The custom property of each is its function's name with `_` written as `-`, such as `--muted-foreground` for `muted_foreground()`:",
                ja: "名前とライトの値は shadcn/ui の zinc テーマに合わせています。カスタムプロパティの名前は関数名の `_` を `-` にしたもので、たとえば `muted_foreground()` は `--muted-foreground` です。",
            },
        ),
        table_of(
            site,
            L {
                en: &["Function", "Light", "Dark", "What reads it"],
                ja: &["関数", "ライト", "ダーク", "使う場所"],
            },
            L {
                en: en.as_slice(),
                ja: ja.as_slice(),
            },
        ),
        para(
            site,
            L {
                en: "Each name links to its rustdoc entry from the [`token`](api:hypersig_ui/token/index.html) module page, such as [`token::primary`](api:hypersig_ui/token/fn.primary.html) and [`token::radius`](api:hypersig_ui/token/fn.radius.html).",
                ja: "各関数の rustdoc は [`token`](api:hypersig_ui/token/index.html) モジュールのページから開けます。たとえば [`token::primary`](api:hypersig_ui/token/fn.primary.html) や [`token::radius`](api:hypersig_ui/token/fn.radius.html) です。",
            },
        ),
    ])
}

fn contrast(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Where the values differ from shadcn/ui",
                ja: "shadcn/ui と値が異なるところ",
            },
        ),
        para(
            site,
            L {
                en: "Three values differ from shadcn/ui's, where shadcn/ui falls short of WCAG 2.2 AA:",
                ja: "shadcn/ui の値が WCAG 2.2 AA を満たさないところでは、次の 3 つの値を変えています。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "`input` is dark enough for the 3:1 contrast that a control's boundary needs (1.4.11): 3.3:1 or more on `background` and on `card` in both schemes.",
                    ja: "`input` は、コントロールの境界に必要な 3:1 のコントラスト（1.4.11）を満たす濃さです。両方の配色で、`background` と `card` の上で 3.3:1 以上あります。",
                },
                L {
                    en: "`muted_foreground` keeps 4.5:1 on `muted` as well as on `background`: 4.8:1 or more on both in both schemes, so a description inside a muted panel stays readable.",
                    ja: "`muted_foreground` は `background` だけでなく `muted` の上でも 4.5:1 を保ちます。両方の配色で、どちらの上でも 4.8:1 以上あるので、muted のパネルの中の説明も読めます。",
                },
                L {
                    en: "`ring` is drawn as a solid 2px outline rather than a translucent shadow (2.4.7), so forced-colors mode, which drops shadows, still shows focus.",
                    ja: "`ring` は半透明の影ではなく 2px の実線の outline として描きます（2.4.7）。影を描かない forced-colors モードでもフォーカスが見えます。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The five chart colors keep 3:1 or more on `background`, `card`, and `muted` in both schemes, the contrast that a graphical object needs.",
                ja: "チャートの 5 色は両方の配色で `background`、`card`、`muted` の上で 3:1 以上を保ちます。これはグラフィックのオブジェクトに必要なコントラストです。",
            },
        ),
    ])
}

fn setting(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Setting a token",
                ja: "トークンを設定する",
            },
        ),
        para(
            site,
            L {
                en: "[`Theme::set`](api:hypersig/style/struct.Theme.html#method.set) sets a token for the whole page, and [`Style::token`](api:hypersig/style/struct.Style.html#method.token) sets it for one element and those inside it. Both take the token and a value that converts into the token's type:",
                ja: "[`Theme::set`](api:hypersig/style/struct.Theme.html#method.set) はページ全体のトークンを、[`Style::token`](api:hypersig/style/struct.Style.html#method.token) は 1 つの要素とその内側のトークンを設定します。どちらもトークンと、トークンの型に変換できる値を受け取ります。",
            },
        ),
        code::block(
            Lang::Rust,
            None,
            "impl Theme {\n    pub fn set<T: TokenValue>(self, token: &Token<T>, value: impl Into<T>) -> Self\n}\n\
             impl Style {\n    pub fn token<T: TokenValue>(self, token: &Token<T>, value: impl Into<T>) -> Self\n}",
        ),
        show(Snippet::UiTokens),
        para(
            site,
            L {
                en: "The brand theme writes `--primary` and `--radius` on `:root`, and the button's style writes `--primary` on the button alone:",
                ja: "ブランドのテーマは `:root` に `--primary` と `--radius` を書き出し、ボタンのスタイルはそのボタンだけに `--primary` を書き出します。",
            },
        ),
        code::block(
            Lang::Html,
            None,
            ":root { --primary: #2563eb; --radius: 1rem }\n\
             @media (prefers-color-scheme: dark) { :root { --primary: #60a5fa } }\n\
             /* on the button */\n--primary: #16a34a;",
        ),
        para(
            site,
            L {
                en: "The value converts into the token's type, so a color given to `radius()`, a `Token<Length>`, has no conversion and fails to compile with E0277:",
                ja: "値はトークンの型に変換されるので、`Token<Length>` である `radius()` に色を渡すと変換がなく、E0277 でコンパイルできません。",
            },
        ),
        show(Snippet::UiTokenWrongValue),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What the types leave unchecked",
                ja: "型では検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Nothing checks the contrast of a value you set. A `primary` too close to `primary_foreground` leaves a button's text unreadable, so set the pair together.",
                    ja: "設定した値のコントラストは検査しません。`primary` が `primary_foreground` に近すぎるとボタンの文字が読めなくなるので、組になる 2 つは一緒に設定します。",
                },
                L {
                    en: "The page writes a theme's values without a condition before those under `Media::Dark`. A color set only without a condition therefore gives way to the dark value from `theme()` while the system prefers dark, and the dark scheme shows zinc where the light one shows your color. Set the dark value under `Media::Dark` as well, as the example does. For one token under the same condition, the theme added later replaces the earlier value.",
                    ja: "ページは条件のないテーマの値を `Media::Dark` の下の値より前に書き出します。そのため、条件なしでだけ設定した色は、システムがダークを選んでいる間は `theme()` のダークの値に負けます。ライトの配色では独自の色になり、ダークの配色では zinc の色になります。例のように、ダークの値も `Media::Dark` の下で設定します。同じ条件の同じトークンでは、後に追加したテーマの値が前の値を置き換えます。",
                },
                L {
                    en: "A token of your own made with `Token::new` under one of these names, such as `\"primary\"`, is the same custom property to the browser, and setting it changes the components too.",
                    ja: "`Token::new` で `\"primary\"` のような同じ名前のトークンを独自に作ると、ブラウザにとっては同じカスタムプロパティなので、それを設定するとコンポーネントの色も変わります。",
                },
            ],
        ),
    ])
}

fn next(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Next",
                ja: "次に読むページ",
            },
        ),
        para(
            site,
            L {
                en: "[Theming](components/theming/#light-and-dark) switches every token between its light and dark values. [Tokens](style/tokens/) and [Themes](style/themes/) in the Style docs cover tokens of your own.",
                ja: "[テーマ](components/theming/#light-and-dark)ではすべてのトークンをライトとダークの値の間で切り替えます。独自のトークンについては Style のドキュメントの[トークン](style/tokens/)と[テーマ](style/themes/)で説明しています。",
            },
        ),
    ])
}

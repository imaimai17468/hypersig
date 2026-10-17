# hypersig

[English](README.md) | [ドキュメント（英語）](https://imaimai17468.github.io/hypersig/)

axum の上で動き、画面もブラウザでの動きもサーバーの処理も Rust で書く Web フレームワークです。

- `cargo build` だけでビルドでき、Node も wasm も要りません。
- 画面とサーバーの食い違いは、コンパイルエラーになります。ルートに合わないページ、パスのパラメータ、クエリへのリンクも同じです。
- サーバーの応答を待たずに画面を変える処理（楽観的更新）を、追加のコードなしで書けます。

## フレームワークの比較

Rust のサーバーに画面を付けようとすると、今は 2 通りの選び方になります。Leptos なら画面とサーバーの間で型が検査される代わりに、wasm とそのビルドが要ります。htmx ならビルドは要りませんが、属性に書く URL や式は型で検査されません。

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| 画面を書く言語 | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript と Vue のテンプレート | HTML の属性 |
| サーバーを書く言語 | Rust | Rust | 任意 | TypeScript | TypeScript | TypeScript | 任意 |
| ブラウザ向けのビルドが不要 | はい | いいえ（wasm と cargo-leptos） | いいえ（Vite） | いいえ（Node のバンドラー） | いいえ（Vite） | いいえ（Vite） | はい |
| 送る値の型をコンパイル時に検査 | はい | はい | いいえ | はい | はい | いいえ | いいえ |
| リンクの行き先をコンパイル時に検査 | はい | いいえ（`href` は文字列） | いいえ（ルーターなし） | 設定で有効化（`typedRoutes`） | はい | 実験的（`typedPages`） | いいえ |
| ブラウザの中で状態を持てる | はい | はい | はい | はい | はい | はい | いいえ |

## ベンチマーク

追加、チェック、削除をサーバーに保存する 2000 件の TodoMVC を、Apple M4 Max の Chrome 155 で 3 回測った中央値です。CPU を 4 倍に減速し、通信のたびに 50 ms の遅延を足しています。

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| JS（gzip） | 13.4 KB | 140.5 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 16.5 KB |
| First Contentful Paint | 179 ms | 190 ms | 184 ms | 198 ms | 188 ms | 172 ms | 235 ms |
| Total Blocking Time | 0 ms | 27 ms | 2 ms | 5 ms | 0 ms | 25 ms | 233 ms |
| Interaction to Next Paint | 24 ms | 16 ms | 24 ms | 16 ms | 24 ms | 16 ms | 16 ms |

測ったアプリと計測の仕組みは [`bench/compare`](bench/compare) にあります。ブラウザの中だけで行うリストの操作を比べる [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark) の結果は、[`bench/js-framework-benchmark/results.md`](bench/js-framework-benchmark/results.md) にあります。

## 状態管理とレンダリング

ページの状態は 1 つの Rust の構造体（シグナル構造体）で定義します。画面の式もサーバーのハンドラも、値はこの構造体のフィールドを通してしか読み書きできません。そのため、フィールドの名前を変えたのに画面の式を直し忘れた、表示の条件に `bool` ではなく数値を渡した、別のページのアクションを呼んだ、といった間違いは、ブラウザで動かす前にコンパイルエラーになります。

読み込みと更新でブラウザが実行するものを、ほかのフレームワークと比べると次のようになります。表の「エフェクト」は、シグナルの値を読んで DOM を書き換える処理です。hypersig では `data-hs-text` や `data-hs-show` のような表示の属性 1 つにつき 1 つあります。

| | hypersig | React | Vue | Solid | Qwik |
|---|---|---|---|---|---|
| ブラウザに送るアプリのコード | なし（式は HTML の属性にデータとして入る） | コンポーネント全部 | コンポーネント全部 | コンポーネント全部 | 操作されたハンドラの分だけ、操作したときに |
| 読み込み時にブラウザが実行するもの | すべてのエフェクトを 1 回ずつ実行 | コンポーネントのツリー全体 | コンポーネントのツリー全体 | 各コンポーネントを 1 回（リアクティブな依存関係を作る） | なし（状態とハンドラの場所は HTML に入っている） |
| 更新で再実行する範囲 | 値を読んだエフェクトだけ | 状態を持つコンポーネントと子孫 | コンポーネントの描画関数（仮想 DOM で差分を取る） | 値を読んだエフェクトだけ | 値を読んだエフェクトだけ |

アクションに送る値の型は、呼ぶ側の画面のコードでも受け取るハンドラでも `(i32, bool)` です。TanStack Query の楽観的更新で書くクエリキーの指定、キャッシュの前の値の保存、一覧の取り直しは要りません。

負担になるのは失敗したときです。サーバーがエラーを返しても、ブラウザで変えた値はそのまま残ります。`error(message)` で失敗は表示できますが、値を元に戻す仕組みはありません。

## データを取る単位

RSC と Next.js では、データを取得する処理をサーバーコンポーネントの中に `await` で書くので、取得はコンポーネントごとに別々に始まり、結果は Suspense の境界ごとに届いた順で表示されます。そのため境界の置き方しだいで画面が少しずつ埋まり（ポップコーン UI）、親コンポーネントの `await` が終わるまで子コンポーネントの取得が始まらない書き方をすると、取得が直列に並んで表示が遅れます。変更のあとは `revalidatePath` や `router.refresh` で、ルートのサーバーコンポーネントがもう一度描画されます。

Remix と TanStack Start では、ルートごとの loader がページを表示する前に並列に実行されるので、取得が直列に並ぶことも画面が少しずつ埋まることも起きにくくなります。ただし変更のあとにも loader が再実行され、Remix は action のあと表示中のすべてのルートの loader を、TanStack Router は `router.invalidate()` で loader を、TanStack Query は無効にしたクエリを実行し直すので、チェックボックスを 1 つ切り替えただけでもページや一覧のデータを取得し直します。

hypersig では、最初のデータはページの HTML にシグナル構造体の JSON として入っているので、表示のためにブラウザからデータを取得することはありません。変更のあとはアクションのハンドラが変わったシグナルの値だけを `Counter::count().patch(0)` のように返し、ブラウザはその値を使っている表示だけを書き換えるので、チェックボックスを 1 つ切り替えてもページや一覧を取得し直すことはありません。

## クイックスタート

`Cargo.toml` に依存を追加します。hypersig はまだ crates.io に公開していないので、Git から取得します。

```toml
[dependencies]
hypersig = { git = "https://github.com/imaimai17468/hypersig" }
axum = "0.8"
serde = { version = "1", features = ["derive"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }
```

`src/main.rs` を書きます。

```rust,no_run
use axum::Router;
use hypersig::html::{button, div, span};
use hypersig::{Action, Page, Patches, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

hypersig::routes! {
    fn pages(Router<()>);
    const INDEX: Route = "/" => index;
}

const RESET: Action<Counter> = Action::post("/reset");

async fn index((): (), (): ()) -> Page<Counter> {
    let count = Counter::count();
    Page::new("Counter", Counter { count: 0 }).child(
        div()
            .child(button().on_click(count.set(count + 1)).text("+1"))
            .child(span().bind_text(count.to_text()))
            .child(button().on_click(RESET.call()).text("Reset on the server")),
    )
}

async fn reset((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new())
        .hypersig_runtime()
        .action(RESET, reset);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

`cargo run` を実行し、<http://127.0.0.1:3000> を開いてください。`routes!` が `/` のページをハンドラ `index` と一緒に宣言し、生成した関数 `pages` がそのページを配信します。`+1` のボタンはリクエストを送らず、ブラウザの中で数を増やします。リセットのボタンは `/reset` を呼び、ハンドラが返す差分で `count` が 0 に戻ります。ブラウザ用のランタイムは `hypersig_runtime()` が配信するので、Node も JavaScript のビルドも要りません。

## ページとリンク

ページは `Route` で、配信するハンドラと一緒に `routes!` で宣言します。ハンドラのないルートは宣言できません。`El::href` は同じ値からリンクを張ります。パラメータは `{name}` の形のセグメントで、ルートの型がそれぞれの型を決めます。ルートに合わないパラメータを渡すリンクや、`{name}` のセグメントの数が型の持つ値の数と食い違うパスは、コンパイルエラーになります。型に変換できないセグメントを持つリクエストには、ハンドラを呼ぶ前に 404 を返します。

```rust
use hypersig::html::{a, each, li, ul};
use hypersig::{Page, Route, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Member {
    id: i32,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Team {
    members: Vec<Member>,
}

hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<i32> = "/members/{id}" => member;
}

async fn member((): (), id: i32) -> Option<Page<Team>> {
    (id > 0).then(|| Page::new("Member", Team { members: Vec::new() }))
}

let list = ul::<Team>().child(each(Team::members(), |member| {
    li().child(
        a().href(MEMBER.link_with(member.at(Member::id())))
            .bind_text(member.at(Member::name())),
    )
}));
assert!(list.render().contains("/members/"));
```

クエリ文字列の型は `Route<P, Q>` の 2 つ目の型で指定します。`Q` は `#[derive(Signals, Search)]` を付けた構造体です。各フィールドは文字列、数値、`bool`、またはそれらの `Option` で、リンクとハンドラは `(P, Q)` を受け取ります。リンクをたどると、次のページをサーバーから読み込みます。ほかのサイトへのリンクは、これまでどおり `attr("href", ...)` で書きます。`attr` は渡した文字列を検査しません。

## スタイル

スタイルも Rust の値として書きます。`Style` には MDN の CSS データから生成した、標準の CSS プロパティごとのメソッドがあり、宣言 1 つが、その宣言から名前の決まるクラス 1 つになります（Panda CSS や Tailwind CSS と同じ atomic CSS です）。プロパティやキーワードの綴り違いと、`width` に色を渡すような型の違う値は、コンパイルエラーになります。ページは要素が使うルールを `<head>` に書き、新しい要素を運ぶパッチはそのルールも運びます。ルールは、ブレークポイントの値が基本の値に勝つ順番に並びます。

```rust
use hypersig::html::button;
use hypersig::style::value::{Color, Cursor, rem};
use hypersig::style::{Media, PseudoClass, Style, Theme, Token};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Cart {
    busy: bool,
}

let accent = Token::new("accent", Color::hex(0xb83f45));
let page = Page::new("Cart", Cart { busy: false })
    .theme(Theme::new().on(Media::Dark, Theme::new().set(&accent, Color::hex(0xff6b6b))))
    .child(
        button()
            .css(
                Style::new()
                    .padding(rem(0.5))
                    .color(accent.var())
                    .cursor(Cursor::Pointer)
                    .on(PseudoClass::Hover, Style::new().color(Color::CRIMSON))
                    .on(Media::Md, Style::new().padding(rem(1))),
            )
            .css_when(Style::new().opacity(0.5), Cart::busy())
            .text("Buy"),
    );
assert!(page.render().contains("hover:color_crimson"));
```

`css_when` はシグナルでスタイルを切り替え、`css_match` はシグナルの値ごとにスタイルを選びます（Panda CSS の recipe の variants に当たります）。条件には、擬似クラスと擬似要素、メディア特性、コンテナの幅、印を付けた祖先や兄弟の状態（`group` と `peer`）を使えます。`hypersig::style` には、ページのテーマと組み合わせるデザイントークン、`stack` や `grid` のようなレイアウトのパターン、タグごとの要素へのルールと小さなリセット、`@font-face`、`@keyframes` もあります。例の [`style`](examples/style) は、これらを 1 ページに並べたものです。例の [`todomvc`](examples/todomvc) はこれらだけで書いてあり、Chrome で撮ったスクリーンショットは、以前のスタイルシートで撮ったものとバイト単位で一致しました。

`margin: 0 auto`、影、`calc()` のように複数の部分からなる値は `raw(...)` を通し、ブラウザに検査なしで渡ります。負の幅のような、プロパティの範囲を外れた値も検査しません。既存のスタイルシートを読み込むページは、それをそのまま使えます。`Stylesheet::class("name")` は、そのクラスのセレクタがスタイルシートにあるかをコンパイル時に確かめます。

## コンポーネント

`hypersig_ui` クレートは、shadcn/ui の見た目を持ち、振る舞いをブラウザのネイティブ要素と WAI-ARIA Authoring Practices から取るコンポーネント集です。ボタン、テキストフィールド、チェックボックス、スイッチ、ラジオグループ、セレクト、スライダー、リストボックス、コンボボックス、タブ、アコーディオン、ダイアログ、メニューボタン、ツールチップ、トースト、アラート、バッジ、カード、プログレスバーがあります。どれも `El` を返すので、ほかの要素と同じようにハンドラや子要素を足せます。

テキストフィールドのラベル、説明、エラーメッセージは、束縛するシグナルから作った id で入力欄に結び付きます。タブとパネルも同じ方法で結び付きます。ダイアログ、メニュー、ツールチップ、コンボボックスは、部品どうしで共有する `const` のハンドルから名前を取ります。id に使えない名前はコンパイルエラーになります。ダイアログはブラウザが invoker commands を通して `showModal()` で開き、メニューとツールチップは `popover` として top layer に出て、anchor positioning でボタンの横に置かれます。タブ、リストボックスの選択肢、メニューの項目の間は、`El::roving_focus` を使って矢印キー、Home、End で移ります。コンボボックスはフォーカスをテキスト欄に置いたまま、`El::active_descendant` で表示中の選択肢をたどり、たどった選択肢の id をシグナルに書きます。トーストは `aria-live` の領域に並び、時間を指定しない限り閉じるまで残ります。

色は shadcn/ui の zinc テーマに合わせ、WCAG 2.2 AA を満たすために 3 つの値を変えています。コントロールの境界線は 3:1 以上のコントラストを持ち、補足のテキストは muted の塗りの上でも 4.5:1 を保ち、フォーカスは forced-colors モードでも描かれる単色の outline です。例の [`ui`](examples/ui) は全コンポーネントを 1 ページに並べたもので、その E2E テストはキーボードで操作し、ロールとアクセシブルな名前を確かめます。

## 制限事項

hypersig では、状態をサーバーが持ちます。ブラウザだけで完結するのは、hypersig が用意した演算と命令です。`i32` と `f64` の計算、文字列とリストの操作、フォーカスなどがこれに当たります。それ以外の処理はサーバーのアクションを通すので、通信の分だけ反応が遅れます。

次のようなアプリには向きません。

- ブラウザで JavaScript のライブラリを動かすアプリ（地図、グラフ、リッチテキストエディタなど）。これらはブラウザの中で動く必要があり、サーバーでは代わりができません。
- サーバーにつながらない状態で動くアプリ（オフラインで使うものなど）。

## 開発

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p hypersig --bench render
cd runtime && pnpm install && pnpm check && pnpm test && pnpm build
cd e2e && pnpm install && pnpm check && pnpm test   # 6 つの例を起動して Chrome で操作する
```

ブラウザ用のランタイムは `runtime/` にある TypeScript です。`pnpm build` がビルド済みの `crates/hypersig/assets/hypersig.js`（約 41 KB、gzip 後は約 14 KB）を書き出し、クレートはそれを埋め込みます。リポジトリの構成と、変更のたびに通す確認は `AGENTS.md` にあります。Lint の構成とコーディング規約は、Microsoft の [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) に沿っています。

## License

MIT

<img src="https://raw.githubusercontent.com/imaimai17468/hypersig/main/assets/hypersig.svg" alt="" width="96" height="96">

# hypersig

[English](README.md) | [ドキュメント](https://imaimai17468.github.io/hypersig/ja/)

**サーバーが状態を、型が正しさを。**

Rust だけで書く[ハイパーメディア](https://imaimai17468.github.io/hypersig/ja/learn/why/#hypermedia)の Web フレームワークです。画面もブラウザでの動きもサーバーの処理も Rust で書き、axum の上で動きます。状態はサーバーが持ってブラウザにパッチを送り、ブラウザが扱う文字列（シグナルの名前、式、アクションの URL、リンク）はすべて Rust の型から作るので、その書き間違いはコンパイルエラーになります。コンパイラが見つけられない少数の間違いは[コンパイラが検査しないこと](#コンパイラが検査しないこと)にまとめています。

- `cargo build` だけでビルドでき、Node も wasm も要りません。
- 画面とサーバーの食い違いと、ルートに合わないリンクはコンパイルエラーになります（[hypersig を選ぶ理由](https://imaimai17468.github.io/hypersig/ja/learn/why/)）。
- 最初の HTML にページの状態が入っています。アクションは変わった値だけを返し、ブラウザはその値を表示している箇所だけを書き換えます（[サーバーとのやり取り](https://imaimai17468.github.io/hypersig/ja/learn/server/)）。
- ハイドレーションはありません。読み込み時、ランタイムはサーバーが HTML に書き込んだ状態を読むだけで、コンポーネントを実行し直しません（[hypersig を選ぶ理由](https://imaimai17468.github.io/hypersig/ja/learn/why/)）。
- フォームの検証規則は一度書けば、ブラウザとサーバーの両方が同じ規則で検証します（[フォーム](https://imaimai17468.github.io/hypersig/ja/learn/forms/)）。
- CSS のプロパティと値も型で検査します（[スタイル](https://imaimai17468.github.io/hypersig/ja/style/)）。
- shadcn/ui の見た目のコンポーネント集と、訳の漏れをコンパイルエラーにする多言語化のクレートがあります（[コンポーネント](https://imaimai17468.github.io/hypersig/ja/components/)、[多言語化](https://imaimai17468.github.io/hypersig/ja/i18n/)）。

## クイックスタート

`Cargo.toml` に依存を追加します。

```toml
[dependencies]
hypersig = "0.6"
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
    action RESET: Action<Counter> = post "/reset" => reset;
}

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
    let app = pages(Router::new()).hypersig_runtime();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

`cargo run` を実行し、<http://127.0.0.1:3000> を開いてください。`+1` のボタンはリクエストを送らず、ブラウザの中で数を増やします。リセットのボタンは `/reset` を呼び、ハンドラが返す差分で `count` が 0 に戻ります。`count` のつづりを間違えたり、`count` に文字列を足したりすると、`cargo build` が止まります。`cargo install --locked hypersig_cli` でインストールする `cargo hypersig dev` を使うと、変更のたびにサーバーがビルドされて起動し直し、開いているタブは新しいサーバーから読み込み直されます。続きは [ドキュメント](https://imaimai17468.github.io/hypersig/ja/learn/) にあります。

## フレームワークの比較

hypersig に近いのは Topcoat と Datastar です。どちらもブラウザの中にシグナルを持ち、ブラウザ向けのビルドなしにサーバーから画面を更新します。Topcoat はブラウザで動く式も Rust で書きます。Leptos と Dioxus も画面とサーバーを Rust で書きますが、画面を WebAssembly にコンパイルし、ブラウザでサーバーの HTML をハイドレートします。

|  | Topcoat と Datastar | hypersig |
|---|---|---|
| シグナル名と URL（Datastar） | HTML の属性に文字列で書きます。打ち間違いはブラウザが実行するまで分かりません | Rust の値から書きます。打ち間違いがあると `cargo build` が止まります |
| リンクのパラメータ（Topcoat） | `href!` が検査するのはハンドラだけです。パスパラメータが足りないか多いと、ページを描くときにパニックし、リクエストは 500 を返します | `Route` がパラメータを型の付いた値で受け取ります。間違えるとコンパイルエラーになります |
| 一覧への追加（Topcoat） | 式から項目を足せないので、サーバーが答えて描き直したときに一覧が変わります | 一覧をシグナルに持つので、サーバーが答える前にブラウザが変更を表示します |

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar | Leptos | Dioxus |
|---|---|---|---|---|---|---|---|---|---|
| 画面を書く言語 | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript と Vue のテンプレート | HTML の属性 | Rust | Rust |
| サーバーを書く言語 | Rust | Rust | 任意 | TypeScript | TypeScript | TypeScript | 任意 | Rust | Rust |
| ブラウザ向けのビルドが不要 | はい | はい | いいえ（Vite） | いいえ（Node のバンドラー） | いいえ（Vite） | いいえ（Vite） | はい | いいえ（`cargo leptos` で Wasm に） | いいえ（`dx` で Wasm に） |
| 送る値の型をコンパイル時に検査 | はい | はい | いいえ | はい | はい | いいえ | いいえ | はい | はい |
| リンクの行き先をコンパイル時に検査 | はい | ハンドラだけ（パラメータは実行時） | いいえ（ルーターなし） | 設定で有効化（`typedRoutes`） | はい | 実験的（`typedPages`） | いいえ | いいえ（`href` は文字列） | はい（`Routable` の列挙型の値） |
| ブラウザの中で状態を持てる | はい | はい | はい | はい | はい | はい | はい | はい | はい |

## ベンチマーク

追加、チェック、削除をサーバーに保存する 2000 件の TodoMVC を、Apple M4 Max の Chrome 155 で 10 回測った中央値です。CPU を 4 倍に減速し、通信のたびに 50 ms の遅延を足しています。括弧の中は 10 回のうち最も速い回と遅い回の値です。

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar | Leptos | Dioxus |
|---|---|---|---|---|---|---|---|---|---|
| JS（gzip） | 19.7 KB | 14.8 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 13.1 KB | 5.5 KB | 15.7 KB |
| Wasm（gzip） | - | - | - | - | - | - | - | 145.1 KB | 441.4 KB |
| First Contentful Paint | 239 ms (231–309) | 221 ms (206–241) | 209 ms (198–266) | 240 ms (229–261) | 237 ms (217–251) | 236 ms (223–263) | 217 ms (204–249) | 232 ms (222–240) | 213 ms (205–244) |
| Total Blocking Time | 3 ms (0–37) | 318 ms (311–351) | 16 ms (11–23) | 14 ms (9–16) | 0 ms (0–0) | 36 ms (29–42) | 20 ms (17–23) | 47 ms (42–57) | 122 ms (114–126) |
| Interaction to Next Paint | 24 ms (24–24) | 24 ms (24–24) | 24 ms (16–24) | 24 ms (16–24) | 24 ms (16–24) | 24 ms (24–24) | 16 ms (16–16) | 24 ms (16–24) | 72 ms (72–80) |

測ったアプリと計測の仕組みは [`bench/compare`](bench/compare) にあります。どのアプリもコミット 7d031d0 で測りました。Leptos と Dioxus はコードの大半を WebAssembly で読み込むので、その大きさを JS の行とは別に Wasm の行に数えています。ブラウザの中だけで行うリストの操作を比べる [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark) の結果は、[`bench/js-framework-benchmark/results.md`](bench/js-framework-benchmark/results.md) にあります。

同じページを `hypersig::style`、Tailwind CSS、Panda CSS、StyleX、vanilla-extract でスタイルした計測は、[`hypersig_style` の README](crates/hypersig_style/README.ja.md#ベンチマーク) にあります。

コンパイル時間は、同じマシンの Rust 1.99.0 で `cargo build`（`dev` プロファイル）を使って測りました。空の target ディレクトリからのビルドは 3 回、アプリのコードを 1 行変えた後の再ビルドは 5 回測った中央値です。

| | Rust の行数 | 空の状態から | 1 行変えた後 |
|---|---|---|---|
| [`examples/todomvc`](examples/todomvc) | 485 | 13.4 s (11.0–13.7) | 1.3 s (1.1–1.4) |
| [`site`](site)（ドキュメントサイト） | 19,484 | - | 2.5 s (2.5–3.6) |

再ビルドでコンパイルし直すのはアプリのクレートだけです。`site` のビルドを `cargo build --timings` で測ると、`hypersig` クレートに 2.1 s、`site` のライブラリに 5.6 s かかりました。

## デプロイ

[デプロイ](https://imaimai17468.github.io/hypersig/ja/learn/deploying/) のページでは、`cargo install --locked hypersig_cli` でインストールする `cargo hypersig` でプロジェクトを作り、数行のコマンドで動くところまで進められます。`cargo hypersig new my-app --host server` で作るプロジェクトは、Docker でコンテナのイメージを作り、Fly.io、Cloud Run、Render のようなホストで動かします。`--host cloudflare` は同じページ、アクション、ストリームを Cloudflare Workers で動かし、Worker は受けたリクエストを axum の `Router` に渡します。`--host vercel` はそれらを Vercel の関数として動かし、`vercel_runtime` の `VercelLayer` が同じようにリクエストを `Router` に渡します。[`crates/hypersig_cli/templates/`](crates/hypersig_cli/templates) のテンプレートはコマンドに埋め込まれているので、同じ版で作ったプロジェクトは、ビルドで実行するコマンドも含めて同じファイルになります。Workers と Vercel のプロジェクトでは Node.js も必要です。Cloudflare のコマンドラインツール `wrangler` と Vercel のツール `vercel` が npm で配布されているためです。既存のプロジェクトでは、`cargo hypersig add <ホスト>` が `Cargo.toml` を編集してホストのファイルを書き出し、手で直す Rust のコードの変更を表示します。

## セキュリティ

[セキュリティ](https://imaimai17468.github.io/hypersig/ja/learn/security/) のページに、hypersig が確かめることをまとめています。エスケープ、ページに書ける属性と URL、アクションが拒否するリクエスト、リクエストでサーバーのどのコードを動かすかを指定できない理由です。アクションを誰が呼んでよいかをはじめ、アプリが自分で確かめることも載せています。

## コンパイラが検査しないこと

次の間違いはコンパイルが通り、プログラムを動かして初めて分かります（[コンパイラが検査しないこと](https://imaimai17468.github.io/hypersig/ja/learn/why/#what-the-compiler-does-not-check)）。

- `routes!` が生成する関数をルーターに渡していないと、そのブロックのページとアクションは 404 を返します。その関数の中でだけ使うハンドラの `fn` は rustc の `dead_code` lint が報告しますが、クロージャは報告されません。
- `Base` の下で配信するページが、その `Base` のない `routes!` ブロックのアクションを呼ぶと、呼び出しは接頭辞のないパスに送られ、プロキシか `Router::nest` が 404 を返します。
- 1 つの `node_ref!` か `dialog_ref!` の定数を、リストのテンプレートの外で 2 つの要素に付けると、ページの描画が panic します。
- `#[signals(computed)]` を付けたフィールドを、ページのどの `Page::computed` も指定していないと、そのフィールドは計算し直されず、ページの構造体が与えた値のまま変わりません。
- `label_for(&EMAIL)` のような参照が、ページのどの要素も持たない `Id` を指していると、ブラウザはその参照を無視するので、入力欄にラベルが付きません。
- `data-hs-foreign` の値は `office::MAP` のような定数の完全なパスで、モジュールや定数の名前を変えたり移したりすると変わります。hypersig の外のスクリプトが `El::foreign` を付けた要素をこの値で探していると、名前を変えたときに要素を見つけられなくなります。要素に `Id` を付けて、id で探してください（[JavaScript のライブラリ](https://imaimai17468.github.io/hypersig/ja/learn/javascript/)）。

## 向かないアプリ

ブラウザだけで完結するのは、hypersig が用意した演算と命令と、ページが読み込んだスクリプトだけです。地図やリッチテキストエディタのような JavaScript のライブラリはカスタム要素の中で動かせます。ページは属性で値を渡し、型付きのイベントを受け取ります（[JavaScript のライブラリ](https://imaimai17468.github.io/hypersig/ja/learn/javascript/)）。Custom Elements Manifest（`custom-elements.json`）を同梱したライブラリなら、`cargo hypersig elements` がタグ、属性名、イベント名を Rust の項目として書き出すので、つづりの間違いはコンパイルエラーになります。それ以外の処理はサーバーを通すので、次のようなアプリには向きません。

- お絵かきツールや表計算ソフトのように、主な処理が JavaScript のライブラリの中で動くアプリ
- サーバーにつながらない状態で動くアプリ

## 開発

不具合の報告とプルリクエストについては [`CONTRIBUTING.md`](CONTRIBUTING.md)（英語）に、リポジトリの構成と変更のたびに通す確認は [`AGENTS.md`](AGENTS.md) にあります。

## License

MIT

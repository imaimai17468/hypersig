<img src="https://raw.githubusercontent.com/imaimai17468/hypersig/main/assets/hypersig.svg" alt="" width="96" height="96">

# hypersig

[English](README.md) | [ドキュメント](https://imaimai17468.github.io/hypersig/ja/)

axum の上で動き、画面もブラウザでの動きもサーバーの処理も Rust で書く Web フレームワークです。ブラウザが扱う文字列（シグナルの名前、式、アクションの URL、リンク）はすべて Rust の型から作るので、その書き間違いはコンパイルエラーになります。

- `cargo build` だけでビルドでき、Node も wasm も要りません。
- 画面とサーバーの食い違いと、ルートに合わないリンクはコンパイルエラーになります（[hypersig を選ぶ理由](https://imaimai17468.github.io/hypersig/ja/learn/why/)）。
- 最初の HTML にページの状態が入っています。アクションは変わった値だけを返し、ブラウザはその値を表示している箇所だけを書き換えます（[サーバーとのやり取り](https://imaimai17468.github.io/hypersig/ja/learn/server/)）。
- サーバーの応答を待たずに画面を変える処理（楽観的更新）を、追加のコードなしで書けます。
- フォームの検証規則は一度書けば、ブラウザとサーバーの両方が同じ規則で検証します（[フォーム](https://imaimai17468.github.io/hypersig/ja/learn/forms/)）。
- CSS のプロパティと値も型で検査します（[スタイル](https://imaimai17468.github.io/hypersig/ja/style/)）。
- shadcn/ui の見た目のコンポーネント集と、訳の漏れをコンパイルエラーにする多言語化のクレートがあります（[コンポーネント](https://imaimai17468.github.io/hypersig/ja/components/)、[多言語化](https://imaimai17468.github.io/hypersig/ja/i18n/)）。

## フレームワークの比較

hypersig に近いのは Topcoat と Datastar です。どちらもブラウザの中にシグナルを持ち、ブラウザ向けのビルドなしにサーバーから画面を更新します。Topcoat はブラウザで動く式も Rust で書きます。

|  | Topcoat と Datastar | hypersig |
|---|---|---|
| シグナル名と URL（Datastar） | HTML の属性に文字列で書きます。打ち間違いはブラウザが実行するまで分かりません | Rust の値から書きます。打ち間違いがあると `cargo build` が止まります |
| リンクのパラメータ（Topcoat） | `href!` が検査するのはハンドラだけです。パスパラメータが足りないか多いと、ページを描くときにパニックし、リクエストは 500 を返します | `Route` がパラメータを型の付いた値で受け取ります。間違えるとコンパイルエラーになります |
| 一覧への追加（Topcoat） | 式から項目を足せないので、サーバーが答えて描き直したときに一覧が変わります | 一覧をシグナルに持つので、サーバーが答える前にブラウザが変更を表示します |

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar |
|---|---|---|---|---|---|---|---|
| 画面を書く言語 | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript と Vue のテンプレート | HTML の属性 |
| サーバーを書く言語 | Rust | Rust | 任意 | TypeScript | TypeScript | TypeScript | 任意 |
| ブラウザ向けのビルドが不要 | はい | はい | いいえ（Vite） | いいえ（Node のバンドラー） | いいえ（Vite） | いいえ（Vite） | はい |
| 送る値の型をコンパイル時に検査 | はい | はい | いいえ | はい | はい | いいえ | いいえ |
| リンクの行き先をコンパイル時に検査 | はい | ハンドラだけ（パラメータは実行時） | いいえ（ルーターなし） | 設定で有効化（`typedRoutes`） | はい | 実験的（`typedPages`） | いいえ |
| ブラウザの中で状態を持てる | はい | はい | はい | はい | はい | はい | はい |

## ベンチマーク

追加、チェック、削除をサーバーに保存する 2000 件の TodoMVC を、Apple M4 Max の Chrome 155 で 3 回測った中央値です。CPU を 4 倍に減速し、通信のたびに 50 ms の遅延を足しています。

| | hypersig | Topcoat | React | Next.js | TanStack Start | Nuxt | Datastar |
|---|---|---|---|---|---|---|---|
| JS（gzip） | 19.2 KB | 14.8 KB | 67.1 KB | 133.3 KB | 119.5 KB | 59.2 KB | 13.1 KB |
| First Contentful Paint | 186 ms | 165 ms | 196 ms | 198 ms | 193 ms | 177 ms | 182 ms |
| Total Blocking Time | 0 ms | 313 ms | 5 ms | 10 ms | 0 ms | 29 ms | 16 ms |
| Interaction to Next Paint | 24 ms | 24 ms | 24 ms | 24 ms | 24 ms | 16 ms | 16 ms |

測ったアプリと計測の仕組みは [`bench/compare`](bench/compare) にあります。ブラウザの中だけで行うリストの操作を比べる [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark) の結果は、[`bench/js-framework-benchmark/results.md`](bench/js-framework-benchmark/results.md) にあります。

同じページを `hypersig::style`、Tailwind CSS、Panda CSS、StyleX、vanilla-extract でスタイルした計測は、[`hypersig_style` の README](crates/hypersig_style/README.ja.md#ベンチマーク) にあります。

## クイックスタート

`Cargo.toml` に依存を追加します。

```toml
[dependencies]
hypersig = "0.1"
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

`cargo run` を実行し、<http://127.0.0.1:3000> を開いてください。`+1` のボタンはリクエストを送らず、ブラウザの中で数を増やします。リセットのボタンは `/reset` を呼び、ハンドラが返す差分で `count` が 0 に戻ります。`count` のつづりを間違えたり、`count` に文字列を足したりすると、`cargo build` が止まります。続きは [ドキュメント](https://imaimai17468.github.io/hypersig/ja/learn/) にあります。

## Cloudflare Workers

ページ、アクション、ストリームは Cloudflare Workers でもそのまま動きます。Worker は受けたリクエストを axum の `Router` に渡します。[`examples/cloudflare`](examples/cloudflare) は `worker-build` でビルドし、`wrangler dev` で起動します。空のプロジェクトから Worker を公開するまでの手順は [デプロイ](https://imaimai17468.github.io/hypersig/ja/learn/deploying/) にあります。リクエストごとに別の isolate で動くことがあるので、リクエストをまたいで共有する状態はメモリに置かず、KV、D1、Durable Object に置いてください。axum は SSE の keep-alive のコメントを tokio のタイマーで送るため、Workers のストリームはこのコメントを送りません。

## 向かないアプリ

ブラウザだけで完結するのは、hypersig が用意した演算と命令と、ページが読み込んだスクリプトだけです。地図やリッチテキストエディタのような JavaScript のライブラリはカスタム要素の中で動かせます。ページは属性で値を渡し、型付きのイベントを受け取ります（[JavaScript のライブラリ](https://imaimai17468.github.io/hypersig/ja/learn/javascript/)）。それ以外の処理はサーバーを通すので、次のようなアプリには向きません。

- お絵かきツールや表計算ソフトのように、主な処理が JavaScript のライブラリの中で動くアプリ
- サーバーにつながらない状態で動くアプリ

## 開発

リポジトリの構成と、変更のたびに通す確認は [`AGENTS.md`](AGENTS.md) にあります。

## License

MIT

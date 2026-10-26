<img src="https://raw.githubusercontent.com/imaimai17468/hypersig/main/assets/hypersig.svg" alt="" width="96" height="96">

# hypersig

[English](README.md) | [ドキュメント](https://imaimai17468.github.io/hypersig/ja/)

**サーバーが状態を、型が正しさを。**

Rust だけで書く[ハイパーメディア](https://imaimai17468.github.io/hypersig/ja/learn/why/#hypermedia)の Web フレームワークです。画面もブラウザでの動きもサーバーの処理も Rust で書き、axum の上で動きます。状態はサーバーが持ってブラウザにパッチを送り、ブラウザが扱う文字列（シグナルの名前、式、アクションの URL、リンク）はすべて Rust の型から作るので、その書き間違いはコンパイルエラーになります。

- `cargo build` だけでビルドでき、Node も wasm も要りません。
- 最初の HTML にページの状態が入っているので、ハイドレーションはありません。アクションは変わった値だけを返します（[hypersig を選ぶ理由](https://imaimai17468.github.io/hypersig/ja/learn/why/)）。
- フォームの検証規則、CSS、翻訳もコンパイラが検査します。shadcn/ui の見た目のコンポーネント集も付いています（[フォーム](https://imaimai17468.github.io/hypersig/ja/learn/forms/)、[スタイル](https://imaimai17468.github.io/hypersig/ja/style/)、[多言語化](https://imaimai17468.github.io/hypersig/ja/i18n/)、[コンポーネント](https://imaimai17468.github.io/hypersig/ja/components/)）。

## クイックスタート

`Cargo.toml` に依存を追加します。

```toml
[dependencies]
hypersig = "0.8"
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

`cargo run` を実行し、<http://127.0.0.1:3000> を開いてください。`+1` のボタンはリクエストを送らずにブラウザの中で数を増やし、リセットのボタンは `/reset` を呼んで、ハンドラが返す差分で `count` を 0 に戻します。`count` のつづりを間違えたり、`count` に文字列を足したりすると、`cargo build` が止まります。CLI（`cargo install --locked hypersig_cli`）を入れると、`cargo hypersig new my-app` でプロジェクトを作れ、`cargo hypersig dev` で変更のたびにビルドと再読み込みが走ります。続きは [ドキュメント](https://imaimai17468.github.io/hypersig/ja/learn/) にあります。

## フレームワークの比較

最も近い Topcoat と Datastar との詳しい比較は [hypersig を選ぶ理由](https://imaimai17468.github.io/hypersig/ja/learn/why/) にあります。

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

測ったアプリと計測の仕組みは [`bench/compare`](bench/compare) にあり、どれもコミット 7d031d0 で測りました。Leptos と Dioxus はコードの大半を WebAssembly で読み込むので、その大きさを Wasm の行に数えています。js-framework-benchmark の結果は [`bench/js-framework-benchmark`](bench/js-framework-benchmark/results.md) に、スタイルのクレートの比較は [`hypersig_style` の README](crates/hypersig_style/README.ja.md#ベンチマーク) にあります。同じマシンで [`examples/todomvc`](examples/todomvc)（485 行）をビルドすると、空の target ディレクトリからは 13.4 s、1 行変えた後の再ビルドは 1.3 s でした（`cargo build`、Rust 1.99.0）。

## デプロイとセキュリティ

`cargo hypersig new my-app --host server|cloudflare|vercel` は、コンテナのホスト、Cloudflare Workers、Vercel の関数のどれかで動くプロジェクトを作ります。既存のプロジェクトには `cargo hypersig add <ホスト>` で足せます（[デプロイ](https://imaimai17468.github.io/hypersig/ja/learn/deploying/)）。hypersig がリクエストごとに確かめることと、アクションを誰が呼んでよいかをはじめアプリが自分で確かめることは、[セキュリティ](https://imaimai17468.github.io/hypersig/ja/learn/security/) のページにまとめています。

## コンパイラが検査しないこと

名前、型、リンクが合っているかはコンパイラが検査します。次のミスはそれでもコンパイルが通り、動かして初めて分かります。同じ一覧は[ドキュメント](https://imaimai17468.github.io/hypersig/ja/learn/why/#what-the-compiler-does-not-check)にもあります。

| ミス | 起きること |
|---|---|
| `routes!` が生成する関数をルーターに渡し忘れる | そのページとアクションが 404 を返します |
| `Base` の下の `routes!` ブロックを、その接頭辞で `Router::nest` せず、接頭辞を取り除くプロキシの裏にも置かない | リンクは接頭辞から始まるのにルートには接頭辞がないので、そのページとアクションが 404 を返します |
| `Base` の下のページが、`Base` のない `routes!` ブロックのアクションを呼ぶ | 呼び出しが 404 になります |
| 同じ `node_ref!`、`dialog_ref!`、`Id`、`FileInput` の名前を 2 つの要素に付ける。リストのテンプレートの中で `El::id` を使う | ページの描画が panic します。`Id::from_text` で作った同じ id どうしは検出されず、その id を指す参照は先にある要素に届きます |
| `label_for` などの参照が、どの要素にもない `Id` を指す | 入力欄にラベルが付きません |
| hypersig の外のスクリプトが `El::foreign` の値で要素を探していて、その定数の名前を変える | スクリプトが要素を見つけられません。`Id` で探してください |
| `#[signals(computed)]` を付けたフィールドを `Page::computed` に渡し忘れる | ページの描画が panic します。入れ子の構造体のフィールドで panic するのは、ページがそのフィールドを読む場合だけです |
| リストの項目を、渡されたクロージャの外で使う | ページの組み立てか描画が panic します。ただし、クロージャの中で項目から作った式を、同じ深さの別のリストの中で使うと検出されず、そのリストの項目を読みます |
| シグナルに ±2^53 を超える整数を入れる | ページの描画か差分の作成が panic します |
| `String` のルートの区間か `Rest` の一部が空、`.`、`..` になる | リンクが 404 を返すか、ブラウザがドットを解決して別のページを求めます |

## 向かないアプリ

サーバーを通さずにブラウザで動くのは、hypersig が用意した演算と、ページが読み込んだスクリプトだけです。地図やエディタのような JavaScript のライブラリはカスタム要素の中で動かせます。`cargo hypersig elements` は、ライブラリの `custom-elements.json` にある名前を Rust の項目として書き出します（[JavaScript のライブラリ](https://imaimai17468.github.io/hypersig/ja/learn/javascript/)）。次のようなアプリには向きません。

- お絵かきツールや表計算ソフトのように、主な処理が JavaScript のライブラリの中で動くアプリ
- サーバーにつながらない状態で動くアプリ
- JavaScript を切ってもフォームが動く必要のあるページ。フォームからアクションを呼べるのはランタイムだけです

## 開発

不具合の報告とプルリクエストについては [`CONTRIBUTING.md`](CONTRIBUTING.md)（英語）に、リポジトリの構成と変更のたびに通す確認は [`AGENTS.md`](AGENTS.md) にあります。

## License

MIT

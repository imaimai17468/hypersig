# hypersig

[English](README.md)

axum の上で動き、画面もブラウザでの動きもサーバーの処理も Rust で書く Web フレームワークです。

- `cargo build` だけでビルドでき、Node も wasm も要りません。
- 画面とサーバーの食い違いは、コンパイルエラーになります。
- サーバーの応答を待たずに画面を変える処理（楽観的更新）を、追加のコードなしで書けます。

## フレームワークの比較

Rust のサーバーに画面を付けようとすると、今は 2 通りの選び方になります。Leptos なら画面とサーバーの間で型が検査される代わりに、wasm とそのビルドが要ります。htmx ならビルドは要りませんが、属性に書く URL や式は型で検査されません。

| | hypersig | Leptos | React | Next.js | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| 画面を書く言語 | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript と Vue のテンプレート | HTML の属性 |
| サーバーを書く言語 | Rust | Rust | 任意 | TypeScript | TypeScript | TypeScript | 任意 |
| ブラウザ向けのビルドが不要 | はい | いいえ（wasm と cargo-leptos） | いいえ（Vite） | いいえ（Node のバンドラー） | いいえ（Vite） | いいえ（Vite） | はい |
| 送る値の型をコンパイル時に検査 | はい | はい | いいえ | はい | はい | いいえ | いいえ |
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
use axum::{Router, response::Html, routing::get};
use hypersig::html::{button, div, span};
use hypersig::{Action, Page, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const RESET: Action<Counter> = Action::post("/reset");

async fn index() -> Html<String> {
    let count = Counter::count();
    let page = Page::new("Counter", Counter { count: 0 }).child(
        div()
            .child(button().on_click(count.set(count + 1)).text("+1"))
            .child(span().bind_text(count.to_text()))
            .child(button().on_click(RESET.call()).text("Reset on the server")),
    );
    Html(page.render())
}

async fn reset((): (), (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/", get(index))
        .hypersig_runtime()
        .action(RESET, reset);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
```

`cargo run` を実行し、<http://127.0.0.1:3000> を開いてください。`+1` のボタンはリクエストを送らず、ブラウザの中で数を増やします。リセットのボタンは `/reset` を呼び、ハンドラが返す差分で `count` が 0 に戻ります。ブラウザ用のランタイムは `hypersig_runtime()` が配信するので、Node も JavaScript のビルドも要りません。

## 制限事項

- ハンドラや式で使える演算は、ライブラリが用意したものに限られます。`localStorage` や外部ライブラリの呼び出しのような任意の JavaScript は書けません。
- 部品ごとの状態は、ページのシグナル構造体にあらかじめ置き場所を用意しておく必要があります。実行中に部品を増やすときは、`Vec` の項目にします。
- 算術は `i32` と `f64` だけで、除算は `f64` だけです。JavaScript の数値で同じ結果にならない型は、式に使えません。
- `equals` は JSON の形で比べます。derive した `PartialEq` とは結果が一致しますが、手で書いた `PartialEq` とは一致しない場合があります。
- サーバー側で式を評価するのは `Page::render` だけです。`Patches::element` で送るフラグメントはシグナルの値を持たないので、中の表示はブラウザに届いたあとでエフェクトが埋めます。
- サーバー側とブラウザ側の評価は、`to_fixed` のちょうど半分の丸め、`1e-6` 未満か `1e21` 以上の数の文字列化、基本多言語面の外の文字の大小比較で食い違うことがあります。読み込んだあとは、ブラウザ側の値で上書きされます。
- サーバーからのパッチで `each` を含むフラグメントを差し替えると、その中のリストの項目は作り直されます。項目の値はシグナルにあるので、内容は保たれます。
- シグナルには `#[serde(rename)]` などの属性を使えません。derive がエラーにします。

## 開発

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo bench -p hypersig --bench render
cd runtime && pnpm install && pnpm check && pnpm test && pnpm build
cd e2e && pnpm install && pnpm check && pnpm test   # 4 つの例を起動して Chrome で操作する
```

ブラウザ用のランタイムは `runtime/` にある TypeScript です。`pnpm build` がビルド済みの `crates/hypersig/assets/hypersig.js`（約 38 KB、gzip 後は約 13 KB）を書き出し、クレートはそれを埋め込みます。リポジトリの構成と、変更のたびに通す確認は `AGENTS.md` にあります。Lint の構成とコーディング規約は、Microsoft の [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) に沿っています。

## License

MIT

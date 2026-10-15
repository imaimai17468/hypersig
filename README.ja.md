# hypersig

[English](README.md)

axum の上で動く Web フレームワークです。画面、ブラウザ側の振る舞い、サーバーの処理を Rust で書きます。状態はサーバーが 1 つの Rust の構造体（シグナル構造体）として持ち、最初の HTML もそこから描きます。ブラウザでは小さなランタイムが変更をその場で反映し、サーバーにはあとから知らせるので、クリックがサーバーとの往復を待つことはありません。

## Leptos、Next.js、React、TanStack Start、Nuxt、htmx との比較

`bench/compare/` では、同じ 2000 件の TodoMVC をそれぞれで作り、動作が同じであることを確かめています。

| | hypersig | Leptos | Next.js | React | TanStack Start | Nuxt | htmx |
|---|---|---|---|---|---|---|---|
| 書く言語 | Rust | Rust | TypeScript | TypeScript | TypeScript | TypeScript と Vue のテンプレート | HTML の属性と、任意のサーバー言語 |
| ブラウザ向けのビルド | なし | wasm（cargo-leptos） | Node のバンドラー | Vite | Vite | Vite | なし |
| 最初の HTML にデータが入っている | はい | はい | はい | いいえ（読み込み後に取得） | はい | はい | はい |
| クリックした画面が、サーバーの応答より先に変わる | はい | はい | はい | はい | はい | はい | いいえ |
| サーバーの状態がブラウザに届く形 | シグナル構造体 | サーバー関数 | サーバーアクションと props | JSON の API | サーバー関数からクエリキャッシュへ | API ルート | HTML の断片 |

この中で hypersig だけが持つ点は 3 つです。

- **JavaScript のツールチェインも wasm も要りません。** アプリ全体が 1 つの Rust のクレートで、`cargo build` だけでビルドできます。ブラウザ用のランタイムはビルド済みのものをクレートに埋め込んでいます。
- **1 つの構造体が、サーバーとブラウザ両方でページの状態になります。** ページのシグナルは Rust の構造体で、ブラウザはその写しを持ち、サーバーのハンドラはそれに対する差分を返します。アクションの引数は、ページが呼ぶ側でもハンドラが受け取る側でも型付きです。フィールドの名前を変えると、それを使っている式、呼び出し、差分がすべてコンパイルエラーになるので、気づかないうちにずれることはありません。
- **ブラウザが実行する文字列を、ソースに書きません。** シグナル名、式、アクションの URL、差分の宛先はどれも Rust の値です。そのため、シグナル名の綴り違いや、`bool` を渡すべき所に数値を渡す間違いはコンパイルエラーになります。リポジトリの `crates/hypersig/tests/ui/fail/` には、こうした間違いを 19 件置いています。

## 同じアプリでの計測

2000 件の TodoMVC をそれぞれで作り、Apple M4 Max の headless Chrome で測りました。CPU は Lighthouse のモバイル設定と同じく 4 倍に減速し、通信のたびに 50 ms の遅延を足しています。FCP と TBT は Lighthouse の値、INP は Chrome の Event Timing API で 30 回の操作（todo のチェックと追加）から求めた値です。3 回の中央値で、詳しい条件は `bench/compare/SPEC.md` にあります。

| | JS（gzip） | FCP | TBT | INP |
|---|---|---|---|---|
| hypersig | 6.5 KB | 178 ms | 71 ms | 16 ms |
| Leptos | 140.5 KB | 196 ms | 28 ms | 24 ms |
| Next.js | 133.3 KB | 191 ms | 8 ms | 24 ms |
| React | 67.1 KB | 178 ms | 4 ms | 16 ms |
| TanStack Start | 119.5 KB | 179 ms | 0 ms | 24 ms |
| Nuxt | 59.2 KB | 167 ms | 26 ms | 24 ms |
| htmx | 16.5 KB | 230 ms | 248 ms | 16 ms |

hypersig が送る JavaScript は、Leptos、Next.js、React、TanStack Start、Nuxt の 1/9 から 1/22 で、INP は最も小さいグループに入ります。負けているのは TBT で、htmx 以外のすべてより長くなっています。まだプロファイルで確かめてはいませんが、サーバーが描いた 2000 件をランタイムが引き継ぐ処理が、Next.js、TanStack Start、Nuxt、Leptos のハイドレーションより長くメインスレッドを止めていると考えられます。INP はクリックの直後のフレームまでしか測らないので、htmx のチェックボックスがサーバーの応答を待って 59 ms 後に変わることは表に出ません。ほかのアプリでは 17 ms 以内に変わります。

## 変更のあとの更新

todo をチェックしてサーバーに保存する処理を、TanStack Query のドキュメントにある楽観的更新で書くと次のようになります。

```tsx
const queryClient = useQueryClient();
const toggle = useMutation({
  mutationFn: (change: { id: number; done: boolean }) => setTodoDone({ data: change }),
  onMutate: async ({ id, done }) => {
    await queryClient.cancelQueries({ queryKey: ["todos"] });
    const previous = queryClient.getQueryData<Todo[]>(["todos"]);
    queryClient.setQueryData<Todo[]>(["todos"], (list) =>
      list?.map((todo) => (todo.id === id ? { ...todo, done } : todo)),
    );
    return { previous };
  },
  onError: (_error, _change, context) => queryClient.setQueryData(["todos"], context?.previous),
});

<input type="checkbox" checked={todo.done}
  onChange={(event) => toggle.mutate({ id: todo.id, done: event.target.checked })} />
```

同じ処理を hypersig で書くと次のとおりです。

```rust
use hypersig::html::{El, each_keyed, input, li, ul};
use hypersig::{Action, Patches, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
}

const SET_DONE: Action<Todos, (i32, bool)> = Action::post("/done");

fn list() -> El<Todos> {
    ul().child(each_keyed(Todos::todos(), |todo| todo.at(Todo::id()), |todo| {
        let done = todo.at(Todo::done());
        li().child(
            input()
                .attr("type", "checkbox")
                .bind(done)
                .on_change(SET_DONE.call_with((todo.at(Todo::id()), done))),
        )
    }))
}

async fn set_done((): (), (id, done): (i32, bool)) -> Patches<Todos> {
    store(id, done);
    Patches::new()
}

fn store(_id: i32, _done: bool) {}
```

チェックボックスはブラウザで `done` を書き換え、その項目の `done` を読んでいる束縛だけが動き直します。そのあと `on_change` が、id と新しい値を `set_done` に送ります。型はどちらの側でも `(i32, bool)` です。クエリキーを決めることも、キャッシュの写しを取っておくことも、一覧を取り直すこともありません。

負担になるのは失敗したときです。サーバーがエラーを返しても、ブラウザで変えた値はそのまま残ります。`error(message)` で失敗は表示できますが、値を元に戻す仕組みはありません。

## はじめかた

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

## できないこと

- ハンドラや式で使える演算は、ライブラリが用意したものに限られます。`localStorage` や外部ライブラリの呼び出しのような任意の JavaScript は書けません。
- 部品ごとの状態は、ページのシグナル構造体にあらかじめ置き場所を用意しておく必要があります。実行中に部品を増やすときは、`Vec` の項目にします。
- 算術は `i32` と `f64` だけで、除算は `f64` だけです。JavaScript の数値で同じ結果にならない型は、式に使えません。
- `equals` は JSON の形で比べます。derive した `PartialEq` とは結果が一致しますが、手で書いた `PartialEq` とは一致しない場合があります。
- サーバー側で式を評価するのは `Page::render` だけです。`Patches::element` で送るフラグメントはシグナルの値を持たないので、中の束縛はブラウザが埋めます。
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

ブラウザ用のランタイムは `runtime/` にある TypeScript です。`pnpm build` がビルド済みの `crates/hypersig/assets/hypersig.js`（約 14 KB、gzip 後は約 5 KB）を書き出し、クレートはそれを埋め込みます。リポジトリの構成と、変更のたびに通す確認は `AGENTS.md` にあります。Lint の構成とコーディング規約は、Microsoft の [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) に沿っています。

## License

MIT

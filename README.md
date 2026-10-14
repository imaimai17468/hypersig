# hypersig

Rust 向けのサーバー駆動ハイパーメディアライブラリ。[Datastar](https://data-star.dev/) の設計（状態はサーバーが持ち、HTML とシグナルの差分を SSE で流し、ブラウザ側は小さなランタイムが適用する）を借りて、Datastar が手書きの文字列で書かせる部分をすべて Rust の型から生成する。

| Datastar で手書きする文字列 | hypersig での書き方 | 間違えたとき |
|---|---|---|
| シグナル名 `$count` | `#[derive(Signals)]` が生成する `Counter::count()` | 綴りを間違えるとコンパイルエラー |
| 属性の式 `data-show="$count > 0"` | `.show(count.gt(0))`。`show` は `bool` の式しか受け取らない | 型が合わないとコンパイルエラー |
| アクションの URL `@post('/increment')` | `const INC: Action<Counter> = Action::post("/increment")` を画面とルーターで共有 | 別ページのアクションを呼ぶとコンパイルエラー |
| パッチ先の id `#cart` | `impl Fragment for Cart { const ID = "cart"; ... }` | id を書く場所がひとつしかない |

式は JSON の AST として属性に書き出され、ランタイムがそれを解釈する。`eval` も `new Function` も使わないので、CSP で `unsafe-eval` を許可する必要がない（`runtime/test/bundle.test.ts` がバンドルを検査している）。

## React でできることとの対応

React で書く画面の部品を、hypersig では次のように書く。✅ は `examples/todomvc` と `examples/showcase` で実ブラウザ（headless Chrome）の操作テストを通したもの。

| React | hypersig | |
|---|---|---|
| `useState` | シグナル構造体のフィールドと `sig.set(...)` | ✅ |
| 部品ごとの状態（同じ部品を何度置いても別々の状態） | 入れ子の `Signals` 構造体を `Sig::at` でたどり、部品関数に `Sig<S, Disclosure>` を渡す | ✅ |
| props、children | Rust の関数引数と `El::child` | ✅ |
| `{cond && <X/>}` | `when(cond, el)` は DOM から外す。`.show(cond)` は隠すだけ | ✅ |
| `items.map(item => <li key=...>)` | `each(list, \|item\| li().key(item.at(Todo::id())))`。キーで要素を保ったまま並べ替える | ✅ |
| 配列の state 更新 | `push`、`remove_at`、`item.remove()`、`retain`、`set_all`、`count` | ✅ |
| 派生値（`useMemo` を使わない計算） | `Expr`。三項演算は `if_else`、文字列・数値・`Option`・構造体の操作がある | ✅ |
| `onClick` などのイベント | `on_click`、`on_double_click`、`on_key(Key::Enter, ...)`、`on_blur`、マウスの出入り | ✅ |
| 制御されたフォーム部品 | `bind` で text・textarea・select・number・checkbox、`bind_radio`、enum を選ぶ `bind_select` | ✅ |
| `className`、`style`、`disabled` の切り替え | `class_when`、`bind_style`、`bind_attr`、`attr_when` | ✅ |
| `useEffect(fn, [deps])` | `.effect(deps, stmt)`。要素が DOM に入ったときと、`deps` の値が変わったときに動く | ✅ |
| `setInterval` を張る effect | `.every(Duration, stmt)`。要素が DOM から外れると止まる | ✅ |
| `useRef` と `focus()` | `const R: NodeRef<S> = NodeRef::new(...)`、`R.focus()`。リストの中では同じ項目の要素を先に探す | ✅ |
| データ取得と読み込み中の表示（`useTransition` の `isPending`） | `ACTION.call().pending(flag).error(message)` | ✅ |
| 入力のデバウンス（React では別ライブラリ） | `on_debounced(Event::Input, Duration, stmt)` | ✅ |
| Context | ページのシグナル構造体はどの部品からも届く | ✅ |
| カスタムフック | `Expr` や `Stmt` を返す Rust の関数 | ✅ |
| SSR | サーバーが HTML を返すが、`each`、`when`、`bind_text` の中身は JS が動いてから埋まる | 一部 |
| effect の後始末関数、Portal、Suspense、Error Boundary | なし | ❌ |
| ハンドラ内の任意の JS（`Date.now()`、`localStorage`、外部ライブラリの呼び出し） | 式の演算は上の表にあるものに限る | ❌ |

ルーティングは React 本体の機能ではないので表に入れていない。hypersig ではページごとに axum のルートを分ける。

## 例

```rust
use hypersig::html::{Key, button, div, each, input, li, span, ul};
use hypersig::{Expr, Signals, Stmt};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
struct Todo {
    id: i32,
    title: String,
    done: bool,
}

#[derive(Signals, Serialize, Deserialize)]
struct Todos {
    todos: Vec<Todo>,
    draft: String,
    next_id: i32,
}

fn app() -> hypersig::html::El<Todos> {
    let (todos, draft, next_id) = (Todos::todos(), Todos::draft(), Todos::next_id());
    let blank = Expr::lit(&Todo { id: 0, title: String::new(), done: false });
    let add = Stmt::when(
        !draft.trim().is_empty(),
        todos
            .push(blank.with(Todo::id(), next_id).with(Todo::title(), draft.trim()))
            .then(next_id.set(next_id + 1))
            .then(draft.set("")),
    );
    div()
        .child(input().bind(draft).on_key(Key::Enter, add))
        .child(ul().child(each(todos, |todo| {
            li().key(todo.at(Todo::id()))
                .class_when("done", todo.at(Todo::done()))
                .child(input().attr("type", "checkbox").bind(todo.at(Todo::done())))
                .child(span().bind_text(todo.at(Todo::title())))
                .child(button().on_click(todo.remove()).text("×"))
        })))
        .child(span().bind_text(todos.count(|todo| !todo.at(Todo::done())).to_text()))
}
```

この画面の操作はすべてブラウザ内で完結する。サーバーに処理を渡すときは `Action` を宣言して `on_click(SAVE.call())` のように呼び、ハンドラはシグナル構造体を受け取って `Patches` を返す。

コンパイルエラーになる例は `crates/hypersig/tests/ui/fail/` に 17 件ある。たとえば `div().show(Counter::count())` は次のように止まる。

```
error[E0277]: `Sig<Counter, i32>` is not an expression of type `bool` on the page whose signals are `Counter`
```

リストの項目のフィールドに `patch` を呼ぶと、そのメソッドが存在しないというエラーになる。サーバーは「ブラウザがいま描いている項目」を指せないので、`patch` はページを起点にしたシグナルにだけある。

## 動かす

```sh
cargo run -p example-counter    # http://127.0.0.1:3000
cargo run -p example-todo       # http://127.0.0.1:3001 を 2 タブで開くと変更が同期する
cargo run -p example-todomvc    # http://127.0.0.1:3002 TodoMVC。再読み込みしても一覧が残る
cargo run -p example-showcase   # http://127.0.0.1:3003 上の表の残りの項目
```

Node は不要。ブラウザ用ランタイムはビルド済みの `crates/hypersig/assets/hypersig.js`（約 13 KB、gzip 前）をコミットしてあり、`include_str!` で埋め込まれる。

## 構成

| パス | 内容 |
|---|---|
| `crates/hypersig` | 本体。シグナル、式、HTML ビルダー、アクション、フラグメント、パッチ、axum 連携 |
| `crates/hypersig_proc` | `#[derive(Signals)]` の proc-macro シム |
| `crates/hypersig_proc_impl` | derive の実装とスナップショットテスト |
| `crates/hypersig/tests/fixtures.rs` | Rust で組んだ画面を `runtime/test/fixtures/` に書き出す。ランタイムのテストはこれを読み込むので、Rust 側とランタイム側の形式がずれるとどちらかのテストが落ちる |
| `runtime/` | ブラウザ用ランタイム（TypeScript）。`pnpm build` で上記 `assets/` に出力する |
| `examples/` | counter、todo、todomvc、showcase |

## できないこと

- 上の表の ❌ の行
- `each` と `when` の中身はブラウザが描くので、JS が動くまで空のまま表示される
- 部品ごとの状態は、ページのシグナル構造体にあらかじめ場所を用意しておく必要がある。実行中に部品を増やすときは `Vec` の項目にする
- 算術は `i32` と `f64` だけ、除算は `f64` だけ。JavaScript の数値で同じ結果にならない型は式に使えない
- `equals` は JSON の形で比べる。derive した `PartialEq` とは一致するが、手で書いた `PartialEq` とは一致しない場合がある
- 再描画は依存の追跡をせず、シグナルが変わるたびにページ上のすべての束縛を評価し直す。項目が数千を超えるリストでは遅くなる見込みだが、計測はしていない
- サーバーからのパッチで `each` を含むフラグメントを差し替えると、その中のリスト項目は作り直される（項目の値はシグナルにあるので内容は保たれる）
- シグナルは `#[serde(rename)]` などを使えない。derive が拒否する

## 開発

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd runtime && pnpm install && pnpm check && pnpm test && pnpm build
```

Lint の構成とコーディング規約は Microsoft の [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) に沿っている。エージェント向けの指示は `AGENTS.md` と `.claude/` にある。

## License

MIT

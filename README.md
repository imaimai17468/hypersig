# hypersig

Rust 向けのサーバー駆動ハイパーメディアライブラリ。[Datastar](https://data-star.dev/) の設計（状態はサーバーが持ち、HTML とシグナルの差分を SSE で流し、ブラウザ側は小さなランタイムが適用する）を借りて、Datastar が手書きの文字列で書かせる部分をすべて Rust の型から生成する。

| Datastar で手書きする文字列 | hypersig での書き方 | 間違えたとき |
|---|---|---|
| シグナル名 `$count` | `#[derive(Signals)]` が生成する `Counter::count()` | 綴りを間違えるとコンパイルエラー |
| 属性の式 `data-show="$count > 0"` | `.show(count.gt(0))`。`show` は `bool` の式しか受け取らない | 型が合わないとコンパイルエラー |
| アクションの URL `@post('/increment')` | `const INC: Action<Counter> = Action::post("/increment")` を画面とルーターで共有 | 別ページのアクションを呼ぶとコンパイルエラー |
| パッチ先の id `#cart` | `impl Fragment for Cart { const ID = "cart"; ... }` | id を書く場所がひとつしかない |

式は JSON の AST として属性に書き出され、ランタイムがそれを解釈する。`eval` も `new Function` も使わないので、CSP で `unsafe-eval` を許可する必要がない（`runtime/test/bundle.test.ts` がバンドルを検査している）。

## Datastar の属性との対応

Datastar の属性ごとに、hypersig で同じことをする書き方を並べる。どれも引数が型で縛られ、別ページのシグナルや型の合わない式はコンパイルで止まる。

| Datastar | hypersig |
|---|---|
| `data-signals` | `#[derive(Signals)]` した構造体を `Page::new` に渡す。入れ子の構造体は `Sig::at` でたどる |
| `data-text`、`data-show` | `bind_text`、`show` |
| `data-class`、`data-attr`、`data-style` | `class_when`、`bind_attr` と `attr_when`、`bind_style` |
| `data-bind` | `bind`（text、textarea、select、number、checkbox）、`bind_radio`、enum を選ぶ `bind_select` |
| `data-on` | `on_click`、`on_key(Key::Enter, ...)`、`on_blur` など。`__debounce` は `on_debounced` |
| `data-on-interval` | `on_interval(Duration, stmt)` |
| `data-init` | `on_load` |
| `data-effect` | `effect(stmt)`。文が読んだシグナルが変わると再実行する |
| `data-on-signal-patch` | `on_signal_change(sig, stmt)`。そのシグナルか中のフィールドが変わると動く |
| `data-indicator` | `ACTION.call().indicator(flag)`。失敗時のメッセージは `.error(message)` |
| `data-ref` | `const R: NodeRef<S> = NodeRef::new(...)` と `R.focus()` など |
| `@post`、`@get` | `const A: Action<S, Args> = Action::post(...)` と `A.call()` |
| `datastar-patch-elements`、`datastar-patch-signals` | `Patches::element(&fragment)`、`Patches::signals(sig.patch(v))` |

Datastar にないものも 2 つ足した。

- **ブラウザ側のリストと条件付き要素**: `each(list, |item| ...)` が `Vec` のシグナルを項目ごとに描き、`key` で要素を保ったまま並べ替える。`when(cond, el)` は条件が偽の間、要素を DOM から外す。項目は `push`、`item.remove()`、`retain`、`set_all`、`count` で操作する。
- **サーバー側での式の評価**: `Page::render` が初期シグナルで式を評価し、テキスト、表示切り替え、class、属性、フォームの値、リストの項目を HTML に書き込む。ランタイムは読み込み後、その要素を作り直さずに引き継ぐ。JS を切ったブラウザでも最初の画面はそのまま見える。

ランタイムは束縛ごとに読んだシグナルを記録し、パスの木から、変わったシグナルを読んでいる束縛だけを探して評価し直す。リストの束縛はリスト自体を浅く読むので、項目のキー以外のフィールドが変わってもキーの照合は走らない。2000 件の TodoMVC（headless Chrome、release ビルド、5 回の中央値）では、入力欄への 1 文字入力が 0.1 ms 未満、1 件のチェックが 3.3 ms だった。チェックに残っている時間の多くは、残数を数える `count` の 4 か所がそれぞれ 2000 件をたどる分。

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

この画面の操作はすべてブラウザ内で完結し、最初の HTML にはサーバーが項目と残数を書き込む。サーバーに処理を渡すときは `Action` を宣言して `on_click(SAVE.call())` のように呼び、ハンドラはシグナル構造体を受け取って `Patches` を返す。

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
cargo run -p example-showcase   # http://127.0.0.1:3003 状態を持つ部品、インターバル、検索、フォーム
```

Node は不要。ブラウザ用ランタイムはビルド済みの `crates/hypersig/assets/hypersig.js`（約 14 KB、gzip 後は約 5 KB）をコミットしてあり、`include_str!` で埋め込まれる。

## 構成

| パス | 内容 |
|---|---|
| `crates/hypersig` | 本体。シグナル、式、HTML ビルダー、アクション、フラグメント、パッチ、axum 連携 |
| `crates/hypersig_proc` | `#[derive(Signals)]` の proc-macro シム |
| `crates/hypersig_proc_impl` | derive の実装とスナップショットテスト |
| `crates/hypersig/tests/fixtures.rs` | Rust で組んだ画面を `runtime/test/fixtures/` に書き出す。ランタイムのテストはこれを読み込むので、Rust 側とランタイム側の形式がずれるとどちらかのテストが落ちる |
| `runtime/` | ブラウザ用ランタイム（TypeScript）。`pnpm build` で上記 `assets/` に出力する |
| `examples/` | counter、todo、todomvc、showcase |
| `e2e/` | 例を起動して Chrome で操作する Playwright のテスト |

## できないこと

- ハンドラや式で使える演算は上に挙げたものに限る。`Date.now()`、`localStorage`、外部ライブラリの呼び出しなど、任意の JS は書けない
- 部品ごとの状態は、ページのシグナル構造体にあらかじめ場所を用意しておく必要がある。実行中に部品を増やすときは `Vec` の項目にする
- 算術は `i32` と `f64` だけ、除算は `f64` だけ。JavaScript の数値で同じ結果にならない型は式に使えない
- `equals` は JSON の形で比べる。derive した `PartialEq` とは一致するが、手で書いた `PartialEq` とは一致しない場合がある
- サーバー側で式を評価するのは `Page::render` だけ。`Patches::element` で送るフラグメントはシグナルの値を持たないので、中の束縛はブラウザが埋める
- サーバー側の評価とブラウザ側の評価は、`to_fixed` のちょうど半分の丸め、`1e-6` 未満か `1e21` 以上の数の文字列化、基本多言語面の外の文字の大小比較で食い違うことがある。読み込み後はブラウザ側の値で上書きされる
- サーバーからのパッチで `each` を含むフラグメントを差し替えると、その中のリスト項目は作り直される（項目の値はシグナルにあるので内容は保たれる）
- シグナルは `#[serde(rename)]` などを使えない。derive が拒否する

## 開発

```sh
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cd runtime && pnpm install && pnpm check && pnpm test && pnpm build
cd e2e && pnpm install && pnpm check && pnpm test   # 4 つの例を起動して Chrome で操作する
```

Lint の構成とコーディング規約は Microsoft の [Pragmatic Rust Guidelines](https://microsoft.github.io/rust-guidelines/) に沿っている。エージェント向けの指示は `AGENTS.md` と `.claude/` にある。

## License

MIT

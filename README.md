# hypersig

画面もブラウザ側の振る舞いもサーバーの処理も Rust だけで書く Web フレームワーク。状態の正本はサーバーにあり、サーバーは HTML を返し、変更は SSE（Server-Sent Events）で差分として送る。ブラウザ側は約 5 KB（gzip 後）のランタイムが差分を当て、入力やクリックへの反応をその場で処理する。

## 特徴

**ブラウザが実行するものを、すべて Rust の型から作る。** シグナル名、属性に書く式、呼び出すエンドポイント、差し替える要素の id、フォーカスする要素の名前は、どれも Rust の値として書き、ライブラリが HTML に書き出す。手で文字列を書く場所がないので、次の間違いはどれもコンパイルエラーになる（`crates/hypersig/tests/ui/fail/` に 17 件）。

| 間違い | 止まる理由 |
|---|---|
| シグナル名の綴り違い | `#[derive(Signals)]` が生成する `Counter::count()` しか使えない |
| 表示切り替えに数値を渡す | `show` は `bool` の式しか受け取らない |
| 別ページのシグナルやアクションを使う | 式と文はページのシグナル構造体を型引数に持つ |
| アクションの引数を渡し忘れる、型を間違える | `Action<Page, Args>` を画面とルーターの両方で使う |
| リストの項目をサーバーから書き換えようとする | 項目を指すシグナルには `patch` がない |

**サーバーが書いた HTML がそのまま最初の画面になる。** `Page::render` は初期の値で式を評価し、テキスト、表示切り替え、class、属性、フォームの値、リストの項目を HTML に書き込む。ブラウザ側のランタイムは読み込み後にその要素を作り直さずに引き継ぐ。JS を切ったブラウザでも最初の画面は同じに見える。

**変わったところだけを描き直す。** ランタイムは束縛ごとに読んだシグナルのパスを記録し、変更されたパスからたどれる束縛だけを評価し直す。2000 件の TodoMVC（headless Chrome、release ビルド、5 回の中央値）で、入力欄への 1 文字入力が 0.1 ms、1 件のチェックが 2.8 ms だった。チェックのうち約 1.2 ms は、この例が変更のたびに一覧全体をサーバーへ保存している分。

**CSP を緩めなくてよい。** 式は JSON の構文木として属性に入り、ランタイムが解釈する。`eval` も `new Function` も使わないので `unsafe-eval` は要らない（`runtime/test/bundle.test.ts` がバンドルを検査している）。

**利用者のビルドに Node が要らない。** ランタイムはビルド済みのものをクレートに埋め込んである。`cargo build` だけで動く。

## 書けること

| やりたいこと | 書き方 |
|---|---|
| ページの状態 | `#[derive(Signals)]` した構造体を `Page::new` に渡す。入れ子の構造体は `Sig::at` でたどる |
| 状態を持つ部品の再利用 | 部品関数に `Sig<S, Disclosure>` を渡す。置いた場所ごとに別の状態になる |
| 他のシグナルから計算する値 | `Page::computed(sig, expr)`。読むシグナルが変わったときだけ計算し直す |
| テキスト、表示切り替え | `bind_text`、`show` |
| class、属性、style | `class_when`、`bind_attr`、`attr_when`、`bind_style` |
| フォーム部品 | `bind`（text、textarea、select、number、checkbox）、`bind_radio`、enum を選ぶ `bind_select` |
| リスト | `each(list, \|item\| ...)` と `key`。`push`、`item.remove()`、`retain`、`set_all`、`count` |
| 条件付きの要素 | `when(cond, el)`。偽の間は DOM から外す |
| イベント | `on_click`、`on_key(Key::Enter, ...)`、`on_blur`、`on_debounced` など |
| 表示時と定期的な処理 | `on_load`、`on_interval(Duration, stmt)` |
| 値の変化に反応する処理 | `effect(stmt)` は文が読んだシグナルの変化で、`on_signal_change(sig, stmt)` は指定したシグナルの変化で動く |
| サーバーの処理を呼ぶ | `const A: Action<S, Args> = Action::post(...)` と `A.call()`。`indicator(flag)` で通信中、`error(message)` で失敗を表示 |
| サーバーからの更新 | ハンドラが `Patches` を返す。`Fragment` で要素を、`sig.patch(v)` でシグナルを差し替える。`stream` で送り続けることもできる |
| DOM の操作 | `const R: NodeRef<S> = NodeRef::new(...)` と `R.focus()` など |

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

- ハンドラや式で使える演算は、ライブラリが用意したものに限る。`Date.now()`、`localStorage`、外部ライブラリの呼び出しなど、任意の JS は書けない
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

# hypersig

Rust 向けのサーバー駆動ハイパーメディアライブラリ。[Datastar](https://data-star.dev/) の設計（状態はサーバーが持ち、HTML とシグナルの差分を SSE で流し、ブラウザ側は小さなランタイムが適用する）を借りて、Datastar が手書きの文字列で書かせる部分をすべて Rust の型から生成する。

| Datastar で手書きする文字列 | hypersig での書き方 | 間違えたとき |
|---|---|---|
| シグナル名 `$count` | `#[derive(Signals)]` が生成する `Counter::count()` | 綴りを間違えるとコンパイルエラー |
| 属性の式 `data-show="$count > 0"` | `.show(count.gt(0))`。`show` は `bool` の式しか受け取らない | 型が合わないとコンパイルエラー |
| アクションの URL `@post('/increment')` | `const INC: Action<Counter> = Action::post("/increment")` を画面とルーターで共有 | 別ページのアクションを呼ぶとコンパイルエラー |
| パッチ先の id `#cart` | `impl Fragment for Cart { const ID = "cart"; ... }` | id を書く場所がひとつしかない |

式は JSON の AST として属性に書き出され、ランタイムがそれを解釈する。`eval` も `new Function` も使わないので、CSP で `unsafe-eval` を許可する必要がない（`runtime/test/bundle.test.ts` がバンドルを検査している）。

## 例

```rust
use hypersig::html::{button, div, span};
use hypersig::{Action, Page, Patches, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Counter {
    count: i32,
}

const RESET: Action<Counter> = Action::post("/reset");

fn page() -> String {
    let count = Counter::count();
    Page::new("Counter", Counter { count: 0 })
        .child(
            div()
                .child(span().bind_text(count.to_text()))
                .child(button().on_click(count.set(count + 1)).text("+"))
                .child(button().show(count.gt(0)).on_click(RESET.call()).text("reset")),
        )
        .render()
}

async fn reset(_state: (), _signals: Counter, (): ()) -> Patches<Counter> {
    Counter::count().patch(0).into()
}

// axum::Router::new().hypersig_runtime().action(RESET, reset)
```

`+` はブラウザ内だけで完結し、`reset` はサーバーに現在のシグナルを POST して、返ってきたパッチを適用する。

コンパイルエラーになる例は `crates/hypersig/tests/ui/fail/` にあり、たとえば `div().show(Counter::count())` は次のように止まる。

```
error[E0277]: `Sig<Counter, i32>` is not an expression of type `bool` on the page whose signals are `Counter`
```

## 動かす

```sh
cargo run -p example-counter   # http://127.0.0.1:3000
cargo run -p example-todo      # http://127.0.0.1:3001 を 2 タブで開くと変更が同期する
```

Node は不要。ブラウザ用ランタイムはビルド済みの `crates/hypersig/assets/hypersig.js`（約 6 KB）をコミットしてあり、`include_str!` で埋め込まれる。

## 構成

| パス | 内容 |
|---|---|
| `crates/hypersig` | 本体。シグナル、式、HTML ビルダー、アクション、フラグメント、パッチ、axum 連携 |
| `crates/hypersig_proc` | `#[derive(Signals)]` の proc-macro シム |
| `crates/hypersig_proc_impl` | derive の実装とスナップショットテスト |
| `runtime/` | ブラウザ用ランタイム（TypeScript）。`pnpm build` で上記 `assets/` に出力する |
| `examples/` | counter と todo |

## できないこと

PoC なので、範囲を意図的に狭めている。

- 1 ページにつきシグナル構造体はひとつ。部品ごとにシグナルの範囲を入れ子にする仕組みはない
- シグナルはフラットなフィールドのみで、`#[serde(rename)]` などは derive が拒否する
- 算術は `i32` と `f64` だけ、除算は `f64` だけ。JavaScript の数値で同じ結果にならない型は式に使えない
- `<input>` に束縛できるのは `String`（テキスト）と `bool`（チェックボックス）だけ
- DOM の差分適用は id と位置で突き合わせる簡易版で、idiomorph ほど賢くない

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

# hypersig_style

[English](README.md) | [ドキュメント](https://imaimai17468.github.io/hypersig/ja/style/)

[hypersig](https://github.com/imaimai17468/hypersig) のための型付きの atomic CSS です。`hypersig` クレートから `hypersig::style` として使います。

- `Style` には標準の CSS プロパティごとに 1 つずつメソッドがあり、MDN の CSS データから生成しています。プロパティ名やキーワードの綴りを間違えるとコンパイルエラーになります。
- 宣言はそれぞれ、宣言から名前を付けた 1 つのクラスになり、ページは使うルールを 1 回ずつ `<head>` に書きます。`cargo build` のほかにビルドの手順はありません。
- ホバー、ブレークポイント、グループやピアの状態、トークン、テーマ、キーフレームも Rust の値で書きます（[スタイル](https://imaimai17468.github.io/hypersig/ja/style/)）。

## ベンチマーク

[hypersig のフレームワークのベンチマーク](https://github.com/imaimai17468/hypersig/tree/main/bench/compare) にある 2000 件の TodoMVC のページを、6 通りのやり方でスタイルし、同じマークアップで hypersig に描かせたものです。違うのはクラス名と CSS だけです。Apple M4 Max の Chrome 155 で、CPU を 4 倍に減速して測りました。ブラウザの値は 5 回、サーバーで描く時間は 50 回の中央値です。

| | hypersig | Tailwind CSS | Panda CSS | StyleX | vanilla-extract | 手書きの CSS |
|---|---|---|---|---|---|---|
| CSS（gzip） | 1.0 KB | 1.6 KB | 1.0 KB | 1.0 KB | 0.6 KB | 0.8 KB |
| CSS を除いた HTML（gzip） | 30.2 KB | 29.1 KB | 26.9 KB | 26.8 KB | 27.6 KB | 27.3 KB |
| 送る合計（gzip） | 31.5 KB | 30.7 KB | 27.8 KB | 27.8 KB | 28.2 KB | 28.1 KB |
| CSS のビルド | なし | 268 ms | 574 ms | 182 ms | 74 ms | 3.1 ms |
| サーバーで描く時間 | 1.7 ms | 1.5 ms | 1.4 ms | 1.4 ms | 0.8 ms | 0.8 ms |
| First Contentful Paint | 136 ms | 140 ms | 136 ms | 144 ms | 140 ms | 140 ms |
| 読み込み時のスタイル計算とレイアウト | 76 ms | 74 ms | 73 ms | 81 ms | 67 ms | 68 ms |
| 2000 件を完了にする時間 | 27 ms | 37 ms | 26 ms | 27 ms | 24 ms | 24 ms |

| hypersig が不利な点 | 理由 |
|---|---|
| HTML の大きさ | クラス名を宣言だけから作るので長くなります。名前は条件、プロパティの正式な名前、値をつないだ `font-size_24px` のような形で、空白など英数字と `.`、`#`、`%`、`-` 以外の文字を含む値は 16 桁のハッシュになります。サーバーのパッチはページにあるクラスを知らずに要素を描くので、同じ宣言はいつ描いても同じ名前になる必要があります |
| サーバーで描く時間 | `Page::render` がその長い名前を項目ごとに書き出します。gzip 前の HTML は hypersig が 1.21 MB、Panda CSS が 0.88 MB で、描く時間も同じ順に長くなります |

ページと計測の仕組みは [`bench/style`](https://github.com/imaimai17468/hypersig/tree/main/bench/style) にあります。

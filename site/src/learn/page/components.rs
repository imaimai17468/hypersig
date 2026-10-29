//! The page on components: functions that return elements, and parts that
//! several pages embed.

use crate::Site;
use crate::doc::{Doc, Learn};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, heading, lead, para, parts, subheading, title};

pub fn page(site: Site) -> Part {
    parts(vec![
        title(Doc::Learn(Learn::Components).title(site)),
        lead(site, Doc::Learn(Learn::Components).leads()),
        functions(site),
        shared_parts(site),
        next(site),
    ])
}

fn functions(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "A function that returns an element",
                ja: "要素を返す関数",
            },
        ),
        para(
            site,
            L {
                en: "A component is a Rust function that returns an `El<S>`. Make it generic over `S` and take the signals it reads as arguments, and it works on any page. Its signature says what it reads, as `fn field<S>(text: &str, value: Sig<S, String>) -> El<S>` reads one `String` signal of whichever page calls it.",
                ja: "コンポーネントは `El<S>` を返す Rust の関数です。`S` についてジェネリックにし、読むシグナルを引数で受け取れば、どのページでも使えます。何を読むかはシグネチャに表れます。`fn field<S>(text: &str, value: Sig<S, String>) -> El<S>` は、呼び出したページの `String` のシグナルを 1 つ読みます。",
            },
        ),
        show(Snippet::Component),
        para(
            site,
            L {
                en: "The compiler takes `S` from the signal passed in, so a component given another page's signal returns an element of that page, which `child` refuses as [Elements and attributes](learn/elements/#an-element-belongs-to-one-page) shows. A signal of the wrong type, such as a `Sig<Signup, bool>` for `value`, is a compile error too. A component that keeps its own state, such as a disclosure that is open or closed, takes a signal to a nested struct, so each place it sits on a page has its own state, as [Signals inside signals](learn/nested-signals/) shows.",
                ja: "コンパイラは渡されたシグナルから `S` を決めるので、別のページのシグナルを渡したコンポーネントはそのページの要素を返します。[要素と属性](learn/elements/#an-element-belongs-to-one-page) のとおり、`child` はその要素を受け付けません。`value` に `Sig<Signup, bool>` を渡すように、型の違うシグナルを渡してもコンパイルエラーになります。開閉する折りたたみのように自分の状態を持つコンポーネントは、入れ子の構造体へのシグナルを受け取ります。するとページに置いた場所ごとに別々の状態を持てます。これは [入れ子のシグナル](learn/nested-signals/) で説明します。",
            },
        ),
        para(
            site,
            L {
                en: "[`hypersig_ui`](components/) is a set of such functions styled after shadcn/ui. The demos on this site use its buttons and text fields.",
                ja: "[`hypersig_ui`](components/) はこうした関数を集めたクレートで、見た目は shadcn/ui に倣っています。このサイトのデモはそのボタンとテキスト欄を使っています。",
            },
        ),
    ])
}

fn shared_parts(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Parts shared across pages",
                ja: "複数のページで使う部品",
            },
        ),
        para(
            site,
            L {
                en: "A piece that sits on several pages, such as a sign-out button in an app's shell or a newsletter form in a footer, is a [`Part`](api:hypersig/trait.Part.html): its own signal struct, an element built by `Part::render`, and actions over that struct. The trait is `trait Part: Signals { fn render() -> El<Self>; }`. A page whose struct has a field of the part's type embeds it with `html::part(Home::footer())`, and the part writes its paths under that field. Each call made inside the part tells the server the field, so the one handler registered for the part's action serves every page that embeds it, and the signals it patches and the fragments it renders again land under the field the call came from.",
                ja: "アプリの枠にあるサインアウトのボタンや、フッターのニュースレターのフォームのように複数のページに置くものは、[`Part`](api:hypersig/trait.Part.html) として書きます。部品は自分のシグナル構造体と、`Part::render` で組み立てる要素と、その構造体に対するアクションを持ちます。トレイトは `trait Part: Signals { fn render() -> El<Self>; }` です。部品の型のフィールドを持つページは `html::part(Home::footer())` で部品を埋め込み、部品はそのフィールドの下にパスを書きます。部品の中の呼び出しはサーバーにフィールドを伝えるので、部品のアクションに登録したハンドラ 1 つが、部品を埋め込んだどのページにも応答します。ハンドラが書き換えるシグナルと描き直すフラグメントは、呼び出し元のフィールドの下に入ります。",
            },
        ),
        show(Snippet::Parts),
        para(
            site,
            L {
                en: "`render` takes no arguments, so nothing built outside it reaches the part, and what the part shows comes from its own signals, which the page fills in as the value of its field. [`part`](api:hypersig/html/fn.part.html) returns a [`Node<S>`](api:hypersig/html/struct.Node.html), so nothing of the page can be added inside the part's element.",
                ja: "`render` は引数を取らないので、外で作ったものは部品に入りません。部品が表示するものは部品自身のシグナルから来ており、その値はページがフィールドの値として渡します。[`part`](api:hypersig/html/fn.part.html) は [`Node<S>`](api:hypersig/html/struct.Node.html) を返すので、部品の要素の中にページのものを加えることはできません。",
            },
        ),
        subheading(
            site,
            L {
                en: "What a part cannot do",
                ja: "部品でできないこと",
            },
        ),
        para(
            site,
            L {
                en: "Calling the part's action outside the part is a compile error, because a call made inside the part is what tells the server the field its patches land under. The action belongs to the part's struct, so the button that calls it is an element of the part, which the page's element refuses:",
                ja: "部品のアクションを部品の外から呼ぶとコンパイルエラーになります。パッチを書き込むフィールドをサーバーに伝えるのは、部品の中で行った呼び出しだからです。アクションは部品の構造体に属するので、それを呼ぶボタンは部品の要素になり、ページの要素はそれを受け付けません。",
            },
        ),
        show(Snippet::PartOutside),
        para(
            site,
            L {
                en: "Embedding the part at a field of a list item is a compile error too, because an item of a list has no path the server can patch. `part` takes a `Sig<S, P>` that starts at the page's struct, and an item's field is a `Sig<S, P, Local>`:",
                ja: "リストの項目のフィールドに部品を埋め込むこともコンパイルエラーになります。リストの項目には、サーバーが書き換えられるパスがないからです。`part` はページの構造体から始まる `Sig<S, P>` を受け取り、項目のフィールドは `Sig<S, P, Local>` です。",
            },
        ),
        show(Snippet::PartInList),
        para(
            site,
            L {
                en: "Two fields of one page may hold the same part as long as the part names no element id, node ref, or file input. Otherwise `Page::render` panics on the two elements that carry one of those.",
                ja: "部品が要素の id、ノード参照、ファイル入力のどれにも名前を付けていなければ、1 つのページの 2 つのフィールドに同じ部品を置けます。名前を付けている場合、それを持つ 2 つの要素について `Page::render` がパニックします。",
            },
        ),
        subheading(
            site,
            L {
                en: "The result of one call",
                ja: "1 回の呼び出しの結果",
            },
        ),
        para(
            site,
            L {
                en: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error) runs a statement with the error text when that one call fails, and `Call::on_success` runs one after its patches are applied. Both run where the call was made, and `let_in_stmt` names a value when the call is sent, so a page that adds several items before the server answers marks only the item whose save failed. The [`storefront`](https://github.com/imaimai17468/hypersig/tree/main/examples/storefront) example puts one newsletter part on two pages and saves a wish list this way. [Busy and failed calls](learn/call-status/) covers the state of a call.",
                ja: "[`Call::on_error`](api:hypersig/struct.Call.html#method.on_error) はその呼び出しが失敗したときにエラーの文言を受け取って文を実行し、`Call::on_success` は差分を適用し終えたあとに文を実行します。どちらも呼び出しを書いた場所で動き、`let_in_stmt` で送信時の値に名前を付けられるので、サーバーの応答より先に項目を複数追加するページでも、保存に失敗した項目だけに印を付けられます。[`storefront`](https://github.com/imaimai17468/hypersig/tree/main/examples/storefront) の例はニュースレターの部品を 2 つのページに置き、この方法でほしいものリストを保存します。呼び出しの状態は [呼び出し中と失敗](learn/call-status/) で説明します。",
            },
        ),
    ])
}

fn next(site: Site) -> Part {
    para(
        site,
        L {
            en: "Next, [Styling](learn/styling/) gives components their look with typed CSS. [Actions](learn/actions/) covers the action a part calls.",
            ja: "次の [スタイル](learn/styling/) では、型の付いた CSS でコンポーネントの見た目を決めます。部品が呼ぶアクションは [アクション](learn/actions/) で説明します。",
        },
    )
}

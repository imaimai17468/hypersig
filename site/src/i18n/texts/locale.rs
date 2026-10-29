//! The i18n page on the `Locale` trait: its bounds, `Text`, `pick`, `tag`,
//! `locales`, and the default language.

use crate::Site;
use crate::code::{self, Lang};
use crate::doc::{Doc, I18nPage};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, para, parts};

pub fn page(site: Site, head: Part) -> Part {
    parts(vec![
        head,
        lead(site, Doc::I18n(I18nPage::Locale).leads()),
        the_trait(site),
        pick(site),
        tag(site),
        variants(site),
        unchecked(site),
        example(site),
    ])
}

const TRAIT: &str = "pub trait Locale: Variants + Serialize + Copy + Eq {
    type Text<T>;

    fn pick<T>(self, text: Self::Text<T>) -> T;

    fn tag(self) -> LanguageTag { ... }
}

pub fn locales<L: Locale>() -> &'static [L];";

fn the_trait(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The trait",
                ja: "トレイト",
            },
        ),
        code::block(Lang::Rust, None, TRAIT),
        para(
            site,
            L {
                en: "[`Locale`](api:hypersig_i18n/trait.Locale.html) is implemented by an enum whose variants are the languages of the application. Each bound lets the enum be used in one way:",
                ja: "[`Locale`](api:hypersig_i18n/trait.Locale.html) を実装するのは、アプリケーションの言語をバリアントとして並べた enum です。境界はそれぞれ、enum の次の使い方を可能にします。",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "[`Variants`](api:hypersig/trait.Variants.html) comes only from `#[derive(Segment)]`, which also implements [`Segment`](api:hypersig/trait.Segment.html). It lists the variants for `locales`, and `Segment` lets the language be a parameter of a route.",
                    ja: "[`Variants`](api:hypersig/trait.Variants.html) は `#[derive(Segment)]` だけが実装し、この derive は [`Segment`](api:hypersig/trait.Segment.html) も実装します。`Variants` は `locales` のためにバリアントを並べ、`Segment` によって言語をルートのパラメータにできます。",
                },
                L {
                    en: "`Serialize` gives `tag` the language's tag, the string serde writes for the variant.",
                    ja: "`Serialize` は、serde がバリアントに対して書く文字列を言語のタグとして `tag` に渡します。",
                },
                L {
                    en: "`Copy` and `Eq` let a handler pass the language around by value and compare it, as in `lang == Lang::En`.",
                    ja: "`Copy` と `Eq` によって、ハンドラは言語を値のまま渡し、`lang == Lang::En` のように比べられます。",
                },
            ],
        ),
        para(
            site,
            L {
                en: "The trait does not ask for `Deserialize`. The route reads the language from the URL through `Segment`. A signal struct needs it to hold a `Lang` in a field, because the signals are deserialized.",
                ja: "トレイトは `Deserialize` を求めません。ルートは URL から `Segment` を通して言語を読みます。シグナルはデシリアライズされるので、シグナルの構造体のフィールドに `Lang` を持たせるには `Deserialize` が必要です。",
            },
        ),
        para(
            site,
            L {
                en: "An enum without `#[derive(Segment)]` fails to compile at the `impl` with E0277, `` `Lang` does not list its variants``:",
                ja: "`#[derive(Segment)]` のない enum は `impl` の行で E0277（`` `Lang` does not list its variants``）になります。",
            },
        ),
        show(Snippet::I18nNoSegment),
        para(
            site,
            L {
                en: "An enum without `Serialize` is E0277 at the `impl` too, because `tag` has no string to read:",
                ja: "`Serialize` のない enum も、`tag` が読む文字列がないので、`impl` の行で E0277 になります。",
            },
        ),
        show(Snippet::I18nNoSerialize),
    ])
}

fn pick(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Text and pick",
                ja: "Text と pick",
            },
        ),
        para(
            site,
            L {
                en: "`type Text<T>` is the type that holds one value per language, and `pick` returns the value of `self`. Usually `Text<T>` is a struct with one field of type `T` per language, and `pick` is a `match` with one arm per variant, as `Lang` and `ByLang` in [Declare the languages](i18n/overview/#declare-the-languages) are. `T` is any type, so the same struct holds a `&str`, a function, or an element, which [Writing a text](i18n/texts/) shows.",
                ja: "`type Text<T>` は言語ごとに 1 つの値を持つ型で、`pick` は `self` の言語の値を返します。[言語を宣言する](i18n/overview/#declare-the-languages) の `Lang` と `ByLang` のように、普通は `Text<T>` を言語ごとに型 `T` のフィールドを 1 つ持つ構造体にし、`pick` をバリアントごとに 1 つの分岐を持つ `match` にします。`T` はどの型でもよいので、同じ構造体に `&str` も関数も要素も入ります。その例は [文を書く](i18n/texts/) にあります。",
            },
        ),
        para(
            site,
            L {
                en: "`pick` takes the text by value and moves the chosen field out. A `match` without the arm of a variant is E0004, so every language has its field read:",
                ja: "`pick` は文を値で受け取り、選んだフィールドを取り出します。あるバリアントの分岐がない `match` は E0004 になるので、どの言語のフィールドも必ず読まれます。",
            },
        ),
        show(Snippet::I18nMissingArm),
        para(
            site,
            L {
                en: "A wildcard arm, `_ => text.en`, compiles and gives every other language the English text. It also keeps a language added later from failing to compile here, so write one arm per variant.",
                ja: "ワイルドカードの分岐 `_ => text.en` はコンパイルが通り、ほかのすべての言語に英語の文を返します。あとで追加した言語もここではコンパイルエラーにならなくなるので、分岐はバリアントごとに 1 つずつ書いてください。",
            },
        ),
    ])
}

fn tag(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "The tag",
                ja: "タグ",
            },
        ),
        para(
            site,
            L {
                en: "`tag` returns the variant's BCP 47 language tag as a [`LanguageTag`](api:hypersig/struct.LanguageTag.html), which `Page::lang` and `Page::alternate` take. It is the string serde writes for the variant, and `#[derive(Segment)]` reads the same `#[serde(rename_all)]` and `#[serde(rename)]` attributes, so the tag and the route segment are one string. `rename_all = \"kebab-case\"` turns `PtBr` into `pt-br`, and `#[serde(rename = \"zh-Hant\")]` names one variant by hand:",
                ja: "`tag` はバリアントの BCP 47 の言語タグを [`LanguageTag`](api:hypersig/struct.LanguageTag.html) として返し、`Page::lang` と `Page::alternate` はこれを受け取ります。タグは serde がバリアントに対して書く文字列です。`#[derive(Segment)]` も同じ `#[serde(rename_all)]` と `#[serde(rename)]` を読むので、タグとルートのセグメントは同じ文字列になります。`rename_all = \"kebab-case\"` は `PtBr` を `pt-br` にし、`#[serde(rename = \"zh-Hant\")]` は 1 つのバリアントに直接名前を付けます。",
            },
        ),
        show(Snippet::I18nRegions),
        para(
            site,
            L {
                en: "`tag` checks only the shape of the string: subtags of one to eight ASCII letters and digits joined by `-`, the first of two, three, or five to eight letters. It panics where the string does not have that shape, as `en_US` does, and the panic happens on the first call, often while rendering a page. Whether the language exists in the IANA registry is not checked.",
                ja: "`tag` が検査するのは文字列の形だけです。1〜8 文字の ASCII の英数字からなるサブタグを `-` でつなぎ、最初のサブタグは 2、3、または 5〜8 文字の英字でなければなりません。`en_US` のようにこの形でない文字列ではパニックし、パニックは最初の呼び出しで、多くはページの描画中に起きます。その言語が IANA のレジストリにあるかどうかは検査しません。",
            },
        ),
        show(Snippet::I18nBadTag),
    ])
}

fn variants(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Every language and the default",
                ja: "すべての言語と既定の言語",
            },
        ),
        para(
            site,
            L {
                en: "[`locales::<L>()`](api:hypersig_i18n/fn.locales.html) returns every variant in declaration order. The first variant is the default language: [`negotiate`](api:hypersig_i18n/fn.negotiate.html) returns it when the `Accept-Language` header matches no language, and [`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) writes it as `x-default`. To change the default, move its variant to the top.",
                ja: "[`locales::<L>()`](api:hypersig_i18n/fn.locales.html) はすべてのバリアントを宣言の順に返します。最初のバリアントが既定の言語です。[`negotiate`](api:hypersig_i18n/fn.negotiate.html) は `Accept-Language` ヘッダーがどの言語にも一致しないときにこれを返し、[`LocalePage::alternates`](api:hypersig_i18n/trait.LocalePage.html#tymethod.alternates) はこれを `x-default` として書きます。既定の言語を変えるには、そのバリアントを先頭に移します。",
            },
        ),
        show(Snippet::I18nTag),
        para(
            site,
            L {
                en: "`#[derive(Segment)]` refuses an enum with no variant, with the error `` `Segment` needs an enum with at least one variant``, so every language enum has a default.",
                ja: "`#[derive(Segment)]` はバリアントのない enum を `` `Segment` needs an enum with at least one variant`` というエラーで拒否します。そのため、言語の enum には必ず既定の言語があります。",
            },
        ),
    ])
}

fn unchecked(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "What stays unchecked",
                ja: "検査しないこと",
            },
        ),
        bullets(
            site,
            &[
                L {
                    en: "Which field `pick` returns for each variant. An arm `Self::Ja => text.en` compiles, and Japanese pages show the English text.",
                    ja: "`pick` が各バリアントにどのフィールドを返すか。`Self::Ja => text.en` と書いてもコンパイルは通り、日本語のページに英語の文が表示されます。",
                },
                L {
                    en: "The shape of the tag, until `tag` runs. A tag such as `en_US` panics then.",
                    ja: "タグの形は `tag` を呼ぶまで検査しません。`en_US` のようなタグはそのときにパニックします。",
                },
                L {
                    en: "Whether the tag names a real language. `xx` is shaped as a tag and passes.",
                    ja: "タグが実在する言語を指しているか。`xx` はタグの形をしているので通ります。",
                },
            ],
        ),
    ])
}

fn example(site: Site) -> Part {
    parts([
        heading(
            site,
            L {
                en: "Example",
                ja: "例",
            },
        ),
        para(
            site,
            L {
                en: "The declaration of [Declare the languages](i18n/overview/#declare-the-languages), complete:",
                ja: "[言語を宣言する](i18n/overview/#declare-the-languages) の宣言の全体です。",
            },
        ),
        show(Snippet::I18nDeclare),
        para(
            site,
            L {
                en: "Next, [Writing a text](i18n/texts/) covers the texts that `pick` chooses from.",
                ja: "次の [文を書く](i18n/texts/) で `pick` が選ぶ文の書き方を説明します。",
            },
        ),
    ])
}

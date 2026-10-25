//! The use cases page: the three kinds of request where hypersig and Next.js
//! differ in the code a change takes, and the applications that get them.

use hypersig::html::{a, attr, details, div, h3, h4, li, p, span, summary, ul};
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::{
    AlignItems, BorderLeftStyle, BorderStyle, Cursor, Display, FontStyle, FontWeight,
    JustifyContent, LineHeight, ListStyleType, px, raw, rem,
};
use hypersig::style::{Condition, PseudoClass, Style};
use hypersig_ui::{ButtonVariant, token};

use crate::Site;
use crate::doc::Doc;
use crate::landing::{self, hero, titled};
use crate::lang::L;
use crate::learn::snippet::{Snippet, show_change};
use crate::prose::{Part, bullets, href, inline, para};

pub fn render(site: Site) -> String {
    landing::page(
        site,
        Doc::UseCases,
        [
            hero(
                site,
                Doc::UseCases.title(site),
                site.t(L {
                    en: "What a new request changes in the code",
                    ja: "仕様の追加で、コードのどこが変わるか",
                }),
                site.t(L {
                    en: "A screen built with hypersig can look and behave like one built with Next.js. The difference shows when someone asks for one more thing, in which files change and in what has to be written by hand. Each hypersig change below is code that `cargo test` compiles, beside the work the same request takes in Next.js 16 with the App Router.",
                    ja: "hypersig で作った画面も Next.js で作った画面も、見た目と動きは同じにできます。差が出るのは、あとから仕様を 1 つ足すときに、どのファイルが変わり、何を手で書くかです。下の hypersig の差分は `cargo test` がコンパイルしているコードで、それぞれに Next.js 16 の App Router で同じ依頼に要る作業を並べています。",
                }),
                &[
                    (
                        site.t(L {
                            en: "Get started",
                            ja: "はじめる",
                        }),
                        "learn/",
                        ButtonVariant::Default,
                    ),
                    (
                        site.t(L {
                            en: "Why hypersig",
                            ja: "hypersig を選ぶ理由",
                        }),
                        "learn/why/",
                        ButtonVariant::Outline,
                    ),
                ],
            ),
            titled(
                site,
                L {
                    en: "Where the difference comes from",
                    ja: "差が出る場面",
                },
                [
                    para(
                        site,
                        L {
                            en: "The requests on this page fall into three kinds, by what the framework handles for them. The Next.js work starts from the shape of the TodoMVC in `bench/compare/next`, where a Server Component reads the data and a Client Component holds it in `useState`. In each hypersig change, a green line was added and a red line removed.",
                            ja: "このページの依頼は、フレームワークが何を受け持つかで 3 種類に分かれます。Next.js の作業は `bench/compare/next` の TodoMVC と同じ構成から始めた場合のものです。その構成では、Server Component がデータを読み、Client Component がそれを `useState` に持ちます。hypersig の差分では、緑の行は足した行、赤の行は消した行です。",
                        },
                    ),
                    div().children(CONCEPTS.iter().map(|concept| concept_section(site, concept))),
                ],
            ),
            titled(
                site,
                L {
                    en: "Applications that get these requests",
                    ja: "この差が出るアプリ",
                },
                [
                    para(
                        site,
                        L {
                            en: "Each application below gets several requests of the kinds above. Open one to read the change one of them makes.",
                            ja: "下のアプリには、上の種類の依頼がいくつも来ます。開くと、そのうち 1 つの依頼の差分を読めます。",
                        },
                    ),
                    div()
                        .css(stack(rem(0.75)).margin_top(rem(1.5)))
                        .children(CASES.iter().map(|case| case_card(site, case))),
                ],
            ),
        ],
    )
}

/// The three kinds of request, each the heading of a concept section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    /// A change made on the server reaches the open tabs.
    Push,
    /// The browser and the server use one name from a Rust type.
    Names,
    /// The runtime orders the requests and handles their failures.
    Calls,
}

impl Kind {
    fn letter(self) -> &'static str {
        match self {
            Self::Push => "A",
            Self::Names => "B",
            Self::Calls => "C",
        }
    }

    /// The id of the kind's section, which a request's badge links to.
    fn id(self) -> &'static str {
        match self {
            Self::Push => "push",
            Self::Names => "names",
            Self::Calls => "calls",
        }
    }

    /// The short name a badge shows.
    fn short(self) -> L<&'static str> {
        match self {
            Self::Push => L {
                en: "Server pushes",
                ja: "サーバーから届く",
            },
            Self::Names => L {
                en: "One name",
                ja: "同じ名前",
            },
            Self::Calls => L {
                en: "Order and failure",
                ja: "順序と失敗",
            },
        }
    }

    fn title(self) -> L<&'static str> {
        match self {
            Self::Push => L {
                en: "A change made on the server reaches every open tab",
                ja: "サーバーで起きた変更が開いているタブに届く",
            },
            Self::Names => L {
                en: "The browser and the server use one name from a Rust type",
                ja: "ブラウザとサーバーが Rust の型から同じ名前を使う",
            },
            Self::Calls => L {
                en: "The runtime orders the requests and handles their failures",
                ja: "リクエストの順序と失敗をランタイムが受け持つ",
            },
        }
    }
}

/// One request, with the work it takes in Next.js and the change it makes
/// in hypersig.
struct Detail {
    next: &'static [L<&'static str>],
    change: Snippet,
    caption: L<&'static str>,
}

/// A kind of request: what differs, and one request of the kind in detail.
struct Concept {
    kind: Kind,
    text: L<&'static str>,
    request: L<&'static str>,
    detail: Detail,
}

/// An application, the requests it gets, and one of them in detail.
struct Case {
    title: L<&'static str>,
    scene: L<&'static str>,
    /// The requests listed before the one `detail` is about.
    before: &'static [(Kind, L<&'static str>)],
    detailed: (Kind, L<&'static str>),
    after: &'static [(Kind, L<&'static str>)],
    detail: Detail,
}

fn concept_section(site: Site, concept: &Concept) -> Part {
    let kind = concept.kind;
    div()
        .css(Style::new().margin_top(rem(3)))
        .child(
            h3().id(kind.id())
                .css(
                    Style::new()
                        .display(Display::Flex)
                        .align_items(AlignItems::Center)
                        .gap(rem(0.75))
                        .font_size(rem(1.375))
                        .line_height(LineHeight::Length(rem(1.875)))
                        .font_weight(FontWeight::Number(600.0)),
                )
                .child(letter(kind))
                .child(span().text(site.t(kind.title()))),
        )
        .child(para(site, concept.text))
        .child(
            p().css(
                Style::new()
                    .margin_top(rem(1.5))
                    .padding_left(rem(1))
                    .border_left_width(px(3))
                    .border_left_style(BorderLeftStyle::Solid)
                    .border_left_color(token::border().var())
                    .font_style(FontStyle::Italic)
                    .line_height(LineHeight::Length(rem(1.75))),
            )
            .children(inline(site, site.t(concept.request))),
        )
        .child(detail(site, &concept.detail))
}

impl Case {
    /// Every request of the application, each with whether it is the one
    /// `detail` is about.
    fn requests(&self) -> impl Iterator<Item = (Kind, L<&'static str>, bool)> {
        let listed = |requests: &'static [(Kind, L<&'static str>)]| {
            requests
                .iter()
                .map(|&(kind, request)| (kind, request, false))
        };
        listed(self.before)
            .chain(std::iter::once((self.detailed.0, self.detailed.1, true)))
            .chain(listed(self.after))
    }
}

fn case_card(site: Site, case: &Case) -> Part {
    let kinds = [Kind::Push, Kind::Names, Kind::Calls]
        .into_iter()
        .filter(|&kind| case.requests().any(|(of, _, _)| of == kind));
    details()
        .css(
            Style::new()
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::border().var())
                .border_radius(token::radius().var())
                .background_color(token::card().var())
                .group(),
        )
        .child(
            summary()
                .css(
                    // A summary that is not `display: list-item` draws no
                    // disclosure marker.
                    Style::new()
                        .display(Display::Flex)
                        .align_items(AlignItems::Center)
                        .justify_content(JustifyContent::SpaceBetween)
                        .gap(rem(1))
                        .padding(rem(1.25))
                        .list_style_type(ListStyleType::None)
                        .cursor(Cursor::Pointer),
                )
                .child(
                    div()
                        .css(stack(rem(0.375)))
                        .child(
                            h3().css(
                                Style::new()
                                    .font_size(rem(1.125))
                                    .font_weight(FontWeight::Number(600.0)),
                            )
                            .text(site.t(case.title)),
                        )
                        .child(
                            p().css(small_muted())
                                .children(inline(site, site.t(case.scene))),
                        )
                        .child(
                            div()
                                .css(wrap(rem(0.5)).margin_top(rem(0.25)))
                                .children(kinds.map(|kind| badge(site, kind))),
                        ),
                )
                .child(
                    span()
                        .attr(attr::ARIA_HIDDEN, "true")
                        .css(
                            Style::new()
                                .font_size(rem(1.25))
                                .color(token::muted_foreground().var())
                                .on(
                                    Condition::Group(PseudoClass::Open),
                                    Style::new().rotate(raw("90deg")),
                                ),
                        )
                        .text("›"),
                ),
        )
        .child(
            div()
                .css(
                    Style::new()
                        .padding_inline(rem(1.25))
                        .padding_bottom(rem(1.5)),
                )
                .child(
                    ul().css(
                        stack(rem(0.5))
                            .padding_left(px(0))
                            .list_style_type(ListStyleType::None),
                    )
                    .children(case.requests().map(
                        |(kind, request, detailed)| {
                            // The change below is about one of the requests, so
                            // it stands out from the others.
                            let text = if detailed {
                                Style::new().font_weight(FontWeight::Number(500.0))
                            } else {
                                Style::new().color(token::muted_foreground().var())
                            };
                            li().css(
                                Style::new()
                                    .display(Display::Flex)
                                    .align_items(AlignItems::Baseline)
                                    .gap(rem(0.75))
                                    .font_size(rem(0.9375))
                                    .line_height(LineHeight::Length(rem(1.5)))
                                    .merge(text),
                            )
                            .child(
                                a().href(href(site, &format!("use-cases/#{}", kind.id())))
                                    .css(Style::new().flex_shrink(0))
                                    .child(letter(kind)),
                            )
                            .child(span().children(inline(site, site.t(request))))
                        },
                    )),
                )
                .child(detail(site, &case.detail)),
        )
}

/// The work a request takes in Next.js, and the hypersig change.
fn detail(site: Site, detail: &Detail) -> Part {
    div()
        .child(label(site.t(L {
            en: "In Next.js 16 with the App Router",
            ja: "Next.js 16（App Router）での作業",
        })))
        .child(bullets(site, detail.next))
        .child(label(site.t(L {
            en: "In hypersig",
            ja: "hypersig の差分",
        })))
        .child(show_change(detail.change))
        .child(
            p().css(small_muted().margin_top(rem(0.75)))
                .children(inline(site, site.t(detail.caption))),
        )
}

fn label(text: &str) -> Part {
    h4().css(
        Style::new()
            .margin_top(rem(1.5))
            .font_size(rem(0.875))
            .font_weight(FontWeight::Number(600.0))
            .color(token::muted_foreground().var()),
    )
    .text(text)
}

/// The letter of `kind` in a circle.
fn letter(kind: Kind) -> Part {
    span()
        .css(
            Style::new()
                .display(Display::InlineFlex)
                .align_items(AlignItems::Center)
                .justify_content(JustifyContent::Center)
                .width(rem(1.5))
                .height(rem(1.5))
                .border_radius(px(999))
                .background_color(token::primary().var())
                .color(token::primary_foreground().var())
                .font_size(rem(0.75))
                .font_weight(FontWeight::Number(700.0)),
        )
        .text(kind.letter())
}

/// The letter and the short name of `kind`, as a summary lists them.
fn badge(site: Site, kind: Kind) -> Part {
    span()
        .css(
            Style::new()
                .display(Display::InlineFlex)
                .align_items(AlignItems::Center)
                .gap(rem(0.375))
                .padding_block(rem(0.125))
                .padding_left(rem(0.125))
                .padding_right(rem(0.625))
                .border_width(px(1))
                .border_style(BorderStyle::Solid)
                .border_color(token::border().var())
                .border_radius(px(999))
                .font_size(rem(0.75))
                .color(token::muted_foreground().var()),
        )
        .child(letter(kind))
        .text(site.t(kind.short()))
}

fn small_muted() -> Style {
    Style::new()
        .font_size(rem(0.875))
        .line_height(LineHeight::Length(rem(1.5)))
        .color(token::muted_foreground().var())
}

const CONCEPTS: &[Concept] = &[
    Concept {
        kind: Kind::Push,
        text: L {
            en: "Another visitor's write, a job's progress, or a deadline changes the screen while the visitor does nothing. A Server Action answers only the tab that called it, so in Next.js these changes need a second path: a stream of their own, and client code that merges what arrives into the state. In hypersig, a stream sends the same `Patches` an action answers with, and the runtime applies both the same way. [Talking to the server](learn/server/) covers streams.",
            ja: "ほかの訪問者の書き込み、ジョブの進み具合、締め切りの到来は、訪問者が何もしなくても画面を変えます。Server Action は呼び出したタブにしか応答しないので、Next.js ではこうした変更に別の通り道が要ります。専用のストリームと、届いた値を状態に取り込むクライアントのコードです。hypersig のストリームはアクションと同じ `Patches` を送り、ランタイムはどちらも同じ処理で反映します。ストリームは [サーバーとのやり取り](learn/server/) で説明しています。",
        },
        request: L {
            en: "Show the tasks other people add, without a reload.",
            ja: "ほかの人が足したタスクも、読み込み直さずに出してください。",
        },
        detail: Detail {
            next: &[
                L {
                    en: "Add a Route Handler such as `app/api/tasks/stream/route.ts` that returns a `ReadableStream` with the `text/event-stream` content type. Keep the open streams in a list, and write to each of them after a Server Action saves a task.",
                    ja: "`app/api/tasks/stream/route.ts` のような Route Handler を足し、`text/event-stream` の `ReadableStream` を返します。開いているストリームを一覧に持ち、Server Action がタスクを保存するたびに、その全部へ書き込みます。",
                },
                L {
                    en: "In the Client Component, open an `EventSource` in `useEffect`, read each event with `JSON.parse`, and put the result into `useState`. `JSON.parse` returns `any`, so the shape of what arrives is not checked.",
                    ja: "Client Component の `useEffect` で `EventSource` を開き、届いたイベントを `JSON.parse` で読んで `useState` に入れます。`JSON.parse` の戻り値は `any` なので、届いた値の形は型で検査されません。",
                },
                L {
                    en: "`EventSource` reconnects by itself, and the changes made while it was away are lost unless the handler sends them again, from the `Last-Event-ID` header or from a fresh read of the list.",
                    ja: "`EventSource` は自分でつなぎ直します。ただ、切れていた間の変更は、ハンドラが `Last-Event-ID` ヘッダーを読むか一覧を読み直して送らない限り失われます。",
                },
            ],
            change: Snippet::UseOthersChanges,
            caption: L {
                en: "The list arrives as a typed patch of `Board::tasks()`. Every connection, a reconnection included, starts with the list as it is now.",
                ja: "一覧は `Board::tasks()` の型の付いた差分として届きます。接続のたびに、つなぎ直したときも含めて、ストリームはその時点の一覧から送ります。",
            },
        },
    },
    Concept {
        kind: Kind::Names,
        text: L {
            en: "A message the server reports under a field, a filter kept in the URL, the status of a failed request, and a translated text each work only while the browser and the server agree on a name. In Next.js, a query key and a translation key are strings, and an error from a Server Action reaches the page through a Client Component that holds the action's state. In hypersig, the name comes from a Rust item that both sides read, so a name that does not match fails to compile. The item is a field of a `Signals` struct, a `Search` struct, a `StatusCode`, or a field of the translation struct. [Forms](learn/forms/) covers the rules both sides check.",
            ja: "欄の下に出すサーバーのエラー、URL に入れる絞り込み、失敗したリクエストのステータス、訳した文言は、どれもブラウザとサーバーが同じ名前を使っている間だけ動きます。Next.js ではクエリのキーも訳のキーも文字列で、Server Action のエラーは、アクションの状態を持つ Client Component を通ってページに届きます。hypersig では、名前は両側が読む Rust の項目から作られます。その項目は `Signals` 構造体のフィールド、`Search` 構造体、`StatusCode`、訳の構造体のフィールドなので、名前が合わなければコンパイルエラーになります。両側が検査するルールは [フォーム](learn/forms/) で説明しています。",
        },
        request: L {
            en: "If the email address is already registered, say so under the email field.",
            ja: "登録済みのメールアドレスなら、メールアドレスの欄の下にそう出してください。",
        },
        detail: Detail {
            next: &[
                L {
                    en: "Make the Server Action return the errors, as in `{ errors: { email?: string } }`, where it only saved and redirected before.",
                    ja: "保存してリダイレクトするだけだった Server Action が、`{ errors: { email?: string } }` のようにエラーを返すようにします。",
                },
                L {
                    en: "Mark the form `\"use client\"` and call the action through `useActionState`, which hands back what the action returned.",
                    ja: "フォームに `\"use client\"` を付け、`useActionState` でアクションを呼びます。`useActionState` がアクションの戻り値を返します。",
                },
                L {
                    en: "Render `state.errors?.email` under the field. Where a library checks the field in the browser, its message lives in another state, and showing both in one place takes code written by hand.",
                    ja: "欄の下に `state.errors?.email` を表示します。ブラウザ側で欄を検査するライブラリを使っていると、そのメッセージは別の状態に入るので、両方を同じ場所に出すには手でつなぎます。",
                },
            ],
            change: Snippet::UseServerError,
            caption: L {
                en: "The browser's rule and the server's check both write into `signup.error(email)`, which the page already shows.",
                ja: "ブラウザのルールもサーバーの検査も、ページがすでに表示している `signup.error(email)` に書き込みます。",
            },
        },
    },
    Concept {
        kind: Kind::Calls,
        text: L {
            en: "A slow answer arrives after a newer one, the connection drops, or the visitor presses the button again while it waits. In Next.js, the component handles these with an `AbortController`, a pending flag, and code that reconnects. In hypersig, each is a method on the call: `.latest()` aborts the earlier call, `.indicator()` sets a flag while the call waits, and `.reconnect()` opens a stream again. [Calls in depth](learn/calls/) covers calls that overlap.",
            ja: "遅い応答が新しい応答のあとに届く、接続が切れる、待っている間にボタンがもう一度押される、という場面です。Next.js では、`AbortController`、処理中のフラグ、つなぎ直す処理をコンポーネントに書きます。hypersig では、どれも呼び出しのメソッドです。`.latest()` は前の呼び出しを止め、`.indicator()` は待っている間フラグを立て、`.reconnect()` はストリームをつなぎ直します。重なった呼び出しは [呼び出しの詳細](learn/calls/) で説明しています。",
        },
        request: L {
            en: "Search as the visitor types, show that it is searching, and never let an older result replace a newer one.",
            ja: "入力に合わせて検索し、検索中であることを出してください。古い結果が新しい結果を上書きしないようにしてください。",
        },
        detail: Detail {
            next: &[
                L {
                    en: "Move the search box into a Client Component, and from the input's change handler, after a timer for the debounce, call a Route Handler with `fetch`.",
                    ja: "検索欄を Client Component に移し、入力が変わったらタイマーで間を置いてから、Route Handler を `fetch` で呼びます。",
                },
                L {
                    en: "Keep the `AbortController` of the request in flight, abort it when the next request starts, and ignore the `AbortError` the aborted one throws.",
                    ja: "送信中のリクエストの `AbortController` を持ち、次のリクエストを送るときに止め、止めたリクエストが投げる `AbortError` を無視します。",
                },
                L {
                    en: "Keep a `searching` flag in `useState`, set it before the request, and clear it after both a success and a failure.",
                    ja: "`useState` に検索中のフラグを持ち、送る前に立て、成功でも失敗でも終わったら下ろします。",
                },
            ],
            change: Snippet::UseLatest,
            caption: L {
                en: "`.latest()` aborts the call still waiting, and drops those of its patches that had not arrived yet.",
                ja: "`.latest()` は待っている呼び出しを止め、その呼び出しのまだ届いていない差分を捨てます。",
            },
        },
    },
];

const CASES: &[Case] = &[
    Case {
        title: L {
            en: "Auction",
            ja: "オークション",
        },
        scene: L {
            en: "Many visitors watch one item, and each bid changes the price on every screen.",
            ja: "大勢が 1 つの品物を見ていて、入札のたびに全員の画面の価格が変わります。",
        },
        before: &[(
            Kind::Push,
            L {
                en: "Show each bid on every open screen as it is placed.",
                ja: "入札をすぐに全員の画面に出してください。",
            },
        )],
        detailed: (
            Kind::Push,
            L {
                en: "Tell each bidder whether they hold the highest bid.",
                ja: "自分が最高額かどうかを、入札者ごとに出してください。",
            },
        ),
        after: &[
            (
                Kind::Names,
                L {
                    en: "Refuse a bid placed after the close, with the reason under the bid field.",
                    ja: "締め切り後の入札は断り、入札欄の下に理由を出してください。",
                },
            ),
            (
                Kind::Calls,
                L {
                    en: "Keep the bid button disabled while a bid is being sent.",
                    ja: "入札を送っている間は、ボタンを押せないようにしてください。",
                },
            ),
        ],
        detail: Detail {
            next: &[
                L {
                    en: "In the stream's Route Handler, read the session with `cookies()` and keep the bidder beside each open stream.",
                    ja: "ストリームの Route Handler で `cookies()` からセッションを読み、開いているストリームごとに入札者を覚えます。",
                },
                L {
                    en: "For each bid, write a different event to each stream, by comparing the bid's bidder with the stream's.",
                    ja: "入札のたびに、入札者とストリームの入札者を比べ、ストリームごとに違うイベントを書きます。",
                },
                L {
                    en: "Add `leading` to the state the `EventSource` handler merges in the Client Component, and render it.",
                    ja: "Client Component では、`EventSource` のハンドラが取り込む状態に `leading` を足し、表示します。",
                },
            ],
            change: Snippet::UseAuction,
            caption: L {
                en: "A stream's handler takes an extractor as an action's does, so the stream of each tab compares the bid with its own member.",
                ja: "ストリームのハンドラも、アクションと同じくエクストラクタを受け取ります。そのため、タブごとのストリームが入札を自分のメンバーと比べます。",
            },
        },
    },
    Case {
        title: L {
            en: "Deploy progress",
            ja: "デプロイの進捗画面",
        },
        scene: L {
            en: "A deploy writes its log for minutes, and the people waiting on it keep the page open.",
            ja: "デプロイは数分かけてログを書き、待っている人はその間ページを開いたままにします。",
        },
        before: &[(
            Kind::Push,
            L {
                en: "Stream each log line as the deploy writes it.",
                ja: "デプロイが書いたログを、1 行ずつ流してください。",
            },
        )],
        detailed: (
            Kind::Calls,
            L {
                en: "After the connection drops, continue from the next line without repeating a line.",
                ja: "接続が切れたら、同じ行を繰り返さずに、続きの行から再開してください。",
            },
        ),
        after: &[(
            Kind::Names,
            L {
                en: "When the session has expired, show a link to sign in again in place of the error.",
                ja: "セッションが切れていたら、エラーの代わりにサインインし直すリンクを出してください。",
            },
        )],
        detail: Detail {
            next: &[
                L {
                    en: "Write an `id:` line with each event, so that `EventSource` sends the last one back in the `Last-Event-ID` header when it reconnects.",
                    ja: "イベントごとに `id:` の行を書きます。`EventSource` はつなぎ直すとき、最後に受け取った値を `Last-Event-ID` ヘッダーで送り返します。",
                },
                L {
                    en: "Read that header in the Route Handler, and start the log after the line it names.",
                    ja: "Route Handler でそのヘッダーを読み、その値が指す行の次からログを送ります。",
                },
            ],
            change: Snippet::UseDeployLog,
            caption: L {
                en: "The id of the last line is a signal the stream's patches keep up to date, and each reconnection sends it as the stream's argument. A response the host closes before the stream ends, as a host does at its time limit, also counts as broken off, and the runtime opens it again.",
                ja: "最後の行の id は、ストリームの差分が更新し続けるシグナルで、つなぎ直すたびにストリームの引数として送られます。時間の上限に達したホストがするように、ストリームが終わる前に応答が閉じられた場合も、ランタイムは途切れたものとしてつなぎ直します。",
            },
        },
    },
    Case {
        title: L {
            en: "Restaurant orders",
            ja: "飲食店の注文と厨房",
        },
        scene: L {
            en: "Staff on the floor send orders from tablets, and a screen in the kitchen lists them.",
            ja: "ホールのスタッフがタブレットから注文を送り、厨房の画面に一覧が出ます。",
        },
        before: &[(
            Kind::Push,
            L {
                en: "Show an order on the kitchen screen as soon as it is sent.",
                ja: "送った注文をすぐに厨房の画面に出してください。",
            },
        )],
        detailed: (
            Kind::Names,
            L {
                en: "If a dish is sold out, say so on that line of the order.",
                ja: "品切れの料理は注文のその行に品切れと出してください。",
            },
        ),
        after: &[],
        detail: Detail {
            next: &[
                L {
                    en: "Make the Server Action return which line failed and why. The Next.js guide to error handling models an expected error as a returned value, and leaves thrown errors to error boundaries.",
                    ja: "Server Action が、どの行がなぜ失敗したかを返すようにします。Next.js のエラー処理のガイドは、想定できるエラーを戻り値で表し、投げたエラーはエラー境界に任せています。",
                },
                L {
                    en: "Keep a map from a line's id to its message in the Client Component's state, set an entry on a failure, and remove it on a success.",
                    ja: "Client Component の状態に、行の id からメッセージへの対応を持ちます。失敗したら入れ、成功したら消します。",
                },
                L {
                    en: "Read each line's message from that map where the line renders.",
                    ja: "各行を描画するところで、その対応から行のメッセージを読みます。",
                },
            ],
            change: Snippet::UseKitchen,
            caption: L {
                en: "`.on_error` runs where the call was made, so inside `each` it reaches the line whose button was pressed.",
                ja: "`.on_error` は呼び出した場所で実行されるので、`each` の中ではボタンを押した行に届きます。",
            },
        },
    },
    Case {
        title: L {
            en: "Seat booking",
            ja: "座席の予約",
        },
        scene: L {
            en: "Visitors pick from one seat map at the same time, and two of them can pick one seat.",
            ja: "訪問者が同じ座席表から同時に席を選ぶので、2 人が同じ席を選ぶことがあります。",
        },
        before: &[(
            Kind::Push,
            L {
                en: "Mark a seat as taken on every screen once someone holds it.",
                ja: "だれかが押さえた席は、すぐ全員の画面で埋まった表示にしてください。",
            },
        )],
        detailed: (
            Kind::Names,
            L {
                en: "If someone else took the seat a moment ago, tell the visitor so.",
                ja: "直前にほかの人が取った席なら、そのことを訪問者に伝えてください。",
            },
        ),
        after: &[],
        detail: Detail {
            next: &[
                L {
                    en: "Return a value that tells the outcomes apart from the Server Action, such as `{ ok: false, reason: \"taken\" }`, as the Next.js guide to error handling does for an expected error.",
                    ja: "Server Action は、Next.js のエラー処理のガイドが想定できるエラーにするのと同じく、`{ ok: false, reason: \"taken\" }` のように結果を区別できる値を返します。",
                },
                L {
                    en: "Compare the reason in the Client Component and render the message for `\"taken\"`. A misspelled reason is caught only where the result has a union type written for it.",
                    ja: "Client Component で理由を比べ、`\"taken\"` のときのメッセージを表示します。理由の打ち間違いが見つかるのは、結果にユニオン型を書いた場合だけです。",
                },
            ],
            change: Snippet::UseSeats,
            caption: L {
                en: "The status of the failed answer goes into a signal, and the page compares it with `StatusCode::CONFLICT`, the status the handler returns.",
                ja: "失敗した応答のステータスはシグナルに入り、ページはそれを、ハンドラが返す `StatusCode::CONFLICT` と比べます。",
            },
        },
    },
    Case {
        title: L {
            en: "Expense approvals",
            ja: "経費の申請と承認",
        },
        scene: L {
            en: "Employees file expenses, and approvers work through the list of the ones waiting.",
            ja: "社員が経費を申請し、承認する人が待っている申請の一覧を順に片付けます。",
        },
        before: &[
            (
                Kind::Push,
                L {
                    en: "When an expense is approved, change its status on the screen of the person who filed it.",
                    ja: "承認されたら、申請した人の画面の状態を変えてください。",
                },
            ),
            (
                Kind::Names,
                L {
                    en: "Check the budget on the server, and show an overrun under the amount field.",
                    ja: "予算の超過をサーバーで判定し、金額の欄の下に出してください。",
                },
            ),
        ],
        detailed: (
            Kind::Names,
            L {
                en: "Keep the list's filter in the URL, so a link to it can be shared.",
                ja: "一覧の絞り込みを URL に入れて、リンクで共有できるようにしてください。",
            },
        ),
        after: &[],
        detail: Detail {
            next: &[
                L {
                    en: "Read the filter from the page's `searchParams`, whose values arrive as `string | string[] | undefined`. Parse the page number, and check the status against the ones that exist.",
                    ja: "ページの `searchParams` から絞り込みを読みます。値は `string | string[] | undefined` で届くので、ページ番号をパースし、状態の値がありうる値か確かめます。",
                },
                L {
                    en: "Build each link's query with `URLSearchParams` or a template string. A misspelled key builds a link that ignores the filter, with no error.",
                    ja: "リンクのクエリは `URLSearchParams` か文字列で組み立てます。キーを打ち間違えると、絞り込みが効かないリンクができ、エラーは出ません。",
                },
            ],
            change: Snippet::UseUrlFilter,
            caption: L {
                en: "The handler receives a `Filter`, and a link passes one. A field that is `None` stays out of the link, and a missing key arrives as `None`, so the bare `/requests` loads where a required field would get a 400. `Route::link` exists only on a route without parameters, so once the route takes a filter, each link that passes none fails to compile.",
                ja: "ハンドラは `Filter` を受け取り、リンクは `Filter` を渡します。`None` のフィールドはリンクに入らず、クエリにないキーは `None` として届くので、クエリのない `/requests` も読み込めます。フィールドが必須なら 400 が返るところです。`Route::link` はパラメータのないルートにしかないので、ルートが絞り込みを受け取るようになると、絞り込みを渡していないリンクはどれもコンパイルエラーになります。",
            },
        },
    },
    Case {
        title: L {
            en: "An online shop in two languages",
            ja: "2 か国語の EC サイト",
        },
        scene: L {
            en: "Visitors search for products as they type and filter the results, and the pages come in English and Japanese.",
            ja: "訪問者は入力しながら商品を探して絞り込み、ページは英語と日本語で出ます。",
        },
        before: &[
            (
                Kind::Calls,
                L {
                    en: "Search as the visitor types, and never show an older result over a newer one.",
                    ja: "入力しながら検索し、古い結果で新しい結果を上書きしないでください。",
                },
            ),
            (
                Kind::Names,
                L {
                    en: "Keep the filters in the URL.",
                    ja: "絞り込みを URL に入れてください。",
                },
            ),
        ],
        detailed: (
            Kind::Names,
            L {
                en: "Add an English version of each page.",
                ja: "各ページに英語版を足してください。",
            },
        ),
        after: &[],
        detail: Detail {
            next: &[
                L {
                    en: "With a library such as next-intl, move each text to a key in `messages/en.json` and `messages/ja.json`, and call `t(\"cart.title\")` where the text was.",
                    ja: "next-intl のようなライブラリを使い、文言をキーにして `messages/en.json` と `messages/ja.json` に移し、元の場所で `t(\"cart.title\")` を呼びます。",
                },
                L {
                    en: "Put the pages under a `[locale]` segment, and pick the language for a visitor who opens `/`.",
                    ja: "ページを `[locale]` のセグメントの下に移し、`/` を開いた訪問者の言語を選ぶ処理を足します。",
                },
                L {
                    en: "Unless the project's types check every language's file, a key missing from one file is not a type error, and shows up as a missing text at run time.",
                    ja: "型の設定で言語ごとのファイルをすべて検査していなければ、1 つのファイルに足りないキーは型エラーにならず、実行時に訳のない文言として出ます。",
                },
            ],
            change: Snippet::UseTranslations,
            caption: L {
                en: "A text holds its translations in one struct, so a text without its English fails to compile, and a language added to `Lang` fails to compile until every text has it. [i18n](i18n/) covers the crate.",
                ja: "文言は訳を 1 つの構造体に持つので、英語のない文言はコンパイルエラーになります。`Lang` に言語を足すと、すべての文言に訳が入るまでコンパイルが通りません。クレートは [多言語化](i18n/) で説明しています。",
            },
        },
    },
];

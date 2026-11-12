//! Messages that open one context menu, for the context menu page.

use hypersig::html::{Role, div, each, li, p, ul};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::{BorderStyle, px, raw, rem};
use hypersig::{Expr, Page, Sig, Signals};
use hypersig_ui::{ContextMenu, MenuItem, token};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize, Clone, PartialEq)]
pub(super) struct Message {
    author: String,
    text: String,
}

#[derive(Signals, Serialize, Deserialize)]
pub(super) struct Chat {
    messages: Vec<Message>,
    /// The author of the message whose menu is open.
    picked: String,
    status: String,
}

hypersig::node_ref!(const MESSAGE_ACTIONS: NodeRef<Chat>);
const MESSAGE_MENU: ContextMenu<Chat> = ContextMenu::new(MESSAGE_ACTIONS);

/// `before`, the author `picked` names, and `after`, which English and
/// Japanese put on different sides of the name.
fn said(site: Site, picked: Sig<Chat, String>, before: L<&str>, after: L<&str>) -> Expr<Chat, String> {
    Expr::lit(&site.t(before).to_owned()).concat(picked).concat(site.t(after))
}

pub(super) fn page(site: Site) -> Page<Chat> {
    let picked = Chat::picked();
    let status = Chat::status();
    let message = |author: &str, text: L<&str>| Message {
        author: author.to_owned(),
        text: site.t(text).to_owned(),
    };
    let initial = Chat {
        messages: vec![
            message(
                "Ada",
                L {
                    en: "The build is green again.",
                    ja: "ビルドがまた通るようになりました。",
                },
            ),
            message(
                "Grace",
                L {
                    en: "Thanks, I'll tag the release.",
                    ja: "ありがとう。リリースのタグを付けます。",
                },
            ),
        ],
        picked: String::new(),
        status: String::new(),
    };
    Page::new(
        site.t(L {
            en: "Chat",
            ja: "チャット",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(p().text(site.t(L {
                en: "Right-click a message.",
                ja: "メッセージを右クリックしてください。",
            })))
            .child(
                ul().css(stack(rem(0.5)).padding(px(0)).margin(px(0)))
                    .child(each(Chat::messages(), |message| {
                        li().css(
                            Style::new()
                                .list_style_type(raw("none"))
                                .padding(rem(0.5))
                                .border_width(px(1))
                                .border_style(BorderStyle::Solid)
                                .border_color(token::border().var())
                                .border_radius(px(6)),
                        )
                        .bind_text(message.at(Message::author()).concat(": ").concat(message.at(Message::text())))
                        .on_context_menu(move |at| {
                            picked
                                .set(message.at(Message::author()))
                                .then(MESSAGE_MENU.open_at(at))
                        })
                    })),
            )
            .child(MESSAGE_MENU.menu(
                site.t(L {
                    en: "Message",
                    ja: "メッセージ",
                }),
                [
                    MenuItem::new(
                        site.t(L {
                            en: "Reply",
                            ja: "返信",
                        }),
                        status.set(said(site, picked, L { en: "Replying to ", ja: "" }, L { en: ".", ja: " さんに返信します。" })),
                    ),
                    MenuItem::new(
                        site.t(L {
                            en: "Delete",
                            ja: "削除",
                        }),
                        status.set(said(
                            site,
                            picked,
                            L { en: "Deleted the message from ", ja: "" },
                            L { en: ".", ja: " さんのメッセージを削除しました。" },
                        )),
                    )
                    .destructive(),
                ],
            ))
            .child(p().role(Role::Status).bind_text(status)),
    )
}

//! A text field with a description and an error, for the text field page.

use hypersig::html::div;
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals, if_else};
use hypersig_ui::{Autocomplete, InputKind, TextField};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Signup {
    email: String,
    bio: String,
}

pub fn page(site: Site) -> Page<Signup> {
    let email = Signup::email();
    let looks_wrong = !email.is_empty().or(email.contains("@"));
    let initial = Signup {
        email: String::new(),
        bio: String::new(),
    };
    Page::new(
        site.t(L {
            en: "Sign up",
            ja: "登録",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1.25)).max_width(rem(24)))
            .child(
                TextField::new(
                    site.t(L {
                        en: "Email",
                        ja: "メールアドレス",
                    }),
                    email,
                )
                .kind(InputKind::Email)
                .autocomplete(Autocomplete::Email)
                .placeholder("ada@example.com")
                .description(site.t(L {
                    en: "We send the receipt here.",
                    ja: "領収書をこのアドレスに送ります。",
                }))
                .error(if_else(
                    looks_wrong,
                    site.t(L {
                        en: "Enter an email address.",
                        ja: "メールアドレスを入力してください。",
                    }),
                    "",
                ))
                .required(),
            )
            .child(
                TextField::new(
                    site.t(L {
                        en: "Bio",
                        ja: "自己紹介",
                    }),
                    Signup::bio(),
                )
                .multiline()
                .description(site.t(L {
                    en: "A sentence or two for your profile.",
                    ja: "プロフィールに載せる 1、2 文です。",
                })),
            ),
    )
}

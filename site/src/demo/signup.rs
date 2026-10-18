//! A sign-up form checked by its rules as the visitor types, for the page on
//! forms.

use hypersig::html::{div, p};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::rem;
use hypersig::{Form, Page, Rules, Sig, Signals, Validate};
use hypersig_ui::{
    Autocomplete, ButtonSize, ButtonVariant, InputKind, TextField, button, submit_button, token,
};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
pub struct Signup {
    email: String,
    password: String,
    confirm: String,
}

impl Validate for Signup {
    fn rules<S>(form: Sig<S, Self>, rules: &mut Rules<S, Self>) {
        let site = Site::rendering();
        let email = form.at(Signup::email());
        let password = form.at(Signup::password());
        rules
            .check(
                Signup::email(),
                email.contains("@"),
                site.t(L {
                    en: "Enter an email address.",
                    ja: "メールアドレスを入力してください。",
                }),
            )
            .check(
                Signup::password(),
                password.char_count().ge(8),
                site.t(L {
                    en: "Use 8 or more characters.",
                    ja: "8 文字以上にしてください。",
                }),
            )
            .check(
                Signup::confirm(),
                form.at(Signup::confirm()).equals(password),
                site.t(L {
                    en: "The passwords differ.",
                    ja: "パスワードが一致しません。",
                }),
            );
    }
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Join {
    signup: Form<Signup>,
    joined: bool,
}

pub fn page(site: Site) -> Page<Join> {
    let signup = Join::signup();
    let joined = Join::joined();
    let initial = Join {
        signup: Form::new(Signup::default()),
        joined: false,
    };
    Page::new(
        site.t(L {
            en: "Sign up",
            ja: "登録",
        }),
        initial,
    )
    .child(
        signup
            // The site has no server, so a valid submit sets a signal where
            // an application would call an action.
            .form(joined.set(true).then(signup.reset()))
            .css(stack(rem(1.25)).max_width(rem(24)))
            .child(
                TextField::in_form(
                    site.t(L {
                        en: "Email",
                        ja: "メールアドレス",
                    }),
                    signup,
                    Signup::email(),
                )
                .kind(InputKind::Email)
                .autocomplete(Autocomplete::Email),
            )
            .child(
                TextField::in_form(
                    site.t(L {
                        en: "Password",
                        ja: "パスワード",
                    }),
                    signup,
                    Signup::password(),
                )
                .kind(InputKind::Password)
                .autocomplete(Autocomplete::NewPassword),
            )
            .child(
                TextField::in_form(
                    site.t(L {
                        en: "Confirm password",
                        ja: "パスワードの確認",
                    }),
                    signup,
                    Signup::confirm(),
                )
                .kind(InputKind::Password)
                .autocomplete(Autocomplete::NewPassword),
            )
            .child(
                div()
                    .css(hstack(rem(0.75)))
                    .child(
                        submit_button(ButtonVariant::Default, ButtonSize::Default).text(site.t(
                            L {
                                en: "Create account",
                                ja: "アカウントを作成",
                            },
                        )),
                    )
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Default)
                            .show(signup.is_dirty())
                            .on_click(signup.reset())
                            .text(site.t(L {
                                en: "Clear",
                                ja: "クリア",
                            })),
                    ),
            )
            .child(
                p().attr("role", "status")
                    .css(Style::new().color(token::muted_foreground().var()))
                    .show(joined.and(!signup.is_dirty()))
                    .text(site.t(L {
                        en: "Every rule passed.",
                        ja: "すべてのルールを通りました。",
                    })),
            ),
    )
}

//! A sign-up form checked by its rules as the visitor types, with a select
//! and a checkbox beside the text fields, for the page on forms.

use hypersig::html::{div, p, attr};
use hypersig::style::Style;
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::rem;
use hypersig::{Form, Page, Rules, Sig, Signals, Validate};
use hypersig_ui::{
    Autocomplete, ButtonSize, ButtonVariant, InputKind, TextField, button, checkbox_in_form,
    select_field_in_form, submit_button, token,
};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Default)]
pub enum Plan {
    #[default]
    Unset,
    Free,
    Pro,
}

#[derive(Signals, Serialize, Deserialize, Clone, Default)]
pub struct Signup {
    email: String,
    password: String,
    confirm: String,
    plan: Plan,
    terms: bool,
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
            )
            .check(
                Signup::plan(),
                !form.at(Signup::plan()).equals(&Plan::Unset),
                site.t(L {
                    en: "Choose a plan.",
                    ja: "プランを選んでください。",
                }),
            )
            .check(
                Signup::terms(),
                form.at(Signup::terms()),
                site.t(L {
                    en: "Accept the terms to continue.",
                    ja: "続けるには規約に同意してください。",
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
            .child(select_field_in_form(
                site.t(L {
                    en: "Plan",
                    ja: "プラン",
                }),
                signup,
                Signup::plan(),
                [
                    (
                        Plan::Unset,
                        site.t(L {
                            en: "Choose…",
                            ja: "選んでください",
                        }),
                    ),
                    (Plan::Free, "Free"),
                    (Plan::Pro, "Pro"),
                ],
            ))
            .child(checkbox_in_form(
                site.t(L {
                    en: "I accept the terms",
                    ja: "規約に同意します",
                }),
                signup,
                Signup::terms(),
            ))
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
                p().attr(attr::ROLE, "status")
                    .css(Style::new().color(token::muted_foreground().var()))
                    .show(joined.and(!signup.is_dirty()))
                    .text(site.t(L {
                        en: "Every rule passed.",
                        ja: "すべてのルールを通りました。",
                    })),
            ),
    )
}

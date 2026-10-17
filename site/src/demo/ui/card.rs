//! A card holding a small form, for the card page.

use hypersig::html::{div, form};
use hypersig::style::Style;
use hypersig::style::pattern::stack;
use hypersig::style::value::{JustifyContent, rem};
use hypersig::{Page, Signals};
use hypersig_ui::{
    ButtonSize, ButtonVariant, Level, TextField, button, card, card_content, card_description,
    card_footer, card_header, card_title, submit_button,
};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct NewProject {
    name: String,
    created: i32,
}

pub fn page(site: Site) -> Page<NewProject> {
    let created = NewProject::created();
    let initial = NewProject {
        name: String::new(),
        created: 0,
    };
    Page::new(
        site.t(L {
            en: "New project",
            ja: "新しいプロジェクト",
        }),
        initial,
    )
    .child(
        card().css(Style::new().max_width(rem(24))).child(
            form()
                .on_submit(created.set(created + 1).then(NewProject::name().set("")))
                .css(stack(rem(1.5)))
                .child(
                    card_header()
                        .child(card_title(Level::H2).text(site.t(L {
                            en: "Create project",
                            ja: "プロジェクトを作成",
                        })))
                        .child(card_description().text(site.t(L {
                            en: "Deploy a new project in one click.",
                            ja: "新しいプロジェクトを 1 クリックでデプロイします。",
                        }))),
                )
                .child(card_content().child(TextField::new(
                    site.t(L {
                        en: "Name",
                        ja: "名前",
                    }),
                    NewProject::name(),
                )))
                .child(
                    card_footer()
                        .css(Style::new().justify_content(JustifyContent::SpaceBetween))
                        .child(
                            button(ButtonVariant::Outline, ButtonSize::Default).text(site.t(L {
                                en: "Cancel",
                                ja: "キャンセル",
                            })),
                        )
                        .child(div().child(
                            submit_button(ButtonVariant::Default, ButtonSize::Default).text(
                                site.t(L {
                                    en: "Create",
                                    ja: "作成",
                                }),
                            ),
                        )),
                ),
        ),
    )
}

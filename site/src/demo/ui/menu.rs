//! A menu button, for the menu page.

use hypersig::html::{div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Menu, MenuItem};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Project {
    archived: bool,
    status: String,
}

const ACTIONS: Menu<Project> = Menu::new("actions");

pub fn page(site: Site) -> Page<Project> {
    let archived = Project::archived();
    let status = Project::status();
    let initial = Project {
        archived: false,
        status: String::new(),
    };
    Page::new(
        site.t(L {
            en: "Project",
            ja: "プロジェクト",
        }),
        initial,
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div()
                    .child(
                        ACTIONS
                            .trigger(ButtonVariant::Outline, ButtonSize::Default)
                            .text(site.t(L {
                                en: "Project actions",
                                ja: "プロジェクトの操作",
                            })),
                    )
                    .child(
                        ACTIONS.menu([
                            MenuItem::new(
                                site.t(L {
                                    en: "Rename",
                                    ja: "名前を変更",
                                }),
                                status.set(site.t(L {
                                    en: "Renamed the project.",
                                    ja: "プロジェクトの名前を変更しました。",
                                })),
                            ),
                            MenuItem::new(
                                site.t(L {
                                    en: "Duplicate",
                                    ja: "複製",
                                }),
                                status.set(site.t(L {
                                    en: "Duplicated the project.",
                                    ja: "プロジェクトを複製しました。",
                                })),
                            ),
                            MenuItem::new(
                                site.t(L {
                                    en: "Archive",
                                    ja: "アーカイブ",
                                }),
                                archived.set(true).then(status.set(site.t(L {
                                    en: "Archived the project.",
                                    ja: "プロジェクトをアーカイブしました。",
                                }))),
                            )
                            .disabled(archived),
                            MenuItem::new(
                                site.t(L {
                                    en: "Delete",
                                    ja: "削除",
                                }),
                                status.set(site.t(L {
                                    en: "Deleted the project.",
                                    ja: "プロジェクトを削除しました。",
                                })),
                            )
                            .destructive(),
                        ]),
                    ),
            )
            .child(p().attr("role", "status").bind_text(status)),
    )
}

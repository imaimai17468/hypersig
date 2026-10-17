//! A dialog and an alert dialog, for the dialog page.

use hypersig::html::{div, p};
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, TextField, dialog_footer};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Project {
    name: String,
    status: String,
}

const RENAME: Dialog<Project> = Dialog::new("rename");
const DELETE: Dialog<Project> = Dialog::new("delete");

pub fn page(site: Site) -> Page<Project> {
    let name = Project::name();
    let status = Project::status();
    let initial = Project {
        name: "hypersig".into(),
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
                    .css(wrap(rem(0.5)))
                    .child(
                        RENAME
                            .trigger(ButtonVariant::Outline, ButtonSize::Default)
                            .text(site.t(L {
                                en: "Rename",
                                ja: "名前を変更",
                            })),
                    )
                    .child(
                        DELETE
                            .trigger(ButtonVariant::Destructive, ButtonSize::Default)
                            .text(site.t(L {
                                en: "Delete",
                                ja: "削除",
                            })),
                    ),
            )
            .child(p().attr("role", "status").bind_text(status))
            .child(
                RENAME.dialog(
                    site.t(L {
                        en: "Rename project",
                        ja: "プロジェクトの名前を変更",
                    }),
                    site.t(L {
                        en: "The new name shows everywhere the project appears.",
                        ja: "新しい名前は、プロジェクトが表示されるすべての場所に反映されます。",
                    }),
                    div()
                        .css(stack(rem(1)))
                        .child(TextField::new(
                            site.t(L {
                                en: "Name",
                                ja: "名前",
                            }),
                            name,
                        ))
                        .child(
                            dialog_footer()
                                .child(
                                    RENAME
                                        .close(ButtonVariant::Outline, ButtonSize::Default)
                                        .text(site.t(L {
                                            en: "Cancel",
                                            ja: "キャンセル",
                                        })),
                                )
                                .child(
                                    RENAME
                                        .close(ButtonVariant::Default, ButtonSize::Default)
                                        .on_click(status.set(site.t(L {
                                            en: "Renamed the project.",
                                            ja: "プロジェクトの名前を変更しました。",
                                        })))
                                        .text(site.t(L {
                                            en: "Save",
                                            ja: "保存",
                                        })),
                                ),
                        ),
                ),
            )
            .child(
                DELETE.alert_dialog(
                    site.t(L {
                        en: "Delete this project?",
                        ja: "このプロジェクトを削除しますか？",
                    }),
                    site.t(L {
                        en: "The project and its deployments are removed. This cannot be undone.",
                        ja: "プロジェクトとそのデプロイが削除されます。この操作は取り消せません。",
                    }),
                    dialog_footer()
                        .child(
                            DELETE
                                .close(ButtonVariant::Outline, ButtonSize::Default)
                                .text(site.t(L {
                                    en: "Cancel",
                                    ja: "キャンセル",
                                })),
                        )
                        .child(
                            DELETE
                                .close(ButtonVariant::Destructive, ButtonSize::Default)
                                .on_click(status.set(site.t(L {
                                    en: "Deleted the project.",
                                    ja: "プロジェクトを削除しました。",
                                })))
                                .text(site.t(L {
                                    en: "Delete",
                                    ja: "削除",
                                })),
                        ),
                ),
            ),
    )
}

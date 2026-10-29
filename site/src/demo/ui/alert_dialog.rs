//! An alert dialog that asks before deleting, for the alert dialog page.

use hypersig::html::{Role, div, p};
use hypersig::style::pattern::stack;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, dialog_footer};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Project {
    status: String,
}

hypersig::dialog_ref!(const DELETE_DIALOG: DialogRef<Project>);
const DELETE: Dialog<Project> = Dialog::new(DELETE_DIALOG);

pub fn page(site: Site) -> Page<Project> {
    let status = Project::status();
    Page::new(
        site.t(L {
            en: "Project",
            ja: "プロジェクト",
        }),
        Project {
            status: String::new(),
        },
    )
    .child(
        div()
            .css(stack(rem(1)))
            .child(
                div().child(
                    DELETE
                        .trigger(ButtonVariant::Destructive, ButtonSize::Default)
                        .text(site.t(L {
                            en: "Delete",
                            ja: "削除",
                        })),
                ),
            )
            .child(p().role(Role::Status).bind_text(status))
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

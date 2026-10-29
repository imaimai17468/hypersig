//! Sheets along the right and the bottom edges, for the sheet page.

use hypersig::html::div;
use hypersig::style::pattern::{stack, wrap};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Dialog, DialogText, Side, checkbox, dialog_footer};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Project {
    this_week: bool,
}

hypersig::dialog_ref!(const DETAILS_SHEET: DialogRef<Project>);
hypersig::dialog_ref!(const FILTERS_SHEET: DialogRef<Project>);
const DETAILS: Dialog<Project> = Dialog::new(DETAILS_SHEET);
const FILTERS: Dialog<Project> = Dialog::new(FILTERS_SHEET);

pub fn page(site: Site) -> Page<Project> {
    let text = DialogText {
        close: site
            .t(L {
                en: "Close",
                ja: "閉じる",
            })
            .into(),
    };
    let done = site.t(L {
        en: "Done",
        ja: "完了",
    });
    Page::new(
        site.t(L {
            en: "Project",
            ja: "プロジェクト",
        }),
        Project { this_week: false },
    )
    .child(
        div()
            .css(wrap(rem(0.5)))
            .child(
                DETAILS
                    .trigger(ButtonVariant::Outline, ButtonSize::Default)
                    .text(site.t(L {
                        en: "Details",
                        ja: "詳細",
                    })),
            )
            .child(
                FILTERS
                    .trigger(ButtonVariant::Outline, ButtonSize::Default)
                    .text(site.t(L {
                        en: "Filters",
                        ja: "絞り込み",
                    })),
            ),
    )
    .child(
        DETAILS.sheet(
            Side::Right,
            site.t(L {
                en: "Project details",
                ja: "プロジェクトの詳細",
            }),
            site.t(L {
                en: "Created on March 3. Deployed 12 times.",
                ja: "3 月 3 日に作成し、12 回デプロイしました。",
            }),
            dialog_footer().child(
                DETAILS
                    .close(ButtonVariant::Outline, ButtonSize::Default)
                    .text(done),
            ),
            &text,
        ),
    )
    .child(
        FILTERS.sheet(
            Side::Bottom,
            site.t(L {
                en: "Filters",
                ja: "絞り込み",
            }),
            site.t(L {
                en: "Choose which projects the list shows.",
                ja: "一覧に表示するプロジェクトを選びます。",
            }),
            div()
                .css(stack(rem(1)))
                .child(checkbox(
                    site.t(L {
                        en: "Deployed this week",
                        ja: "今週デプロイしたもの",
                    }),
                    Project::this_week(),
                ))
                .child(
                    dialog_footer().child(
                        FILTERS
                            .close(ButtonVariant::Outline, ButtonSize::Default)
                            .text(done),
                    ),
                ),
            &text,
        ),
    )
}

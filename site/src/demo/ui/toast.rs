//! Toasts pushed from the browser, for the toast page.

use hypersig::html::div;
use hypersig::style::pattern::wrap;
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Toast, ToastVariant, Toaster, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct App {
    toasts: Vec<Toast>,
}

pub fn page(site: Site) -> Page<App> {
    let toasts = App::toasts();
    let initial = App { toasts: Vec::new() };
    Page::new(
        site.t(L {
            en: "App",
            ja: "アプリ",
        }),
        initial,
    )
    .child(
        div()
            .css(wrap(rem(0.5)))
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(
                        Toast::new(site.t(L {
                            en: "Event created",
                            ja: "予定を作成しました",
                        }))
                        .description(site.t(L {
                            en: "Sunday, December 3 at 9:00",
                            ja: "12 月 3 日（日）9:00",
                        }))
                        .push(toasts),
                    )
                    .text(site.t(L {
                        en: "Show a toast",
                        ja: "トーストを表示",
                    })),
            )
            .child(
                button(ButtonVariant::Outline, ButtonSize::Default)
                    .on_click(
                        Toast::new(site.t(L {
                            en: "Upload failed",
                            ja: "アップロードに失敗しました",
                        }))
                        .description(site.t(L {
                            en: "The file is larger than 10 MB.",
                            ja: "ファイルが 10 MB を超えています。",
                        }))
                        .variant(ToastVariant::Destructive)
                        .push(toasts),

                    )
                    .text(site.t(L {
                        en: "Show an error",
                        ja: "エラーを表示",
                    })),
            )
            .child(
                Toaster::new(toasts)
                    .label(site.t(L {
                        en: "Notifications",
                        ja: "通知",
                    }))
                    .dismiss_label(site.t(L {
                        en: "Dismiss",
                        ja: "閉じる",
                    })),
            ),
    )
}

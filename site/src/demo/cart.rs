//! A cart line whose total is a computed signal, for the page on signals.

use hypersig::html::{div, p, span, attr};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::rem;
use hypersig::{Page, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, button};
use serde::{Deserialize, Serialize};

use crate::Site;
use crate::lang::L;

#[derive(Signals, Serialize, Deserialize)]
pub struct Item {
    name: String,
    price: f64,
}

#[derive(Signals, Serialize, Deserialize)]
pub struct Cart {
    item: Item,
    quantity: i32,
    total: f64,
}

pub fn page(site: Site) -> Page<Cart> {
    let item = Cart::item();
    let quantity = Cart::quantity();
    let total = Cart::total();
    let initial = Cart {
        item: Item {
            name: site
                .t(L {
                    en: "Green tea",
                    ja: "緑茶",
                })
                .into(),
            price: 4.5,
        },
        quantity: 1,
        total: 0.0,
    };
    Page::new(
        site.t(L {
            en: "Cart",
            ja: "カート",
        }),
        initial,
    )
    .computed(total, item.at(Item::price()) * quantity.to_f64())
    .child(
        div()
            .css(stack(rem(0.75)))
            .child(
                p().child(span().bind_text(item.at(Item::name())))
                    .text(site.t(L {
                        en: " at $",
                        ja: "、単価 $",
                    }))
                    .child(span().bind_text(item.at(Item::price()).to_fixed(2))),
            )
            .child(
                div()
                    .css(hstack(rem(0.75)))
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm)
                            .attr_when(attr::DISABLED, quantity.le(1))
                            .on_click(quantity.set(quantity - 1))
                            .text("-"),
                    )
                    .child(span().bind_text(quantity.to_text()))
                    .child(
                        button(ButtonVariant::Outline, ButtonSize::Sm)
                            .on_click(quantity.set(quantity + 1))
                            .text("+"),
                    ),
            )
            .child(
                p().text(site.t(L {
                    en: "Total: $",
                    ja: "合計 $",
                }))
                .child(span().bind_text(total.to_fixed(2))),
            ),
    )
}

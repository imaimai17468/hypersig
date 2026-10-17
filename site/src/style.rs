//! The Style section: typed atomic CSS from `hypersig::style`.

use crate::Site;
use crate::demo::{self, Demo};
use crate::doc::{Doc, StylePage};
use crate::learn::snippet::{Snippet, show};
use crate::prose::{Part, bullets, heading, lead, note, para, parts, table_of, title};

/// The article of `page`.
pub fn content(site: Site, page: StylePage) -> Part {
    let head = title(Doc::Style(page).title());
    match page {
        StylePage::Overview => overview(site, head),
        StylePage::Values => values(site, head),
        StylePage::Conditions => conditions(site, head),
        StylePage::Tokens => tokens(site, head),
        StylePage::Global => global(site, head),
        StylePage::Patterns => patterns(site, head),
        StylePage::Animation => animation(site, head),
        StylePage::Unchecked => unchecked(site, head),
    }
}

fn overview(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "`hypersig::style` is typed atomic CSS. A misspelled property or keyword, or a value of the wrong type, is a compile error, and each declaration becomes one class.",
        ),
        heading("A style is a Rust value"),
        para(
            site,
            "[`Style`](api:hypersig_style/struct.Style.html) has one method per standard CSS property, and per anchor positioning property. The property list, each property's keywords, the pseudo-classes, the pseudo-elements, and the named colors are generated from MDN's CSS data (`mdn-data` 2.37.2). `El::css` adds a style to an element.",
        ),
        show(Snippet::StyleBasics),
        para(
            site,
            "A later call for the same property under the same conditions replaces the earlier value, as it would later in a rule. `merge` adds the declarations of another style, which win over the first one's.",
        ),
        heading("One class per declaration"),
        para(
            site,
            "Each declaration becomes one class named after it, such as `cursor_pointer` or `hover:color_crimson`, as in the atomic CSS of Panda CSS and Tailwind CSS. A value with characters other than letters, digits, `.`, `#`, `%`, and `-` is named by a hash of it. Two elements with the same declaration share the class, so the stylesheet grows with the number of distinct declarations, not with the number of elements.",
        ),
        para(
            site,
            "The page collects the styles its elements use and writes each rule once into `<head>`. A patch that brings new elements brings their rules, placed so that a breakpoint still overrides the base value.",
        ),
        heading("Styles that follow a signal"),
        para(
            site,
            "`css_when(style, cond)` adds a style while a signal condition holds, and `css_match(value, cases)` picks one style per value of a signal, the variants of a Panda CSS recipe. Their rules sit in a later cascade layer than those of `css`, so while switched on they override the element's own declarations for the same property.",
        ),
        table_of(
            site,
            &["Cascade layer", "Holds"],
            &[
                &["`hypersig-tokens`", "The values a `Theme` sets on `:root`"],
                &[
                    "`hypersig-base`",
                    "The rules of a `Global`, such as the reset",
                ],
                &["`hypersig-atoms`", "The classes of `El::css`"],
                &[
                    "`hypersig-toggled`",
                    "The classes of `El::css_when` and `El::css_match`",
                ],
            ],
        ),
        para(
            site,
            "The layers come in that order, so an element's classes override the global rules, and a switched-on style overrides both. A stylesheet added with `Page::style` sits outside any layer and overrides all four.",
        ),
    ])
}

fn values(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "Each property method takes its own value type, which lists the property's keywords and accepts the units it allows.",
        ),
        heading("Typed values"),
        para(
            site,
            "Where a property accepts one keyword or one value of a type in [`value`](api:hypersig_style/value/index.html), that value is typed. `display` takes a `Display`, whose variants are its keywords. `width` takes a `Width`, which converts from a length or a percentage.",
        ),
        show(Snippet::StyleValues),
        table_of(
            site,
            &["Value", "Written"],
            &[
                &["Length", "`px(1)`, `rem(1.5)`, and the other units"],
                &["Percentage", "`pct(50)`"],
                &[
                    "Time and angle",
                    "`ms(150)`, `secs(1)`, `deg(90)`, `turn(0.5)`",
                ],
                &[
                    "Color",
                    "`Color::CRIMSON` and the other named colors, `Color::hex`, `Color::rgb`, `opacity`, `mix`, `Color::CURRENT_COLOR`, `Color::TRANSPARENT`",
                ],
                &[
                    "Text and URLs",
                    "`quoted(\"→\")` escapes the text, and `url(\"/a.png\")` writes `url(\"...\")`",
                ],
                &[
                    "Every property",
                    "`keyword(Property::Color, CssWideKeyword::Inherit)` for `inherit`, `initial`, `unset`, `revert`, and `revert-layer`",
                ],
            ],
        ),
        heading("What does not compile"),
        para(
            site,
            "A misspelled property has no method, a misspelled keyword has no variant, and a value of the wrong type has no conversion:",
        ),
        show(Snippet::WrongValue),
        note(
            site,
            "Ranges are not checked, so a negative width compiles and the browser drops it. A value with several parts, such as `margin: 0 auto`, goes through `raw`, which [Unchecked values](style/unchecked/) covers.",
        ),
    ])
}

fn conditions(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "`Style::on` applies part of a style under a condition: a pseudo-class, a media feature, a container width, or the state of a marked ancestor or sibling.",
        ),
        heading("Pseudo-classes, media, and containers"),
        show(Snippet::Conditions),
        bullets(
            site,
            &[
                "[`PseudoClass`](api:hypersig_style/enum.PseudoClass.html) and [`PseudoElement`](api:hypersig_style/enum.PseudoElement.html) are generated from MDN's data. `pseudo` puts declarations on a pseudo-element, and a pseudo-class inside it applies to the element, so `pseudo(Before, Style::new().on(Hover, ..))` is written `:hover::before`.",
                "The user-action pseudo-classes are written in a fixed order, so `:active` wins over `:hover` while a button is pressed, and `:disabled` wins over all of them.",
                "[`Media`](api:hypersig_style/enum.Media.html) has the breakpoints `Sm`, `Md`, `Lg`, `Xl`, and `Xxl`, min-width queries at 40, 48, 64, 80, and 96rem, the widths Tailwind CSS and Panda CSS use. It also has `Dark`, `Light`, `ReducedMotion`, `Print`, `Landscape`, `Portrait`, `CanHover`, `CoarsePointer`, `ForcedColors`, and `MoreContrast`.",
                "[`Container`](api:hypersig_style/enum.Container.html) has Panda CSS's container sizes from 16rem to 90rem. An ancestor becomes a query container with `container_type`, and without one the condition never matches.",
                "Conditions nest: a style passed to `on` may use `on` itself, and its declarations then apply under both.",
            ],
        ),
        heading("Group and peer"),
        para(
            site,
            "`Style::group` marks an ancestor, and `Condition::Group(PseudoClass::Hover)` applies a style while that ancestor is hovered. `Style::peer` marks an earlier sibling, and `Condition::Peer(PseudoClass::Checked)` applies a style while that sibling is checked. They work like the `group` and `peer` of Tailwind CSS and Panda CSS.",
        ),
        show(Snippet::GroupPeer),
        demo::show(site, Demo::Hover),
    ])
}

fn tokens(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "A token is a named value written as a CSS custom property, which a theme sets for the page and a style sets for one element.",
        ),
        heading("Tokens"),
        para(
            site,
            "[`Token::new(name, default)`](api:hypersig_style/struct.Token.html) creates a token of a value type: `Color`, `Length`, `Percentage`, `Time`, or `Angle`. `var()` reads it as `var(--name, default)`, so a page that sets nothing shows the default, and it is accepted wherever a value of that type is.",
        ),
        show(Snippet::Tokens),
        bullets(
            site,
            &[
                "`Page::theme` takes a [`Theme`](api:hypersig_style/struct.Theme.html), whose values are written on `:root`. `Theme::on(Media::Dark, ..)` sets values under a media condition, and a later `theme` call adds to the earlier one.",
                "`Style::token` sets a token on one element and those inside it, in place of the default and the page's theme.",
                "`El::bind_token` keeps a token equal to a string expression over the signals. The value is not checked, as for `El::bind_style`.",
            ],
        ),
        para(
            site,
            "The swatches and the button below read one token, which `bind_token` sets from the radio group's signal.",
        ),
        demo::show(site, Demo::Accent),
        note(
            site,
            "Two tokens with the same name are one custom property to the browser, and nothing checks that each name is used once. The components of `hypersig_ui` read the tokens in its [`token`](api:hypersig_ui/token/index.html) module, which [Theming](components/theming/) covers.",
        ),
    ])
}

fn global(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "A `Global` holds rules for elements by their tag, a small reset, and the fonts the page loads.",
        ),
        heading("Rules by tag"),
        para(
            site,
            "[`Global::rule`](api:hypersig_style/struct.Global.html#method.rule) adds a style for every element a selector matches: an `Element` by its tag, `Selector::Root`, or `Selector::Everything`. The rules sit in the base cascade layer, before the classes of `El::css`, so an element's own style overrides them. Conditions in a rule's style apply to the selector as they do to a class.",
        ),
        show(Snippet::GlobalRules),
        bullets(
            site,
            &[
                "`Global::preflight()` starts from a small reset: `border-box` sizing everywhere, no default margins, form controls in the page's font, and media that shrinks to its box.",
                "`FontFace::new(family, url(..))` loads a font as `@font-face`, with `weights` for a variable font and `italic` for an italic face. `family()` returns the value `font-family` takes.",
                "`Page::global` adds the rules, and a later call adds to the earlier ones. `hypersig_ui::global()` is a `Global` built this way.",
            ],
        ),
    ])
}

fn patterns(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "[`hypersig::style::pattern`](api:hypersig_style/pattern/index.html) has functions that return the styles of common layouts, which you merge with your own.",
        ),
        show(Snippet::Patterns),
        table_of(
            site,
            &["Pattern", "Lays out"],
            &[
                &[
                    "`stack(gap)`, `vstack(gap)`",
                    "Children in a column, the second centered across it",
                ],
                &["`hstack(gap)`", "Children in a row, centered on its line"],
                &["`wrap(gap)`", "Children in rows that wrap onto new lines"],
                &[
                    "`grid(columns, gap)`, `grid_item(columns)`",
                    "A grid of equal columns, and a child spanning several",
                ],
                &[
                    "`container(max, padding)`",
                    "A block no wider than `max`, centered, with padding on both sides",
                ],
                &[
                    "`center()`, `spacer()`",
                    "Children centered on both axes, and an empty flex child that takes the free space",
                ],
                &[
                    "`square(size)`, `circle(size)`, `aspect_ratio(ratio)`",
                    "Boxes of a fixed shape",
                ],
                &["`divider(width, color)`", "A line across the box"],
                &[
                    "`visually_hidden()`",
                    "Hidden from sight but read by screen readers",
                ],
                &[
                    "`link_overlay()`",
                    "A link that covers its positioned ancestor, so a whole card is clickable",
                ],
            ],
        ),
        demo::show(site, Demo::Layout),
    ])
}

fn animation(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "Keyframes are Rust values too, and an element that uses them carries their rule to the page.",
        ),
        heading("Keyframes"),
        para(
            site,
            "[`Keyframes::new(name)`](api:hypersig_style/struct.Keyframes.html) starts an animation, and `at(percent, style)` adds a step. `Style::keyframes` sets an element's `animation-name` and carries the `@keyframes` rule with it, so the page or the patch that brings the element also brings the rule. A keyframe writes only the unconditioned declarations of its style, because a keyframe takes no selector or media query.",
        ),
        show(Snippet::Animation),
        demo::show(site, Demo::Spinner),
        heading("Motion and the visitor"),
        para(
            site,
            "`Media::ReducedMotion` applies a style for a visitor who asks for reduced motion. The components of `hypersig_ui` make their color transitions instant there.",
        ),
        note(
            site,
            "Two `Keyframes` with the same name are one rule to the browser, and the later one in the stylesheet wins. Nothing checks that each name is used once.",
        ),
    ])
}

fn unchecked(site: Site, head: Part) -> Part {
    parts([
        head,
        lead(
            site,
            "A few forms go to the browser without a check. Each is named, so a search for it finds every place that relies on one.",
        ),
        show(Snippet::Unchecked),
        bullets(
            site,
            &[
                "`raw(css)` writes a value as given, for the forms a property's generated type leaves out, such as `margin: 0 auto`, a shadow, or a `calc()`. A value the browser rejects drops the declaration. `raw` panics on `;`, `{`, `}`, and `<`, which would end the declaration or the rule, or close the `<style>` element.",
                "`Stylesheet::new(css)` wraps CSS you already have, and `Page::style` adds it to the page. `Stylesheet::class(\"name\")` checks at compile time that the CSS has a selector for the class, and `Class::new` names a class without that check.",
                "`El::bind_style` and `El::bind_token` set a property from a string expression over the signals, which nothing checks.",
            ],
        ),
    ])
}

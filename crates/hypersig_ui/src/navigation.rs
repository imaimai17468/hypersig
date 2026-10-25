//! Navigation between pages: a breadcrumb trail, a list of the sections of
//! an application, and the links between the pages of a long list.

use hypersig::html::{El, Role, a, attr, li, nav, ol, span, ul};
use hypersig::style::pattern::{hstack, stack};
use hypersig::style::value::{AlignItems, FlexWrap, JustifyContent, pct, rem};
use hypersig::style::{PseudoClass, Style};
use hypersig::{Link, Params, Route, Search, Url};
use serde::Serialize;

use crate::button::{ButtonSize, ButtonVariant, button_style};
use crate::look::{self, icon};
use crate::token;

/// A trail of links to the pages above the current one, ending at it.
///
/// It is a `<nav>` named by its label, holding an ordered list. The current
/// page ends the trail as text with `aria-current="page"`, and
/// [`Breadcrumb::current`] is the only way to finish one, so a trail always
/// names the page it is on and nothing follows it. The separators are
/// hidden from assistive technology.
///
/// ```
/// use hypersig::{Page, Route, Signals};
/// use hypersig_ui::Breadcrumb;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Issue {}
///
/// # async fn page<P>((): (), _: P) -> Option<Page<Issue>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const HOME: Route = "/" => page;
///     const ISSUES: Route = "/issues" => page;
/// }
///
/// let trail = Breadcrumb::<Issue>::new("Breadcrumb")
///     .link("Home", HOME.link())
///     .link("Issues", ISSUES.link())
///     .current("Login is slow");
/// let html = trail.render();
/// assert!(html.contains(r#"<nav aria-label="Breadcrumb""#));
/// assert!(html.contains(r#"aria-current="page""#));
/// # let _ = pages;
/// ```
#[derive(Debug)]
pub struct Breadcrumb<S> {
    label: String,
    links: Vec<(String, Link<S>)>,
}

impl<S> Breadcrumb<S> {
    /// An empty trail whose `<nav>` is named `label`, such as "Breadcrumb".
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            links: Vec::new(),
        }
    }

    /// Adds a link to a page above the current one, after the links before it.
    #[must_use]
    pub fn link(mut self, text: impl Into<String>, link: Link<S>) -> Self {
        self.links.push((text.into(), link));
        self
    }

    /// The trail, ending at the current page `text`.
    #[must_use]
    pub fn current(self, text: impl Into<String>) -> El<S> {
        let separator = || {
            li().role(Role::Presentation)
                .attr(attr::ARIA_HIDDEN, "true")
                .css(Style::new().display(hypersig::style::value::Display::InlineFlex))
                .child(icon(look::icon::CHEVRON_RIGHT, rem(0.875)))
        };
        let mut list = ol().css(
            hstack(rem(0.375))
                .flex_wrap(FlexWrap::Wrap)
                .merge(look::small_text())
                .color(token::muted_foreground().var()),
        );
        for (text, link) in self.links {
            list = list
                .child(
                    li().child(
                        a().href(link)
                            .css(
                                Style::new()
                                    .merge(look::transition())
                                    .merge(look::focus_ring())
                                    .on(
                                        PseudoClass::Hover,
                                        Style::new().color(token::foreground().var()),
                                    ),
                            )
                            .text(text),
                    ),
                )
                .child(separator());
        }
        list = list.child(
            li().child(
                span()
                    .attr(attr::ARIA_CURRENT, "page")
                    .css(Style::new().color(token::foreground().var()))
                    .text(text),
            ),
        );
        nav().attr(attr::ARIA_LABEL, self.label).child(list)
    }
}

/// A list of links to the sections of an application, marking the one the
/// page is in.
///
/// It is a `<nav>` named by its label, holding a list of links. The page
/// passes its own [`Url`], which the server writes with [`Route::url`], and
/// each link is compared with it by path, without the query string. A link
/// to the page's path has `aria-current="page"`. A link to a path that the
/// page's path continues, such as `/users` on the page `/users/7`, has
/// `aria-current="true"`, so the section stays marked on the pages under it.
///
/// ```
/// use hypersig::{Page, Route, Signals};
/// use hypersig_ui::NavList;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Admin {
///     name: String,
/// }
///
/// # async fn page<P>((): (), _: P) -> Option<Page<Admin>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const HOME: Route = "/" => page;
///     const USERS: Route = "/users" => page;
///     const USER: Route<i32> = "/users/{id}" => page;
/// }
///
/// let sidebar = NavList::<Admin>::new("Main", USER.url(7))
///     .item("Dashboard", HOME, ())
///     .item("Users", USERS, ())
///     .render();
/// let html = sidebar.render();
/// assert!(html.contains(r#"href="/users" aria-current="true""#));
/// # let _ = pages;
/// ```
#[derive(Debug)]
pub struct NavList<S> {
    label: String,
    current: Url,
    items: Vec<(String, Link<S>, Url)>,
}

impl<S> NavList<S> {
    /// An empty list whose `<nav>` is named `label`, on the page at `current`.
    #[must_use]
    pub fn new(label: impl Into<String>, current: Url) -> Self {
        Self {
            label: label.into(),
            current,
            items: Vec::new(),
        }
    }

    /// Adds a link to `route` with the arguments `args`, as [`Route::url`]
    /// takes them.
    #[must_use]
    pub fn item<P: Params, Q: Search>(
        mut self,
        text: impl Into<String>,
        route: Route<P, Q>,
        args: Q::Args<P>,
    ) -> Self
    where
        Q::Args<P>: Serialize,
    {
        let link = route.link_with(&args);
        self.items.push((text.into(), link, route.url(args)));
        self
    }

    /// The `<nav>` and its list.
    #[must_use]
    pub fn render(self) -> El<S> {
        let path = |url: &Url| {
            let url = url.as_str();
            url.split_once('?').map_or(url, |(path, _)| path).to_owned()
        };
        let current = path(&self.current);
        let items = self.items.into_iter().map(|(text, link, url)| {
            let target = path(&url);
            let mark = if target == current {
                Some("page")
            } else if current
                .strip_prefix(target.trim_end_matches('/'))
                .is_some_and(|rest| rest.starts_with('/') && target != "/")
            {
                Some("true")
            } else {
                None
            };
            let mut style = button_style(ButtonVariant::Ghost, ButtonSize::Sm)
                .width(pct(100))
                .justify_content(JustifyContent::FlexStart);
            if mark.is_some() {
                style = style.merge(
                    Style::new()
                        .background_color(token::accent().var())
                        .color(token::accent_foreground().var()),
                );
            }
            let mut item = a().href(link).css(style).text(text);
            if let Some(mark) = mark {
                item = item.attr(attr::ARIA_CURRENT, mark);
            }
            li().child(item)
        });
        nav()
            .attr(attr::ARIA_LABEL, self.label)
            .child(ul().css(stack(rem(0.25))).children(items))
    }
}

impl<S> From<NavList<S>> for El<S> {
    fn from(list: NavList<S>) -> Self {
        list.render()
    }
}

impl<S> From<NavList<S>> for hypersig::html::Node<S> {
    fn from(list: NavList<S>) -> Self {
        list.render().into()
    }
}

/// The words [`pagination`] names its parts with.
///
/// They are English until replaced, so a page in another language passes
/// its own:
///
/// ```
/// use hypersig_ui::PaginationText;
///
/// let japanese = PaginationText {
///     label: "ページ送り".into(),
///     previous: "前へ".into(),
///     next: "次へ".into(),
/// };
/// assert_ne!(japanese, PaginationText::default());
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaginationText {
    /// The name of the `<nav>`, "Pagination".
    pub label: String,
    /// The link to the page before, "Previous".
    pub previous: String,
    /// The link to the page after, "Next".
    pub next: String,
}

impl Default for PaginationText {
    fn default() -> Self {
        Self {
            label: "Pagination".to_owned(),
            previous: "Previous".to_owned(),
            next: "Next".to_owned(),
        }
    }
}

/// Links to the pages of a list of `count` pages, on page `current`.
///
/// Pages are counted from 1, and `link` gives the link to a page. The
/// links are the page before and the page after, the first and the last
/// page, and the pages next to the current one, with `…` for the pages
/// left out. The current page's link has `aria-current="page"`. On the
/// first page the link to the page before is text marked
/// `aria-disabled="true"`, as is the link to the page after on the last.
///
/// ```
/// use hypersig::{Page, Route, Signals};
/// use hypersig_ui::{PaginationText, pagination};
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Signals, Serialize, Deserialize)]
/// struct Catalog {}
///
/// # async fn page((): (), _: u32) -> Option<Page<Catalog>> { None }
/// hypersig::routes! {
///     fn pages(Router<()>);
///     const PRODUCTS: Route<u32> = "/products/{page}" => page;
/// }
///
/// let pager = pagination::<Catalog>(5, 10, |page| PRODUCTS.link_with(&page), &PaginationText::default());
/// let html = pager.render();
/// assert!(html.contains(r#"href="/products/5" aria-current="page""#));
/// assert!(html.contains(r#"href="/products/4""#) && !html.contains(r#"href="/products/3""#));
/// # let _ = pages;
/// ```
///
/// # Panics
///
/// Panics if `current` is 0 or above `count`.
#[must_use]
pub fn pagination<S>(
    current: u32,
    count: u32,
    link: impl Fn(u32) -> Link<S>,
    text: &PaginationText,
) -> El<S> {
    assert!(
        current >= 1 && current <= count,
        "the current page is between 1 and the number of pages, not {current} of {count}"
    );
    let square = |variant| {
        button_style(variant, ButtonSize::Default)
            .width(rem(2.25))
            .padding_inline(rem(0))
    };
    let step = |target: Option<u32>, label: &str, rel: &'static str, back: bool| {
        let arrow = icon(
            if back {
                look::icon::CHEVRON_LEFT
            } else {
                look::icon::CHEVRON_RIGHT
            },
            rem(1),
        );
        let style =
            button_style(ButtonVariant::Ghost, ButtonSize::Default).padding_inline(rem(0.625));
        let el = match target {
            Some(page) => a().href(link(page)).attr(attr::REL, rel).css(style),
            None => span().attr(attr::ARIA_DISABLED, "true").css(
                style
                    .opacity(hypersig::style::value::Opacity::Number(0.5))
                    .cursor(hypersig::style::value::Cursor::NotAllowed),
            ),
        };
        let el = if back {
            el.child(arrow).child(span().text(label))
        } else {
            el.child(span().text(label)).child(arrow)
        };
        li().child(el)
    };

    let mut list = ul()
        .css(
            hstack(rem(0.25))
                .flex_wrap(FlexWrap::Wrap)
                .align_items(AlignItems::Center),
        )
        .child(step(
            (current > 1).then(|| current - 1),
            &text.previous,
            "prev",
            true,
        ));
    let mut shown = 0;
    for page in 1..=count {
        let near = page.abs_diff(current) <= 1;
        // A gap of one page shows that page, since `…` would take its place.
        let fills_gap = (page == 2 && current == 4) || (page + 1 == count && current + 3 == count);
        if !(page == 1 || page == count || near || fills_gap) {
            continue;
        }
        if page > shown + 1 {
            list = list.child(
                li().attr(attr::ARIA_HIDDEN, "true")
                    .css(
                        Style::new()
                            .width(rem(2.25))
                            .text_align(hypersig::style::value::TextAlign::Center),
                    )
                    .text("…"),
            );
        }
        shown = page;
        let mut item = a().href(link(page));
        if page == current {
            item = item
                .attr(attr::ARIA_CURRENT, "page")
                .css(square(ButtonVariant::Outline));
        } else {
            item = item.css(square(ButtonVariant::Ghost));
        }
        list = list.child(li().child(item.text(page.to_string())));
    }
    list = list.child(step(
        (current < count).then(|| current + 1),
        &text.next,
        "next",
        false,
    ));
    nav()
        .attr(attr::ARIA_LABEL, text.label.clone())
        .css(
            Style::new()
                .display(hypersig::style::value::Display::Flex)
                .justify_content(JustifyContent::Center),
        )
        .child(list)
}

#[cfg(test)]
mod tests {
    use hypersig::{Page, Route, Signals};
    use serde::{Deserialize, Serialize};

    use super::{PaginationText, pagination};

    #[derive(Signals, Serialize, Deserialize)]
    struct Catalog {
        query: String,
    }

    async fn page((): (), _: u32) -> Option<Page<Catalog>> {
        None
    }

    hypersig::routes! {
        fn pages(Router<()>);
        const PRODUCTS: Route<u32> = "/products/{page}" => page;
    }

    /// The numbers of the page links `pagination` shows, with 0 for `…`.
    fn shown(current: u32, count: u32) -> Vec<u32> {
        let _ = pages;
        let html = pagination::<Catalog>(
            current,
            count,
            |page| PRODUCTS.link_with(&page),
            &PaginationText::default(),
        )
        .render();
        let mut out = Vec::new();
        for part in html.split("<li").skip(1) {
            if part.contains("…") && !part.contains("href") {
                out.push(0);
            } else if let Some(rest) = part.split(r#"href="/products/"#).nth(1)
                && !part.contains("rel=")
            {
                out.push(rest.split('"').next().unwrap().parse().unwrap());
            }
        }
        out
    }

    #[test]
    fn a_gap_of_one_page_shows_the_page() {
        assert_eq!(shown(1, 1), [1]);
        assert_eq!(shown(1, 3), [1, 2, 3]);
        assert_eq!(shown(4, 10), [1, 2, 3, 4, 5, 0, 10]);
        assert_eq!(shown(5, 10), [1, 0, 4, 5, 6, 0, 10]);
        assert_eq!(shown(7, 10), [1, 0, 6, 7, 8, 9, 10]);
        assert_eq!(shown(10, 10), [1, 0, 9, 10]);
    }

    #[test]
    #[should_panic(expected = "between 1 and the number of pages")]
    fn a_page_past_the_last_panics() {
        let _ = shown(4, 3);
    }
}

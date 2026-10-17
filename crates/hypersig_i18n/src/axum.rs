//! Sending a visitor to the page in their language, from an axum [`Router`].

use axum::Router;
use axum::http::{HeaderMap, HeaderValue, header};
use axum::response::{IntoResponse, Redirect};
use axum::routing::get;
use hypersig::Route;

use crate::{Locale, negotiate};

/// Adds the redirect from `/` to the page in the visitor's language to an axum [`Router`].
pub trait LocaleRouterExt<St>: Sized {
    /// Answers `GET /` with a redirect to `home` in the locale the visitor's browser asks for.
    ///
    /// The locale comes from [`negotiate`] over the `Accept-Language` header.
    /// The redirect is a `307 Temporary Redirect` with `Vary: Accept-Language`,
    /// because another browser may be sent to another locale.
    ///
    /// ```
    /// # use axum::Router;
    /// # use hypersig::{Page, Route, Segment, Signals};
    /// # use hypersig_i18n::{Locale, LocaleRouterExt};
    /// # use serde::{Deserialize, Serialize};
    /// # #[derive(Segment, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
    /// # #[serde(rename_all = "lowercase")]
    /// # enum Lang { En, Ja }
    /// # struct ByLang<T> { en: T, ja: T }
    /// # impl Locale for Lang {
    /// #     const ALL: &'static [Self] = &[Self::En, Self::Ja];
    /// #     type Text<T> = ByLang<T>;
    /// #     fn pick<T>(self, t: ByLang<T>) -> T { match self { Self::En => t.en, Self::Ja => t.ja } }
    /// # }
    /// # #[derive(Signals, Serialize, Deserialize)]
    /// # struct Home {}
    /// # async fn home((): (), _: Lang) -> Option<Page<Home>> { None }
    /// hypersig::routes! {
    ///     fn pages(Router<()>);
    ///     const HOME: Route<Lang> = "/{lang}" => home;
    /// }
    ///
    /// let _app: Router = pages(Router::new()).redirect_to_locale(HOME);
    /// ```
    #[must_use]
    fn redirect_to_locale<L: Locale + Send + Sync>(self, home: Route<L>) -> Self;
}

impl<St: Clone + Send + Sync + 'static> LocaleRouterExt<St> for Router<St> {
    fn redirect_to_locale<L: Locale + Send + Sync>(self, home: Route<L>) -> Self {
        self.route(
            "/",
            get(move |headers: HeaderMap| async move {
                let accept = headers
                    .get(header::ACCEPT_LANGUAGE)
                    .and_then(|value| value.to_str().ok())
                    .unwrap_or_default();
                let url = home.url(negotiate::<L>(accept));
                (
                    [(header::VARY, HeaderValue::from_static("accept-language"))],
                    Redirect::temporary(url.as_str()),
                )
                    .into_response()
            }),
        )
    }
}

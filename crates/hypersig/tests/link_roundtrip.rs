//! A property test that a URL [`Route::url`] writes reaches its route's handler with the values it was built from.
//!
//! Each case draws values from a seeded generator, writes the URL, sends the
//! request target a browser would send for it, and compares what the
//! handler received with the values the URL was built from.

use std::fmt::{Debug, Write as _};
use std::sync::Mutex;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode, header};
use http_body_util::BodyExt as _;
use hypersig::{Base, Page, PageResponse, PathSegment, Rest, Route, Search, Segment, Signals, Url};
use serde::{Deserialize, Serialize};
use tower::ServiceExt as _;

#[derive(Signals, Serialize, Deserialize)]
struct Seen {
    reached: bool,
}

#[derive(Segment, Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
#[serde(rename_all = "kebab-case")]
enum Kind {
    Overview,
    TeamSettings,
    #[serde(rename = "日本 語")]
    Japanese,
    #[serde(rename = "a b/c?d#e&f=g%h+i")]
    Punctuated,
    #[serde(rename = "...")]
    Dots,
}

const KINDS: [Kind; 5] = [
    Kind::Overview,
    Kind::TeamSettings,
    Kind::Japanese,
    Kind::Punctuated,
    Kind::Dots,
];

#[derive(Signals, Search, Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Filter {
    q: String,
    note: Option<String>,
    tags: Vec<String>,
    page: i32,
    count: u32,
    big: i64,
    huge: u64,
    ratio: f64,
    share: f32,
    open: bool,
    kind: Kind,
    kinds: Vec<Kind>,
    maybe: Option<Kind>,
}

#[derive(Signals, Search, Serialize, Deserialize, Debug, Clone, PartialEq)]
struct Sparse {
    word: Option<String>,
    words: Vec<String>,
    flag: Option<bool>,
}

/// A way a URL from [`Route::url`] fails to reach its handler with its values.
///
/// Each case lists the findings its values can run into, and the test
/// skips a failure only where the case lists one. Each finding must still
/// fail at least one case, so a fixed finding fails the test until its
/// entry here goes. Both are the float values that the docs of `Search`
/// list as not reaching the handler.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Finding {
    /// A NaN or infinite float search value is written as `null` and left
    /// out of the query, so a required field answers 400 and an `Option`
    /// field arrives as `None`.
    NonFinite,
    /// A float search value of `-0.0` is written `0` and arrives as `0.0`.
    NegativeZero,
}

const SKIPS: [Finding; 2] = [Finding::NonFinite, Finding::NegativeZero];

impl Filter {
    fn findings(&self) -> Vec<Finding> {
        let mut found = Vec::new();
        if !self.ratio.is_finite() || !self.share.is_finite() {
            found.push(Finding::NonFinite);
        }
        let negative_zero = |x: f64| x == 0.0 && x.is_sign_negative();
        if negative_zero(self.ratio) || negative_zero(f64::from(self.share)) {
            found.push(Finding::NegativeZero);
        }
        found
    }
}

const APP: Base = Base::new("/app");

/// What the last handler received, written with `Debug`.
static RECEIVED: Mutex<Option<String>> = Mutex::new(None);

/// The URL the redirecting handler sends the browser to.
static REDIRECT_TO: Mutex<Option<Url>> = Mutex::new(None);

fn seen(args: &impl Debug) -> Page<Seen> {
    *RECEIVED.lock().expect("lock") = Some(format!("{args:?}"));
    Page::new("Seen", Seen { reached: true })
}

fn redirect() -> PageResponse {
    PageResponse::redirect(REDIRECT_TO.lock().expect("lock").take().expect("a target"))
}

hypersig::routes! {
    fn pages(Router<()>);
    const HOME: Route<(), Sparse> = "/" => |(), args: Sparse| async move { seen(&args) };
    const FIND: Route<(), Filter> = "/find" => |(), args: Filter| async move { seen(&args) };
    const SIGNED: Route<i32> = "/i/{a}" => |(), args: i32| async move { seen(&args) };
    const UNSIGNED: Route<u32, Sparse> = "/u/{a}" =>
        |(), args: (u32, Sparse)| async move { seen(&args) };
    const SLUG: Route<PathSegment, Filter> = "/s/{a}" =>
        |(), args: (PathSegment, Filter)| async move { seen(&args) };
    const KIND: Route<Kind> = "/k/{a}" => |(), args: Kind| async move { seen(&args) };
    const FILE: Route<Rest, Sparse> = "/r/{*rest}" =>
        |(), args: (Rest, Sparse)| async move { seen(&args) };
    const TWO: Route<(PathSegment, i32), Filter> = "/two/{s}/{i}" =>
        |(), args: ((PathSegment, i32), Filter)| async move { seen(&args) };
    const THREE: Route<(Kind, PathSegment, Rest)> = "/three/{k}/{s}/{*rest}" =>
        |(), args: (Kind, PathSegment, Rest)| async move { seen(&args) };
    const ORG: Route<PathSegment> = "/orgs/{org}" => |(), args: PathSegment| async move { seen(&args) };
    const MEMBER: Route<(PathSegment, u32), Sparse> = "/orgs/{org}/m/{id}" in ORG =>
        |(), args: ((PathSegment, u32), Sparse)| async move { seen(&args) };
    const ROLE: Route<(PathSegment, u32, Kind)> = "/orgs/{org}/m/{id}/{role}" in MEMBER =>
        |(), args: (PathSegment, u32, Kind)| async move { seen(&args) };
    const GO: Route = "/go" => |(), ()| async { redirect() };
}

hypersig::routes! {
    fn rooted(Router<()>);
    const ANY: Route<Rest, Sparse> = "/{*rest}" => |(), args: (Rest, Sparse)| async move { seen(&args) };
}

hypersig::routes! {
    fn under_app(Router<()>) under APP;
    const APP_HOME: Route<(), Filter> = "/" => |(), args: Filter| async move { seen(&args) };
    const APP_ITEM: Route<(i32, PathSegment)> = "/items/{id}/{s}" =>
        |(), args: (i32, PathSegment)| async move { seen(&args) };
    const APP_REVIEW: Route<(i32, PathSegment, u32), Sparse> = "/items/{id}/{s}/{review}" in APP_ITEM =>
        |(), args: ((i32, PathSegment, u32), Sparse)| async move { seen(&args) };
    const APP_FILE: Route<(Kind, Rest)> = "/files/{k}/{*rest}" =>
        |(), args: (Kind, Rest)| async move { seen(&args) };
    const APP_GO: Route = "/go" => |(), ()| async { redirect() };
}

/// The seeded generator, `SplitMix64`.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).expect("small")).expect("small")
    }

    fn chance(&mut self, one_in: usize) -> bool {
        self.below(one_in) == 0
    }

    fn pick<T: Copy>(&mut self, items: &[T]) -> T {
        items[self.below(items.len())]
    }

    fn text(&mut self) -> String {
        const PIECES: &[&str] = &[
            "a",
            "Z",
            "0",
            "9",
            "-",
            "_",
            "~",
            " ",
            "  ",
            "%",
            "%25",
            "%2F",
            "%2e",
            "%2E%2E",
            "%zz",
            "+",
            "&",
            "=",
            "#",
            "?",
            "/",
            "//",
            ".",
            "..",
            "'",
            "\"",
            "<",
            ">",
            "`",
            "{",
            "}",
            "|",
            "\\",
            "^",
            "!",
            "*",
            "(",
            ")",
            ";",
            ":",
            "@",
            "$",
            ",",
            "[",
            "]",
            "\t",
            "\n",
            "\r",
            "\u{7f}",
            "é",
            "日本",
            "語",
            "チーム",
            "😀",
            "👨\u{200d}👩\u{200d}👧",
            "\u{200b}",
            "\u{feff}",
            "\u{fffd}",
            "a b",
            "x=y&z",
            "?q=1#f",
        ];
        let count = self.below(6);
        (0..count).map(|_| self.pick(PIECES)).collect()
    }

    fn segment(&mut self) -> PathSegment {
        loop {
            if let Ok(segment) = PathSegment::parse(self.text()) {
                return segment;
            }
        }
    }

    fn rest(&mut self) -> Rest {
        loop {
            let parts = 1 + self.below(4);
            let text: Vec<String> = (0..parts)
                .map(|_| {
                    if self.chance(4) {
                        String::new()
                    } else {
                        self.text()
                    }
                })
                .collect();
            if let Ok(rest) = Rest::parse(text.join("/")) {
                return rest;
            }
        }
    }

    fn kind(&mut self) -> Kind {
        self.pick(&KINDS)
    }

    fn i32(&mut self) -> i32 {
        if self.chance(3) {
            self.pick(&[i32::MIN, i32::MIN + 1, -1, 0, 1, i32::MAX - 1, i32::MAX])
        } else {
            #[expect(clippy::cast_possible_truncation, reason = "any 32 bits")]
            let value = self.next() as i32;
            value
        }
    }

    fn u32(&mut self) -> u32 {
        if self.chance(3) {
            self.pick(&[0, 1, u32::MAX - 1, u32::MAX])
        } else {
            #[expect(clippy::cast_possible_truncation, reason = "any 32 bits")]
            let value = self.next() as u32;
            value
        }
    }

    fn i64(&mut self) -> i64 {
        const EXACT: i64 = 1 << 53;
        if self.chance(3) {
            self.pick(&[
                i64::MIN,
                -EXACT - 1,
                -EXACT,
                -1,
                0,
                1,
                EXACT,
                EXACT + 1,
                i64::MAX,
            ])
        } else if self.chance(2) {
            #[expect(clippy::cast_possible_wrap, reason = "any 64 bits")]
            let value = self.next() as i64;
            value
        } else {
            i64::from(self.i32())
        }
    }

    fn u64(&mut self) -> u64 {
        if self.chance(3) {
            self.pick(&[0, 1, (1 << 53) - 1, 1 << 53, (1 << 53) + 1, u64::MAX])
        } else if self.chance(2) {
            self.next()
        } else {
            u64::from(self.u32())
        }
    }

    fn f64(&mut self) -> f64 {
        if self.chance(3) {
            self.pick(&[
                0.0,
                -0.0,
                0.1,
                -1.5,
                1e21,
                1e-7,
                5e-324,
                f64::MIN_POSITIVE,
                f64::MAX,
                f64::MIN,
                f64::EPSILON,
                123_456_789.125,
                9_007_199_254_740_993.0,
                f64::NAN,
                f64::INFINITY,
                f64::NEG_INFINITY,
            ])
        } else {
            let value = f64::from_bits(self.next());
            if value.is_finite() { value } else { 2.5 }
        }
    }

    fn f32(&mut self) -> f32 {
        if self.chance(3) {
            self.pick(&[
                0.0,
                -0.0,
                0.1,
                1e-7,
                f32::MIN_POSITIVE,
                f32::MAX,
                f32::MIN,
                1e-45,
                f32::NAN,
                f32::INFINITY,
            ])
        } else {
            #[expect(clippy::cast_possible_truncation, reason = "any 32 bits")]
            let value = f32::from_bits(self.next() as u32);
            if value.is_finite() { value } else { 2.5 }
        }
    }

    fn list<T>(&mut self, mut item: impl FnMut(&mut Self) -> T) -> Vec<T> {
        let count = self.below(4);
        (0..count).map(|_| item(self)).collect()
    }

    fn option<T>(&mut self, item: impl FnOnce(&mut Self) -> T) -> Option<T> {
        (!self.chance(3)).then(|| item(self))
    }

    fn filter(&mut self) -> Filter {
        Filter {
            q: self.text(),
            note: self.option(Self::text),
            tags: self.list(Self::text),
            page: self.i32(),
            count: self.u32(),
            big: self.i64(),
            huge: self.u64(),
            ratio: self.f64(),
            share: self.f32(),
            open: self.chance(2),
            kind: self.kind(),
            kinds: self.list(Self::kind),
            maybe: self.option(Self::kind),
        }
    }

    fn sparse(&mut self) -> Sparse {
        Sparse {
            word: self.option(Self::text),
            words: self.list(Self::text),
            flag: self.option(|rng| rng.chance(2)),
        }
    }
}

/// The bytes the WHATWG URL parser percent-encodes in a path of an `http` URL.
fn in_path_set(byte: u8) -> bool {
    !(0x20..0x7f).contains(&byte)
        || matches!(
            byte,
            b' ' | b'"' | b'#' | b'<' | b'>' | b'?' | b'`' | b'{' | b'}'
        )
}

/// The bytes the WHATWG URL parser percent-encodes in a query of an `http` URL.
fn in_query_set(byte: u8) -> bool {
    !(0x20..0x7f).contains(&byte) || matches!(byte, b' ' | b'"' | b'#' | b'<' | b'>' | b'\'')
}

fn percent_encode(text: &str, set: fn(u8) -> bool) -> String {
    let mut out = String::new();
    for &byte in text.as_bytes() {
        if set(byte) {
            write!(out, "%{byte:02X}").expect("writing to a String");
        } else {
            out.push(char::from(byte));
        }
    }
    out
}

fn is_single_dot(segment: &str) -> bool {
    segment == "." || segment.eq_ignore_ascii_case("%2e")
}

fn is_double_dot(segment: &str) -> bool {
    matches!(
        segment.to_ascii_lowercase().as_str(),
        ".." | ".%2e" | "%2e." | "%2e%2e"
    )
}

/// The request target a browser sends for a link to `url` on a page of `http://host/`.
///
/// It follows the WHATWG URL parser for a path-absolute URL: the fragment is
/// dropped, a `\` is a `/`, each path segment and the query are
/// percent-encoded with their sets, and `.` and `..` segments are removed.
fn browser_target(url: &str) -> String {
    assert!(url.starts_with('/'), "a path-absolute URL: {url}");
    assert!(!url.starts_with("//"), "a URL on this host: {url}");
    let url = url.split_once('#').map_or(url, |(before, _)| before);
    let (path, query) = match url.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (url, None),
    };
    let path = path.replace('\\', "/");
    let given: Vec<&str> = path[1..].split('/').collect();
    let mut segments: Vec<String> = Vec::new();
    for (index, segment) in given.iter().enumerate() {
        let last = index + 1 == given.len();
        let encoded = percent_encode(segment, in_path_set);
        if is_double_dot(&encoded) {
            segments.pop();
            if last {
                segments.push(String::new());
            }
        } else if is_single_dot(&encoded) {
            if last {
                segments.push(String::new());
            }
        } else {
            segments.push(encoded);
        }
    }
    let mut target = format!("/{}", segments.join("/"));
    if let Some(query) = query {
        target.push('?');
        target.push_str(&percent_encode(query, in_query_set));
    }
    target
}

/// How a request ended, where it did not reach the handler with the values.
#[derive(Debug)]
#[expect(
    dead_code,
    reason = "the report of a failure reads the fields through `Debug`"
)]
enum Outcome {
    Status(StatusCode, String),
    Wrong { got: Option<String> },
    BadLocation(Option<String>),
}

#[derive(Debug)]
#[expect(
    dead_code,
    reason = "the report of a failure reads the fields through `Debug`"
)]
struct Failure {
    route: &'static str,
    expected: String,
    url: String,
    sent: String,
    outcome: Outcome,
}

/// A router and how a request reaches it.
#[derive(Clone, Copy)]
enum Serve {
    /// At the root of the host.
    Root,
    /// Inside `Router::nest` at the base.
    Nested,
    /// Behind a proxy that strips the base before the request arrives.
    Stripped,
}

struct Apps {
    pages: Router,
    rooted: Router,
    nested: Router,
    stripped: Router,
}

impl Apps {
    fn new() -> Self {
        Self {
            pages: pages(Router::new()),
            rooted: rooted(Router::new()),
            nested: Router::new().nest(APP.as_str(), under_app(Router::new())),
            stripped: under_app(Router::new()),
        }
    }

    fn router(&self, serve: Serve, rooted: bool) -> Router {
        match (serve, rooted) {
            (Serve::Root, false) => self.pages.clone(),
            (Serve::Root, true) => self.rooted.clone(),
            (Serve::Nested, _) => self.nested.clone(),
            (Serve::Stripped, _) => self.stripped.clone(),
        }
    }
}

fn strip_base(target: &str) -> String {
    let rest = target.strip_prefix(APP.as_str()).expect("under the base");
    if rest.is_empty() || rest.starts_with('?') {
        format!("/{rest}")
    } else {
        rest.to_owned()
    }
}

async fn get(router: Router, target: &str) -> (StatusCode, Option<String>, String) {
    let response = router
        .oneshot(Request::get(target).body(Body::empty()).expect("request"))
        .await
        .expect("the router is infallible");
    let status = response.status();
    let location = response
        .headers()
        .get(header::LOCATION)
        .map(|value| value.to_str().expect("ASCII").to_owned());
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body")
        .to_bytes();
    (
        status,
        location,
        String::from_utf8_lossy(&body).chars().take(200).collect(),
    )
}

/// One case: the route, the values, and the URL written from them.
struct Case {
    route: &'static str,
    expected: String,
    url: Url,
    serve: Serve,
    rooted: bool,
    /// The known findings the values can run into.
    findings: Vec<Finding>,
}

fn case<A: Debug>(
    route: &'static str,
    serve: Serve,
    findings: Vec<Finding>,
    args: A,
    url: impl FnOnce(A) -> Url,
) -> Case {
    let expected = format!("{args:?}");
    Case {
        route,
        expected,
        url: url(args),
        serve,
        rooted: false,
        findings,
    }
}

fn draw(rng: &mut Rng) -> Case {
    use Serve::{Nested, Root, Stripped};
    let base = if rng.chance(2) { Nested } else { Stripped };
    let none = Vec::new;
    match rng.below(17) {
        0 => case("HOME", Root, none(), rng.sparse(), |a| HOME.url(a)),
        1 => {
            let filter = rng.filter();
            case("FIND", Root, filter.findings(), filter, |a| FIND.url(a))
        }
        2 => case("SIGNED", Root, none(), rng.i32(), |a| SIGNED.url(a)),
        3 => case("UNSIGNED", Root, none(), (rng.u32(), rng.sparse()), |a| {
            UNSIGNED.url(a)
        }),
        4 => {
            let filter = rng.filter();
            case(
                "SLUG",
                Root,
                filter.findings(),
                (rng.segment(), filter),
                |a| SLUG.url(a),
            )
        }
        5 => case("KIND", Root, none(), rng.kind(), |a| KIND.url(a)),
        6 => case("FILE", Root, none(), (rng.rest(), rng.sparse()), |a| {
            FILE.url(a)
        }),
        7 => {
            let filter = rng.filter();
            case(
                "TWO",
                Root,
                filter.findings(),
                ((rng.segment(), rng.i32()), filter),
                |a| TWO.url(a),
            )
        }
        8 => case(
            "THREE",
            Root,
            none(),
            (rng.kind(), rng.segment(), rng.rest()),
            |a| THREE.url(a),
        ),
        9 => case("ORG", Root, none(), rng.segment(), |a| ORG.url(a)),
        10 => case(
            "MEMBER",
            Root,
            none(),
            ((rng.segment(), rng.u32()), rng.sparse()),
            |a| MEMBER.url(a),
        ),
        11 => case(
            "ROLE",
            Root,
            none(),
            (rng.segment(), rng.u32(), rng.kind()),
            |a| ROLE.url(a),
        ),
        12 => Case {
            rooted: true,
            ..case("ANY", Root, none(), (rng.rest(), rng.sparse()), |a| {
                ANY.url(a)
            })
        },
        13 => {
            let filter = rng.filter();
            case("APP_HOME", base, filter.findings(), filter, |a| {
                APP_HOME.url(a)
            })
        }
        14 => case("APP_ITEM", base, none(), (rng.i32(), rng.segment()), |a| {
            APP_ITEM.url(a)
        }),
        15 => case(
            "APP_REVIEW",
            base,
            none(),
            ((rng.i32(), rng.segment(), rng.u32()), rng.sparse()),
            |a| APP_REVIEW.url(a),
        ),
        _ => case("APP_FILE", base, none(), (rng.kind(), rng.rest()), |a| {
            APP_FILE.url(a)
        }),
    }
}

/// Sends `target` as `case` is served, and checks what the handler received.
async fn reach(apps: &Apps, case: &Case, target: &str) -> Result<(), Outcome> {
    let target = match case.serve {
        Serve::Stripped => strip_base(target),
        Serve::Root | Serve::Nested => target.to_owned(),
    };
    RECEIVED.lock().expect("lock").take();
    let (status, _, body) = get(apps.router(case.serve, case.rooted), &target).await;
    if status != StatusCode::OK {
        return Err(Outcome::Status(status, body));
    }
    let got = RECEIVED.lock().expect("lock").take();
    if got.as_deref() == Some(case.expected.as_str()) {
        Ok(())
    } else {
        Err(Outcome::Wrong { got })
    }
}

/// Follows the link, then a redirect to the same URL from a page handler.
async fn run(apps: &Apps, case: &Case) -> Result<(), Failure> {
    let url = case.url.as_str();
    let sent = browser_target(url);
    let fail = |sent: &str, outcome| Failure {
        route: case.route,
        expected: case.expected.clone(),
        url: url.to_owned(),
        sent: sent.to_owned(),
        outcome,
    };
    reach(apps, case, &sent)
        .await
        .map_err(|outcome| fail(&sent, outcome))?;

    *REDIRECT_TO.lock().expect("lock") = Some(case.url.clone());
    let (router, go) = match case.serve {
        Serve::Root => (apps.router(Serve::Root, false), GO.url(()).to_string()),
        Serve::Nested => (apps.nested.clone(), APP_GO.url(()).to_string()),
        Serve::Stripped => (apps.stripped.clone(), strip_base(APP_GO.url(()).as_str())),
    };
    let (status, location, _) = get(router, &go).await;
    match location {
        Some(location) if status == StatusCode::SEE_OTHER && location == url => {
            let sent = browser_target(&location);
            reach(apps, case, &sent)
                .await
                .map_err(|outcome| fail(&sent, outcome))
        }
        location => Err(fail(&go, Outcome::BadLocation(location))),
    }
}

/// The cases drawn, of which more than 10,000 list no finding.
const CASES: usize = 20_000;
const SEED: u64 = 0x6879_7065_7273_6967;

#[tokio::test]
async fn a_url_reaches_its_handler_with_the_values_it_was_written_from() {
    let apps = Apps::new();
    let mut rng = Rng(SEED);
    let mut unexpected = Vec::new();
    let mut clean = 0;
    let mut skipped = [0; SKIPS.len()];
    for _ in 0..CASES {
        let case = draw(&mut rng);
        if case.findings.is_empty() {
            clean += 1;
        }
        let Err(failure) = run(&apps, &case).await else {
            continue;
        };
        let mut listed = false;
        for (index, finding) in SKIPS.iter().enumerate() {
            if case.findings.contains(finding) {
                skipped[index] += 1;
                listed = true;
            }
        }
        if !listed {
            unexpected.push(failure);
        }
    }
    assert!(clean > 10_000, "{clean} cases list no finding");
    for failure in unexpected.iter().take(20) {
        eprintln!("{failure:#?}");
    }
    assert!(
        unexpected.is_empty(),
        "{} of {CASES} cases failed outside the skip set",
        unexpected.len()
    );
    for (finding, count) in SKIPS.iter().zip(skipped) {
        assert!(count > 0, "{finding:?} fails no case, so its skip goes");
    }
}

# use hypersig::Page;
# async fn tab((): (), _: (PathSegment, Tab)) -> Option<Page<Team>> { None }
# async fn file((): (), _: Rest) -> Option<Page<Team>> { None }
use hypersig::{PathSegment, Rest, Route, Segment, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Team {}

// Each variant is one segment, spelled as serde writes the variant.
#[derive(Segment, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum Tab {
    Overview,
    TeamSettings,
}

hypersig::routes! {
    fn pages(Router<()>);
    const TAB: Route<(PathSegment, Tab)> = "/teams/{team}/{tab}" => tab;
    const FILE: Route<Rest> = "/files/{*path}" => file;
}

// In a `const`, an empty segment, `.`, or `..` fails to compile.
const RED: PathSegment = PathSegment::new("red team");
assert_eq!(TAB.url((RED, Tab::TeamSettings)).as_str(), "/teams/red%20team/team-settings");

// Text read at run time, such as a name from a database, goes through `parse`.
assert!(PathSegment::parse("..").is_err());
let blue = PathSegment::parse("blue").expect("a segment");
assert_eq!(TAB.url((blue, Tab::Overview)).as_str(), "/teams/blue/overview");

// A `Rest` keeps its `/`s and percent-encodes each part between them.
assert_eq!(FILE.url(Rest::new("docs/a b.md")).as_str(), "/files/docs/a%20b.md");
assert!(Rest::parse("docs/../../etc").is_err());
# let _ = pages;

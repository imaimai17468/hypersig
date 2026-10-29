# use serde::{Deserialize, Serialize};
use hypersig::html::{a, each, h1, li, ul};
use hypersig::{Page, PathSegment, Route, Segment, Signals};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Member {
    id: i32,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Org {
    org: PathSegment,
    members: Vec<Member>,
}

#[derive(Segment, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Tab {
    Profile,
    Activity,
}

hypersig::routes! {
    fn pages(Router<()>);
    const ORG: Route<PathSegment> = "/orgs/{org}" => org;
    const MEMBER: Route<(PathSegment, i32)> = "/orgs/{org}/members/{id}" in ORG => member;
    // A child of a child: the org, the member, then the tab.
    const MEMBER_TAB: Route<(PathSegment, i32, Tab)> =
        "/orgs/{org}/members/{id}/{tab}" in MEMBER => member_tab;
}

# fn load(_org: &PathSegment) -> Vec<Member> { vec![Member { id: 1, name: "Ada".to_owned() }] }
async fn org((): (), org: PathSegment) -> Page<Org> {
    let members = load(&org);
    Page::new(org.to_string(), Org { org, members })
        .child(h1().bind_text(Org::org().to_text()))
        .child(ul().child(each(Org::members(), |member| {
            // The link reads the org from the signals and the id from the item.
            li().child(
                a().href(MEMBER.link_with((Org::org(), member.at(Member::id()))))
                    .bind_text(member.at(Member::name())),
            )
        })))
}

async fn member((): (), (org, id): (PathSegment, i32)) -> Option<Page<Org>> {
    let members: Vec<Member> = load(&org).into_iter().filter(|m| m.id == id).collect();
    let name = members.first()?.name.clone();
    // A URL written on the server is an `href` too.
    let activity = MEMBER_TAB.url((org.clone(), id, Tab::Activity));
    Some(
        Page::new(name, Org { org: org.clone(), members })
            .child(a().href(ORG.link_with(org)).text("Back to the org"))
            .child(a().href(activity).text("Activity")),
    )
}

# async fn member_tab((): (), _: (PathSegment, i32, Tab)) -> Option<Page<Org>> { None }
# let _ = pages;

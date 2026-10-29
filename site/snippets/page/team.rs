use hypersig::html::{El, Id, a, article, attr, h2, h3, img, li, p, section, ul};
use hypersig::{Page, Route, SafeUrl, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Team {}

# async fn member((): (), _: u32) -> Option<Page<Team>> { None }
hypersig::routes! {
    fn pages(Router<()>);
    const MEMBER: Route<u32> = "/members/{id}" => member;
}

struct Member {
    id: u32,
    name: &'static str,
    job: &'static str,
    photo: SafeUrl,
}

const HEADING: Id = Id::new("team");
const NAME: Id = Id::new("member");

/// One member's card, named by its heading for a screen reader.
fn card(member: &Member) -> El<Team> {
    let name = NAME.with_key(member.id);
    li().child(
        article()
            .aria_labelledby(&[&name])
            .child(img().src(member.photo.clone()).attr(attr::ALT, ""))
            .child(h3().id(name).text(member.name))
            .child(p().text(member.job))
            .child(a().href(MEMBER.url(member.id)).text("Profile")),
    )
}

let members = [
    Member { id: 1, name: "Ada", job: "Engineer", photo: SafeUrl::new("/photos/1.jpg") },
    Member { id: 2, name: "Grace", job: "Admiral", photo: SafeUrl::new("/photos/2.jpg") },
];
let page = Page::new("Team", Team {}).child(
    section()
        .aria_labelledby(&[&HEADING])
        .child(h2().id(HEADING).text("Team"))
        .child(ul().children(members.iter().map(card))),
);
assert!(page.render().contains(concat!(
    r#"<li><article aria-labelledby="member-1"><img src="/photos/1.jpg" alt="">"#,
    r#"<h3 id="member-1">Ada</h3><p>Engineer</p><a href="/members/1">Profile</a></article></li>"#,
)));
# let _ = pages;

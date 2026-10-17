//! Two pages joined by typed links, for the page on pages and links.

use axum::Router;
use hypersig::html::{a, each, h1, li, p, ul};
use hypersig::{Page, Route, RouterExt, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize, Clone)]
struct Member {
    id: i32,
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Team {
    members: Vec<Member>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Profile {
    name: String,
}

hypersig::routes! {
    fn pages(Router<()>);
    const TEAM: Route = "/" => team;
    const MEMBER: Route<i32> = "/members/{id}" => member;
}

fn members() -> Vec<Member> {
    vec![
        Member {
            id: 1,
            name: "Ada".to_owned(),
        },
        Member {
            id: 2,
            name: "Grace".to_owned(),
        },
    ]
}

async fn team((): (), (): ()) -> Page<Team> {
    Page::new("Team", Team { members: members() })
        .child(h1().text("Team"))
        .child(ul().child(each(Team::members(), |member| {
            li().child(
                a().href(MEMBER.link_with(member.at(Member::id())))
                    .bind_text(member.at(Member::name())),
            )
        })))
}

async fn member((): (), id: i32) -> Option<Page<Profile>> {
    let member = members().into_iter().find(|member| member.id == id)?;
    Some(
        Page::new(member.name.clone(), Profile { name: member.name })
            .child(h1().bind_text(Profile::name()))
            .child(p().child(a().href(TEAM.link()).text("Back to the team"))),
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = pages(Router::new()).hypersig_runtime();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}

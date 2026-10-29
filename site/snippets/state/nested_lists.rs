use hypersig::html::{each, li, p, ul};
use hypersig::{Page, Signals};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct Tag {
    name: String,
}

#[derive(Signals, Serialize, Deserialize)]
struct Post {
    title: String,
    tags: Vec<Tag>,
}

#[derive(Signals, Serialize, Deserialize)]
struct Blog {
    posts: Vec<Post>,
}

let posts = ul::<Blog>().child(each(Blog::posts(), |post| {
    li().child(p().bind_text(post.at(Post::title())))
        // The inner closure reads its own item and the post around it.
        .child(ul().child(each(post.at(Post::tags()), |tag| {
            li().bind_text(tag.at(Tag::name()).concat(" in ").concat(post.at(Post::title())))
        })))
}));
# let post = Post { title: "Hello".into(), tags: vec![Tag { name: "rust".into() }] };
# let html = Page::new("Blog", Blog { posts: vec![post] }).child(posts).render();
# assert!(html.contains(">rust in Hello</li>"), "{html}");

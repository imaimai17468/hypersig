use hypersig::html::{Id, Role, attr, div, input, label, p};

// Each id is declared once, and the element and its references share it.
const EMAIL: Id = Id::new("email");
const HINT: Id = Id::new("email-hint");

let field = div::<()>()
    .child(label().label_for(&EMAIL).text("Email"))
    .child(
        input()
            .id(EMAIL)
            .aria_describedby(&[&HINT])
            .attr(attr::TYPE, "email"),
    )
    .child(p().id(HINT).text("We never share it."))
    .child(div().role(Role::Status).text("Saved"));
assert_eq!(
    field.render(),
    concat!(
        r#"<div><label for="email">Email</label>"#,
        r#"<input id="email" aria-describedby="email-hint" type="email">"#,
        r#"<p id="email-hint">We never share it.</p>"#,
        r#"<div role="status">Saved</div></div>"#,
    ),
);

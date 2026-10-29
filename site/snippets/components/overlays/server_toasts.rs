use axum::Router;
use hypersig::html::div;
use hypersig::{Action, Expr, Page, Patches, Signals};
use hypersig_ui::{ButtonSize, ButtonVariant, Toast, ToastVariant, Toaster, button};
use serde::{Deserialize, Serialize};

#[derive(Signals, Serialize, Deserialize)]
struct App {
    toasts: Vec<Toast>,
}

hypersig::routes! {
    fn pages(Router<()>);
    action SAVE: Action<App> = post "/save" => save;
}

// The server adds its toast after the ones the browser already shows.
async fn save((): (), (): ()) -> Patches<App> {
    App::toasts()
        .patch_append(&[Toast::new("Saved").description("Your changes are live.")])
        .into()
}

// A failed call shows the server's message under the title.
let failed = Toast::new("Save failed").variant(ToastVariant::Destructive);
let call = SAVE.call().on_error(|message| {
    Toast::push_expr(App::toasts(), Expr::lit(&failed).with(Toast::body(), message))
});

// A toast in the first signals is in the first HTML.
let signals = App { toasts: vec![Toast::new("Welcome back")] };
let html = Page::new("App", signals)
    .child(
        div().child(
            button(ButtonVariant::Default, ButtonSize::Default).on_click(call).text("Save"),
        ),
    )
    .child(Toaster::new(App::toasts()))
    .render();
let _app: Router = pages(Router::new());
# assert!(html.contains(r#"aria-atomic="true""#));
# assert!(html.contains(">Welcome back</p>"));

# use hypersig::Signals;
# use serde::{Deserialize, Serialize};
use axum::http::StatusCode;
use hypersig::html::{h1, p};
use hypersig::{Page, PageResponse, Route};

#[derive(Signals, Serialize, Deserialize)]
struct Order {
    total: i64,
}

#[derive(Signals, Serialize, Deserialize)]
struct Failure {}

# fn load_total(_id: i32) -> Result<i64, std::io::Error> { Ok(1200) }
hypersig::routes! {
    fn pages(Router<()>);
    const ORDER: Route<i32> = "/orders/{id}" => order;
}

// One error page that every handler can answer with.
fn error_page(status: StatusCode, message: &str) -> PageResponse {
    let page = Page::new("Something went wrong", Failure {})
        .child(h1().text("Something went wrong"))
        .child(p().text(message));
    PageResponse::with_status(status, page)
}

async fn order((): (), id: i32) -> PageResponse {
    let Ok(total) = load_total(id) else {
        return error_page(StatusCode::SERVICE_UNAVAILABLE, "Orders are unavailable. Try again later.");
    };
    Page::new("Order", Order { total }).child(p().bind_text(Order::total().to_text())).into()
}
# let _ = (pages, ORDER);

# use axum::extract::FromRequestParts;
# use axum::http::StatusCode;
# use axum::http::request::Parts;
# use futures::Stream;
# use hypersig::html::{div, p, span};
# use hypersig::{Page, Patches, Signals, StreamAction};
# use serde::{Deserialize, Serialize};
# use tokio::sync::broadcast;
#
#[derive(Signals, Serialize, Deserialize)]
struct Auction {
    price: i32,
    leading: bool, //+
}

# /// The highest bid so far.
# #[derive(Clone)]
# struct Bid {
#     amount: i32,
#     bidder: String,
# }
#
# #[derive(Clone)]
# struct AppState {
#     bids: broadcast::Sender<Bid>,
# }
#
# /// The signed-in member, read from the session cookie.
# struct Member(String);
#
# impl FromRequestParts<AppState> for Member {
#     type Rejection = StatusCode;
#
#     async fn from_request_parts(_: &mut Parts, _: &AppState) -> Result<Self, StatusCode> {
#         Ok(Member("ada".to_owned()))
#     }
# }
#
# hypersig::routes! {
#     fn pages(Router<AppState>);
#     stream BIDS: StreamAction<Auction> = "/bids" => bids;
# }
#
# let page = Page::new("Auction", Auction { price: 0, leading: false }).child(
div()
    .on_load(BIDS.call().reconnect())
    .child(span().bind_text(Auction::price().to_text()))
    .child(p().show(Auction::leading()).text("You are the highest bidder.")) //+
# );

// The stream knows whose tab it serves, so each tab receives its own answer. //+
//-fn bids(state: AppState, (): ()) -> impl Stream<Item = Patches<Auction>> {
fn bids(state: AppState, Member(me): Member, (): ()) -> impl Stream<Item = Patches<Auction>> { //+
    futures::stream::unfold(state.bids.subscribe(), move |mut bids| {
        let me = me.clone(); //+
        async move {
            let bid = bids.recv().await.ok()?;
            let price = Auction::price().patch(bid.amount);
//-            Some((price.into(), bids))
            let leading = Auction::leading().patch(bid.bidder == me); //+
            Some((price.and(leading).into(), bids)) //+
        }
    })
}
# let _ = (page.render(), pages);

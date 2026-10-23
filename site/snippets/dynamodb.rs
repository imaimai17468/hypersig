# use axum::Router;
# use axum::http::StatusCode;
# use axum::response::{IntoResponse, Response};
# use hypersig::html::{li, ul};
# use hypersig::{Page, Route, Signals};
# use serde::{Deserialize, Serialize};
# #[derive(Signals, Serialize, Deserialize)]
# struct Todos {
#     count: i32,
# }
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Clone)]
struct AppState {
    dynamo: Client,
}

struct DbError(String);

impl IntoResponse for DbError {
    fn into_response(self) -> Response {
        eprintln!("DynamoDB: {}", self.0);
        let message = "The list could not be read. Try again.";
        (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
    }
}

/// The titles of the todos under the partition key `list = "home"`.
async fn titles(state: &AppState) -> Result<Vec<String>, DbError> {
    let output = state
        .dynamo
        .query()
        .table_name("todos")
        .key_condition_expression("list = :list")
        .expression_attribute_values(":list", AttributeValue::S("home".to_owned()))
        .send()
        .await
        .map_err(|error| DbError(error.to_string()))?;
    let titles = output.items().iter().filter_map(|item| item.get("title")?.as_s().ok());
    Ok(titles.cloned().collect())
}

async fn index(state: AppState, (): ()) -> Result<Page<Todos>, DbError> {
    let titles = titles(&state).await?;
    let count = i32::try_from(titles.len()).unwrap_or(i32::MAX);
    let list = ul().children(titles.into_iter().map(|title| li().text(title)));
    Ok(Page::new("Todos", Todos { count }).child(list))
}

hypersig::routes! {
    fn pages(Router<AppState>);
    const INDEX: Route = "/" => index;
}

/// The region and the credentials come from the environment, as the AWS
/// CLI reads them.
async fn state() -> AppState {
    let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    AppState {
        dynamo: Client::new(&config),
    }
}
# let _ = (pages, state);

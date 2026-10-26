use tower::ServiceBuilder;
use vercel_runtime::axum::VercelLayer;

#[tokio::main]
async fn main() -> Result<(), vercel_runtime::Error> {
    let app = pages(Router::new()).hypersig_runtime();
    vercel_runtime::run(ServiceBuilder::new().layer(VercelLayer::new()).service(app)).await
}

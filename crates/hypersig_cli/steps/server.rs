# use axum::Router;
# async fn serve(app: Router) -> std::io::Result<()> {
// Listens on every interface at the port in `PORT`, which the Dockerfile
// sets, and on 127.0.0.1:3000 outside the container. Ctrl+C and the
// SIGTERM a host sends stop it.
hypersig::serve(app, 3000).await
# }

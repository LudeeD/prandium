use axum::Router;
use tower_http::services::ServeDir;
use crate::cookbook;

pub async fn start(cookbook: cookbook::PrandiumCookbook) {
    let output_folder = cookbook.config.get_output_folder();

    let serve_dir = ServeDir::new(output_folder);

    // build our application with a route
    let app = Router::new().nest_service("/", serve_dir);

    // run our app with hyper, listening globally on port 3000
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:8765").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

use axum::{extract::Path, response::Html, routing::{get, get_service}, Router};
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with a route
    let app = Router::new()
        .route("/:count", get(handler))
        .nest_service("/resources", get_service(ServeDir::new("./resources")));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler(Path(count): Path<String>) -> Result<Html<String>, String> {
    if !count.chars().all(|c| c.is_numeric()) {
        return Err(String::from("Parameter is not a unsigned int"));
    }
    let mut html_string = String::new();

    for digit in count.chars() {
        html_string.push_str(&format!("<img src=\"/resources/{}.gif\" border=\"0\" alt=\"{}\">", digit, digit));
    }

    // tracing::info!(html_string);

    Ok(Html(html_string))
}


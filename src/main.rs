use axum::{
    routing::get,
    Router,
    response::Html,
};
use std::fs;

#[tokio::main]
async fn main() {
    // Read the contents of the HTML file
    let html_content = fs::read_to_string("src/assets/template/home/index.html")
        .expect("Failed to read HTML file");

    // Build our application with a single route
    let app = Router::new().route("/", get(move || {
        // Return the HTML content as the response
        async move { Html(html_content.clone()) }
    }));

    // Run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

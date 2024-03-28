use axum::{
    extract::Path,
    response::{IntoResponse, Json},
    routing::get,
    Router,
};
use hyper::StatusCode;
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/test_json", get(get_test_json))
        .route("/test_path/:id", get(get_test_path))
        .route("/test_body/:id", get(get_test_body));

    let app = app.fallback(handler_404);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "nothing to see here")
}

async fn get_test_json() -> Json<Value> {
    Json(json!(
        {
            "success": true
        }
    ))
}

async fn get_test_path(Path(test_id): Path<String>) -> Json<Value> {
    Json(json!(
        {
            "success": true,
            "test_id": test_id,
        }
    ))
}

async fn get_test_body(Path(test_id): Path<String>, Json(payload): Json<Value>) -> Json<Value> {
    Json(json! {
        {
            "success": true,
            "test_id": test_id,
            "body": payload,
        }
    })
}

use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::{OpenApi, ToSchema};

#[derive(Serialize, ToSchema)]
pub struct HelloResponse {
    pub message: String,
}

/// GET /api/v1/customer/hello
#[utoipa::path(
    get,
    path = "/hello",
    responses(
        (status = 200, description = "Hello from Customer v1", body = HelloResponse)
    )
)]
async fn hello_handler() -> Json<HelloResponse> {
    Json(HelloResponse {
        message: "Hello, Customer v1!".into(),
    })
}

#[derive(OpenApi)]
#[openapi(components(schemas(HelloResponse)), paths(hello_handler))]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new().route("/hello", get(hello_handler))
}

use axum::Router;
use utoipa::OpenApi;

pub mod v1;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "api/v1", api = v1::ApiDoc)
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new().nest("/api/v1", v1::router())
}

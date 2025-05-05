use axum::Router;
use utoipa::OpenApi;

pub mod customer;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/customer", api = customer::ApiDoc)
    )
)]
pub struct ApiDoc;

pub fn router() -> Router {
    Router::new().nest("/customer", customer::router())
}

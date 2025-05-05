use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa_swagger_ui::SwaggerUi;

use internal::api;

static OPENAPI_JSON: &str = include_str!("../../../docs/openapi.json");

#[tokio::main]
async fn main() {
    let spec_value: serde_json::Value =
        serde_json::from_str(OPENAPI_JSON).expect("docs/openapi.json is invalid JSON");

    let swagger_router =
        SwaggerUi::new("/docs").external_url_unchecked("/api-doc/openapi.json", spec_value);

    let app = api::router().merge(swagger_router);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    println!("Start server");
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

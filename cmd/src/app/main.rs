use std::net::SocketAddr;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use internal::api;

#[derive(OpenApi)]
#[openapi(
    paths(
        api::v1::customer::hello_handler,      // экспортируем хэндлеры для схем
    ),
    components(schemas(
        api::v1::customer::HelloResponse       // и их схемы
    ))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    let app = api::router()
        // Swagger UI на /swagger-ui
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = SocketAddr::from(([0, 0, 0, 0], 8888));
    println!("Start server");
    let listener = TcpListener::bind(addr).await.unwrap();
    // free-function вместо метода Server::bind
    axum::serve(listener, app).await.unwrap();
}

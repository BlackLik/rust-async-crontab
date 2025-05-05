use std::{fs, path::Path};

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path="/", api=internal::api::ApiDoc)
    )
)]
struct ApiDoc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let spec = ApiDoc::openapi();

    let json = serde_json::to_string_pretty(&spec)?;

    let out_path = Path::new("docs").join("openapi.json");
    if let Some(dir) = out_path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&out_path, json)?;
    println!("✅ OpenAPI spec generated to {}", out_path.display());
    Ok(())
}

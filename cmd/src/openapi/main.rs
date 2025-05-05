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
    let yaml = ApiDoc::openapi().to_yaml()?;
    // let yaml = serde_yaml::to_string(&spec)?;

    let out_path = Path::new("docs").join("openapi.yaml");
    if let Some(dir) = out_path.parent() {
        fs::create_dir_all(dir)?;
    }
    fs::write(&out_path, yaml)?;
    println!("✅ OpenAPI spec generated to {}", out_path.display());
    Ok(())
}

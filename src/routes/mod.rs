use rocket::response::content;

pub mod org;

#[get("/info")]
pub fn info() -> content::Json<String> {
  let version = env!("CARGO_PKG_VERSION");
  let content = format!(
    "{{ service_name: 'org-api-svc', service_version: '{}' }}",
    version
  );
  content::Json(content)
}

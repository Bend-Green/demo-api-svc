use rocket::response::content;
use rocket::serde::json::{json, Json, Value};

use crate::dal;
use crate::dal::org::{NewOrg, UpdatedOrg };

#[post("/org", format = "json", data = "<new_org>")]
pub fn org_create(new_org: Json<NewOrg>) -> Value {
  let org = dal::org::Org::create(&*new_org);
  json!(org)
}

#[get("/org/list")]
pub fn org_read() -> Value {
  let orgs = dal::org::Org::read();
  json!(orgs)
}

#[put("/org", format = "json", data = "<updated_org>")]
pub fn org_update(updated_org: Json<UpdatedOrg>) -> Value {
  let org = dal::org::Org::update(&*updated_org);
  json!(org)
}

#[delete("/org/<id>")]
pub fn org_delete(id: &str) -> content::Json<String> {
  println!("Deleting org with id {}", id);
  let id_parse_result = id.parse::<i64>();

  let response = match id_parse_result {
    Ok(i64_id) => {
      let success = dal::org::Org::delete(i64_id);
      let content = format!("{{ success: {} }}", success);
      content::Json(content)
    }
    Err(_e) => {
      let content = format!("{{ error: '{}' }}", "Invalid id");
      content::Json(content)
    }
  };
  return response;
}

#[get("/org/<name>")]
pub fn org_by_name(name: &str) -> Value {
  println!("Finding org with name {}", &name);
  let org = dal::org::Org::get_by_name(name);
  json!(org)
}

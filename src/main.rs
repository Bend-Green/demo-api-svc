#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;

mod dal;
mod routes;
mod schema;

use routes::org::*;
use routes::*;

#[launch]
fn rocket() -> _ {
  rocket::build()
    .mount(
      "/api",
      routes![
        info,
        org_create,
        org_read,
        org_update,
        org_delete,
        org_by_name,
      ],
    )
}

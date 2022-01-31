#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Deserialize;

#[get("/test")]
fn index() -> &'static str {
    "Testing the server😁"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, add_todo])
}

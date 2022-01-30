#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Task {
    description: String,
    completed: bool,
}

#[get("/test")]
fn index() -> &'static str {
    "Testing the server😁"
}

#[post("/todo", format = "json", data = "<todo>")]
fn add_todo(todo: Json<Task>) {}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/api", routes![index, add_todo])
}

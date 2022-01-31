#[macro_use]
extern crate rocket;
use rusqlite;

#[get("/test")]
fn index() -> &'static str {
    "Testing the server😁"
}

#[launch]
fn rocket() -> _ {
    let db_connection = rusqlite::Connection::open("data.sqlite").unwrap();
    rocket::build().mount("/", routes![index])
}

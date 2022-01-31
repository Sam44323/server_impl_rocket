#[macro_use]
extern crate rocket;

#[get("/test")]
fn index() -> &'static str {
    "Testing the server😁"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

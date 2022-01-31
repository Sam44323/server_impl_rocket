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

    db_connection
        .execute(
            "create table if not exists todo_list (id integer primary key, item varchar(255) not null)",
            [],
        )
        .unwrap();

    rocket::build().mount("/", routes![index])
}

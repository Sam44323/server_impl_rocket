#[macro_use]
extern crate rocket;
use rusqlite::*;
use serde::*;

#[derive(Serialize)] // converting object to stream of bytes and then converting to actual data using deserialize
struct ToDoList {
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64,
    title: String,
}

#[derive(Serialize)]
struct StatusMessage {
    message: String,
}

#[get("/test")]
fn index() -> &'static str {
    "Testing the server😁"
}

#[launch]
fn rocket() -> _ {
    let db_connection = Connection::open("data.sqlite").unwrap();

    db_connection
        .execute(
            "create table if not exists todo_list 
            (id integer primary key,
            item varchar(255) not null
        );",
            [],
        )
        .unwrap();

    rocket::build().mount("/", routes![index])
}

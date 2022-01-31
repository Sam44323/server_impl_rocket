#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
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

#[get("/todos")]
fn fetch_all_todo_items() -> Result<Json<ToDoList>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to the database!".into()),
    };

    let mut statement = match db_connection.prepare("SELECT id, title FROM todo_list;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare the statement!".into()),
    };

    let result = statement.query_map([], |row| {
        Ok(ToDoItem {
            id: row.get(0)?, // this ? will automatically return and error if the value is not found
            title: row.get(1)?,
        })
    });

    match result {
        Ok(items) => {
            let collection: rusqlite::Result<Vec<_>> = items.collect();

            match collection {
                Ok(items) => Ok(Json(ToDoList { items })),
                Err(_) => Err("Failed to fetch todo items!".into()),
            }
        }
        Err(_) => Err("Failed to fetch todo items!".into()),
    }
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

    rocket::build().mount("/", routes![index, fetch_all_todo_items])
}

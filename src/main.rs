#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;
use rusqlite::*;
use serde::*;

mod routes;

#[derive(Serialize)] // converting object to stream of bytes and then converting to actual data using deserialize
struct ToDoList {
    items: Vec<ToDoItem>,
}

#[derive(Serialize)]
struct ToDoItem {
    id: i64,
    item: String,
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

    let mut statement = match db_connection.prepare("select id, item from todo_list;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let result = statement.query_map([], |row| {
        Ok(ToDoItem {
            id: row.get(0)?, // this ? will automatically return and error if the value is not found
            item: row.get(1)?,
        })
    });

    match result {
        Ok(items) => {
            let collection: Result<Vec<_>> = items.collect();

            match collection {
                Ok(items) => Ok(Json(ToDoList { items })),
                Err(_) => Err("Failed to fetch todo items!".into()),
            }
        }
        Err(_) => Err("Failed to fetch todo items!".into()),
    }
}

#[post("/add", data = "<item>", format = "json")]
fn add_todo_item(item: Json<String>) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to the database!".into()),
    };

    let mut statement =
        match db_connection.prepare("insert into todo_list (id, item) values (null, $1);") {
            Ok(statement) => statement,
            Err(_) => return Err("Failed to prepare query".into()),
        };

    let result = statement.execute(&[&item.0]); // .0 gives the data

    match result {
        Ok(rows_affected) => {
            return Ok(StatusMessage {
                message: format!("{} rows affected", rows_affected),
            }
            .into())
        }
        Err(_) => Err("Failed to insert todo item!".into()),
    }
}

#[delete("/todo/<id>")]
fn delete_todo_item(id: i64) -> Result<Json<StatusMessage>, String> {
    let db_connection = match Connection::open("data.sqlite") {
        Ok(connection) => connection,
        Err(_) => return Err("Failed to connect to the database!".into()),
    };

    let mut statement = match db_connection.prepare("delete from todo_list where id = $1;") {
        Ok(statement) => statement,
        Err(_) => return Err("Failed to prepare query".into()),
    };

    let result = statement.execute(&[&id]); // .0 gives the data

    match result {
        Ok(rows_affected) => {
            return Ok(StatusMessage {
                message: format!("{} rows affected", rows_affected),
            }
            .into())
        }
        Err(_) => Err("Failed to insert todo item!".into()),
    }
}

#[launch]
fn rocket() -> _ {
    {
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
    }
    rocket::build().mount(
        "/",
        routes![index, fetch_all_todo_items, add_todo_item, delete_todo_item],
    )
}

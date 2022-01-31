use rocket::serde::json::Json;
use rusqlite::*;

mod data;

/**
 * Route for testing-api uptime
 */

#[get("/")]
pub fn index() -> &'static str {
  "Testing the server"
}

/**
 * Route for getting all the todo
 */

#[get("/todos")]
pub fn fetch_all_todo_items() -> Result<Json<data::ToDoList>, String> {
  let db_connection = match Connection::open("data.sqlite") {
    Ok(connection) => connection,
    Err(_) => return Err("Failed to connect to the database!".into()),
  };

  let mut statement = match db_connection.prepare("select id, item from todo_list;") {
    Ok(statement) => statement,
    Err(_) => return Err("Failed to prepare query".into()),
  };

  let result = statement.query_map([], |row| {
    Ok(data::ToDoItem {
      id: row.get(0)?, // this ? will automatically return data or an error if the value is not found
      item: row.get(1)?,
    })
  });

  match result {
    Ok(items) => {
      let collection: Result<Vec<_>> = items.collect();

      match collection {
        Ok(items) => Ok(Json(data::ToDoList { items })),
        Err(_) => Err("Failed to fetch todo items!".into()),
      }
    }
    Err(_) => Err("Failed to fetch todo items!".into()),
  }
}

/**
 * Route for adding a new todo
 */

#[post("/add", data = "<item>", format = "json")]
pub fn add_todo_item(item: Json<String>) -> Result<Json<data::StatusMessage>, String> {
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
      return Ok(
        data::StatusMessage {
          message: format!("{} rows affected", rows_affected),
        }
        .into(),
      )
    }
    Err(_) => Err("Failed to insert todo item!".into()),
  }
}

/**
 * Route for deleting a todo
 */

#[delete("/todo/<id>")]
pub fn delete_todo_item(id: i64) -> Result<Json<data::StatusMessage>, String> {
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
      return Ok(
        data::StatusMessage {
          message: format!("{} rows affected", rows_affected),
        }
        .into(),
      )
    }
    Err(_) => Err("Failed to insert todo item!".into()),
  }
}

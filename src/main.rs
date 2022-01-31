#[macro_use]
extern crate rocket;
use rusqlite::*;

mod routes;

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
        routes![
            routes::index,
            routes::fetch_all_todo_items,
            routes::add_todo_item,
            routes::delete_todo_item
        ],
    )
}

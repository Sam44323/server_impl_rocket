#[macro_use]
extern crate rocket;

mod routes;

mod database;

#[launch]
fn rocket() -> _ {
    // connecting to database
    database::connect_db();

    rocket::build().mount(
        "/api",
        routes![
            routes::index,
            routes::fetch_all_todo_items,
            routes::add_todo_item,
            routes::delete_todo_item
        ],
    )
}

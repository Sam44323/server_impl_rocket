use rusqlite::*;

pub fn connect_db() {
  // creating a db connection
  let db_connection = Connection::open("data.sqlite").unwrap();

  // creating a table for the database
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

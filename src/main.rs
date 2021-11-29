#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

// Utils
pub mod util;

mod user;
//mod content;

#[launch]
fn rocket() -> _ {
    {
        let db_connection = rusqlite::Connection::open("data.sqlite").unwrap();

        db_connection
            .execute_batch(
                "BEGIN;
        CREATE TABLE IF NOT EXISTS users (
            id	INTEGER NOT NULL,
            username	TEXT NOT NULL,
            email       TEXT NOT NULL,
            password	TEXT NOT NULL,
            PRIMARY KEY(id AUTOINCREMENT)
        );
        COMMIT;",
            )
            .unwrap();
    }

    rocket::build().mount("/user", user::routes())
}

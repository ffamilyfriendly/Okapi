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
                    id	INTEGER NOT NULL, /* id of user, going to be the relative tie to anything user related */
                    flag INTEGER, /* flag of int type. Going to specify if user is entitled to perms/functions like admin and whatevs */
                    username	TEXT NOT NULL, /* the name that will displayed, cannot be logged in with */
                    email       TEXT NOT NULL UNIQUE, /* the users email, will be used to log in. */
                    password	TEXT NOT NULL, /* password crypt. password will NEVER be stored plain */
                    PRIMARY KEY(id AUTOINCREMENT)
                );
            COMMIT;",
            )
            .unwrap();
    }

    rocket::build().mount("/user", user::routes())
}

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use serde::{Deserialize};

#[derive(Deserialize)]
pub struct config {
    hostname: String,
    invite_only: bool
}

pub fn get_config() -> config {
    let config_str = std::fs::read_to_string("./config.json");
    if config_str.is_err() {
        println!("could not find config.json");
    }
    let parsed = serde_json::from_str(&config_str.unwrap());
    if parsed.is_err() {
        println!("config file has faulty structure");
    }
    parsed.unwrap()
}

// Utils
pub mod util;

pub mod user;
mod invite;
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

                CREATE TABLE IF NOT EXISTS invites (
                    id	TEXT NOT NULL UNIQUE,
                    created_by	INTEGER NOT NULL, /* id of the user who created the invite */
                    user_flag INTEGER, /* gives user these perms after signup */
                    expires	INTEGER, /* unix timestamp when invite expires. If invite does not expire with time, set to -1 */
                    uses	INTEGER, /* how many times the invite can be used. If it can be used unlimited times, set to -1 */
                    PRIMARY KEY(id)
                );
            COMMIT;",
            )
            .unwrap();
    }

    let cnf: config = get_config();

    rocket::build()
        .mount("/user", user::routes())
        .mount("/invite", invite::routes())
        .manage(cnf)
}

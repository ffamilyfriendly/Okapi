#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    hostname: String,
    invite_only: bool
}

pub fn get_config() -> Config {
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

use rocket::serde::json::Json;

#[get("/")]
pub fn get_config_http() -> Json<Config> {
    get_config().into()
}

// Utils
pub mod util;

pub mod user;
pub mod invite;
pub mod content;
//mod content;

// cors
mod cors;

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

                CREATE TABLE IF NOT EXISTS entities (
                    id	TEXT NOT NULL UNIQUE,
                    parent	TEXT,
                    next    TEXT,
                    flag	INTEGER NOT NULL,
                    type	TEXT NOT NULL,
                    creator_uid	INTEGER NOT NULL,
                    position	INTEGER,
                    PRIMARY KEY(id)
                );

                CREATE TABLE IF NOT EXISTS metadata (
                    id	TEXT NOT NULL UNIQUE,
                    thumbnail	TEXT,
                    banner	TEXT,
                    description	TEXT,
                    name	TEXT,
                    rating	INTEGER,
                    age_rating	TEXT,
                    language	TEXT,
                    year	INTEGER,
                    PRIMARY KEY(id)
                );

                CREATE TABLE IF NOT EXISTS sources (
                    id	TEXT NOT NULL UNIQUE,
                    parent	TEXT NOT NULL,
                    path	TEXT NOT NULL,
                    position	INTEGER,
                    PRIMARY KEY(id)
                );

                CREATE TABLE IF NOT EXISTS lastwatched (
                    id TEXT NOT NULL UNIQUE,
                    cid TEXT NOT NULL,
                    uid TEXT NOT NULL,
                    timestamp REAL NOT NULL,
                    PRIMARY KEY(id)
                );

                CREATE TABLE IF NOT EXISTS sessions (
                    token TEXT NOT NULL UNIQUE,
                    user INTEGER NOT NULL,
                    created TEXT NOT NULL,
                    PRIMARY KEY(token)
                );

            COMMIT;",
            )
            .unwrap();
    }

    let cnf: Config = get_config();

    rocket::build()
        .mount("/", routes![get_config_http])
        .mount("/user", user::routes())
        .mount("/invite", invite::routes())
        .mount("/content", content::routes())
        .attach(cors::CORS)
        .manage(cnf)
}

use crate::util;
use rocket::response::status::{Created};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;
use rusqlite::{Connection};

#[derive(Validate, Serialize, Deserialize)]
pub struct NewUser {
    #[validate(email(message = "that is not what an email adress looks like, dummy"))]
    email: String,
    #[validate(length(
        min = 0,
        max = 25,
        message = "your username must be no longer than 25 characters"
    ))]
    username: String,
    #[validate(length(min = 5, message = "your password must be 5 characters or longer"))]
    password: String,
}

#[post("/", data = "<input>")]
pub fn new_user(input: Json<NewUser>) -> Result<Created<String>, util::ferr::Ferr> {
    // Validates the request. If fields does not satisfy validation tell client to fix their shit
    input.validate().map_err(util::validate::verify_respond)?;

    // Establish db connection
    let con = match Connection::open("data.sqlite") {
        Ok(connection) => connection, // Established!
        Err(_) => return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(500), err_msg: "db fail idk".to_string() })
        // ^^^ could not establish; tell user in a very helpfull way that something done goofed
    };

    // call password hashing function here and exchange "test" below for that value 

    // insert new user into db
    match con.execute("INSERT INTO users (email, username, password) VALUES (?, ?, ?) ", [ &input.email, &input.username, "test" ]) {
        Ok(_) => return Ok(Created::new(format!("{}/user/me", "HOSTNAME"))), // User created! Might instead resolve to login page
        // vvv something went wrong when creating account. Normally this might be UNIQUE constraint on email col failing, otherwise assume server err
        Err(e) => return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(500), err_msg: match e.to_string().as_str() {
            "UNIQUE constraint failed: users.email" => util::ferr::json_err("that email is already in use".into(), "emailAlreadyUsed".into()),
            _ => util::ferr::json_err("something went wrong".into(), "serverError".into())
        } } )
    }
}

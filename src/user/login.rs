//extern crate argon2;
use crate::{util};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;
use rusqlite::{Connection};
use argon2::{
    password_hash::{
        PasswordVerifier, PasswordHash
    },
    Argon2
};

#[derive(Validate, Serialize, Deserialize)]
pub struct UserLogin {
    #[validate(email(message = "that is not what an email adress looks like, dummy"))]
    email: String,
    #[validate(length(min = 5, message = "your password must be 5 characters or longer"))]
    password: String,
}

use crate::user;

#[post("/login", data = "<input>")]
pub fn login(input: Json<UserLogin>) -> Result<String, util::ferr::Ferr> {
    // Validates the request. If fields does not satisfy validation tell client to fix their shit
    input.validate().map_err(util::validate::verify_respond)?;

    // Establish db connection
    let con = match Connection::open("data.sqlite") {
        Ok(connection) => connection, // Established!
        Err(_) => return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(500), err_msg: "db fail idk".to_string() })
    };

    let curr_user: user::userutil::CoolStructThing = match con.query_row("SELECT * FROM users WHERE email = ?", [&input.email], |row| user::userutil::get_cool_struct_thing(&row) ) {
        Ok(v) => v,
        Err(_) => return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(401), err_msg: "Unauthorized".to_string() })
    };

    let hash: PasswordHash = match PasswordHash::new(&curr_user.token) {
        Ok(v) => v,
        _ => return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(500), err_msg: "could not generate hash".to_string() })
    };

    if Argon2::default().verify_password(&input.password.as_bytes(), &hash).is_ok() {
        return Ok(user::userutil::genToken(&curr_user.claim));
    } else {
        return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(401), err_msg: "Unauthorized".to_string() })
    }    
}

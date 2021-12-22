//extern crate argon2;
use crate::{util,config};
use rocket::State;
use rocket::response::status::{Created};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;
use rusqlite::{Connection};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Argon2
};

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
    #[validate(length(min = 3, message = "your invite code must be 3 characters or longer"))]
    invite: Option<String>
}

pub fn hash_password(psw: &String) -> Result<String,argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(psw.as_bytes(), &salt)?.to_string())
}

#[post("/", data = "<input>")]
pub fn new_user(state: &State<crate::config> ,input: Json<NewUser>) -> Result<Created<String>, util::ferr::Ferr> {
    // Validates the request. If fields does not satisfy validation tell client to fix their shit
    input.validate().map_err(util::validate::verify_respond)?;
    
    if state.inner().invite_only && input.invite.is_none() {
        return Err(util::ferr::q_err(403, &util::ferr::json_err("invite only. Provide an invite code".into(), "inviteonly".into())))
    }


    // Establish db connection
    let con = match Connection::open("data.sqlite") {
        Ok(connection) => connection, // Established!
        Err(_) => return Err(util::ferr::q_err(500, "db fail idk"))
        // ^^^ could not establish; tell user in a very helpfull way that something done goofed
    };

    // hash the password
    let hash = hash_password( &input.password );
    if !hash.is_ok() { return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(500), err_msg: "could not generate password hash".into() }) }

    // insert new user into db
    match con.execute("INSERT INTO users (email, username, password) VALUES (?, ?, ?) ", [ &input.email, &input.username, &hash.unwrap() ]) {
        Ok(_) => return Ok(Created::new(format!("{}/user/me", &state.inner().hostname))), // User created! Might instead resolve to login page
        // vvv something went wrong when creating account. Normally this might be UNIQUE constraint on email col failing, otherwise assume server err
        Err(e) => return Err(util::ferr::Ferr { err_type: rocket::http::Status::new(500), err_msg: match e.to_string().as_str() {
            "UNIQUE constraint failed: users.email" => util::ferr::json_err("that email is already in use".into(), "emailAlreadyUsed".into()),
            _ => util::ferr::json_err("something went wrong".into(), "serverError".into())
        } } )
    }
}
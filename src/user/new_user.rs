use crate::util;
use rocket::response::status::{Created};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use validator::Validate;

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

    let pswd: &String = &input.password;
    if pswd == "cummmmmmmmmm" {
        return Err(util::ferr::Ferr { err_msg: "aaa".to_string(), err_type: rocket::http::Status::Ok });
    }

    Ok(Created::new(format!("{}/user/me", "HOSTNAME")))
}

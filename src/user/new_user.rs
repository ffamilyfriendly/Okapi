//extern crate argon2;
use crate::{util,Config,invite, user};
use rocket::State;
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
    #[validate(length(min = 3, message = "your invite code must be 3 characters or longer"))]
    invite: Option<String>
}

#[post("/", data = "<input>")]
pub fn new_user(state: &State<Config> ,input: Json<NewUser>) -> Result<Created<String>, util::ferr::Ferr> {
    // Validates the request. If fields does not satisfy validation tell client to fix their shit
    input.validate().map_err(util::validate::verify_respond)?;
    
    if state.inner().invite_only && input.invite.is_none() {
        return Err(util::ferr::q_err(403, &util::ferr::json_err("invite only. Provide an invite code".into(), "inviteonly".into())))
    }

    match user::manager::get_user(input.email.clone()) {
        Some(_) => return Err(util::ferr::q_err(401, "that email is already in use")),
        None => { }
    }

    let mut user_flag: u16 = 0;

    if input.invite.is_some() {
        match invite::manager::get_invite(&input.invite.as_ref().unwrap()) {
            Some(mut inv) => {
                user_flag = inv.user_flag;
                inv.Use();
            },
            None => {
                if state.inner().invite_only {
                    return Err(util::ferr::q_err(403, "invalid invite code"))
                }
            }
        }
    }

    match user::manager::new_user(input.email.clone(), input.username.clone(), input.password.clone(), user_flag) {
        Some(_) => Ok(Created::new(format!("{}/user/me", &state.inner().hostname))),
        None => Err(util::ferr::q_err(500, "something went wrong"))
    }
}
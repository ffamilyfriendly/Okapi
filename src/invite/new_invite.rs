use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use serde::{Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::State;

#[derive(Serialize, Deserialize)]
pub struct NewInvite {
    user_flag: Option<u16>,
    expires: Option<u64>,
    uses: Option<i16>
}

#[derive(Serialize)]
pub struct CreatedInvite {
    url: String
}

#[post("/", data="<input>")]
pub fn new_invite(state: &State<crate::Config>, user: Token, input: Json<NewInvite>) -> Result<Json<CreatedInvite>, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::GenerateInvite) {
        return Err(ferr::q_err(403, "endpoint requires GenerateInvite permission"))
    }

    let mut user_flag: u16 = permissions::UserPermissions::PrivateContent as u16;

    // if user is admin and user_flag does exist in request allow the user_flag u16 to be modified
    if permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) && input.user_flag.is_some() {
        user_flag = input.user_flag.unwrap();
    }

    let expires: u64 = input.expires.unwrap_or(0);
    let uses: i16 = input.uses.unwrap_or(-1);
    let created_by = user.0.uid;

    match crate::invite::manager::generate_invite(created_by, user_flag, expires, uses) {
        Ok(id) => Ok( (CreatedInvite { url: format!("{}/invite/{}", state.inner().hostname, id) }).into() ),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}
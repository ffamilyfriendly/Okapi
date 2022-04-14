use crate::util::{ ferr, permissions };
use crate::invite::manager;
use rocket::serde::json::Json;
use crate::user::userutil::Token;

#[get("/<id>")]
pub fn get_invite(id: String) -> Result<Json<manager::Invite>, ferr::Ferr > {
    let inv = match manager::get_invite(&id) {
        Some(i) => i,
        None => return Err(ferr::q_err(404, "invite not found"))
    };

    Ok(inv.into())
}

#[get("/all/all")]
pub fn get_invites(user: Token) -> Result<Json<Vec<manager::Invite>>, ferr::Ferr > {
    match manager::get_invites(match permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        true => None,
        false => Some(user.0.uid)
    }) {
        Ok(v) => Ok(v.into()),
        Err(_) => Err(ferr::q_err(500, "Something went wrong"))
    }
}
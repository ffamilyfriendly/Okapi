//extern crate argon2;
use crate::util::{ ferr, permissions };
use crate::user::manager;
use crate::user::userutil::Token;

#[post("/logout/<id>")]
pub fn logout(user: Token, id: String) -> Result<String, ferr::Ferr> {
    let decoded = Token(id.as_str().into());

    if decoded.0.uid != user.0.uid && !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "You cannot delete that session. You need to be the account owner or have Administrator permissions"));
    }

    match manager::dereg_session(id.clone()) {
        Ok(_) => Ok(ferr::json_err("session removed".into(), id)),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}

#[post("/clearsessions/<id>")]
pub fn logout_all(user: Token, id: u16) -> Result<String, ferr::Ferr> {
    if id != user.0.uid && !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "You cannot delete those sessions. You need to be the account owner or have Administrator permissions"));
    }

    match manager::clear_all_by_user(id) {
        Ok(_) => Ok(ferr::json_err("sessions removed".into(), id.to_string())),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}
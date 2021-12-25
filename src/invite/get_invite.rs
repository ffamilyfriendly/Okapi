use crate::util::{ ferr };
use crate::invite::manager;
use rocket::serde::json::Json;

#[get("/<id>")]
pub fn get_invite(id: String) -> Result<Json<manager::Invite>, ferr::Ferr > {
    let inv = match manager::get_invite(&id) {
        Some(i) => i,
        None => return Err(ferr::q_err(404, "invite not found"))
    };

    Ok(inv.into())
}
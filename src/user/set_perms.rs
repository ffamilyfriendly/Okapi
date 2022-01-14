use crate::user::userutil::{ Token };
use crate::user::manager;
use crate::util::{ permissions, ferr };
use serde::{Deserialize};
use rocket::response::status::NoContent;
use rocket::serde::json::Json;

#[derive(Deserialize)]
pub struct Perms {
    flag: u16
}

#[patch("/<id>/flag", data = "<input>")]
pub fn set_perms(user: Token, id: u16, input: Json<Perms>) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "endpoint requires Administrator permission"))
    }

    match manager::set_flag(id, input.flag) {
        Ok(_) => Ok(NoContent),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}

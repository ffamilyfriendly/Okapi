use crate::user::userutil::{ Token };
use crate::user::manager;
use crate::util::{ permissions, ferr };
use rocket::serde::json::Json;

#[get("/all")]
pub fn get_users(user: Token) -> Result<Json<Vec<manager::User>>, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "endpoint requires Administrator permission"))
    }

    match manager::get_users() {
        Ok(v) => Ok(v.into()),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}

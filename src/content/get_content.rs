use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use crate::content::{ manager };
use rocket::serde::json::Json;

#[get("/<id>")]
pub fn get_content(user: Option<Token>, id: String) -> Result<Json<manager::Entity>, ferr::Ferr> {
    if user.is_some() {
        let t = user.unwrap();
        if !permissions::has_permission(t.0.permissions, permissions::UserPermissions::PrivateContent) {
            return Err(ferr::q_err(403, "endpoint requires PrivateContent permission"))
        }
        match manager::get_collection(id, false) {
            Some(v) => Ok(v.into()),
            None => Err(ferr::q_err(404, "not found"))
        }
    } else {
        match manager::get_collection(id, true) {
            Some(v) => Ok(v.into()),
            None => Err(ferr::q_err(404, "not found"))
        }
    }
}
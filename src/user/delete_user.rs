use crate::util::{ ferr, permissions };
use crate::user::userutil::Token;
use crate::user::manager;
use rocket::response::status::NoContent;


#[delete("/<id>")]
pub fn delete_user(id: u16, user: Token) -> Result<NoContent, ferr::Ferr> {
    if id != user.0.uid && !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "You cannot delete that account. You need to be the account owner or have Administrator permissions"));
    }

    match manager::delete_user(id) {
        Ok(_) => Ok(NoContent),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}
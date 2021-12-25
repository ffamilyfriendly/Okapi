use crate::user::userutil::Token;
use crate::util::{ ferr, permissions };
use crate::invite::manager;
use rocket::response::status::NoContent;

#[delete("/<id>")]
pub fn delete_invite(user: Token, id: String) -> Result<NoContent, ferr::Ferr> {
    let inv: manager::Invite = match manager::get_invite(&id) {
        Some(i) => i,
        None => return Err(ferr::q_err(404, "no such invite found"))
    };

    if inv.created_by != user.0.uid && !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "you are not allowed to delete this invite. You need to be it's creator or an admin to proceed"));
    }

    match manager::delete_invite(&id) {
        Ok(_) => Ok(NoContent),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}
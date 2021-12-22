use crate::util::{ ferr, permissions };
use crate::user::userutil::Token;
use rocket::response::status::NoContent;
use rusqlite::Connection;


#[delete("/<id>")]
pub fn delete_user(id: u16, user: Token) -> Result<NoContent, ferr::Ferr> {
    if id != user.0.uid && !permissions::has_permission(user.0.permissions, permissions::UserPermissions::Administrator) {
        return Err(ferr::q_err(403, "You cannot delete that account. You need to be the account owner or have Administrator permissions"));
    }

    let connection = match Connection::open("data.sqlite") {
        Ok(con) => con,
        Err(_) => return Err(ferr::q_err(500, "something went wrong"))
    };

    match connection.execute("DELETE FROM users WHERE id = ?", [&id]) {
        Ok(_) => Ok(NoContent),
        Err(_) => return Err(ferr::q_err(500, "something went wrong"))
    }
}
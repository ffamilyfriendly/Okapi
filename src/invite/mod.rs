use rocket::Route;
mod new_invite;
mod delete_invite;
mod get_invite;
pub mod manager;

pub fn routes() -> Vec<Route> {
    routes![new_invite::new_invite, delete_invite::delete_invite, get_invite::get_invite, get_invite::get_invites]
}

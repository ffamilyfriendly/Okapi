use rocket::Route;
mod new_invite;
pub mod manager;

pub fn routes() -> Vec<Route> {
    routes![new_invite::new_invite]
}

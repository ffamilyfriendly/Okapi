use rocket::Route;

mod new_user;
mod login;
pub mod userutil;

pub fn routes() -> Vec<Route> {
    routes![new_user::new_user, login::login]
}

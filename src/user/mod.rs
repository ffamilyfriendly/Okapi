use rocket::Route;

mod new_user;
mod delete_user;
mod login;
pub mod userutil;

pub fn routes() -> Vec<Route> {
    routes![new_user::new_user, delete_user::delete_user, login::login]
}

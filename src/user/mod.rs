use rocket::Route;

mod new_user;
mod delete_user;
mod login;
mod set_perms;
mod get_users;
pub mod userutil;
pub mod manager;

pub fn routes() -> Vec<Route> {
    routes![new_user::new_user, delete_user::delete_user, login::login, set_perms::set_perms, get_users::get_users]
}

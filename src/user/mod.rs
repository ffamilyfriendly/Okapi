use rocket::Route;

mod new_user;

pub fn routes() -> Vec<Route> {
    routes![new_user::new_user]
}

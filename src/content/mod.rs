use rocket::Route;
pub mod manager;
pub mod new_content;
pub mod get_content;

pub fn routes() -> Vec<Route> {
    routes![new_content::new_content, new_content::new_metadata, new_content::new_source, get_content::get_content]
}

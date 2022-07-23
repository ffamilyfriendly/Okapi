use rocket::Route;
mod get_metadata;
pub mod manager;

pub fn routes() -> Vec<Route> {
    routes![
        get_metadata::search_metadata
    ]
}

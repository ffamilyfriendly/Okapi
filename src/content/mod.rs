use rocket::Route;
pub mod manager;
pub mod new_content;
pub mod get_content;
pub mod delete_content;
pub mod edit_content;

pub fn routes() -> Vec<Route> {
    routes![
        // Creation of content
        new_content::new_content, new_content::new_metadata, new_content::new_source,
        // Editing of content
        edit_content::edit_entity, edit_content::edit_metadata, edit_content::edit_source,
        // Get content
        get_content::get_content, get_content::get_multiple, get_content::get_sources,
        // Deletion of content
        delete_content::delete_entity, delete_content::delete_metadata, delete_content::delete_source, delete_content::delete_sources
    ]
}

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
        edit_content::edit_entity, edit_content::edit_metadata, edit_content::edit_source, edit_content::get_last_watched,
        // Get content
        get_content::get_content, get_content::get_multiple, get_content::get_sources, get_content::search_collections,
        // Deletion of content
        delete_content::delete_entity, delete_content::delete_metadata, delete_content::delete_source, delete_content::delete_sources,
        // Sources
        get_content::get_source_info, get_content::get_source_media, get_content::get_last_watched, get_content::get_files_in_dir
    ]
}

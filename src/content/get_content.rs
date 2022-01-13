use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use crate::content::{ manager };
use rocket::serde::json::Json;

#[get("/<id>")]
pub fn get_content(user: Option<Token>, id: String) -> Result<Json<manager::Entity>, ferr::Ferr> {
    let public: bool = match user.is_some() {
        true => {
            let t = user.unwrap();
            !permissions::has_permission(t.0.permissions, permissions::UserPermissions::PrivateContent)
        },
        false => true
    };

    match manager::get_collection(id, public) {
        Some(v) => Ok(v.into()),
        None => Err(ferr::q_err(404, "not found"))
    }
}

#[get("/<parent>/children")]
pub fn get_multiple(user: Option<Token>, parent: String) -> Result<Json<Vec<manager::Entity>>, ferr::Ferr> {
    let public: bool = match user.is_some() {
        true => {
            let t = user.unwrap();
            !permissions::has_permission(t.0.permissions, permissions::UserPermissions::PrivateContent)
        },
        false => true
    };

    match manager::get_collection_list(&parent, public) {
        Ok(v) => Ok(v.into()),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}

#[get("/<parent>/sources")]
pub fn get_sources(user: Option<Token>, parent: String) -> Result<Json<Vec<manager::Source>>, ferr::Ferr> {

    let public: bool = match user.is_some() {
        true => {
            let t = user.unwrap();
            !permissions::has_permission(t.0.permissions, permissions::UserPermissions::PrivateContent)
        },
        false => true
    };

    match manager::get_collection(parent, public) {
        Some(col) => Ok(col.sources.unwrap_or(vec![]).into()),
        None => Ok(vec![].into())
    }
}

#[get("/search?<name>&<description>&<entity_type>")]
pub fn search_collections(user: Option<Token>, name: Option<String>, description: Option<String>, entity_type: Option<String>) -> Result<Json<Vec<manager::Entity>>, ferr::Ferr> {
    let public: bool = match user.is_some() {
        true => {
            let t = user.unwrap();
            !permissions::has_permission(t.0.permissions, permissions::UserPermissions::PrivateContent)
        },
        false => true
    };

    match manager::search_collection(name, description, entity_type, public) {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(ferr::q_err(500, &e.to_string()))
    }
}
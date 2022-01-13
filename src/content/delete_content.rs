use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use crate::content::{ manager };
use rocket::response::status::NoContent;
use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub parent: Option<String>,
    pub flag: u16,
    pub entity_type: String,
    pub position: Option<u16>,
    pub next: Option<String>
}


// as objects related to a deleted entity are useless we will also remove all tied objects in this endpoint
#[delete("/<id>/entity")]
pub fn delete_entity(user: Token, id: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    match manager::delete_collection(&id) {
        Ok(_) => Ok(NoContent),
        Err(_) => return Err(ferr::q_err(500, "something went wrong"))
    }
}

#[delete("/<id>/metadata")]
pub fn delete_metadata(user: Token, id: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    match manager::delete_metadata(&id) {
        Ok(_) => Ok(NoContent),
        Err(e) => return Err(ferr::q_err(500, &e.to_string()))
    }
}

// the below looks dumb but otherwise rocket will scream and be angry at me
#[delete("/<id>/source")]
pub fn delete_source(user: Token, id: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    match manager::delete_source(&id) {
        Ok(_) => Ok(NoContent),
        Err(_) => return Err(ferr::q_err(500, "something went wrong"))
    }
}

#[delete("/<parent>/sources")]
pub fn delete_sources(user: Token, parent: String) -> Result<NoContent, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    match manager::delete_sources(&parent) {
        Ok(_) => Ok(NoContent),
        Err(_) => return Err(ferr::q_err(500, "something went wrong"))
    }
}
use crate::user::userutil::{ Token };
use crate::util::{ permissions, ferr };
use crate::content::{ manager };
use rocket::response::status::Created;
use serde::{ Deserialize, Serialize };
use rocket::serde::json::Json;
use rocket::State;

#[derive(Serialize, Deserialize, Clone)]
pub struct Entity {
    pub parent: Option<String>,
    pub flag: u16,
    pub entity_type: String,
    pub position: Option<u16>,
    pub next: Option<String>
}

#[post("/entity", data="<input>")]
pub fn new_content(state: &State<crate::Config>, user: Token, input: Json<Entity>) -> Result<Created<String>, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    let next = input.next.as_ref().unwrap_or(&"".to_string()).to_string();
    let parent = input.parent.as_ref().unwrap_or(&"root".to_string()).to_string();
    let entity_type: manager::EntityType = (&input.entity_type).into();

    let ent = match manager::generate_entity(input.flag, entity_type, user.0.uid, input.position, Some(parent), Some(next)) {
        Ok(v) => v,
        Err(e) => return Err(ferr::q_err(500, &e.to_string()))
    };

    Ok(Created::new(format!("{}/content/entity/{}", state.inner().hostname, ent.id)))
}

#[post("/metadata", data="<input>")]
pub fn new_metadata(state: &State<crate::Config>, user: Token, input: Json<manager::MetaData>) -> Result<Created<String>, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    match manager::generate_metadata(&input.parent, &input.thumbnail, &input.banner, &input.description, &input.name, input.rating, &input.age_rating, &input.language, input.year) {
        Ok(_) => Ok(Created::new(format!("{}/content/metadata/{}", state.inner().hostname, input.parent))),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    pub parent: String,
    pub path: String,
    pub position: Option<u16>
}

#[post("/source", data="<input>")]
pub fn new_source(state: &State<crate::Config>, user: Token, input: Json<Source>) -> Result<String, ferr::Ferr> {
    if !permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        return Err(ferr::q_err(403, "endpoint requires ManageContent permission"))
    }

    match manager::generate_source(&input.parent, &input.path, input.position) {
        Ok(_) => Ok(format!("{}/content/source/{}", state.inner().hostname, input.parent)),
        Err(_) => Err(ferr::q_err(500, "something went wrong"))
    }
}
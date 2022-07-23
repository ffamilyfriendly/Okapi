use crate::user::userutil::{ Token };
use crate::Config;
use crate::util::{ permissions, ferr };
use crate::content::{ manager as content_manager };
use crate::metadata::{ manager };
use rocket::serde::json::Json;
use rocket::State;
extern crate rocket;

#[get("/audio?<query>")]
pub async fn search_metadata(user: Token, query: String) -> Result<Json<Vec<content_manager::MetaData>>, ferr::Ferr> {
    match permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        true => { },
        false => return Err(ferr::q_err(403, "you need ManageContent"))
    }


    match manager::get_audio_metadata(query).await {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(ferr::q_err(500, &e.to_string()))
    }
}

//state: &State<Config>

#[get("/movie?<query>")]
pub async fn search_movie_metadata(user: Token, query: String, state: &State<Config>) -> Result<Json<Vec<content_manager::MetaData>>, ferr::Ferr> {
    match permissions::has_permission(user.0.permissions, permissions::UserPermissions::ManageContent) {
        true => { },
        false => return Err(ferr::q_err(403, "you need ManageContent"))
    }


    match manager::get_movie_metadata(query, &state.tmdb_key).await {
        Ok(v) => Ok(v.into()),
        Err(e) => Err(ferr::q_err(500, &e.to_string()))
    }
}
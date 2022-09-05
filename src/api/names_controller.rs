use crate::{
    config::db::Pool,
    constants,
    models::{
        names::{NamesRepo, NewNameDTO},
        response::ResponseBody,
    },
    services::names_service,
};
use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Params {
    pub ticker: String,
}

// pub async fn get_all(pool: web::Data<Pool>) -> Result<HttpResponse> {
//     let repo = NamesRepo::new(pool);

//     match names_service::get_all(repo) {
//         Ok(stocks) => Ok(HttpResponse::Ok().json(ResponseBody::new(constants::MESSAGE_OK, stocks))),
//         Err(err) => Ok(err.response()),
//     }
// }

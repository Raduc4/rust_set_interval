use crate::{
    constants,
    error::ServiceError,
    models::names::{Name, NamesRepoTrait, NewNameDTO},
};
use actix_web::http::StatusCode;

pub fn get_all(sentiment_repo: impl NamesRepoTrait) -> Result<Vec<Name>, ServiceError> {
    match sentiment_repo.get_all() {
        Ok(item) => Ok(item),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

pub fn create_name(
    names_repo: impl NamesRepoTrait,
    new_name: NewNameDTO,
) -> Result<i32, ServiceError> {
    match names_repo.create(new_name) {
        Ok(item) => Ok(item),
        Err(_) => Err(ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_CAN_NOT_INSERT_DATA.to_string(),
        )),
    }
}

use crate::{AppError, AppResult, MessageResponse};
use serde::Serialize;

pub fn handle_response<T, E>(res: Result<T, E>) -> AppResult
where
    T: Serialize,
    E: std::error::Error + 'static,
    AppError: From<E>,
{
    match res {
        Ok(l) => Ok(MessageResponse::new(Some(
            serde_json::to_value(&l).unwrap(),
        ))),
        Err(e) => Err(e.into()),
    }
}

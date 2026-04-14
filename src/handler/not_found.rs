use crate::handler::error_messages::NOT_FOUND;
use worker::*;

pub fn handle() -> Result<Response> {
    Response::error(NOT_FOUND, 404)
}

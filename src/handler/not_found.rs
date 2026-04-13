use worker::*;

pub fn handle() -> Result<Response> {
    Response::error("Not Found", 404)
}

use crate::handler;
use worker::*;

const FRONTEND_ORIGIN: &str = "http://localhost:3000";

pub async fn dispatch(request: Request) -> Result<Response> {
    let response = match (request.method(), request.path().as_str()) {
        (Method::Options, _) => handler::preflight::handle(),
        (Method::Post, "/login") => handler::login::handle(request).await,
        (Method::Get, "/lectures") => handler::lectures::handle(request).await,
        (Method::Post, "/view") => handler::view::handle(request).await,
        _ => handler::not_found::handle(),
    }?;

    set_cors(response)
}

fn set_cors(mut response: Response) -> Result<Response> {
    let headers = response.headers_mut();
    headers.set("Access-Control-Allow-Origin", FRONTEND_ORIGIN)?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set("Vary", "Origin")?;
    Ok(response)
}

use crate::handlers;
use worker::*;

const FRONTEND_ORIGIN: &str = "https://psvm203.github.io";

pub async fn dispatch(request: Request) -> Result<Response> {
    let response = match (request.method(), request.path().as_str()) {
        (Method::Options, _) => handlers::preflight::handle(),
        (Method::Post, "/login") => handlers::login::handle(request).await,
        (Method::Get, "/lectures") => handlers::lectures::handle(request).await,
        (Method::Post, "/view") => handlers::view::handle(request).await,
        (Method::Get, "/video") => handlers::video::handle(request).await,
        (Method::Post, "/reset") => handlers::reset::handle(request).await,
        _ => handlers::not_found::handle(),
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

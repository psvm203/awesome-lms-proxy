use worker::*;

pub async fn handle() -> Result<Response> {
    let headers = Headers::new();
    headers.set("Access-Control-Allow-Methods", "OPTIONS, GET, POST")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;

    Ok(Response::empty()?.with_status(204).with_headers(headers))
}

use worker::*;

pub async fn fetch(url: &str) -> Result<Response> {
    let request = Request::new(url, Method::Get)?;

    Fetch::Request(request).send().await
}

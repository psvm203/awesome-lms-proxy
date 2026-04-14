use crate::{clients::BASE_URL, utils::HeadersExt as _};
use worker::*;

const CONNECT_PATH: &str = "/ilos/mp/todo_list_connect.acl";

pub async fn fetch(params: &str, cookie: &str) -> Result<Response> {
    let url = format!("{BASE_URL}{CONNECT_PATH}?{params}");
    let mut init = RequestInit::new();
    init.with_method(Method::Get)
        .with_headers(Headers::new().with_cookie(cookie));

    let request = Request::new_with_init(&url, &init)?;

    Fetch::Request(request).send().await
}

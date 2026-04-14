use crate::{clients::BASE_URL, utils::HeadersExt as _};
use worker::*;

const TODO_PATH: &str = "/ilos/mp/todo_list.acl";

pub async fn fetch(cookie: &str) -> Result<Response> {
    let url = format!("{BASE_URL}{TODO_PATH}");
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(Headers::new().with_cookie(cookie));

    let request = Request::new_with_init(&url, &init)?;

    Fetch::Request(request).send().await
}

use worker::*;

use crate::{clients::BASE_URL, utils::HeadersExt};

const LIST_PATH: &str = "/ilos/st/course/online_list.acl";

pub async fn fetch(cookie: &str) -> Result<Response> {
    let url = format!("{BASE_URL}{LIST_PATH}");
    let mut init = RequestInit::new();
    init.with_method(Method::Get)
        .with_headers(Headers::new().with_cookie(cookie));

    let request = Request::new_with_init(&url, &init)?;

    Fetch::Request(request).send().await
}

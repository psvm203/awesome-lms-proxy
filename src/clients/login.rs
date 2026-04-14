use crate::{clients::BASE_URL, utils::HeadersExt as _};
use worker::{wasm_bindgen::JsValue, *};

const LOGIN_PATH: &str = "/ilos/lo/login.acl";

pub async fn fetch(body: Option<JsValue>) -> Result<Response> {
    let url = format!("{BASE_URL}{LOGIN_PATH}");
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(Headers::new().with_urlencoded())
        .with_body(body)
        .with_redirect(RequestRedirect::Manual);

    let request = Request::new_with_init(&url, &init)?;

    Fetch::Request(request).send().await
}

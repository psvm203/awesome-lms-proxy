use crate::{clients::BASE_URL, utils::HeadersExt as _};
use worker::{wasm_bindgen::JsValue, *};

const VIEW_FORM_PATH: &str = "/ilos/st/course/online_view_form.acl";

pub async fn fetch(cookie: &str, body: Option<JsValue>) -> Result<Response> {
    let url = format!("{BASE_URL}{VIEW_FORM_PATH}");
    let mut init = RequestInit::new();
    init.with_method(Method::Post)
        .with_headers(Headers::new().with_urlencoded().with_cookie(cookie))
        .with_body(body);

    let request = Request::new_with_init(&url, &init)?;

    Fetch::Request(request).send().await
}

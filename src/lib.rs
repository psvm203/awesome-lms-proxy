use worker::*;

const FRONTEND_ORIGIN: &str = "http://localhost:3000";
const LMS_URL: &str = "https://lms.pknu.ac.kr/ilos/main/main_form.acl";

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    let method = req.method().clone();
    let path = req.path();

    if method == Method::Options {
        return cors_preflight();
    }

    match (method, path.as_str()) {
        (Method::Get, "/session") => create_session().await,
        _ => with_cors(Response::error("Not Found", 404)?),
    }
}

async fn create_session() -> Result<Response> {
    let mut init = RequestInit::new();
    init.with_method(Method::Get);
    init.with_redirect(RequestRedirect::Manual);
    let lms_req = Request::new_with_init(LMS_URL, &init)?;
    let lms_res = Fetch::Request(lms_req).send().await?;

    let Some(set_cookie) = lms_res.headers().get("set-cookie")? else {
        return with_cors(Response::error("LMS did not return a session cookie", 502)?);
    };

    let Some(jsessionid) = extract_jsessionid(&set_cookie) else {
        return with_cors(Response::error("JSESSIONID not found in LMS response", 502)?);
    };

    let mut res = Response::empty()?.with_status(204);
    let headers = res.headers_mut();
    headers.set(
        "Set-Cookie",
        &format!("JSESSIONID={jsessionid}; Path=/; HttpOnly; SameSite=Lax"),
    )?;
    set_cors_headers(headers)?;
    Ok(res)
}

fn extract_jsessionid(set_cookie: &str) -> Option<String> {
    for cookie in set_cookie.split(',') {
        let candidate = cookie.trim_start();
        if let Some(raw_value) = candidate.strip_prefix("JSESSIONID=") {
            let value = raw_value.split(';').next()?.trim();
            if !value.is_empty() {
                return Some(value.to_string());
            }
        }
    }
    None
}

fn cors_preflight() -> Result<Response> {
    let mut res = Response::empty()?.with_status(204);
    let headers = res.headers_mut();
    set_cors_headers(headers)?;
    headers.set("Access-Control-Allow-Methods", "GET, OPTIONS")?;
    headers.set("Access-Control-Allow-Headers", "Content-Type")?;
    Ok(res)
}

fn with_cors(mut res: Response) -> Result<Response> {
    set_cors_headers(res.headers_mut())?;
    Ok(res)
}

fn set_cors_headers(headers: &mut Headers) -> Result<()> {
    headers.set("Access-Control-Allow-Origin", FRONTEND_ORIGIN)?;
    headers.set("Access-Control-Allow-Credentials", "true")?;
    headers.set("Vary", "Origin")?;
    Ok(())
}

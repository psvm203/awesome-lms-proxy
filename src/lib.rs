use worker::*;

const FRONTEND_ORIGIN: &str = "http://localhost:3000";
const LMS_LOGIN_URL: &str = "https://lms.pknu.ac.kr/ilos/lo/login.acl";

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let method = req.method().clone();
    let path = req.path();

    if method == Method::Options {
        return cors_preflight();
    }

    match (method, path.as_str()) {
        (Method::Post, "/login") => login(req).await,
        _ => with_cors(Response::error("Not Found", 404)?),
    }
}

async fn login(mut req: Request) -> Result<Response> {
    let login_body = req.text().await?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    init.with_redirect(RequestRedirect::Manual);
    init.with_body(Some(login_body.into()));

    let mut lms_req = Request::new_with_init(LMS_LOGIN_URL, &init)?;
    let lms_headers = lms_req.headers_mut()?;
    lms_headers.set("Content-Type", "application/x-www-form-urlencoded")?;

    let mut lms_res = Fetch::Request(lms_req).send().await?;
    let lms_body = lms_res.text().await?;
    let login_success = is_login_success(&lms_body);

    let mut res = if login_success {
        Response::empty()?.with_status(204)
    } else {
        Response::error("Invalid LMS credentials", 401)?
    };

    if let Some(set_cookie) = lms_res.headers().get("set-cookie")? {
        if let Some(jsessionid) = extract_jsessionid(&set_cookie) {
            res.headers_mut().set(
                "Set-Cookie",
                &format!("JSESSIONID={jsessionid}; Path=/; HttpOnly; SameSite=Lax"),
            )?;
        }
    }

    with_cors(res)
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

fn is_login_success(body: &str) -> bool {
    !body.contains("alert")
}

fn cors_preflight() -> Result<Response> {
    let mut res = Response::empty()?.with_status(204);
    let headers = res.headers_mut();
    set_cors_headers(headers)?;
    headers.set("Access-Control-Allow-Methods", "POST, OPTIONS")?;
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

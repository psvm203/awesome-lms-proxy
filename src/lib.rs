use regex::Regex;
use scraper::{ElementRef, Html, Selector};
use serde::Serialize;
use std::sync::OnceLock;
use worker::*;

const FRONTEND_ORIGIN: &str = "http://localhost:3000";
const LMS_LOGIN_URL: &str = "https://lms.pknu.ac.kr/ilos/lo/login.acl";
const LMS_MAIN_URL: &str = "https://lms.pknu.ac.kr/ilos/main/main_form.acl";
const LMS_TODO_URL: &str = "https://lms.pknu.ac.kr/ilos/mp/todo_list.acl";

#[event(fetch)]
async fn fetch(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    let method = req.method().clone();
    let path = req.path();

    if method == Method::Options {
        return cors_preflight();
    }

    match (method, path.as_str()) {
        (Method::Post, "/login") => login(req).await,
        (Method::Get, "/lectures") => lectures(req).await,
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

async fn lectures(req: Request) -> Result<Response> {
    let cookie = req.headers().get("Cookie")?;

    let mut warmup_init = RequestInit::new();
    warmup_init.with_method(Method::Get);
    let mut warmup_req = Request::new_with_init(LMS_MAIN_URL, &warmup_init)?;
    if let Some(cookie) = cookie.as_deref() {
        warmup_req.headers_mut()?.set("Cookie", cookie)?;
    }
    let _ = Fetch::Request(warmup_req).send().await?;

    let mut init = RequestInit::new();
    init.with_method(Method::Post);
    let mut lms_req = Request::new_with_init(LMS_TODO_URL, &init)?;
    if let Some(cookie) = cookie.as_deref() {
        lms_req.headers_mut()?.set("Cookie", cookie)?;
    }

    let mut lms_res = Fetch::Request(lms_req).send().await?;
    let lms_body = lms_res.text().await?;
    let lectures = extract_lectures(&lms_body);

    let res = Response::from_json(&lectures)?;
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

#[derive(Serialize)]
struct Lecture {
    subject_id: String,
    week: String,
    subject_name: String,
    title: String,
}

fn extract_lectures(body: &str) -> Vec<Lecture> {
    let document = Html::parse_fragment(body);
    let todo_selector = Selector::parse(".todo_wrap").expect("todo selector should compile");
    let title_selector = Selector::parse(".todo_title").expect("title selector should compile");
    let subject_selector = Selector::parse(".todo_subjt").expect("subject selector should compile");

    document
        .select(&todo_selector)
        .filter_map(|el| {
            let onclick = el.value().attr("onclick")?;
            let (subject_id, week, kind) = parse_go_lecture_args(onclick)?;
            if kind != "lecture_weeks" {
                return None;
            }

            let subject_name = normalize_whitespace(&first_text(&el, &subject_selector));
            let title = normalize_whitespace(&first_text(&el, &title_selector));

            Some(Lecture {
                subject_id,
                week,
                subject_name,
                title,
            })
        })
        .collect()
}

fn normalize_whitespace(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn first_text(el: &ElementRef<'_>, selector: &Selector) -> String {
    el.select(selector)
        .next()
        .map(|node| node.text().collect::<String>())
        .unwrap_or_default()
}

fn parse_go_lecture_args(onclick: &str) -> Option<(String, String, String)> {
    let caps = go_lecture_regex().captures(onclick)?;
    Some((
        caps.get(1)?.as_str().to_string(),
        caps.get(2)?.as_str().to_string(),
        caps.get(3)?.as_str().to_string(),
    ))
}

fn go_lecture_regex() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| {
        Regex::new(r"^goLecture\('([^']*)','([^']*)','([^']*)'\)$")
            .expect("goLecture regex should compile")
    })
}

fn cors_preflight() -> Result<Response> {
    let mut res = Response::empty()?.with_status(204);
    let headers = res.headers_mut();
    set_cors_headers(headers)?;
    headers.set("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
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

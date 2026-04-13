use crate::{
    clients,
    handler::error_messages::*,
    utils::{HeadersExt, ResponseExt},
};
use regex::Regex;
use worker::*;

pub async fn handle(mut request: Request) -> Result<Response> {
    let request_body = Some(request.text().await?.into());
    let mut login_response = clients::login::fetch(request_body).await?;
    if !login_response.redirect() {
        return Response::error(LOGIN_UNAVAILABLE, 503);
    }

    let login_response_body = login_response.text().await?;
    if too_many_requests(&login_response_body) {
        return Response::error(TOO_MANY_REQUESTS, 429);
    }

    if invalid_credentials(&login_response_body) {
        return Response::error(INVALID_CREDENTIALS, 401);
    }

    if login_error(&login_response_body) {
        return Response::error(LOGIN_ERROR, 500);
    }

    let cookies = login_response.headers().get_all("Set-Cookie")?;
    let session_id = match extract_session_id(cookies) {
        Some(id) => id,
        None => return Response::error(LOGIN_UNAVAILABLE, 503),
    };

    let headers =
        Headers::new().with_set_cookie(&format!("JSESSIONID={session_id}; Path=/; HttpOnly"));

    Ok(Response::empty()?.with_status(204).with_headers(headers))
}

fn too_many_requests(body: &str) -> bool {
    body.contains("5분 후에 다시 접속하시기 바랍니다.")
}

fn invalid_credentials(body: &str) -> bool {
    body.contains("로그인 정보가 일치하지 않습니다.")
}

fn login_error(body: &str) -> bool {
    body.contains("에러 발생시 강제로 메인으로 보냄")
}

fn extract_session_id(cookies: Vec<String>) -> Option<String> {
    let re = Regex::new(r"JSESSIONID=([^;]+)").expect("Invalid regex");

    cookies.into_iter().find_map(|cookie| {
        re.captures(&cookie)
            .and_then(|cap| cap.get(1))
            .map(|m| m.as_str().to_owned())
    })
}

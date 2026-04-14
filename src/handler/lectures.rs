use crate::{
    clients,
    handler::error_messages::*,
    model::lecture::Lecture,
    utils::{RequestExt as _, ResponseExt as _},
};
use worker::*;

pub async fn handle(request: Request) -> Result<Response> {
    let Some(cookie) = request.get_cookie() else {
        return Response::error(NO_CREDENTIALS, 401);
    };

    let main_page_response = clients::main_page::fetch(&cookie).await?;
    if !main_page_response.ok() {
        return Response::error(MAIN_PAGE_UNAVAILABLE, 503);
    }

    let mut todo_response = clients::todo::fetch(&cookie).await?;
    if !todo_response.ok() {
        return Response::error(TODO_PAGE_UNAVAILABLE, 503);
    }

    let response_body = todo_response.text().await?;
    if is_session_expired(&response_body) {
        return Response::error(SESSION_EXPIRED, 401);
    }

    if !is_body_valid(&response_body) {
        return Response::error(PARSE_ERROR, 503);
    }

    let Some(lectures) = Lecture::extract_lectures(&response_body) else {
        return Response::error(PARSE_ERROR, 500);
    };

    Response::from_json(&lectures)
}

fn is_session_expired(body: &str) -> bool {
    body.contains("접속이 종료 되었습니다.")
}

fn is_body_valid(body: &str) -> bool {
    !body.contains("DOCTYPE")
}

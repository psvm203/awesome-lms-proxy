use crate::handler::error_messages::*;
use crate::model::lecture::Lecture;
use crate::{clients, utils::ResponseExt};
use worker::*;

pub async fn handle(request: Request) -> Result<Response> {
    let cookie = match request.headers().get("Cookie")? {
        Some(c) => c,
        None => return Response::error(NO_CREDENTIALS, 401),
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
        return Response::error(TODO_PAGE_UNAVAILABLE, 503);
    }

    let lectures = Lecture::extract_lectures(&response_body);

    Response::from_json(&lectures)
}

fn is_session_expired(body: &str) -> bool {
    body.contains("접속이 종료 되었습니다.")
}

fn is_body_valid(body: &str) -> bool {
    !body.contains("DOCTYPE")
}

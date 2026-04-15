use crate::{
    clients,
    handlers::{error_messages::*, lectures},
    models::{
        history_response_data::HistoryResponseData, navi_response_data::NaviResponseData,
        view_request_data::ViewRequestData,
    },
    utils::{RequestExt as _, ResponseExt as _},
};
use regex::Regex;
use worker::*;

pub async fn handle(mut request: Request) -> Result<Response> {
    let request_body = request.text().await?;
    let Ok(view_request_data) = serde_urlencoded::from_str::<ViewRequestData>(&request_body) else {
        return Response::error(INVALID_REQUEST_BODY, 400);
    };

    let connect_params = view_request_data.to_connect_params();
    let Some(cookie) = request.get_cookie() else {
        return Response::error(NO_CREDENTIALS, 401);
    };

    let connect_response = clients::connect::fetch(&connect_params, &cookie).await?;
    if !connect_response.ok() {
        return Response::error(CONNECT_PAGE_UNAVAILABLE, 503);
    }

    let sequence = view_request_data.sequence.as_str();
    let view_form_request_body = Some(format!("lecture_weeks={sequence}").into());
    let mut view_form_response = clients::view_form::fetch(&cookie, view_form_request_body).await?;
    if !view_form_response.ok() {
        return Response::error(VIEW_FORM_UNAVAILABLE, 503);
    }

    let view_form_response_body = view_form_response.text().await?;
    let items = extract_items(&view_form_response_body);
    let subject_id = view_request_data.subject_id.as_str();

    for item_id in items {
        let navi_request_body =
            Some(format!("lecture_weeks={sequence}&item_id={item_id}&ky={subject_id}").into());

        let mut navi_response = clients::navi::fetch(&cookie, navi_request_body).await?;
        let navi_response_data: NaviResponseData = navi_response.json().await?;
        let link_sequence = navi_response_data.link_seq.as_str();
        let mut list_response = clients::list::fetch(&cookie).await?;
        if !list_response.ok() {
            return Response::error(LIST_PAGE_UNAVAILABLE, 503);
        }

        let list_response_body = list_response.text().await?;
        let duration = match extract_video_duration(&item_id, &list_response_body) {
            Some(d) => d,
            None => return Response::error(PARSE_ERROR, 500),
        };

        let history_request_body = Some(format!("lecture_weeks={sequence}&kjkey={subject_id}&ky={subject_id}&interval_time=-{duration}").into());
        let mut history_response = clients::history::fetch(&cookie, history_request_body).await?;
        let history_response_data: HistoryResponseData = history_response.json().await?;
        let his_no = history_response_data.his_no.as_str();
        let view_request_body = Some(format!("lecture_weeks={sequence}&link_seq={link_sequence}&his_no={his_no}&ky={subject_id}&interval_time=-{duration}").into());
        clients::view::fetch(&cookie, view_request_body.clone()).await?;
        clients::view::fetch(&cookie, view_request_body.clone()).await?;
    }

    lectures::handle(request).await
}

pub fn extract_items(body: &str) -> Vec<String> {
    let re = Regex::new(r#"<div class="item-title-lesson.*val="([^\^]*)"#).expect("Invalid regex");

    re.captures_iter(body)
        .filter_map(|capture| capture.get(1).map(|item_id| item_id.as_str().to_owned()))
        .collect()
}

fn extract_video_duration(item_id: &str, body: &str) -> Option<u32> {
    let re = Regex::new(&format!(
        "(?s){item_id}.*?([0-9:]{{4,}})\\s\\/\\s[0-9:]{{4,}}<\\/div>"
    ))
    .expect("Invalid regex");

    let capture = re.captures(body)?;
    let current_time = parse_time(capture.get(1)?.as_str())?;

    Some(current_time)
}

fn parse_time(time: &str) -> Option<u32> {
    let parts: Vec<u32> = time
        .split(':')
        .map(|p| p.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    match parts.as_slice() {
        [h, m, s] => Some(h * 3600 + m * 60 + s),
        [m, s] => Some(m * 60 + s),
        _ => None,
    }
}

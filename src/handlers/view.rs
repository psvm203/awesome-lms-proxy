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
use scraper::{Html, Selector};
use worker::*;

const INTERVAL_TIME: u32 = 240;

pub async fn handle(mut request: Request) -> Result<Response> {
    let request_body = request.text().await?;
    let view_request_data: ViewRequestData = match serde_urlencoded::from_str(&request_body) {
        Ok(request) => request,
        Err(_) => return Response::error(INVALID_REQUEST_BODY, 400),
    };

    let sequence = view_request_data.sequence.as_str();
    let connect_params = view_request_data.to_connect_params();
    let Some(cookie) = request.get_cookie() else {
        return Response::error(NO_CREDENTIALS, 401);
    };

    let connect_response = clients::connect::fetch(&connect_params, &cookie).await?;
    if !connect_response.ok() {
        return Response::error(CONNECT_PAGE_UNAVAILABLE, 503);
    }

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
        let path = navi_response_data.path.as_str();
        let video_url = extract_video_url(path);
        let mut video_response = clients::video::fetch(&video_url).await?;
        if !video_response.ok() {
            return Response::error(VIDEO_UNAVAILABLE, 503);
        }

        let video_response_body = video_response.text().await?;
        let link_sequence = navi_response_data.link_seq.as_str();
        let history_request_body = Some(
            format!("lecture_weeks={sequence}&kjkey={subject_id}&ky={subject_id}&interval_time={INTERVAL_TIME}")
                .into(),
        );

        let mut history_response = clients::history::fetch(&cookie, history_request_body).await?;
        let history_response_data: HistoryResponseData = history_response.json().await?;
        let his_no = history_response_data.his_no.as_str();
        let view_request_body = Some(
            format!("lecture_weeks={sequence}&link_seq={link_sequence}&his_no={his_no}&ky={subject_id}&interval_time={INTERVAL_TIME}")
                .into(),
        );

        let duration = match extract_video_duration(&video_response_body) {
            Some(d) => d,
            None => 3600,
        };

        for _ in 0..get_fetch_counts(duration) {
            clients::view::fetch(&cookie, view_request_body.clone()).await?;
        }
    }

    lectures::handle(request).await
}

fn extract_items(body: &str) -> Vec<String> {
    let re = Regex::new(r#"<div class="item-title-lesson.*val="([^\^]*)"#).expect("Invalid regex");

    re.captures_iter(body)
        .filter_map(|capture| capture.get(1).map(|item_id| item_id.as_str().to_owned()))
        .collect()
}

fn extract_video_url(path: &str) -> String {
    path.replacen("/http-server", "https://vod.pknu.ac.kr/contents", 1)
        .to_owned()
}

fn extract_video_duration(body: &str) -> Option<u32> {
    let document = Html::parse_document(body);
    let selector = Selector::parse(r#"meta[name="duration"]"#).ok()?;

    document
        .select(&selector)
        .next()?
        .value()
        .attr("content")
        .and_then(|value| value.parse().ok())
}

const fn get_fetch_counts(duration: u32) -> u32 {
    duration / INTERVAL_TIME + 2
}

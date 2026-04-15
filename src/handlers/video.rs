use crate::{
    clients,
    handlers::{error_messages::*, view::extract_items},
    models::{navi_response_data::NaviResponseData, video_request_data::VideoRequestData},
    utils::{RequestExt as _, ResponseExt as _},
};
use worker::*;

pub async fn handle(mut request: Request) -> Result<Response> {
    let request_body = request.text().await?;
    let Ok(video_request_data) = serde_urlencoded::from_str::<VideoRequestData>(&request_body)
    else {
        return Response::error(INVALID_REQUEST_BODY, 400);
    };

    let connect_params = video_request_data.to_connect_params();
    let Some(cookie) = request.get_cookie() else {
        return Response::error(NO_CREDENTIALS, 401);
    };

    let connect_response = clients::connect::fetch(&connect_params, &cookie).await?;
    if !connect_response.ok() {
        return Response::error(CONNECT_PAGE_UNAVAILABLE, 503);
    }

    let sequence = video_request_data.sequence.as_str();
    let view_form_request_body = Some(format!("lecture_weeks={sequence}").into());
    let mut view_form_response = clients::view_form::fetch(&cookie, view_form_request_body).await?;
    if !view_form_response.ok() {
        return Response::error(VIEW_FORM_UNAVAILABLE, 503);
    }

    let view_form_response_body = view_form_response.text().await?;
    let items = extract_items(&view_form_response_body);
    let subject_id = video_request_data.subject_id.as_str();
    let mut videos = vec![];

    for item_id in items {
        let navi_request_body =
            Some(format!("lecture_weeks={sequence}&item_id={item_id}&ky={subject_id}").into());

        let mut navi_response = clients::navi::fetch(&cookie, navi_request_body).await?;
        let navi_response_data: NaviResponseData = navi_response.json().await?;
        let path = navi_response_data.path.as_str();
        let video_url = path.replacen("/http-server", "https://vod.pknu.ac.kr/contents", 1);
        videos.push(video_url);
    }

    Response::from_json(&videos)
}

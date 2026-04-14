use serde::Deserialize;

#[derive(Deserialize)]
pub struct NaviResponseData {
    pub path: String,
    pub link_seq: String,
}

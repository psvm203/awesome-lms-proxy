use serde::Deserialize;

#[derive(Deserialize)]
pub struct NaviResponseData {
    pub link_seq: String,
}

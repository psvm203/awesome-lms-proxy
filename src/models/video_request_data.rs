use serde::Deserialize;

#[derive(Deserialize)]
pub struct VideoRequestData {
    pub subject_id: String,
    pub sequence: String,
}

impl VideoRequestData {
    pub fn to_connect_params(&self) -> String {
        format!("SEQ={}&KJKEY={}", self.sequence, self.subject_id)
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ViewRequestData {
    pub subject_id: String,
    pub sequence: String,
}

impl ViewRequestData {
    pub fn to_connect_params(&self) -> String {
        format!(
            "SEQ={}&gubun=lecture_weeks&KJKEY={}",
            self.sequence, self.subject_id
        )
    }
}

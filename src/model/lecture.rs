use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct Lecture {
    subject_id: String,
    week: String,
    subject_name: String,
    title: String,
}

impl Lecture {
    pub fn extract_lectures(body: &str) -> Vec<Lecture> {
        let fragment = Html::parse_fragment(body);
        let todo_selector = match Selector::parse(".todo_wrap.on") {
            Ok(selector) => selector,
            Err(_) => return Vec::new(),
        };

        let title_selector = match Selector::parse(".todo_title") {
            Ok(selector) => selector,
            Err(_) => return Vec::new(),
        };

        let subject_selector = match Selector::parse(".todo_subjt") {
            Ok(selector) => selector,
            Err(_) => return Vec::new(),
        };

        fragment
            .select(&todo_selector)
            .filter_map(|element| {
                let onclick = element.attr("onclick")?;
                let (subject_id, week, kind) = Self::parse_go_lecture_args(onclick)?;
                if kind != "lecture_weeks" {
                    return None;
                }

                let subject_name = element
                    .select(&subject_selector)
                    .next()?
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_owned();

                let title = element
                    .select(&title_selector)
                    .next()?
                    .text()
                    .collect::<String>()
                    .trim()
                    .to_owned();

                Some(Lecture {
                    subject_id,
                    week,
                    subject_name,
                    title,
                })
            })
            .collect()
    }

    fn parse_go_lecture_args(onclick: &str) -> Option<(String, String, String)> {
        let re =
            Regex::new(r#"goLecture\('([^,]+)','([^,]+)','([^,]+)'\)"#).expect("Invalid regex");

        re.captures(onclick)
            .map(|caps| (caps[1].to_owned(), caps[2].to_owned(), caps[3].to_owned()))
    }
}

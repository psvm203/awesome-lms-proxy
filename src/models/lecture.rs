use regex::Regex;
use scraper::{Html, Selector};
use serde::Serialize;

#[derive(Serialize)]
pub struct Lecture {
    subject_id: String,
    sequence: String,
    subject_name: String,
    title: String,
}

impl Lecture {
    pub fn extract_lectures(body: &str) -> Option<Vec<Self>> {
        let fragment = Html::parse_fragment(body);
        let todo_selector = Selector::parse(".todo_wrap.on").ok()?;
        let title_selector = Selector::parse(".todo_title").ok()?;
        let subject_selector = Selector::parse(".todo_subjt").ok()?;

        let lectures = fragment
            .select(&todo_selector)
            .filter_map(|element| {
                let onclick = element.attr("onclick")?;
                let (subject_id, sequence, kind) = Self::parse_go_lecture_args(onclick)?;
                if kind != "lecture_weeks" {
                    return None;
                }

                let extract_text = |selector: &Selector| -> Option<String> {
                    let text = element
                        .select(selector)
                        .next()?
                        .text()
                        .collect::<String>()
                        .trim()
                        .to_owned();

                    Some(text)
                };

                let subject_name = extract_text(&subject_selector)?;
                let title = extract_text(&title_selector)?;

                Some(Self {
                    subject_id,
                    sequence,
                    subject_name,
                    title,
                })
            })
            .collect();

        Some(lectures)
    }

    fn parse_go_lecture_args(onclick: &str) -> Option<(String, String, String)> {
        let re = Regex::new(r"goLecture\('([^,]+)','([^,]+)','([^,]+)'\)").expect("Invalid regex");

        re.captures(onclick).map(|capture| {
            (
                capture[1].to_owned(),
                capture[2].to_owned(),
                capture[3].to_owned(),
            )
        })
    }
}

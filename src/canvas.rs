use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Deserialize)]
struct Module {
    items: Vec<Note>,
}

#[derive(Debug, Deserialize)]
pub struct Note {
    pub id: usize,
    pub title: String,
    pub html_url: Url,
}

pub struct Canvas {
    access_key: String,
}

impl Canvas {
    pub fn new(access_key: String) -> Self {
        return Self { access_key };
    }

    fn modules_endpoint(&self, course_id: &u32, module_id: &u32) -> Result<Url, url::ParseError> {
        let url = format!(
            "https://canvas.butte.edu/api/v1/courses/{}/modules/{}",
            &course_id, &module_id,
        );

        let params = [
            ("access_token", &self.access_key),
            ("include", &"items".to_string()),
        ];

        Url::parse_with_params(&url, params)
    }

    pub async fn get_lecture_notes(
        &self,
        course_id: &u32,
        module_id: &u32,
    ) -> anyhow::Result<Vec<Note>> {
        let url = self.modules_endpoint(&course_id, &module_id)?;
        Ok(reqwest::get(url).await?.json::<Module>().await?.items)
    }
}

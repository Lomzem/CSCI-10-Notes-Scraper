use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Serialize, Deserialize)]
pub struct LectureNote {
    // pub id: u32,
    pub title: String,
    pub html_url: String,
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
            "https://canvas.butte.edu/api/v1/courses/{}/modules/{}/items",
            &course_id, &module_id,
        );

        let params = [("access_token", &self.access_key)];

        Url::parse_with_params(&url, params)
    }

    pub async fn get_lecture_notes(
        &self,
        course_id: &u32,
        module_id: &u32,
    ) -> anyhow::Result<Vec<LectureNote>> {
        let url = self.modules_endpoint(&course_id, &module_id)?;
        let text = reqwest::get(url.clone()).await?.text().await?;
        std::fs::File("s");
        println!("{text}");
        Ok(reqwest::get(url).await?.json::<Vec<LectureNote>>().await?)
    }
}

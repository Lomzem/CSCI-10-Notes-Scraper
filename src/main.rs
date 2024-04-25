mod canvas;
mod prompt;

use canvas::Canvas;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let access_key = std::env::var("CANVAS_ACCESS_KEY").expect("Unable to find canvas key");

    const CSCI_10_ID: u32 = 33114;
    const CSCI_10_NOTES_MODULE: u32 = 271157;
    // https://canvas.butte.edu/api/v1/courses/33114/modules/271157?include=items

    let canvas = Canvas::new(access_key);
    let notes = canvas
        .get_lecture_notes(&CSCI_10_ID, &CSCI_10_NOTES_MODULE)
        .await?;

    prompt::select_note(&notes);

    // notes.iter().for_each(|l| println!("{:?}", l));
    // TODO! Find way to convert html_url to html to pdf
    // possibly using get request?
    // GET /api/v1/courses/:course_id/pages/:url_or_id
    // returns https://canvas.instructure.com/doc/api/pages.html#Page

    Ok(())
}

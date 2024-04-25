mod canvas;

use canvas::Canvas;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let access_key = std::env::var("CANVAS_ACCESS_KEY").expect("Unable to find canvas key");

    const CSCI_10_ID: u32 = 33114;
    const CSCI_10_NOTES_MODULE: u32 = 271157;

    let canvas = Canvas::new(access_key);
    let lecture_notes = canvas
        .get_lecture_notes(&CSCI_10_ID, &CSCI_10_NOTES_MODULE)
        .await?;

    lecture_notes.iter().for_each(|l| println!("{:?}", l));

    Ok(())
}

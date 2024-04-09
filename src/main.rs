use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Module {
    id: u32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LectureNote {
    id: u32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let access_key = std::env::var("CANVAS_ACCESS_KEY").expect("Unable to find canvas key");

    let canvas_url = Url::parse("https://canvas.butte.edu")?;
    const CSCI_10_ID: u32 = 33114;

    let modules_endpoint = format!("/api/v1/courses/{}/modules", &CSCI_10_ID);
    let api_params = [("access_token", access_key)];

    let modules_url = Url::parse_with_params(
        canvas_url.join(modules_endpoint.as_ref())?.as_str(),
        &api_params,
    )?;

    let module_list: Vec<Module> = reqwest::get(modules_url).await?.json().await?;

    let notes_module: &Module = module_list.last().unwrap();
    let list_module_endpoint = format!(
        "/api/v1/courses/{}/modules/{}/items",
        CSCI_10_ID, notes_module.id,
    );

    let list_module_url = Url::parse_with_params(
        canvas_url.join(list_module_endpoint.as_ref())?.as_str(),
        &api_params,
    )?;

    let module_contents = reqwest::get(list_module_url).await?.bytes().await?;

    // println!("{:?}", module_contents);
    std::fs::write("response.json", module_contents)?;

    Ok(())
}

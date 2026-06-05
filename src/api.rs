use std::{error::Error, path::PathBuf};

use crate::config::Config;
use indicatif::{ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use tokio::{fs::File, io::AsyncWriteExt};

#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub data: Vec<SubtitleData>,
}

#[derive(Deserialize, Debug)]
pub struct SubtitleData {
    pub attributes: Subtitle,
}

#[derive(Deserialize, Debug)]
pub struct Subtitle {
    #[serde(rename = "release")]
    pub name: String,
    pub language: String,
    pub download_count: u32,
    pub files: Vec<SubtitleFile>,
}

#[derive(Deserialize, Debug)]
pub struct SubtitleFile {
    pub file_id: u64,
}

#[derive(Deserialize, Debug)]
pub struct DownloadResponse {
    pub link: String,
    pub file_name: String,
}

#[derive(Serialize, Debug)]
pub struct DownloadPayload {
    pub file_id: u64,
}

pub async fn fetch_subtitles(
    config: &Config,
    query: &str,
    search_type: &str,
) -> Result<Vec<Subtitle>, Box<dyn Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Searching Subtitles");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let url = format!(
        "https://api.opensubtitles.com/api/v1/subtitles?query={}&languages={}&type={}",
        query,
        config.languages.join(",").to_owned(),
        search_type
    );

    let client = reqwest::Client::new();

    let response = client
        .get(&url)
        .header("Api-Key", config.api_key.clone())
        .header("User-Agent", "lighttigerxiv's subtitles script")
        .header("Accept", "application/json")
        .send()
        .await?
        .error_for_status()?
        .json::<ApiResponse>()
        .await?;

    let mut subs: Vec<Subtitle> = response
        .data
        .into_iter()
        .map(|data| data.attributes)
        .collect();

    subs.sort_by_key(|sub| sub.download_count.clone());

    subs.reverse();

    subs.sort_by_key(|sub| sub.language.clone());

    pb.finish_and_clear();

    Ok(subs)
}

pub async fn download_subtitle(sub: &Subtitle, config: &Config) -> Result<PathBuf, Box<dyn Error>> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    pb.set_message("Downloading Subtitles");
    pb.enable_steady_tick(std::time::Duration::from_millis(80));

    let file_info = sub
        .files
        .first()
        .ok_or("No files available for this subtitle")?;
    let client = reqwest::Client::new();

    let payload = DownloadPayload {
        file_id: file_info.file_id,
    };

    let api_response = client
        .post("https://api.opensubtitles.com/api/v1/download")
        .header("Api-Key", config.api_key.clone())
        .header("User-Agent", "lighttigerxiv's subtitles script")
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?
        .error_for_status()?
        .json::<DownloadResponse>()
        .await?;

    let file_response = client
        .get(&api_response.link)
        .send()
        .await?
        .error_for_status()?;

    let mut filename = api_response.file_name;

    if !filename.ends_with(".srt") {
        filename.push_str(".srt");
    }
    let download_path = match config.download_dir.to_owned() {
        Some(dir) => dir.join(filename),
        _ => dirs::download_dir().ok_or("")?.join(filename),
    };

    let mut file = File::create(&download_path).await?;
    let bytes = file_response.bytes().await?;
    file.write_all(&bytes).await?;

    pb.finish_and_clear();

    Ok(download_path)
}

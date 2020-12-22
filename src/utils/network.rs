use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::Path;

use indicatif::{ProgressBar, ProgressStyle};

#[tokio::main]
pub async fn download_form_github(
    url: &str,
    tmp_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Download RUL: {}", &url);

    let mut output_file = match File::create(&tmp_file) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let mut response = reqwest::get(url).await?;
    let mut bar = ProgressBar::new(0);
    match response.content_length() {
        Some(length) => bar = ProgressBar::new(length),
        None => {}
    }

    bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:50.cyan/red}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    let mut downloaded = 0;
    let mut length: u64 = 0;
    while let Some(chunk) = response.chunk().await? {
        length = length + chunk.len() as u64;
        let new = min(downloaded, length);
        downloaded = length;
        bar.set_position(new as u64);
        &output_file.write(&*chunk);
    }
    bar.finish_with_message("Done!");

    Ok(())
}

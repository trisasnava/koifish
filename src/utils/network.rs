use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;
use std::time::Duration;

use indicatif::{ProgressBar, ProgressStyle};
use reqwest;

#[tokio::main]
pub async fn download(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut downloaded = 0;

    let path = Path::new("koi.exe");
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };

    let content = reqwest::get(url).await?.bytes().await?;

    file.write(&content);

    let pb = ProgressBar::new(content.len() as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    // TODO progress bar should be fix
    use std::fs;
    while downloaded < content.len() {
        let new = min(downloaded, content.len());
        let metadata = fs::metadata(path)?;
        downloaded = metadata.len() as usize;
        pb.set_position(new as u64);
        thread::sleep(Duration::from_millis(1));
    }

    pb.finish_with_message("Downloaded");

    Ok(())
}

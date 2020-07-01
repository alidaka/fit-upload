use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::fs;

const GDRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files?uploadType=media";
const STRAVA_UPLOAD_URL: &str = "";
const CONFIG_FILE: &str = "/home/augustus/.fit-uploadrc";
const ACTIVITY_PATH: &str = "GARMIN/ACTIVITY";
/*
  curl -X POST https://www.strava.com/api/v3/uploads \
      -H "Authorization: Bearer abcd123" \
      -F data_type="fit" \
      -F file=@$filename
   */

#[derive(Debug, StructOpt)]
enum Command {
    /// Local filesystem path with activities to upload
    Upload { path: String, },
    /// Test that authorization works
    Test,
    /// API keys (GDrive, then Strava)
    Configure { params: Vec<String>, },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    gdrive_key: String,
    strava_key: String,
}

#[tokio::main]
async fn main() {
    let command = Command::from_args();

    println!("input:");
    println!("{:?}", command);

    match command {
        Command::Upload{ path } => upload(&path),
        Command::Test => test_auth().await,
        Command::Configure{ params } => configure(&params[0], &params[1]),
    }
}

fn upload(path: &str) {
    let config_data = fs::read(CONFIG_FILE).expect("Unable to read config file");
    let serialized = String::from_utf8(config_data).unwrap();
    let config: Config = serde_json::from_str(&serialized).unwrap();
    println!("config that was read:");
    println!("{:?}", config);

    println!("activity path:");
    println!("{:?}", path);
}

async fn test_auth() {
    println!("WIP");
    let client = reqwest::Client::new();
    let res = client.post(GDRIVE_UPLOAD_URL)
        .header("Content-Type", "application/vnd.google-apps.file") // or ...vnd.google-apps.unknown?
        .header("Content-Length", "MIME")
        .body("")
        .send()
        .await;
    res.expect("broken");
}

fn configure(a: &str, b: &str) {
    let config = Config { gdrive_key: a.to_string(), strava_key: b.to_string() };
    let serialized = serde_json::to_string(&config).unwrap();
    fs::write(CONFIG_FILE, serialized).unwrap();
}

use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::fs;

const GDRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files?uploadType=media";
const STRAVA_UPLOAD_URL: &str = "";
const CONFIG_FILE: &str = "/home/augustus/.fit-uploadrc";
/*
  curl -X POST https://www.strava.com/api/v3/uploads \
      -H "Authorization: Bearer abcd123" \
      -F data_type="fit" \
      -F file=@$filename
   */

#[derive(Debug, StructOpt)]
enum Command {
    Upload { path: String, },
    Test,
    Configure,
    T { params: Vec<String>, },
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    gdrive_key: String,
    strava_key: String,
}

fn main() {
    let command = Command::from_args();

    println!("input:");
    println!("{:?}", command);

    match command {
        Command::Upload{ path } => upload(&path),
        Command::Test => println!("WIP"),
        Command::Configure => println!("WIP"),
        Command::T{ params } => t(&params[0], &params[1]),
    }
}

fn upload(path: &str) {
    let config_data = fs::read(CONFIG_FILE).expect("Unable to read config file");
    let serialized = String::from_utf8(config_data).unwrap();
    let config: Config = serde_json::from_str(&serialized).unwrap();
    println!("config that was read:");
    println!("{:?}", config);
}

fn configure() {
}

fn t(a: &str, b: &str) {
    let config = Config { gdrive_key: a.to_string(), strava_key: b.to_string() };
    let serialized = serde_json::to_string(&config).unwrap();
    fs::write(CONFIG_FILE, serialized).unwrap();
}

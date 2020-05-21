use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::fs;

const GDRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files?uploadType=media";
const STRAVA_UPLOAD_URL: &str = "";
const CONFIG_FILE: &str = "~/.fit-uploadrc";
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
    garmin_root: String,
}

fn main() {
    let command = Command::from_args();

    println!("{:?}", command);

    match command {
        Command::Upload{ path } => upload(&path),
        Command::Test => println!("WIP"),
        Command::Configure => println!("WIP"),
        Command::T{ params } => t(&params[0], &params[1], &params[2]),
    }
}

fn upload(path: &str) {
    //let serialized = File
}

fn configure() {
}

fn t(a: &str, b: &str, c: &str) {
    let config = Config { gdrive_key: a.to_string(), strava_key: b.to_string(), garmin_root: c.to_string() };
    let serialized = serde_json::to_string(&config).unwrap();
    fs::write("/home/augustus/code/fit-upload/app-config", serialized).unwrap();
}

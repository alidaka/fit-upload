use structopt::StructOpt;

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
    Upload,
    Test,
    Configure,
}

struct Config {
    gdrive_key: String,
    strava_key: String,
    garmin_root: String,
}

fn main() {
    let command = Command::from_args();

    println!("{:?}", command);

    match command {
        Command::Upload => println!("WIP"),
        Command::Test => println!("WIP"),
        Command::Configure => println!("WIP"),
    }
}

fn upload() {
}

fn configure() {
}

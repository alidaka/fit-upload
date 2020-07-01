use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::fs;
use oauth2::Config;

const GDRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files?uploadType=media";
const STRAVA_UPLOAD_URL: &str = "https://www.strava.com/api/v3/uploads";
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
struct Configuration {
    gdrive_client_id: String,
    gdrive_client_secret: String,
    strava_client_id: String,
    strava_client_secret: String,
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
    let config: Configuration = serde_json::from_str(&serialized).unwrap();
    println!("Config that was read:");
    println!("{:?}", config);

    println!("activity path:");
    println!("{:?}", path);
}

async fn auth() -> String {
    println!("Authorizing...");

    // Create an OAuth2 config by specifying the client ID, client secret, authorization URL and token URL.
    let mut config = Config::new("client_id", "client_secret", "http://authorize", "http://token");

    // Set the desired scopes.
    config = config.add_scope("read");
    config = config.add_scope("write");

    // Set the URL the user will be redirected to after the authorization process.
    config = config.set_redirect_url("http://redirect");

    // Set a state parameter (optional, but recommended).
    config = config.set_state("1234");

    // Generate the full authorization URL.
    // This is the URL you should redirect the user to, in order to trigger the authorization process.
    println!("Browse to: {}", config.authorize_url());

    // TODO: prompt user to hit enter?

    // Once the user has been redirected to the redirect URL, you'll have access to the authorization code.
    // Now you can trade it for an access token.
    return config.exchange_code("some authorization code").unwrap().access_token;
}

async fn test_auth() {
    println!("WIP");
    let client = reqwest::Client::new();
    for entry in fs::read_dir(&format!("{}/{}", "/media/augustus/GARMIN", ACTIVITY_PATH)).unwrap() {
        let entry = entry.unwrap();
        let file_contents = fs::read(entry.path()).unwrap();

        println!("Uploading file to GDrive: {:?}", entry.path());
        /*
        let gdrive_result = client.post(GDRIVE_UPLOAD_URL)
            .header("Content-Type", "application/vnd.google-apps.file") // or ...vnd.google-apps.unknown?
            .header("Content-Length", entry.metadata().unwrap().len())
            .body(file_contents)
            .send()
            .await;
        println!("{:?}", gdrive_result.unwrap());
        */
        // TODO: check that the file creation time and filename are correct

        println!("Uploading file to Strava: {:?}", entry.path());
        /*
        let form = reqwest::multipart::Form::new()
            .text("data_type", "fit")
            .part("file", reqwest::multipart::Part::bytes(file_contents));
        let strava_result = client.post(STRAVA_UPLOAD_URL)
            .bearer_auth("abc123") // maybe? needs the access_token
            .multipart(form)
            .send()
            .await;
        println!("{:?}", strava_result.unwrap());
        */
    }
}

fn configure(a: &str, b: &str) {
    let config = Configuration { gdrive_client_id: a.to_string(), strava_client_id: b.to_string() };
    let serialized = serde_json::to_string(&config).unwrap();
    fs::write(CONFIG_FILE, serialized).unwrap();
}

use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::fs;
use oauth2::Config;
use rouille::Response;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};
use chrono::{DateTime, Local};

const GDRIVE_UPLOAD_URL: &str = "https://www.googleapis.com/upload/drive/v3/files?uploadType=multipart";
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
    gdrive_folder: String,
    strava_client_id: String,
    strava_client_secret: String,
}

#[derive(Debug, Serialize)]
struct GDriveMetadata {
    name: String,

    parents: Vec<String>,

    #[serde(rename(serialize="modifiedTime"))]
    modified_time: DateTime<Local>, // TODO: it's possible this should be Utc
}

#[tokio::main]
async fn main() {
    let command = Command::from_args();

    println!("input:");
    println!("{:?}", command);

    let (tx, rx): (SyncSender<String>, Receiver<String>) = sync_channel(1);

    let _handle = thread::spawn(move || {
        rouille::start_server("localhost:9004", move |request| {
            println!("received {:?}", request);
            let auth_code = request.get_param("code").expect("Failed to parse auth code from response");
            tx.send(auth_code).expect("Failed to send auth code from oauth loopback thread to main thread");
            return Response::empty_204();
        });
    });

    match command {
        Command::Upload{ path } => upload(&path),
        Command::Test => test_auth(rx).await,
        Command::Configure{ params } => configure(&params[0], &params[1], &params[2]),
    }

    // handle.join().unwrap();
}

fn upload(path: &String) {
    let config_data = fs::read(CONFIG_FILE).expect("Unable to read config file");
    let serialized = String::from_utf8(config_data).unwrap();
    let config: Configuration = serde_json::from_str(&serialized).unwrap();
    println!("Config that was read:");
    println!("{:?}", config);

    println!("activity path:");
    println!("{:?}", path);
}

async fn auth(rx: &Receiver<String>, client_id: &String, client_secret: &String, auth_url: &String, token_url: &String, scope: &String) -> String {
    println!("Authorizing...");

    let mut config = Config::new(client_id, client_secret, auth_url, token_url);
    config = config.add_scope(scope);
    config = config.set_redirect_url("http://127.0.0.1:9004");

    println!("Browse to: {}", config.authorize_url());

    let auth_code = rx.recv().expect("Failed to receive auth code from oauth loopback thread");
    return config.exchange_code(auth_code).unwrap().access_token;
}

fn gdrive_body(config: &Configuration, path: &std::path::Path, file_contents: &Vec<u8>) -> Vec<u8> {
    let parent_folder_ids = vec![config.gdrive_folder.clone()];
    let file_metadata = fs::metadata(&path).unwrap();
    let gdrive_metadata = GDriveMetadata {
        name: path.file_name().unwrap().to_str().unwrap().to_string(),
        parents: parent_folder_ids.clone(),
        modified_time: DateTime::from(file_metadata.modified().unwrap())
    };

    // TODO: anything nicer than all these .as_bytes() calls?
    return [
        "--fiiiiit\n".as_bytes(),
        "Content-Type: application/json; charset=UTF-8\n".as_bytes(),
        "\n".as_bytes(),
        serde_json::to_string(&gdrive_metadata).unwrap().as_bytes(),
        "\n".as_bytes(),
        "\n".as_bytes(),
        "--fiiiiit\n".as_bytes(),
        "Content-Type: application/x-binary\n".as_bytes(),
        "\n".as_bytes(),
        &file_contents,
        "\n".as_bytes(),
        "--fiiiiit--\n".as_bytes(),
    ].concat();
}

async fn test_auth(rx: Receiver<String>) {
    let config_data = fs::read(CONFIG_FILE).expect("Unable to read config file");
    let serialized = String::from_utf8(config_data).unwrap();
    let config: Configuration = serde_json::from_str(&serialized).unwrap();

    let gdrive_access_token = auth(
        &rx, &config.gdrive_client_id, &config.gdrive_client_secret,
        &String::from("https://accounts.google.com/o/oauth2/v2/auth"), &String::from("https://oauth2.googleapis.com/token"), &String::from("https://www.googleapis.com/auth/drive.file")).await;

    let strava_access_token = auth(
        &rx, &config.strava_client_id, &config.strava_client_secret,
        &String::from("https://www.strava.com/oauth/authorize"), &String::from("https://www.strava.com/oauth/token"), &String::from("activity:write")).await;

    let client = reqwest::Client::new();
    for entry in fs::read_dir(&format!("{}/{}", "/home/augustus/temp/fit-testing", ACTIVITY_PATH)).unwrap() {
    //for entry in fs::read_dir(&format!("{}/{}", "/media/augustus/GARMIN", ACTIVITY_PATH)).unwrap() {
        let path = entry.unwrap().path();
        let file_contents = fs::read(&path).unwrap();

        let gdrive_body = gdrive_body(&config, &path, &file_contents);

        let gdrive_result = client.post(GDRIVE_UPLOAD_URL)
            .header("Content-Type", "multipart/related; boundary=fiiiiit")
            .header("Content-Length", gdrive_body.len())
            .bearer_auth(&gdrive_access_token)
            .body(gdrive_body)
            .send()
            .await;
        let res = gdrive_result.unwrap();
        println!("GDrive result: {:?}", &res);
        println!("GDrive body: {:?}", res.text().await);

        println!("Uploading file to Strava: {:?}", path);
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

    fn configure(a: &str, b: &str, c: &str) {
        let config = Configuration {
            gdrive_client_id: a.to_string(),
            gdrive_client_secret: b.to_string(),
            gdrive_folder: c.to_string(),
            strava_client_id: "".to_string(),
            strava_client_secret: "".to_string(),
        };
        let serialized = serde_json::to_string(&config).unwrap();
        fs::write(CONFIG_FILE, serialized).unwrap();
    }

use structopt::StructOpt;
use serde::{Serialize, Deserialize};
use std::fs;
use oauth2::Config;
use rouille::Response;
use std::thread;
use std::sync::mpsc::{sync_channel, SyncSender, Receiver};

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

    let (tx, rx): (SyncSender<String>, Receiver<String>) = sync_channel(1);

    let _handle = thread::spawn(move || {
        let sender = tx.clone();
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
        Command::Configure{ params } => configure(&params[0], &params[1]),
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

async fn auth(rx: Receiver<String>, client_id: &String, client_secret: &String) -> String {
    println!("Authorizing...");

    // Create an OAuth2 config by specifying the client ID, client secret, authorization URL and token URL.
    let mut config = Config::new(client_id, client_secret, "https://accounts.google.com/o/oauth2/v2/auth", "https://oauth2.googleapis.com/token");
    config = config.add_scope("https://www.googleapis.com/auth/drive.file");

    // Set the URL the user will be redirected to after the authorization process.
    config = config.set_redirect_url("http://127.0.0.1:9004");

    // Set a state parameter (optional, but recommended).
    //config = config.set_state("1234");

    println!("Browse to: {}", config.authorize_url());

    let auth_code = rx.recv().expect("Failed to receive auth code from oauth loopback thread");

    // Once the user has been redirected to the redirect URL, you'll have access to the authorization code.
    // Now you can trade it for an access token.
    return config.exchange_code(auth_code).unwrap().access_token;
}

async fn test_auth(rx: Receiver<String>) {
    let config_data = fs::read(CONFIG_FILE).expect("Unable to read config file");
    let serialized = String::from_utf8(config_data).unwrap();
    let config: Configuration = serde_json::from_str(&serialized).unwrap();

    println!("WIP");
    let client = reqwest::Client::new();
    let access_token = auth(rx, &config.gdrive_client_id, &config.gdrive_client_secret).await;
    println!("Authorized with GDrive");
    println!("Access token: {}", access_token);
    for entry in fs::read_dir(&format!("{}/{}", "/home/augustus/temp/fit-testing", ACTIVITY_PATH)).unwrap() {
    //for entry in fs::read_dir(&format!("{}/{}", "/media/augustus/GARMIN", ACTIVITY_PATH)).unwrap() {
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
    let config = Configuration { gdrive_client_id: a.to_string(), gdrive_client_secret: b.to_string(), strava_client_id: "".to_string(), strava_client_secret: "".to_string() };
    let serialized = serde_json::to_string(&config).unwrap();
    fs::write(CONFIG_FILE, serialized).unwrap();
}

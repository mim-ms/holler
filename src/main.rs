use clap::Parser;
use config::{Config, File, FileFormat};
use notify_rust::Notification;
use std::thread::sleep;
use std::time::Duration;

const APP_NAME: &str = "holler";
const DEFAULT_ICON_PATH: &str = "assets/icon.png";

#[derive(Debug, serde::Deserialize)]
struct HollerConfig {
    title: Option<String>,
    body: Option<String>,
    icon_path: Option<String>,
    seconds: Option<u64>,
}

impl Default for HollerConfig {
    fn default() -> Self {
        HollerConfig {
            title: Some("holler".to_string()),
            body: Some("Done!".to_string()),
            icon_path: Some(DEFAULT_ICON_PATH.to_string()),
            seconds: Some(0),
        }
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The title of the notification
    /// can be specified with -t or --title
    #[arg(short = 't', long = "title")]
    title: Option<String>,

    /// The body/message of the notification
    /// can be specified with -b or --body
    #[arg(short = 'b', long = "body")]
    body: Option<String>,

    /// Number of seconds to wait before sending the notification
    /// can be specified with -s or --seconds
    #[arg(short = 's', long = "seconds")]
    seconds: Option<u64>,
}

fn load_config() -> HollerConfig {
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => return HollerConfig::default(),
    };
    
    let config_path = home_dir.join(".hollerrc");
    if !config_path.exists() {
        return HollerConfig::default();
    }

    match Config::builder()
        .add_source(File::from(config_path).format(FileFormat::Toml))
        .build()
        .and_then(|c| c.try_deserialize::<HollerConfig>())
    {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Warning: Error parsing config file: {}", e);
            HollerConfig::default()
        }
    }
}

fn main() -> Result<(), notify_rust::error::Error> {
    let args = Args::parse();
    let config = load_config();

    let title = args.title
        .or(config.title)
        .unwrap_or_else(|| "holler".to_string());
    let body = args.body
        .or(config.body)
        .unwrap_or_else(|| "Done!".to_string());
    let icon_path = config.icon_path
        .unwrap_or_else(|| DEFAULT_ICON_PATH.to_string());
    let seconds = args.seconds
        .or(config.seconds)
        .unwrap_or(0);

    if seconds > 0 {
        sleep(Duration::from_secs(seconds));
    }

    // Determine OS and send notification accordingly
    #[cfg(target_os = "macos")]
    {
        Notification::new()
            .summary(&title)
            .body(&body)
            .appname(APP_NAME)
            .icon(&icon_path)
            .timeout(6000)
            .show()?;
    }

    #[cfg(target_os = "linux")]
    {
        let backend = std::env::var("NOTIFY_BACKEND").unwrap_or_else(|_| "dbus-send".to_string());

        match backend.as_str() {
            "dbus-send" | "notify-osd" => {
                Notification::new()
                    .summary(&title)
                    .body(&body)
                    .appname(APP_NAME)
                    .icon(&icon_path)
                    .timeout(6000)
                    .show()?;
            }
            _ => {
                eprintln!("Unsupported notification backend: {}", backend);
                return Err(format!("Unsupported backend: {}", backend).as_str().into());
            }
        }
    }

    Ok(())
}

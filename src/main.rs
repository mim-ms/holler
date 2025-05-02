use clap::Parser;
use notify_rust::Notification;

const APP_NAME: &str = "holler";
const ICON_PATH: &str = "assets/icon.png";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The title of the notification
    /// can be specified with -t or --title
    #[arg(short = 't', long = "title", default_value = "holler")]
    title: String,

    /// The body/message of the notification
    /// can be specified with -b or --body
    #[arg(short = 'b', long = "body", default_value = "Done!")]
    body: String,
}

fn main() -> Result<(), notify_rust::error::Error> {
    let args = Args::parse();

    // Determine OS and send notification accordingly
    #[cfg(target_os = "macos")]
    {
        Notification::new()
            .summary(&args.title)
            .body(&args.body)
            .appname(APP_NAME)
            .icon(ICON_PATH)
            .timeout(6000)
            .show()?;
    }

    #[cfg(target_os = "linux")]
    {
        let backend = std::env::var("NOTIFY_BACKEND").unwrap_or_else(|_| "dbus-send".to_string());

        match backend.as_str() {
            "dbus-send" | "notify-osd" => {
                Notification::new()
                    .summary(&args.title)
                    .body(&args.body)
                    .appname(APP_NAME)
                    .icon(ICON_PATH)
                    .timeout(6000)
                    .show()?;
            }
            _ => {
                eprintln!("Unsupported notification backend: {}", backend);
                return Err(notify_rust::error::Error::Generic(
                    "Unsupported backend".to_string(),
                ));
            }
        }
    }

    Ok(())
}

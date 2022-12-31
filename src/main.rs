use clap::Parser;
use mac_notification_sys;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    notify: bool,
    focus: Vec<String>,
}

use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::Path;

static FOCUS_FILE_PATH: &str = "/tmp/focus";

fn set_focus(focus: String) {
    let mut file = File::create(Path::new(FOCUS_FILE_PATH)).unwrap();
    file.write_all(focus.as_bytes()).unwrap();
}

fn notify() {
    let path = Path::new(FOCUS_FILE_PATH);
    let focus = fs::read_to_string(path);
    // daliver a silent notification
    match focus {
        Ok(focus) => {
            mac_notification_sys::send_notification("Focus on", None, &focus, None)
                .expect("Failed to send MacOS notification.");
        }
        Err(_) => {
            mac_notification_sys::send_notification(
                "Focus on",
                None,
                "Whats your focus today? Run '$ focus-on Finishing that thing' to set your focus.",
                None,
            )
            .expect("Failed to send MacOS notification.");
        }
    }
}

fn main() {
    // this will print and exit if it fails to parse
    let args = Cli::parse();

    if args.notify {
        println!("Running in notify mode.");
        notify()
    } else {
        let focus = args.focus.join(" ");
        set_focus(focus)
    }
}

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    notify: bool,
    focus: Vec<String>,
}

use std::fs::File;
use std::io::Write;
use std::path::Path;

fn set_focus(focus: String) {
    let path = Path::new("/tmp/focus");
    let mut file = File::create(path).unwrap();
    file.write_all(focus.as_bytes()).unwrap();
}

fn main() {
    // this will print and exit if it fails to parse
    let args = Cli::parse();

    if args.notify {
        println!("Running in notify mode.")
    } else {
        let focus = args.focus.join(" ");
        set_focus(focus)
    }
}

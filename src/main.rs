use chrono::prelude::*;
use clap::Parser;
use std::fs::OpenOptions;
use std::io::prelude::*;

#[derive(Parser, Debug)]
struct Opts {
    #[clap(short, long)]
    activity: String,
}

fn main() {
    let opts = Opts::parse();
    let activity = opts.activity;

    let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let log_entry = format!("{}: {}\n", timestamp, activity);

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("worklog.txt")
        .unwrap();

    if let Err(e) = file.write_all(log_entry.as_bytes()) {
        eprintln!("Error writing to file: {}", e);
    } else {
        println!("Activity logged successfully!");
    }
}

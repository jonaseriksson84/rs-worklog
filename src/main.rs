use chrono::Local;
use clap::Parser;

use std::{
    fs::{File, OpenOptions},
    io::{BufRead, BufReader, Write},
};

#[derive(Parser, Debug)]
#[command(name = "rs-worklog", version, author)]
struct Opts {
    #[arg(short, long)]
    activity: String,
}

fn main() {
    let mut exit = false;

    while !exit {
        println!("Choose an option:");
        println!("1. Log activity");
        println!("2. View activities");
        println!("3. Edit activity");
        println!("4. Delete activity");
        println!("5. Exit");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let choice: u8 = input.trim().parse().unwrap_or(0);

        match choice {
            1 => log_activity(),
            2 => view_activities(),
            3 => edit_activity(),
            4 => delete_activity(),
            5 => {
                println!("Exiting...");
                exit = true;
            }
            _ => println!("Invalid option, please try again."),
        }
    }
}

fn log_activity() {
    // Prompt the user for activity input
    println!("Enter the activity:");
    let mut activity = String::new();
    std::io::stdin().read_line(&mut activity).unwrap();
    activity = activity.trim().to_string();

    let date = Local::now().date_naive().format("%Y-%m-%d").to_string();
    let log_entry = format!("{}: {}\n", date, activity);

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

fn view_activities() {
    let file = match File::open("worklog.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            println!("{}", line);
        }
    }
}

fn edit_activity() {
    // TODO: Implement edit_activity
}

fn delete_activity() {
    // TODO: Implement delete_activity
}

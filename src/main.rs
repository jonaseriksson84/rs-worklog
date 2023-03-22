use std::{fs::OpenOptions, io::Write};

use chrono::Local;

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

fn view_activities() {
    // TODO: Implement view_activities
}

fn edit_activity() {
    // TODO: Implement edit_activity
}

fn delete_activity() {
    // TODO: Implement delete_activity
}

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
    let file = match File::open("worklog.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let mut activities: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if activities.is_empty() {
        println!("No activities found.");
        return;
    }

    println!("Activities:");
    for (index, activity) in activities.iter().enumerate() {
        println!("{}: {}", index + 1, activity);
    }

    loop {
        println!("Enter the line number to edit or enter 0 to cancel:");
        let mut line_number = String::new();
        std::io::stdin().read_line(&mut line_number).unwrap();
        let line_number: usize = match line_number.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    return;
                } else if num > activities.len() {
                    println!("Invalid line number, please try again.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Invalid input, please try again.");
                continue;
            }
        };

        println!("Enter the new activity:");
        let mut new_activity = String::new();
        std::io::stdin().read_line(&mut new_activity).unwrap();
        new_activity = new_activity.trim().to_string();
        let date = &activities[line_number - 1][..10];
        activities[line_number - 1] = format!("{}: {}", date, new_activity);

        let mut file = match File::create("worklog.txt") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file: {}", e);
                return;
            }
        };

        for activity in activities {
            if let Err(e) = writeln!(file, "{}", activity) {
                eprintln!("Error writing to file: {}", e);
                break;
            }
        }

        println!("Activity edited successfully!");
        break;
    }
}


fn delete_activity() {
    let file = match File::open("worklog.txt") {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let mut activities: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    if activities.is_empty() {
        println!("No activities found.");
        return;
    }

    println!("Activities:");
    for (index, activity) in activities.iter().enumerate() {
        println!("{}: {}", index + 1, activity);
    }

    loop {
        println!("Enter the line number to delete or enter 0 to cancel:");
        let mut line_number = String::new();
        std::io::stdin().read_line(&mut line_number).unwrap();
        let line_number: usize = match line_number.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    return;
                } else if num > activities.len() {
                    println!("Invalid line number, please try again.");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Invalid input, please try again.");
                continue;
            }
        };

        activities.remove(line_number - 1);

        let mut file = match File::create("worklog.txt") {
            Ok(file) => file,
            Err(e) => {
                eprintln!("Error creating file: {}", e);
                return;
            }
        };

        for activity in activities {
            if let Err(e) = writeln!(file, "{}", activity) {
                eprintln!("Error writing to file: {}", e);
                break;
            }
        }

        println!("Activity deleted successfully!");
        break;
    }
}


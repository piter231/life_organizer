use std::env;
use std::fs::{OpenOptions, read_to_string};
use std::io::{Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <command> [arguments]\nuse help commands to get list of arguments", args[0]);
        return;
    }

    let command = &args[1];
    match command.as_str() {
        "add" => add_task(&args[2..]),
        "list" => list_tasks(),
        "remove" => remove_task(&args[2..]),
        "help" => help(),
        _ => eprintln!("Unknown command: {}", command),
    }
}

fn add_task(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: add <task>");
        return;
    }

    let task = args.join(" ");
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("todo.txt")
        .expect("Unable to open file");

    writeln!(file, "{}", task).expect("Unable to write to file");
    println!("Added task: {}", task);
}

fn list_tasks() {
    let contents = read_to_string("todo.txt").unwrap_or_else(|_| String::from(""));

    if contents.is_empty() {
        println!("No tasks found.");
    } else {
        println!("Tasks:");
        for (i, task) in contents.lines().enumerate() {
            println!("{}. {}", i + 1, task);
        }
    }
}

fn help(){
    println!("Usage: todo <command> [arguments]");
    println!("Commands:");
    println!("  help          Displays this help message");
    println!("  add <task>    Adds a task to the list");
    println!("  list          Lists all tasks");
    println!("  remove <task number>    Removes a task from the list");
}

fn remove_task(args: &[String]) {
    if args.is_empty() {
        eprintln!("Usage: remove <task number>");
        return;
    }

    let task_number: usize = match args[0].parse() {
        Ok(num) => num,
        Err(_) => {
            eprintln!("Invalid task number: {}", args[0]);
            return;
        }
    };

    let contents = read_to_string("todo.txt").unwrap_or_else(|_| String::from(""));
    let mut tasks: Vec<&str> = contents.lines().collect();

    if task_number == 0 || task_number > tasks.len() {
        eprintln!("Task number out of range");
        return;
    }

    tasks.remove(task_number - 1);

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("todo.txt")
        .expect("Unable to open file");

    for task in tasks {
        writeln!(file, "{}", task).expect("Unable to write to file");
    }

    println!("Removed task number {}", task_number);
}
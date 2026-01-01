use std::{io, ops::ControlFlow};
use chrono::{Local, DateTime};

#[derive(Clone)]
#[derive(Debug)]
struct TodoItem {
    task: String,
    status: bool,
}

fn main() {
    println!("Hello, welcome to todo app!");
    let now: DateTime<Local> = Local::now();
    println!("Today is: {}", now); // Prints: Current Date & Time: 2024-01-01 10:00:00 +05:30 (example)
    let mut todo_list: Vec<TodoItem> = Vec::new();

    loop {
        if let ControlFlow::Break(_) = process_user_command(&mut todo_list) {
            break;
        }
    }
    println!("ByeBye...!")
}

fn process_user_command(todo_list: &mut Vec<TodoItem>) -> ControlFlow<()> {
    let mut user_command = String::new();
    println!("\nEnter Command:");
    io::stdin()
        .read_line(&mut user_command)
        .expect("Failed to read line");


    match user_command.as_str().trim() {
        // Match a single value
        "add" => add_todo_item(todo_list),
        // add todo Item
        "print" => {let copy_of_todo = todo_list.clone(); print_todo_list(copy_of_todo)},
        // print all todo items in vector
        "quit" => return ControlFlow::Break(()),
        _ => println!("invalid command"),
        // catch-all arm
    }
    ControlFlow::Continue(())
}

fn add_todo_item(todo_list: &mut Vec<TodoItem>) {
    let mut user_input = String::new();
    println!("Enter Task Description:");
    read_user_input(&mut user_input);

    let new_todo = TodoItem {
        task: String::from(user_input.trim()),
        status: false,
    };

    todo_list.push(new_todo);
}

fn read_user_input(user_input: &mut String) {
    loop{
    io::stdin()
        .read_line(user_input)
        .expect("Failed to read line");

        if user_input.trim().len() == 0 {
            println!("You didnt enter anything ");
            continue;
        } else {
            break;
        }

    }
}

fn print_todo_list(todo_list: Vec<TodoItem>) {
    println!("{:?}", todo_list);

    println!("\n\n== List of Todo Items ==");
    for item in todo_list {
        println!("{}\t\t - {}", item.task, item.status);
        // println!("{:?}", item);
    }
}


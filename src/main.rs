use std::{io, ops::ControlFlow, usize};
use chrono::{Local, DateTime};

#[derive(Clone)]
#[derive(Debug)]
struct TodoItem {
    task: String,
    status: bool,
    cdate: DateTime<Local>
}

fn main() {
    println!("=Hello, welcome to todo app!=");
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
        "remove" => remove_todo_item(todo_list),
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
    let task_create_date: DateTime<Local> = Local::now();

    let new_todo = TodoItem {
        task: String::from(user_input.trim()),
        status: false,
        cdate: task_create_date
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
    // println!("{:?}", todo_list);
    let now: DateTime<Local> = Local::now();
    let mut counter = 1;
    println!("\n\n== List of Todo Items ==");
    for item in todo_list {
        let diff = get_time_diff_string(now, &item);
        let status = get_status_string(&item);
        
        println!("{}. {} ago \t| {}\t\t - {}", counter, diff, item.task, status);
        counter+=1;
    }
}

fn get_time_diff_string(now: DateTime<Local>, item: &TodoItem) -> String {
    let mut diff_string: String = "".to_string();
    let diff = (now - item.cdate).num_seconds();
    
    if diff > 60 {
        diff_string = (diff / 60).to_string() + "min";
    } else {
        diff_string = diff.to_string() + "s"
    }

    diff_string
}

fn get_status_string(item: &TodoItem) -> String {
    let mut status: String = "".to_string();
    if item.status == false { 
        status = "Pending".to_string();
    } else {
        status = "Done".to_string()
    }
    status
}

fn remove_todo_item(todo_list: &mut Vec<TodoItem>) {
    let mut user_input = String::new();
    println!("Enter serial number of Task to remove:");
    read_user_input(&mut user_input);
    user_input = user_input.trim().to_string();
    let index_number = user_input.parse::<usize>().unwrap();
    if index_number < todo_list.len() {
        todo_list.remove(index_number-1);
        println!("Done......");
    } else {
        println!("Sorry, there are only {} Items in the list.",todo_list.len())
    }
}
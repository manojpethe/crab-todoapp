use std::{io, ops::ControlFlow, usize};
use chrono::{Local, DateTime};
use todoapp::common::greet;
use todoapp::common::read_user_input;
use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
#[derive(Clone)]
#[derive(Debug)]
// #[derive(serde::Serialize, serde::Deserialize, Debug,Clone)]
struct TodoItem {
    pub task: String,
    pub status: bool,
    pub cdate: DateTime<Local>
}

fn main() {
    println!("=Hello, welcome to todo app!=");
    greet("Manoj".to_string(),"En".to_string());
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
        // remove todo Item
        "update" => update_todo_item(todo_list),
        // update todo Item
        "print" => {let copy_of_todo = todo_list.clone(); print_todo_list(copy_of_todo)},
        // print all todo items in vector
        "save" => save_to_file(todo_list),
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
    println!("Enter serial number of Task:");
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

fn update_todo_item(todo_list: &mut Vec<TodoItem>) {
    let mut user_input = String::new();
    println!("Enter serial number of Task:");
    read_user_input(&mut user_input);
    user_input = user_input.trim().to_string();
    let index_number = user_input.parse::<usize>().unwrap();

    println!("Shall I mark it as done? (y/n)");
    if index_number <= todo_list.len() {
        let mut user_confirmation = String::new();
        read_user_input(&mut user_confirmation);
        if user_confirmation.trim().to_string() == "y".to_string() {
            todo_list[index_number-1].status = true;
            println!("Marked item as Done....!");
        } else {
            println!(" response is {user_confirmation}");
        }
    } else {
        println!("Sorry, there are only {} Items in the list.",todo_list.len())
    }

}

fn save_to_file(data: &mut Vec<TodoItem> ){
    let serialized_data = serde_json::to_string(&data).unwrap();
    println!("here you go: {}",serialized_data);
}
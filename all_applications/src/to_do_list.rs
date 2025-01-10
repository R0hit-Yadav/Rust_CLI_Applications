use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};
use chrono::NaiveDate;

// Define the CLI structure
#[derive(Parser)]
#[command(name="TO-DO Manager")]
#[command(about="A simple CLI-based to-do-list manager", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Define the possible commands
#[derive(Subcommand)]
enum Commands {
    Add { title: String, due_date: String },
    List,
    Remove { id: usize },
    Complete { id: usize },
}

// Define the Task structure
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: usize,
    title: String,
    due_date: String,
    completed: bool,
}

const FILE_NAME: &str = "tasks.json";  // File to store all tasks

pub fn main() {
    // Display available commands
    println!("Commands to Run: ");
    println!("1. ADD TASK:          cargo run -- add 'Project Name' 10-01-2025");
    println!("2. LIST TASK:         cargo run -- list");
    println!("3. REMOVE TASK:       cargo run -- remove 1");
    println!("4. COMPLETE TASK:     cargo run -- complete 1");
    println!("");

    // Parse the CLI arguments
    let cli = Cli::parse();

    // Match the command and call the corresponding function
    match cli.command {
        Commands::Add { title, due_date } => add_task(title, due_date),
        Commands::List => list_tasks(),
        Commands::Remove { id } => remove_task(id),
        Commands::Complete { id } => complete_task(id),
    }
}

// Load tasks from the file
fn load_tasks() -> Vec<Task> {
    let mut file = match File::open(FILE_NAME) {
        Ok(file) => file,
        Err(_) => return vec![],
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

// Save tasks to the file
fn save_tasks(tasks: &[Task]) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = File::create(FILE_NAME).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

// Add a new task
fn add_task(title: String, due_date: String) {
    let mut tasks = load_tasks();
    let id = tasks.len() + 1;

    // Validate the date format
    if NaiveDate::parse_from_str(&due_date, "%d-%m-%Y").is_err() {
        eprintln!("Invalid date format. Use DD-MM-YYYY");
        return;
    }

    // Add the new task to the list
    tasks.push(Task { id, title, due_date, completed: false });

    // Save the updated task list
    save_tasks(&tasks);
    println!("Task Added!!")
}

// List all tasks
fn list_tasks() {
    let tasks = load_tasks();

    if tasks.is_empty() {
        println!("No tasks found");
        return;
    }

    // Print each task
    for task in &tasks {
        println!("{}. {} (Due: {}) [Status: {}]", task.id, task.title, task.due_date, if task.completed { "Completed..✓" } else { "Pending..✕" });
    }
}

// Remove a task by ID
fn remove_task(id: usize) {
    let mut tasks = load_tasks();

    // Find the task by ID and remove it
    if let Some(index) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(index);
        save_tasks(&tasks);
        println!("Task removed!!")
    } else {
        println!("Task not found!!")
    }
}

// Mark a task as completed by ID
fn complete_task(id: usize) {
    let mut tasks = load_tasks();

    // Find the task by ID and mark it as completed
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.completed = true;
        save_tasks(&tasks);
        println!("Task completed!!")
    } else {
        println!("Task not found!!")
    }
}
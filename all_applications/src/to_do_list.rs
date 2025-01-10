use clap::{Parser,Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read,Write};
use chrono::NaiveDate;

#[derive(Parser)]
#[command(name="TO-DO Manager")]
#[command(about="A simple CLI-based to-do-list manager",long_about=None)]// to print in terminal

struct Cli
{
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add{title:String,due_date:String},
    List,
    Remove{id:usize},
    Complete{id:usize},

    
}

#[derive(Serialize,Deserialize,Debug)]
struct Task
{
    id:usize,
    title:String,
    due_date:String,
    completed:bool,
}

const FILE_NAME: &str = "tasks.json";  // store all task in task.json file
pub fn main() {
    println!("Commands to Run: ");
    println!("1.ADD TASK:          cargo run -- add ''Project Name'' 10-01-2025");
    println!("2.LIST TASK:         cargo run -- list");
    println!("3.REMOVE TASK:       cargo run -- remove 1");
    println!("4.COMPLETE TASK:     cargo run -- complete 1");
    println!("");

    let cli=Cli::parse();// for all commands

    match cli.command
    {
        Commands::Add{title,due_date}=>add_task(title,due_date),
        Commands::List=>list_tasks(),
        Commands::Remove { id }=>remove_task(id),
        Commands::Complete { id }=>complete_task(id),
    }
}

fn load_tasks()->Vec<Task> // load all tasks in vec 
{
    let mut file=match File::open(FILE_NAME)
    {
        Ok(file)=>file,
        Err(_)=>return  vec![],
    };

    let mut data=String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_|vec![])
}

fn save_tasks(tasks:&[Task]) // for add and save tasks
{
    let data=serde_json::to_string_pretty(tasks).unwrap();
    let mut file=File::create(FILE_NAME).unwrap();
    file.write_all(data.as_bytes()).unwrap();

}

fn add_task(title:String,due_date:String) // add task functions
{
    let mut tasks=load_tasks();
    let id=tasks.len()+1;

    if NaiveDate::parse_from_str(&due_date,"%d-%m-%Y").is_err()// for proper format
    {
        eprintln!("Invalid date format.Use DD-MM-YYYY");
        return;
    }

    tasks.push(Task{id,title,due_date,completed:false});//add tasks in vector

    save_tasks(&tasks);
    println!("Task Added!!")

}
fn list_tasks()//for list of tasks
{
    let tasks=load_tasks();

    if tasks.is_empty()
    {
        println!("No tasks found");
        return;
    }

    for task in &tasks
    {
        println!("{}.{} (Due: {}) [Status:{} ]",task.id, task.title,task.due_date,if task.completed{"Completed..✓"} else {"Panding..✕"});
    }
    
}
fn remove_task(id:usize) // for delete a tasks
{
    let mut tasks=load_tasks();

    if let Some(index)=tasks.iter().position(|t|t.id==id)
    {
        tasks.remove(index);
        save_tasks(&tasks);
        println!("Task removed!!")
    }
    else {
        println!("Task not found!!")
    }

}
    
fn complete_task(id:usize) // for marks completed tasks
{
    let mut tasks=load_tasks();

    if let Some(task)=tasks.iter_mut().find(|t| t.id==id)
    {
        task.completed=true;
        save_tasks(&tasks);
        println!("Task completed!!");

    }
    else {
        println!("Task not found!!")
    }

}


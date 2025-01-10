use clap::{Parser,Subcommand};
use chrono::{DateTime, Local};
use serde::{Deserialize,Serialize};
use std::fs::File;
use std::io::{Read,Write};


#[derive(Parser)]
#[command(name="Time Tracker")]
#[command(about="A simple CLI-based time tracker",long_about=None)]

struct Cli {
    #[command(subcommand)]
    command:Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start{task_name:String},
    Stop,
    Report,
    
}

#[derive(Serialize,Deserialize,Debug)]
struct Task 
{
    task_name:String,
    start_time:DateTime<Local>,
    end_time:Option<DateTime<Local>>,
}

const FILE_NAME:&str="time_tracker.json";

pub fn main()
{

    println!("Commands to Run: ");
    println!("1.TASK START:        cargo run -- start ''Project Name''");
    println!("2.TASK STOP:         cargo run -- stop");
    println!("3.TASK REPORT:       cargo run -- report");
    println!("");

    let cli=Cli::parse();

    match cli.command
    {
        Commands::Start { task_name }=>start_task(task_name),
        Commands::Stop=>stop_task(),
        Commands::Report=>generate_report(),

    }
}

fn load_tasks()->Vec<Task>
{
    let mut file=match File::open(FILE_NAME)
    {
        Ok(file)=>file,
        Err(_)=>return  vec![],
    };

    let mut data=String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])

}

fn save_tasks(tasks:&[Task])
{
    let data=serde_json::to_string_pretty(tasks).unwrap();
    let mut file=File::create(FILE_NAME).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

fn start_task(task_name:String)
{

    let mut tasks=load_tasks();

    if tasks.iter().any(|task|task.end_time.is_none())
    {
        eprintln!("A task is already running.please stop it before start new one");
        return;
    }

    let task=Task{task_name, start_time:Local::now(), end_time:None};

    tasks.push(task);
    save_tasks(&tasks);
    println!("Task started!!");
}

fn stop_task()
{

    let mut tasks=load_tasks();

    if let Some(task)=tasks.iter_mut().find(|task|task.end_time.is_none())
    {
        task.end_time=Some(Local::now());
        save_tasks(&tasks);
        println!("Task stopped!!")
    }
    else{
        println!("No task is running");
    }

}

fn generate_report()
{

    let tasks=load_tasks();

    if tasks.is_empty()
    {
        println!("No tasks found");
        return;
    }

    println!("{:<20} {:<20} {:<20} {:<20}", "Task Name", "Start Time", "End Time", "Duration");
    println!("{}","-".repeat(80));

    for task in tasks
    {
        let duration = if let Some(end_time) = task.end_time {
            let duration = end_time.signed_duration_since(task.start_time);
            format!("{:02}:{:02}:{:02}", duration.num_hours(), duration.num_minutes() % 60, duration.num_seconds() % 60)
        } else {
            "In Progress".to_string()
        };

        println!(
            "{:<20} {:<25} {:<25} {:<10}",
            task.task_name,
            task.start_time.format("%d-%m-%Y %H:%M:%S"),
            task.end_time
            .map(|t|t.format("%d-%m-%Y %H:%M:%S").to_string())
            .unwrap_or("in progress".to_string()),
            duration
        );
           
    }

}




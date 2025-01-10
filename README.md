# Rust_CLI_Applications
CLI_Applications 


## Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your system.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/your-username/todo-manager.git
   cd all_applications

## Dependencies
<b>clap</b> for parsing command-line arguments.

<b>serde</b> for serializing and deserializing JSON data.

<b>serde_json</b> for working with JSON.

<b>chrono</b> for date validation.



# 1.TO-DO-LIST 
A simple CLI-based to-do list manager built with Rust. This application allows you to manage tasks by adding, listing, 
removing, and marking them as completed, with tasks stored persistently in a JSON file.

## Features
- Add new tasks with a title and due date.
- View a list of all tasks.
- Remove tasks by their unique ID.
- Mark tasks as completed by their ID.
- Task data is stored persistently in `tasks.json`.

## Commands to run 
- Add Task 
``` bush cargo run -- add "project name" 10-01-2025




# 2.Time Tracker
A simple CLI-based time tracker application written in Rust. This tool allows you to track tasks, 
log their start and stop times, and generate a detailed report of all recorded tasks.

## Features
- Start tracking a new task.
- Stop the currently running task.
- Generate a report of all tasks, including task names, start times, end times, and durations.
- Tasks are stored persistently in a JSON file (`time_tracker.json`).


# 3.CLI-Notebook
A simple CLI-based note-taking application written in Rust. This tool allows you to add, view, delete, and list notes, which are stored persistently in a JSON file.

## Features
- Add new notes with a title and content.
- View a specific note by its title.
- Delete a note by its title.
- List all existing notes.
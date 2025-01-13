# Rust_CLI_Applications
CLI_Applications 


## Prerequisites
- [Rust](https://www.rust-lang.org/) installed on your system.

## Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/R0hit-Yadav/Rust_CLI_Applications.git
   cd all_applications

## Dependencies
<b>clap -></b> for parsing command-line arguments.

<b>serde -></b> for serializing and deserializing JSON data.

<b>serde_json -></b> for working with JSON.

<b>chrono -></b> for date validation.




# 1.TO-DO-LIST 
This program is a command-line interface (CLI) application for managing a to-do list. It allows users to add tasks, list all tasks, mark tasks as completed, and remove tasks. The tasks are stored persistently in a JSON file named tasks.json.

## Imports
``use clap::{Parser, Subcommand};``
Provides a convenient way to define and parse CLI arguments.

``use serde::{Deserialize, Serialize};``
Enables serialization (saving to JSON) and deserialization (loading from JSON) of data structures.

``use std::fs::File;``
Used for reading and writing files.

``use std::io::{Read, Write};``
Used for file I/O operations.

``use chrono::NaiveDate;``
Provides date parsing and validation.

## Features
- Add new tasks with a title and due date.
- View a list of all tasks.
- Remove tasks by their unique ID.
- Mark tasks as completed by their ID.
- Task data is stored persistently in `tasks.json`.

## Commands to run 
- Add Task        ```cargo run -- add "project name" 10-01-2025 ```
- List Task       ```cargo run -- list```
- Remove Task     ```cargo run -- remove 1```
- Complete Task   ```cargo run -- complete 1```



# 2.Time Tracker
This Rust program implements a command-line time tracker. It allows you to start a task, stop the currently running task, and generate a report of all tasks. The program uses the clap crate for command-line argument parsing, the chrono crate for date and time handling, and the serde crate for serialization and deserialization of data.


## Features
- Start tracking a new task.
- Stop the currently running task.
- Generate a report of all tasks, including task names, start times, end times, and durations.
- Tasks are stored persistently in a JSON file (`time_tracker.json`).

## Commands to run 
- Task Start         ```cargo run -- start "project name"```
- Task Stop        ```cargo run -- stop```
- Task Report     ```cargo run -- report```


# 3.CLI-Notebook
This program is a command-line interface (CLI) application for a note-taking application called CLI Notebook, written in Rust. It allows users to perform actions such as adding, viewing, deleting, and listing notes.

## Features
- Add new notes with a title and content.
- View a specific note by its title.
- Delete a note by its title.
- List all existing notes.


## Commands to run 
- Add Note        ```cargo run -- add --title "title of notebook" ```
- View Note       ```cargo run -- view --title "title of notebook"```
- Remove Note     ```cargo run -- delete --title "title of notebook"```
- List Of Notes   ```cargo run -- list```





# 3.Weather API
Weather CLI application that fetches weather information for a given city using the OpenWeatherMap API

## Imports
``clap::Parser``: This is a command-line argument parsing library. It simplifies fetching user-provided inputs via the CLI.

``serde::Deserialize``: serde is used for serializing and deserializing data. Here, it’s used to convert JSON responses from the API into Rust structs.

``std::error::Error``: A trait for defining errors. This allows functions to return errors in a standardized way.


## Commands to run 
- get weather of city : `cargo run -- --city "Ahmedabad"`
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

// Define the CLI structure
#[derive(Parser)]
#[command(name="CLI Notebook")]
#[command(about="A simple CLI-based note-taking application", long_about=None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Define the possible commands
#[derive(Subcommand)]
enum Commands {
    Add {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        content: String,
    },
    View {
        #[arg(short, long)]
        title: String,
    },
    Delete {
        #[arg(short, long)]
        title: String,
    },
    List,
}

// Define the Note structure
#[derive(Serialize, Deserialize, Debug)]
struct Note {
    title: String,
    content: String,
}

const FILE_NAME: &str = "notes.json";

pub fn main() {
   

    // Parse the CLI arguments
    let cli = Cli::parse();

    // Match the command and call the corresponding function
    match cli.command {
        Commands::Add { title, content } => add_note(title, content),
        Commands::View { title } => view_note(title),
        Commands::Delete { title } => delete_note(title),
        Commands::List => list_notes(),
    }
}

// Load notes from the file
fn load_notes() -> Vec<Note> {
    let mut file = match File::open(FILE_NAME) {
        Ok(file) => file,
        Err(_) => return vec![],
    };

    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    serde_json::from_str(&data).unwrap_or_else(|_| vec![])
}

// Save notes to the file
fn save_notes(notes: &[Note]) {
    let data = serde_json::to_string_pretty(notes).unwrap();
    let mut file = File::create(FILE_NAME).unwrap();
    file.write_all(data.as_bytes()).unwrap();
}

// Add a new note
fn add_note(title: String, content: String) {
    let mut notes = load_notes();

    if notes.iter().any(|note| note.title == title) {
        eprint!("A note with the title '{}' already exists", title);
        return;
    }

    notes.push(Note { title, content });
    save_notes(&notes);
    println!("Note added!!");
}

// View a note by title
fn view_note(title: String) {
    let notes = load_notes();

    if let Some(note) = notes.iter().find(|note| note.title == title) {
        println!("Title: {}\nContent: {}", note.title, note.content);
    } else {
        println!("Note Not Found!");
    }
}

// Delete a note by  title
fn delete_note(title: String) {
    let mut notes = load_notes();

    let initial_len = notes.len();
    notes.retain(|note| note.title != title);

    if notes.len() == initial_len {
        println!("Note not found!");
    } else {
        save_notes(&notes);
        println!("Note deleted!");
    }
}

// List all notes
fn list_notes() {
    let notes = load_notes();

    if notes.is_empty() {
        println!("No notes found!");
    } else {
        println!("Notes:");
        for note in notes {
            println!("- {}", note.title);
        }
    }
}
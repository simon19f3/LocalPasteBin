use clap::{Parser, Subcommand};
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs;
use std::path::Path;

const PASTES_DIR: &str = ".pastes";

#[derive(Parser)]
#[command(name = "pastebin")]
#[command(about = "A lightweight local CLI pastebin")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new paste
    New { text: String },
    /// Read an existing paste by ID
    Read { id: String },
    /// List all available paste IDs
    List,
    /// Delete a paste by ID
    Delete { id: String },
}

fn main() {
    // Ensure .pastes directory exists
    if !Path::new(PASTES_DIR).exists() {
        fs::create_dir_all(PASTES_DIR).expect("Failed to create .pastes directory");
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::New { text } => create_paste(&text),
        Commands::Read { id } => read_paste(&id),
        Commands::List => list_pastes(),
        Commands::Delete { id } => delete_paste(&id),
    }
}

fn generate_id() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

fn create_paste(text: &str) {
    let id = generate_id();
    let filepath = format!("{}/{}.txt", PASTES_DIR, id);

    match fs::write(&filepath, text) {
        Ok(_) => println!("Success! Paste saved with ID: {}", id),
        Err(e) => println!("Error: Failed to save paste: {}", e),
    }
}

fn read_paste(id: &str) {
    let filepath = format!("{}/{}.txt", PASTES_DIR, id);

    match fs::read_to_string(&filepath) {
        Ok(content) => println!("{}", content),
        Err(_) => println!("Error: Paste not found."),
    }
}

fn list_pastes() {
    let entries = match fs::read_dir(PASTES_DIR) {
        Ok(entries) => entries,
        Err(e) => {
            println!("Error: Could not read .pastes directory: {}", e);
            return;
        }
    };

    let mut count = 0;
    for entry in entries {
        if let Ok(entry) = entry {
            let filename = entry.file_name();
            let name = filename.to_string_lossy();
            if name.ends_with(".txt") {
                println!("{}", name.trim_end_matches(".txt"));
                count += 1;
            }
        }
    }

    if count == 0 {
        println!("No pastes found.");
    } else {
        println!("Total pastes: {}", count);
    }
}

fn delete_paste(id: &str) {
    let filepath = format!("{}/{}.txt", PASTES_DIR, id);

    match fs::remove_file(&filepath) {
        Ok(_) => println!("Success! Paste {} deleted.", id),
        Err(_) => println!("Error: Paste not found."),
    }
}
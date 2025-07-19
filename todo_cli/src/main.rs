use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader, Write};

const FILE_PATH: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A simple CLI ToDo list written in Rust")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum  Commands {
    Add { description: String },
    List,
    Done { id: u32 },
    Remove { id: u32 },
    Clear,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut tasks = load_tasks().unwrap_or_else(|_| vec![]);

    match cli.command {
        Commands::Add {description } => {
            let new_id = tasks.last().map_or(1, |t| t.id + 1);
            let task = Task {
                id: new_id,
                description,
                done: false,
            };
            tasks.push(task);
            save_tasks(&tasks)?;
            println!("Task added successfully.");
        }
        Commands::List => {
            if tasks.is_empty() {
                println!("No tasks found.");
            } else {
                for task in &tasks {
                    let status = if task.done { "✅" } else { "⌛" };
                    println!("[{}] {}: {}", status, task.id, task.description);
                }
            }
        }
        Commands::Done { id } => {
            if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
                task.done = true;
                save_tasks(&tasks)?;
                println!("Task {} marked as done.", id);
            } else {
                println!("Task with ID {} not found.", id);
            }
        }
        Commands::Remove { id } => {
            if let Some(pos) = tasks.iter().position(|t| t.id == id) {
                tasks.remove(pos);
                save_tasks(&tasks)?;
                println!("Task {} removed successfully.", id);
            } else {
                println!("Task with ID {} not found.", id);
            }
        }
        Commands::Clear => {
            tasks.clear();
            save_tasks(&tasks)?;
            println!("All tasks cleared.");
        }
    }

    Ok(())
}

/**
 * JSONファイルからタスクを読み込む
 * @return タスクリスト
 */
fn load_tasks() -> io::Result<Vec<Task>> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader).unwrap_or_else(|_| vec![]);
    Ok(tasks)
}

/**
 * タスクをJSONファイルに保存する
 * @param tasks タスクリスト
 * @return io::Result<()>
 */
fn save_tasks(tasks: &[Task]) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)?;
    let json = serde_json::to_string_pretty(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}
use eframe::{egui, App, Frame};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

const FILE_PATH: &str = "todo.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

pub struct TodoApp {
    tasks: Vec<Task>,
    new_task: String,
    next_id: u32,
}

impl Default for TodoApp {
    fn default() -> Self {
        let tasks = load_tasks().unwrap_or_default();
        let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0) + 1;
        Self {
            tasks,
            new_task: String::new(),
            next_id,
        }
    }
}

impl App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 ToDo List");

            // 入力欄と追加ボタン
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Add").clicked() {
                    if !self.new_task.trim().is_empty() {
                        self.tasks.push(Task {
                            id: self.next_id,
                            description: self.new_task.trim().to_string(),
                            done: false,
                        });
                        self.next_id += 1;
                        self.new_task.clear();
                        let _ = save_tasks(&self.tasks);
                    }
                }
            });

            ui.separator();

            // タスク一覧
            for task in &mut self.tasks {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut task.done, "");
                    ui.label(if task.done {
                        format!("✅ {}", task.description)
                    } else {
                        task.description.clone()
                    });
                    if ui.button("🗑").clicked() {
                        task.id = 0; // 削除マーク
                    }
                });
            }

            // 削除処理
            let original_len = self.tasks.len();
            self.tasks.retain(|t| t.id != 0);
            if self.tasks.len() != original_len {
                let _ = save_tasks(&self.tasks);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rust GUI ToDo List",
        options,
        Box::new(|_cc| Box::new(TodoApp::default())),
    )
}

fn load_tasks() -> std::io::Result<Vec<Task>> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

fn save_tasks(tasks: &[Task]) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(FILE_PATH)?;
    let json = serde_json::to_string_pretty(tasks)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

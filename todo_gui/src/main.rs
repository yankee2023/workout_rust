use eframe::{egui, App, Frame};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Write};

const FILE_PATH: &str = "todo.json";
const FONT_PATH: &str = "Noto_Sans_JP/static/NotoSansJP-Black.ttf";
const FONT_JP: &str = "my_jp";

/// タスクの構造体
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    done: bool,
}

/// ToDoアプリケーションの状態を保持する構造体
pub struct ToDoApp {
    tasks: Vec<Task>,
    new_task: String,
    next_id: u32,
}

/// ToDoAppのデフォルト実装
impl Default for ToDoApp {
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

/// `App`トレイトの実装
impl App for ToDoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📝 ToDo List");

            // 入力欄と追加ボタン
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("追加").clicked() {
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
                    if ui.button("🗑️").clicked() {
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
        Box::new(|_cc| {
            apply_japanese_font(&_cc.egui_ctx);
            Box::new(ToDoApp::default())
        }),
    )
}

/**
 * タスクをファイルから読み込む
 * @return std::io::Result<Vec<Task>> 成功時はタスクのリスト、失敗時はErr
 */
fn load_tasks() -> std::io::Result<Vec<Task>> {
    let file = File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let tasks = serde_json::from_reader(reader)?;
    Ok(tasks)
}

/**
 * タスクをファイルに保存する
 * @param tasks 保存するタスクのリスト
 * @return std::io::Result<()> 成功時はOk、失敗時はErr
 */
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

/**
 * 日本語フォントを適用する
 * @param ctx eguiのコンテキスト
 */
fn apply_japanese_font(ctx: &egui::Context) {
    use egui::FontData;
    use egui::FontDefinitions;
    use egui::FontFamily;

    let mut fonts = FontDefinitions::default();

    // フォントファイルを読み込む（パスはプロジェクトに応じて変える）
    fonts.font_data.insert(
        FONT_JP.to_string(),
        FontData::from_owned(
            std::fs::read(FONT_PATH).expect("フォント読み込み失敗"),
        ),
    );

    // プロポーショナルフォントの先頭に日本語フォントを設定
    fonts.families
        .get_mut(&FontFamily::Proportional)
        .unwrap()
        .insert(0, FONT_JP.to_string());

    // 等幅フォントにも設定
    fonts.families
        .get_mut(&FontFamily::Monospace)
        .unwrap()
        .insert(0, FONT_JP.to_string());

    ctx.set_fonts(fonts);
}

use serde::{Serialize, Deserialize};
use eframe::egui;
use egui_commonmark::{CommonMarkCache, CommonMarkViewer};
use chrono::Local;
#[derive(Serialize, Deserialize, Clone)]

struct Document {
    name: String,
    content: String,
    date_created: String,
    date_modified: String,
}
struct App {
    documents: Vec<Document>,
    selected: String,
    current_text: String, 
    cache: CommonMarkCache,
}

impl App {
    fn new() -> App {
        App {
            documents: load_documents(),
            selected: String::new(),
            current_text: String::new(),
            cache: CommonMarkCache::default(),
        }
    }
}
impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::SidePanel::left("doc_list").show(ctx, |ui| {
            ui.heading("Files");
            ui.separator(); 

            if ui.button("New Document").clicked() {
                self.current_text.clear();
            }

            if ui.button("Save Current").clicked() {
                let now = Local::now().format("%Y-%m-%d %H:%M").to_string();
                
                let new_doc = Document {
                    name: format!("Document {}", self.documents.len() + 1),
                    content: self.current_text.clone(),
                    date_created: now.clone(),
                    date_modified: now,
                };
                self.documents.push(new_doc);
                save_documents(&self.documents);
            }

            ui.separator();

        
            for doc in &self.documents {
                if ui.selectable_label(self.selected == doc.name, &doc.name).clicked() {
                    self.current_text = doc.content.clone();
                    self.selected = doc.name.clone();
                }
            }
        });
        egui::SidePanel::right("preview")
            .resizable(true)
            .min_width(200.0)
            .max_width(800.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical()
                    .auto_shrink([false, false]) 
                    .show(ui, |ui| {
                    CommonMarkViewer::new().show(ui, &mut self.cache, &self.current_text,);
                    });
            });

        egui::CentralPanel::default().show(ctx, |ui|{
            ui.add_sized(
                ui.available_size(),
                egui::TextEdit::multiline(&mut self.current_text)
            );
        });
    }
}
fn save_documents(docs: &Vec<Document>) {
    let json = serde_json::to_string(docs).unwrap();
    std::fs::write("data.json", json).unwrap();
}
fn load_documents() -> Vec<Document> {
    if let Ok(json) = std::fs::read_to_string("data.json") {
        serde_json::from_str(&json).unwrap_or(Vec::new())
    } else {
        Vec::new()
    }
}
fn main() -> eframe::Result {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Markdown Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(App::new()))),
    )
}
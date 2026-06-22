use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("面板系统", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { current_page: String, content: String, files: Vec<String> }

impl Default for MyApp {
    fn default() -> Self {
        Self { current_page: "首页".into(), content: String::new(),
            files: vec!["main.rs".into(), "app.rs".into(), "utils.rs".into()] }
    }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 顶部工具栏
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("编辑器");
                if ui.button("新建").clicked() { self.content.clear(); }
                if ui.button("保存").clicked() { /* save */ }
            });
        });

        // 左侧文件树
        egui::Panel::left("file_tree").resizable(true).default_size(150.0).show_inside(ui, |ui| {
            ui.heading("文件");
            for file in &self.files {
                let _ = ui.selectable_label(self.current_page == *file, file);
            }
        });

        // 底部状态栏
        egui::Panel::bottom("status").show_inside(ui, |ui| {
            ui.label(format!("{} | UTF-8 | 就绪", self.current_page));
        });

        // 中央编辑区
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading(format!("当前页面: {}", self.current_page));
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.content).desired_width(f32::INFINITY).desired_rows(15));
            });
        });
    }
}

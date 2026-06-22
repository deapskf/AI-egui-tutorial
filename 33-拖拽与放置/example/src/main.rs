use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("拖拽与放置", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { dropped_files: Vec<String>, items: Vec<String> }

impl Default for MyApp {
    fn default() -> Self { Self { dropped_files: vec![], items: (0..6).map(|i| format!("项目 {}", i)).collect() } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if !ctx.input(|i| i.raw.dropped_files.is_empty()) {
            let files = ctx.input(|i| i.raw.dropped_files.clone());
            for f in &files {
                if let Some(path) = &f.path {
                    self.dropped_files.push(path.display().to_string());
                }
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("拖拽与放置演示");
        ui.label("从文件管理器拖文件到此处");

        if !self.dropped_files.is_empty() {
            ui.separator();
            ui.heading("已拖入的文件:");
            for f in &self.dropped_files { ui.label(f); }
            if ui.button("清除列表").clicked() { self.dropped_files.clear(); }
        }

        ui.separator();
        ui.label("文件悬停中...");

        ui.separator();
        ui.heading("可排序列表:");
        for (_i, item) in self.items.clone().iter().enumerate() {
            let resp = ui.label(item);
            if resp.drag_started() { /* 开始拖拽 */ }
            if resp.dragged() { /* 拖拽中 */ }
            if resp.drag_stopped() { /* 拖拽结束 */ }
        }
    }
}

use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("滚动区域", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { messages: Vec<String>, auto_scroll: bool }

impl Default for MyApp {
    fn default() -> Self { Self { messages: (0..5).map(|i| format!("消息 #{}", i)).collect(), auto_scroll: true } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("聊天窗口（自动滚动）");
        ui.checkbox(&mut self.auto_scroll, "自动滚到底部");

        if ui.button("添加消息").clicked() {
            self.messages.push(format!("消息 #{}", self.messages.len()));
        }

        egui::ScrollArea::vertical()
            .id_salt("chat")
            .stick_to_bottom(self.auto_scroll)
            .max_height(200.0)
            .show(ui, |ui| {
                for msg in &self.messages { ui.label(msg); }
            });

        ui.separator();
        ui.label(format!("共 {} 条消息", self.messages.len()));
    }
}

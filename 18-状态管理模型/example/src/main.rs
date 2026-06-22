use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("状态管理", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { first_name: String, last_name: String }

impl Default for MyApp { fn default() -> Self { Self { first_name: String::new(), last_name: String::new() } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("ID 冲突演示");

        // ❌ 危险：两个空字符串的 TextEdit 可能有相同 Id
        ui.label("错误: 两个同名输入框（如均为空字符串）可能导致状态冲突");
        ui.push_id("first", |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.first_name).hint_text("名"));
        });
        ui.push_id("last", |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.last_name).hint_text("姓"));
        });

        ui.separator();
        ui.label(format!("全名: {} {}", self.first_name, self.last_name));

        // 动态列表（更典型的 Id 冲突场景）
        ui.separator();
        ui.heading("动态列表（push_id 演示）");
        for i in 0..3 {
            ui.push_id(i, |ui| {
                ui.label(format!("项目 #{}", i));
            });
        }
    }
}

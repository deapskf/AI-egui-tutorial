use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("输入框示例", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { username: String, password: String, bio: String }

impl Default for MyApp {
    fn default() -> Self { Self { username: String::new(), password: String::new(), bio: String::new() } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("用户信息表单");

        ui.label("用户名:");
        ui.add(egui::TextEdit::singleline(&mut self.username).hint_text("请输入用户名").desired_width(200.0));

        ui.label("密码:");
        ui.add(egui::TextEdit::singleline(&mut self.password).password(true).hint_text("请输入密码").desired_width(200.0));

        ui.label("简介:");
        ui.add(egui::TextEdit::multiline(&mut self.bio).hint_text("写点什么...").desired_rows(4).desired_width(f32::INFINITY));

        ui.label(format!("简介: {} 字符", self.bio.len()));

        ui.separator();
        let preview = format!("{} - {}", self.username, if self.password.is_empty() { "(空)" } else { "***" });
        ui.label("预览（只读）:");
        ui.add(egui::TextEdit::singleline(&mut preview.clone()).interactive(false));
    }
}

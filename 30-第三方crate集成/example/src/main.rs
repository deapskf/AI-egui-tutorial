use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("第三方crate集成", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { time_str: String, log_output: String }

impl Default for MyApp { fn default() -> Self { Self { time_str: String::new(), log_output: String::new() } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 使用 chrono 更新时间
        self.time_str = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("第三方 crate 集成演示");

        ui.label(format!("当前时间: {}", self.time_str));
        ui.separator();

        // rfd 文件对话框
        if ui.button("打开文件对话框").clicked() {
            if let Some(path) = rfd::FileDialog::new().pick_file() {
                self.log_output = format!("选择了: {}", path.display());
            }
        }

        if ui.button("保存文件对话框").clicked() {
            if let Some(path) = rfd::FileDialog::new().save_file() {
                self.log_output = format!("保存到: {}", path.display());
            }
        }

        ui.separator();
        ui.label(format!("日志: {}", self.log_output));

        ui.separator();
        ui.label("提示: 日志输出可通过 RUST_LOG=debug cargo run 查看");
        log::info!("UI 已渲染");
    }
}

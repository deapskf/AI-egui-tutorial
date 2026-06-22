use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("浮动窗口", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { show_settings: bool, show_about: bool }

impl Default for MyApp { fn default() -> Self { Self { show_settings: false, show_about: false } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.horizontal(|ui| {
            if ui.button("打开设置").clicked() { self.show_settings = true; }
        });

        // 设置窗口
        let mut show_about = self.show_about;
        egui::Window::new("设置")
            .open(&mut self.show_settings)
            .resizable(true)
            .default_size([300.0, 200.0])
            .show(ui, |ui| {
                ui.label("这里是设置面板");
                ui.checkbox(&mut show_about, "显示关于窗口");
            });
        self.show_about = show_about;

        // 关于窗口
        egui::Window::new("关于")
            .open(&mut self.show_about)
            .collapsible(true)
            .default_size([250.0, 150.0])
            .show(ui, |ui| {
                ui.label("EGUI 全面教程 - ch16");
                if ui.button("关闭").clicked() { /* handled by open flag */ }
            });

        if ui.button("显示关于").clicked() { self.show_about = true; }
    }
}

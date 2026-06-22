use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("复选框与开关", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { enable_audio: bool, wifi: bool, perms: [bool; 3] }

impl Default for MyApp {
    fn default() -> Self { Self { enable_audio: true, wifi: false, perms: [false; 3] } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("设置面板");

        ui.checkbox(&mut self.enable_audio, "启用音频");

        // 开关风格的 toggle
        ui.horizontal(|ui| {
            ui.label("Wi-Fi:");
            if ui.selectable_label(self.wifi, if self.wifi { "● 开" } else { "○ 关" }).clicked() {
                self.wifi = !self.wifi;
            }
        });
        ui.label(format!("Wi-Fi 状态: {}", if self.wifi { "开" } else { "关" }));

        ui.separator();
        ui.heading("权限设置");

        let mut all_check = self.perms.iter().all(|&p| p);
        if ui.checkbox(&mut all_check, "全选").changed() {
            let new = !all_check;
            for p in &mut self.perms { *p = new; }
        }

        for (i, p) in self.perms.iter_mut().enumerate() {
            ui.checkbox(p, format!("权限 {}", i + 1));
        }

        ui.separator();
        ui.add_enabled(false, egui::Button::new("仅在全部选中时可用"));
    }
}

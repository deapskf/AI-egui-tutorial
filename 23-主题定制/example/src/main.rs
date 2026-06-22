use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("主题定制", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { dark_mode: bool }

impl Default for MyApp { fn default() -> Self { Self { dark_mode: true } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("主题切换");

        ui.horizontal(|ui| {
            if ui.button("🌙 暗色").clicked() {
                self.dark_mode = true;
                ui.ctx().global_style_mut(|s| s.visuals = egui::Visuals::dark());
            }
            if ui.button("☀ 亮色").clicked() {
                self.dark_mode = false;
                ui.ctx().global_style_mut(|s| s.visuals = egui::Visuals::light());
            }
        });

        ui.label(format!("当前: {}", if self.dark_mode { "暗色" } else { "亮色" }));
        ui.separator();

        let _ = ui.button("控件预览");
        ui.checkbox(&mut self.dark_mode, "选项");
        ui.add(egui::Slider::new(&mut 50i32, 0..=100).text("滑动条"));
    }
}

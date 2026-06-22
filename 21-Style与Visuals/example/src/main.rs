use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("样式体系", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { spacing: f32, dummy: bool }

impl Default for MyApp { fn default() -> Self { Self { spacing: 6.0, dummy: false } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("样式参数调节");

        ui.add(egui::Slider::new(&mut self.spacing, 0.0..=20.0).text("控件间距"));
        ui.separator();

        // 应用局部间距
        ui.style_mut().spacing.item_spacing = egui::vec2(self.spacing, self.spacing);

        ui.label("预览区:");
        let _ = ui.button("测试按钮");
        ui.checkbox(&mut self.dummy, "复选框");

        ui.separator();
        // 覆盖文字颜色
        ui.style_mut().visuals.override_text_color = Some(egui::Color32::RED);
        ui.label("红色文字");
        ui.style_mut().visuals.override_text_color = None;
        ui.label("正常颜色");
    }
}

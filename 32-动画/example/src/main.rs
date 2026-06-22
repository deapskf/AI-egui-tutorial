use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("动画", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { panel_open: bool, target_val: f32 }

impl Default for MyApp { fn default() -> Self { Self { panel_open: false, target_val: 0.0 } } }

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint_after(std::time::Duration::from_millis(30));
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("动画演示");

        // animate_bool: 面板开关动画
        let anim = ui.ctx().animate_bool(egui::Id::new("panel"), self.panel_open);
        if ui.checkbox(&mut self.panel_open, "展开面板").changed() || anim > 0.0 && anim < 1.0 {
            // 动画进行中，持续重绘
        }

        ui.add_enabled(anim > 0.1, egui::Label::new(
            egui::RichText::new(format!("面板内容 (透明度: {:.2})", anim)).color(
                egui::Color32::from_rgba_unmultiplied(255, 255, 255, (anim * 255.0) as u8)
            )
        ));

        ui.separator();

        // animate_value_with_time: 数值动画
        ui.add(egui::Slider::new(&mut self.target_val, 0.0..=100.0).text("目标值"));
        let animated = ui.ctx().animate_value_with_time(
            egui::Id::new("val_anim"), self.target_val, 0.3
        );
        ui.label(format!("平滑值: {:.1}", animated));
    }
}

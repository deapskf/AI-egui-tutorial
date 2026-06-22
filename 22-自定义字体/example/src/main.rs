use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("自定义字体", eframe::NativeOptions::default(),
        Box::new(|cc| {
            // 配置字体（第22章示例 - 基础骨架）
            let fonts = egui::FontDefinitions::default();
            // 实际应用中加载中文字体:
            // fonts.font_data.insert("my_font".into(), egui::FontData::from_owned(
            //     std::fs::read("path/to/font.ttf").unwrap_or_default()
            // ));
            // fonts.families.entry(egui::FontFamily::Proportional).or_default().insert(0, "my_font".into());
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(MyApp::default()))
        }),
    )
}

struct MyApp;

impl Default for MyApp { fn default() -> Self { Self } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("字体配置演示");
        ui.label("This text uses the proportional font.");
        ui.label(egui::RichText::new("等宽字体文字").font(egui::FontId::monospace(14.0)));
        ui.label(egui::RichText::new("大字").size(24.0));
        ui.label(egui::RichText::new("小字").size(10.0));
        ui.label(
            egui::RichText::new("中文测试 - Font Test")
                .size(16.0)
                .strong()
        );
    }
}

use eframe::egui;
use eframe::egui::{Color32, RichText};

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "文本与标签",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp;

impl Default for MyApp {
    fn default() -> Self {
        Self
    }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("egui 文本与标签演示");

        // 1. 普通标签
        ui.separator();
        ui.label("1. 普通标签 (Label):");
        ui.label("This is a plain label.");
        ui.label("Line 1\nLine 2\nLine 3 (multi-line)");

        // 2. 富文本颜色
        ui.separator();
        ui.label("2. RichText 颜色:");
        ui.label(RichText::new("Red text").color(Color32::RED));
        ui.label(RichText::new("Blue text").color(Color32::BLUE));
        ui.label(RichText::new("Green text").color(Color32::GREEN));
        ui.label(RichText::new("Gray text").color(Color32::GRAY));
        ui.label(
            RichText::new("Custom #FFAA00")
                .color(Color32::from_rgb(255, 170, 0)),
        );

        // 3. 富文本样式
        ui.separator();
        ui.label("3. RichText 样式:");
        ui.label(RichText::new("Bold (strong)").strong());
        ui.label(RichText::new("Italic (italics)").italics());
        ui.label(RichText::new("Underlined").underline());
        ui.label(RichText::new("Strikethrough").strikethrough());
        ui.label(RichText::new("Code style text").code());

        // 4. 字号对比
        ui.separator();
        ui.label("4. 字号对比:");
        for &size in &[8.0, 12.0, 16.0, 20.0, 28.0, 36.0] {
            ui.label(RichText::new(format!("{}px text", size as i32)).size(size));
        }

        // 5. 样式组合
        ui.separator();
        ui.label("5. 样式组合:");
        ui.label(
            RichText::new("Bold + Italic + Underline + Red")
                .color(Color32::RED)
                .strong()
                .italics()
                .underline(),
        );
        ui.label(
            RichText::new("Highlighted text")
                .background_color(Color32::from_rgb(255, 255, 100))
                .strong(),
        );

        // 6. Heading 标题
        ui.separator();
        ui.label("6. Heading vs Label:");
        ui.heading("This is a heading");
        ui.label("This is a normal label for comparison");

        // 7. 混排
        ui.separator();
        ui.label("7. 混排 (horizontal):");
        ui.horizontal(|ui| {
            ui.label(RichText::new("CPU: ").strong());
            ui.label(RichText::new("45%").color(Color32::GREEN));
            ui.label(" | ");
            ui.label(RichText::new("Memory: ").strong());
            ui.label(RichText::new("2.1 GB").color(Color32::YELLOW));
        });
    }
}

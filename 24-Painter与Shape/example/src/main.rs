use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("自定义绘制", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp;

impl Default for MyApp { fn default() -> Self { Self } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Painter 与 Shape API 演示");

        let (resp, painter) = ui.allocate_painter(egui::vec2(300.0, 150.0), egui::Sense::hover());
        let r = resp.rect;

        // 绘制矩形 — 通过 Shape 添加到 painter
        let shape = egui::Shape::rect_filled(
            egui::Rect::from_min_size(r.min + egui::vec2(10.0, 10.0), egui::vec2(80.0, 40.0)),
            4.0,  // 圆角半径
            egui::Color32::RED,
        );
        painter.add(shape);

        // 绘制文字
        painter.text(
            r.min + egui::vec2(10.0, 70.0),
            egui::Align2::LEFT_TOP,
            "由 Painter 直接绘制的文本",
            egui::FontId::proportional(14.0),
            egui::Color32::WHITE,
        );

        // 绘制线段
        let line_shape = egui::Shape::line_segment(
            [r.min + egui::vec2(10.0, 110.0), r.min + egui::vec2(200.0, 110.0)],
            egui::Stroke::new(2.0, egui::Color32::GREEN),
        );
        painter.add(line_shape);

        if resp.hovered() { ui.label("鼠标在绘制区域中"); }
    }
}

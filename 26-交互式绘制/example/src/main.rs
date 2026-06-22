use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("简易绘图板", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { strokes: Vec<Vec<egui::Pos2>>, current: Vec<egui::Pos2>, color: egui::Color32 }

impl Default for MyApp { fn default() -> Self { Self { strokes: vec![], current: vec![], color: egui::Color32::WHITE } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("简易绘图板");
        ui.horizontal(|ui| {
            if ui.button("清除").clicked() { self.strokes.clear(); }
            ui.label("颜色:");
            for &c in &[egui::Color32::WHITE, egui::Color32::RED, egui::Color32::GREEN, egui::Color32::BLUE, egui::Color32::YELLOW] {
                if ui.add(egui::Button::new("●").fill(c).small()).clicked() { self.color = c; }
            }
        });

        let (resp, painter) = ui.allocate_painter(ui.available_size(), egui::Sense::drag());

        // 绘制历史笔画
        for stroke in &self.strokes {
            if stroke.len() >= 2 {
                for w in stroke.windows(2) {
                    painter.line_segment([w[0], w[1]], egui::Stroke::new(2.0, self.color));
                }
            }
        }

        // 拖拽绘制
        if resp.dragged_by(egui::PointerButton::Primary) {
            if let Some(pos) = resp.hover_pos() {
                self.current.push(pos);
                if self.current.len() >= 2 {
                    for w in self.current.windows(2) {
                        painter.line_segment([w[0], w[1]], egui::Stroke::new(2.0, self.color));
                    }
                }
            }
        } else if !self.current.is_empty() {
            self.strokes.push(std::mem::take(&mut self.current));
        }
    }
}

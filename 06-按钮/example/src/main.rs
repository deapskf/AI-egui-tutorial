use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native(
        "按钮示例",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp { clicks: i32, log: Vec<String> }

impl Default for MyApp {
    fn default() -> Self { Self { clicks: 0, log: Vec::new() } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("按钮示例");

        // 基本按钮
        if ui.button("点击我").clicked() {
            self.clicks += 1;
            self.log.push("左键点击".into());
        }

        // 右键点击
        let resp = ui.button("右键点击测试");
        if resp.clicked_by(egui::PointerButton::Secondary) {
            self.log.push("右键点击".into());
        }

        // 双击
        if resp.double_clicked() {
            self.log.push("双击".into());
        }

        ui.label(format!("按钮被点击了 {} 次", self.clicks));
        ui.separator();

        // 按钮样式
        ui.horizontal(|ui| {
            let _ = ui.button("默认按钮");
            ui.add(egui::Button::new("轮廓按钮").fill(egui::Color32::TRANSPARENT));
            ui.add(egui::Button::new("小按钮").small());
            ui.add_enabled(false, egui::Button::new("禁用按钮"));
        });

        ui.separator();

        // 链接文本
        ui.hyperlink_to("egui 官网", "https://github.com/emilk/egui");

        // 事件日志
        ui.separator();
        ui.heading("事件日志:");
        egui::ScrollArea::vertical().max_height(150.0).show(ui, |ui| {
            for entry in &self.log {
                ui.label(entry);
            }
        });
    }
}

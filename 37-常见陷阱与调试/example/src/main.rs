use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("陷阱与调试", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { name_a: String, name_b: String, frame: u64 }

impl Default for MyApp { fn default() -> Self { Self { name_a: String::new(), name_b: String::new(), frame: 0 } } }

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame = ctx.input(|i| i.time as u64);
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("常见陷阱演示");

        // ✅ 修复 ID 冲突（也可用 ScrollArea::id_salt、Widget::id_source 等方案）
        ui.label("两个输入框——使用 push_id 避免冲突:");
        ui.push_id("a", |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.name_a).hint_text("输入 A"));
        });
        ui.push_id("b", |ui| {
            ui.add(egui::TextEdit::singleline(&mut self.name_b).hint_text("输入 B"));
        });

        ui.label(format!("A: {}, B: {}", self.name_a, self.name_b));

        ui.separator();

        // 调试叠加文本
        ui.ctx().debug_text(format!("帧: {}, A={}, B={}", self.frame, self.name_a, self.name_b));

        ui.label("按下 Ctrl+Shift+D 查看 egui 内置调试面板");
    }
}

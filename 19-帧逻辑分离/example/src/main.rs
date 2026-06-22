use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("帧逻辑分离", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { count: i32, save_requested: bool, saved_count: i32 }

impl Default for MyApp { fn default() -> Self { Self { count: 0, save_requested: false, saved_count: 0 } } }

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 处理快捷键
        ctx.input(|i| {
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.save_requested = true;
            }
        });

        // 消费 ui() 设置的交互标志
        if self.save_requested {
            self.saved_count = self.count;
            self.save_requested = false;
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("logic / ui 分离演示");

        if ui.button("+1").clicked() { self.count += 1; }
        if ui.button("-1").clicked() { self.count -= 1; }

        ui.label(format!("当前计数: {}", self.count));
        ui.label(format!("上次保存: {}", self.saved_count));

        if ui.button("保存").clicked() {
            self.save_requested = true; // 设置标志 → 在 logic 中消费
        }

        ui.separator();
        ui.label("提示: Ctrl+S 也能触发保存（在 logic 中检测）");
    }
}

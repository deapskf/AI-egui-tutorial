use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("输入处理深入", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { text: String, ctrl_s_pressed: bool }

impl Default for MyApp { fn default() -> Self { Self { text: String::new(), ctrl_s_pressed: false } } }

impl eframe::App for MyApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            if i.modifiers.ctrl && i.key_pressed(egui::Key::S) {
                self.ctrl_s_pressed = true;
            }
        });
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("输入处理演示");

        let mouse_pos = ui.ctx().input(|i| i.pointer.latest_pos());
        ui.label(format!("鼠标位置: {:?}", mouse_pos));
        ui.label(format!("Ctrl+S: {}", if self.ctrl_s_pressed { "已按下" } else { "未按下" }));
        if self.ctrl_s_pressed { self.ctrl_s_pressed = false; }

        ui.separator();
        ui.label("文本输入（支持 IME 中文输入）:");
        ui.add(egui::TextEdit::singleline(&mut self.text).hint_text("在此输入中文...").desired_width(250.0));
    }
}

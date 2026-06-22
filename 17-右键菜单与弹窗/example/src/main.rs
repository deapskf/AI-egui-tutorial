use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("菜单与弹窗", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { show_confirm: bool, text: String }

impl Default for MyApp { fn default() -> Self { Self { show_confirm: false, text: String::new() } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 菜单栏（新版 API）
        egui::MenuBar::new().ui(ui, |ui| {
            ui.menu_button("文件", |ui| {
                if ui.button("新建").clicked() { ui.close(); self.text.clear(); }
                if ui.button("退出").clicked() { ui.close(); std::process::exit(0); }
            });
            ui.menu_button("帮助", |ui| {
                if ui.button("关于").clicked() { ui.close(); }
            });
        });

        // 右键菜单
        ui.separator();
        ui.label("右键点击下方文本：");
        let resp = ui.label("右键点击这里");
        resp.context_menu(|ui| {
            if ui.button("复制").clicked() { ui.close(); }
            if ui.button("粘贴").clicked() { ui.close(); }
            ui.separator();
            if ui.button("删除").clicked() { ui.close(); self.text.clear(); }
        });

        // 输入区
        ui.add(egui::TextEdit::singleline(&mut self.text).hint_text("输入文本...").desired_width(200.0));

        // 确认对话框
        if ui.button("删除所有内容").clicked() { self.show_confirm = true; }

        if self.show_confirm {
            egui::Window::new("确认")
                .collapsible(false).resizable(false)
                .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
                .show(ui, |ui| {
                    ui.label("确定要删除吗？");
                    ui.horizontal(|ui| {
                        if ui.button("确定").clicked() { self.text.clear(); self.show_confirm = false; }
                        if ui.button("取消").clicked() { self.show_confirm = false; }
                    });
                });
        }
    }
}

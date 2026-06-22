use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("多窗口", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { show_tool: bool }

impl Default for MyApp { fn default() -> Self { Self { show_tool: false } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("多窗口演示（Viewport）");

        // 在实际项目中，创建独立窗口的代码：
        // ui.ctx().show_viewport_deferred(
        //     egui::ViewportId::from_hash_of(&"tool_window"),
        //     egui::ViewportBuilder::default()
        //         .with_title("工具窗口")
        //         .with_inner_size([300.0, 250.0]),
        //     |ctx, _class| {
        //         egui::CentralPanel::default().show(ctx, |ui| {
        //             ui.label("独立工具窗口");
        //         });
        //     },
        // );

        if ui.button("打开工具面板").clicked() {
            self.show_tool = true;
        }

        let mut tool_open = self.show_tool;
        if tool_open {
            egui::Window::new("工具")
                .open(&mut tool_open)
                .resizable(true)
                .default_size([300.0, 250.0])
                .show(ui, |ui| {
                    ui.label("这是一个嵌入式工具窗口。");
                    ui.label("在支持多视口的平台上，可以变成独立窗口。");
                    if ui.button("关闭").clicked() {
                        // 由 open 标志处理
                    }
                });
        }
        if !tool_open { self.show_tool = false; }

        ui.separator();
        ui.label("eframe 多窗口相关 API:");
        ui.label("  • show_viewport_deferred — 创建独立视口（实验性）");
        ui.label("  • ViewportBuilder — 配置窗口大小/位置/标题");
        ui.label("  • ViewportId::from_hash_of — 生成视口 ID");
        ui.label("  • egui::Window — 单窗口内浮动（兼容所有平台）");
    }
}

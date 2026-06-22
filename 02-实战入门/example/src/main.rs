use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "我的计数器",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

/// 应用状态：存储所有需要在帧之间保留的数据
struct MyApp {
    count: i32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { count: 0 }
    }
}

impl eframe::App for MyApp {
    /// 帧逻辑：数据处理、快捷键检测、后台任务
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 计数器逻辑足够简单，这里不需要额外处理
    }

    /// UI 渲染：每帧描述界面
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("计数器示例");

        ui.separator();

        // 显示当前计数
        ui.label(format!("点击次数: {}", self.count));

        // 按钮：检测点击并修改状态
        if ui.button("+1").clicked() {
            self.count += 1;
        }

        // 添加一个递减按钮
        if ui.button("-1").clicked() {
            self.count -= 1;
        }

        // 添加一个重置按钮
        if ui.button("重置").clicked() {
            self.count = 0;
        }
    }
}

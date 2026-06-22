use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "帧循环验证",
        native_options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

/// 应用状态
struct MyApp {
    count: i32,
    frame_count: u64,      // 帧序号：证明"无输入不重绘"
    last_input_time: f64,   // egui 内部时钟（秒）
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            count: 0,
            frame_count: 0,
            last_input_time: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    /// 帧逻辑：跟踪帧计数和时间
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.frame_count += 1;
        self.last_input_time = ctx.input(|i| i.time);
    }

    /// UI 渲染：显示帧信息和计数器
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // 从 Context 获取信息（在 ui 被借用之前）
        let (width, height) = {
            let ctx = ui.ctx();
            let r = ctx.content_rect();
            (r.width(), r.height())
        };

        ui.heading("帧循环验证");
        ui.separator();

        // --- 交互区 ---
        ui.horizontal(|ui| {
            if ui.button("+1").clicked() {
                self.count += 1;
            }
            if ui.button("-1").clicked() {
                self.count -= 1;
            }
            if ui.button("重置").clicked() {
                self.count = 0;
            }
        });
        ui.label(format!("计数器当前值: {}", self.count));
        ui.separator();

        // --- 帧信息区 ---
        ui.label(format!("帧序号: {}", self.frame_count));
        ui.label(format!("egui 时钟: {:.3} 秒", self.last_input_time));
        ui.label(format!("窗口尺寸: {:.0} x {:.0}", width, height));
        ui.separator();

        // --- 说明区 ---
        ui.label("提示：仅在移动鼠标或点击按钮时帧序号才增长");
        ui.label("这证明 egui 默认只在有输入事件时才重绘");
    }
}

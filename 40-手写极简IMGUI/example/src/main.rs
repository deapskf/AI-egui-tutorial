// 40 · 手写极简 IM GUI（概念演示）
// 完整实现需 winit + wgpu（约300行），此处展示核心逻辑骨架

use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("极简IMGUI概念", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { count: i32 }

impl Default for MyApp { fn default() -> Self { Self { count: 0 } } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Minimal IMGUI Concept");

        ui.label("core loop: | Ui state + user code → drawing commands → GPU → repeat");
        ui.label(format!("Think: count = {}", self.count));

        // 模拟手写版的核心循环
        // 1. fn button(ui, label) → rect + clicked
        // 2. fn label(ui, text) → rect
        // 3. fn paint(draw_commands, renderer)
        // 4. event_loop.run(move |event| { ... })

        if ui.button("+1").clicked() { self.count += 1; }
    }
}

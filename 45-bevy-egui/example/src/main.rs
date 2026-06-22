// 45 · bevy_egui 快速概览
// bevy_egui 需要配合 Bevy 游戏引擎使用（见教程正文）
// 此处展示核心集成方式

use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("bevy_egui概览", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp;

impl Default for MyApp { fn default() -> Self { Self } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("bevy_egui 快速概览");

        ui.label("bevy_egui 将 egui 集成到 Bevy ECS 游戏引擎:");
        ui.label("1. App::new().add_plugins(EguiPlugin) — 添加 egui 插件");
        ui.label("2. 在 Bevy System 中使用 egui::Context");
        ui.label("3. egui 渲染层叠加在 Bevy 3D 场景之上");
        ui.label("4. 适用于: 游戏内 UI、背包、对话框、编辑器");

        ui.separator();
        ui.label("// Bevy 中的代码示例:");
        ui.label("fn ui_system(mut egui_ctx: Query<&mut EguiContext>) {");
        ui.label("    egui::Window::new(\"设置\").show(ctx, |ui| {");
        ui.label("        ui.button(\"游戏设置\");");
        ui.label("    });");
        ui.label("}");

        ui.separator();
        ui.label("何时选择 bevy_egui:");
        ui.label("  ✓ 需要 3D/物理/音频 → bevy_egui");
        ui.label("  ✓ 纯桌面工具/编辑器 → eframe");
    }
}

use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("egui_plot 图表", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp;

impl Default for MyApp { fn default() -> Self { Self } }

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("egui_plot 图表演示");

        // egui_plot 是 egui 官方的 2D 图表库
        // Cargo.toml 中添加: egui_plot = "0.34"
        // 完整 API 参考: https://docs.rs/egui_plot/

        ui.label("基础用法:");
        ui.label("  use egui_plot::{Plot, Line, PlotPoints, Legend};");
        ui.label("  let points = PlotPoints::new(vec![...]);");
        ui.label("  let line = Line::new(points).name(\"series\");");
        ui.label("  Plot::new(\"id\").show(ui, |plot_ui| { plot_ui.line(line); });");

        ui.separator();
        ui.label("可用图表类型:");
        ui.label("  • Line  — 折线图");
        ui.label("  • Points — 散点图");
        ui.label("  • Bar / BarChart — 柱状图");
        ui.label("  • Arrows — 向量场");
        ui.label("  • Polygon — 填充区域");
        ui.label("  • Text — 图表标注");

        ui.separator();
        ui.label("注意: egui_plot 的 API 随版本演进，请以 docs.rs 为准");
        ui.label("参见官方示例: github.com/emilk/egui_plot");
    }
}

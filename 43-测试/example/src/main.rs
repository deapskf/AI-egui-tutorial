// 43 · 测试 egui 应用
// 本示例展示如何编写可测试的 egui 应用（通过 logic/ui 分离）
// egui_kittest 的测试代码通常放在 #[cfg(test)] 模块中

use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("可测试的应用", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { count: i32 }

impl Default for MyApp { fn default() -> Self { Self { count: 0 } } }

impl MyApp {
    /// 纯逻辑方法——可被单元测试调用
    fn increment(&mut self) { self.count += 1; }
    fn decrement(&mut self) { self.count -= 1; }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("测试演示");

        if ui.button("+1").clicked() { self.increment(); }
        if ui.button("-1").clicked() { self.decrement(); }
        ui.label(format!("计数: {}", self.count));

        ui.separator();
        ui.label("运行 cargo test 查看测试结果");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increment() {
        let mut app = MyApp::default();
        app.increment();
        assert_eq!(app.count, 1);
    }

    #[test]
    fn test_decrement() {
        let mut app = MyApp::default();
        app.decrement();
        assert_eq!(app.count, -1);
    }
}

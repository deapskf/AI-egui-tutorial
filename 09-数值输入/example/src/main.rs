use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("数值输入", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { volume: i32, opacity: f32, speed: f64, frequency: f32 }

impl Default for MyApp {
    fn default() -> Self { Self { volume: 50, opacity: 0.8, speed: 1.0, frequency: 440.0 } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("数值输入控件");

        ui.label("音量:");
        ui.add(egui::Slider::new(&mut self.volume, 0..=100));
        ui.label("不透明度:");
        ui.add(egui::Slider::new(&mut self.opacity, 0.0..=1.0).step_by(0.05));
        ui.label("频率 (Hz):");
        ui.add(egui::Slider::new(&mut self.frequency, 20.0..=20000.0).logarithmic(true));

        ui.separator();
        ui.label("速度:");
        ui.add(egui::DragValue::new(&mut self.speed).speed(0.1).suffix("x"));
        ui.label("不透明度:");
        ui.add(egui::DragValue::new(&mut self.opacity).speed(0.01).fixed_decimals(3).prefix("α="));

        ui.separator();
        ui.horizontal(|ui| {
            ui.label("亮度:");
            ui.add(egui::Slider::new(&mut self.volume, 0..=100));
            ui.add(egui::DragValue::new(&mut self.volume).suffix("%"));
        });
    }
}

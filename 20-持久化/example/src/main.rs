use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("持久化", eframe::NativeOptions::default(),
        Box::new(|cc| {
            let mut app = MyApp::default();
            if let Some(storage) = cc.storage {
                if let Some(json) = storage.get_string("app_state") {
                    if let Ok(state) = serde_json::from_str::<AppState>(&json) {
                        app.count = state.count;
                    }
                }
            }
            Ok(Box::new(app))
        }),
    )
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AppState { count: i32 }

struct MyApp { count: i32 }

impl Default for MyApp { fn default() -> Self { Self { count: 0 } } }

impl eframe::App for MyApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        let state = AppState { count: self.count };
        if let Ok(json) = serde_json::to_string(&state) {
            storage.set_string("app_state", json);
        }
    }

    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("持久化演示");
        ui.label("关闭窗口后重新打开，计数会被保留");

        if ui.button("+1").clicked() { self.count += 1; }
        ui.label(format!("计数: {}", self.count));

        ui.separator();
        ui.label("提示：eframe 约每 30 秒自动调用 save()");
    }
}

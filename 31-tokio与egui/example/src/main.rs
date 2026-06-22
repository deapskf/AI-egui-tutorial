use eframe::egui;
use std::sync::mpsc;

fn main() -> eframe::Result<()> {
    eframe::run_native("tokio与egui", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::new()))))
}

#[allow(dead_code)]
enum AsyncResult { Data(String), Error(String) }

struct MyApp { rx: mpsc::Receiver<AsyncResult>, tx: mpsc::Sender<String>, response: String, loading: bool, url: String }

impl MyApp {
    fn new() -> Self {
        let (_tx_resp, rx) = mpsc::channel();
        let (tx_req, _rx_req) = mpsc::channel::<String>();
        Self { rx, tx: tx_req, response: String::new(), loading: false, url: String::new() }
    }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Ok(result) = self.rx.try_recv() {
            match result {
                AsyncResult::Data(text) => { self.response = text; self.loading = false; }
                AsyncResult::Error(e) => { self.response = format!("错误: {}", e); self.loading = false; }
            }
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("异步请求演示");
        ui.horizontal(|ui| {
            ui.add(egui::TextEdit::singleline(&mut self.url).hint_text("输入URL").desired_width(250.0));
            if ui.button("发送").clicked() && !self.loading {
                self.loading = true;
                self.response.clear();
                let _tx = self.tx.clone();
                let _url = self.url.clone();
                std::thread::spawn(move || { /* 用 _tx.send(AsyncResult::Data(...)) 发送结果 */ });
            }
        });

        if self.loading { ui.spinner(); }
        if !self.response.is_empty() { ui.label(&self.response); }
        ui.label("提示: 此示例仅展示 mpsc 通道模式骨架");
    }
}

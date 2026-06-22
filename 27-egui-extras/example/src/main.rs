use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("egui_extras", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct MyApp { items: Vec<(String, i32)> }

impl Default for MyApp {
    fn default() -> Self { Self { items: vec![("苹果".into(), 5), ("香蕉".into(), 3), ("橙子".into(), 7)] } }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("egui_extras::Table 演示");

        use egui_extras::{TableBuilder, Column};

        let mut to_remove = None;
        TableBuilder::new(ui)
            .column(Column::auto().resizable(true))
            .column(Column::auto())
            .column(Column::remainder())
            .header(20.0, |mut h| {
                h.col(|ui| { ui.strong("名称"); });
                h.col(|ui| { ui.strong("数量"); });
                h.col(|ui| { ui.strong("操作"); });
            })
            .body(|mut b| {
                for (i, (name, qty)) in self.items.iter().enumerate() {
                    b.row(18.0, |mut r| {
                        r.col(|ui| { ui.label(name); });
                        r.col(|ui| { ui.label(format!("{}", qty)); });
                        r.col(|ui| { if ui.button("删除").clicked() { to_remove = Some(i); } });
                    });
                }
            });

        if let Some(i) = to_remove { self.items.remove(i); }
    }
}

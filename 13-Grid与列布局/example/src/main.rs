use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("Grid与列布局", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

struct Item { name: String, quantity: u32, price: f64 }

struct MyApp { items: Vec<Item> }

impl Default for MyApp {
    fn default() -> Self {
        Self { items: vec![
            Item { name: "苹果".into(), quantity: 3, price: 5.5 },
            Item { name: "香蕉".into(), quantity: 5, price: 3.0 },
            Item { name: "橙子".into(), quantity: 2, price: 7.0 },
        ]}
    }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("Grid 表格");

        egui::Grid::new("my_grid").striped(true).min_col_width(60.0).show(ui, |ui| {
            ui.strong("名称"); ui.strong("数量"); ui.strong("单价"); ui.strong("小计");
            ui.end_row();
            let mut total = 0.0;
            for item in &self.items {
                ui.label(&item.name);
                ui.label(format!("{}", item.quantity));
                ui.label(format!("¥{:.2}", item.price));
                let sub = item.quantity as f64 * item.price;
                ui.label(format!("¥{:.2}", sub));
                ui.end_row();
                total += sub;
            }
            ui.separator(); ui.separator(); ui.separator(); ui.separator();
            ui.end_row();
            ui.strong("合计:"); ui.label(""); ui.label("");
            ui.strong(format!("¥{:.2}", total));
            ui.end_row();
        });
    }
}

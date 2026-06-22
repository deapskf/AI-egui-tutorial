use eframe::egui;

fn main() -> eframe::Result<()> {
    eframe::run_native("下拉框与列表选择", eframe::NativeOptions::default(), Box::new(|_cc| Ok(Box::new(MyApp::default()))))
}

#[derive(PartialEq, Clone)]
enum Theme { Dark, Light, Auto }

struct MyApp { language: String, theme: Theme, items: Vec<String>, selected: Vec<usize> }

impl Default for MyApp {
    fn default() -> Self {
        Self {
            language: "Rust".into(), theme: Theme::Dark,
            items: vec!["选项A".into(), "选项B".into(), "选项C".into()],
            selected: vec![],
        }
    }
}

impl eframe::App for MyApp {
    fn logic(&mut self, _ctx: &egui::Context, _frame: &mut eframe::Frame) {}
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.heading("选择控件");

        // ComboBox
        egui::ComboBox::from_label("编程语言")
            .selected_text(&self.language)
            .show_ui(ui, |ui| {
                for lang in &["Rust", "Python", "JavaScript", "Go"] {
                    ui.selectable_value(&mut self.language, lang.to_string(), *lang);
                }
            });

        // RadioButton
        ui.separator();
        ui.label("主题:");
        ui.radio_value(&mut self.theme, Theme::Dark, "深色");
        ui.radio_value(&mut self.theme, Theme::Light, "浅色");
        ui.radio_value(&mut self.theme, Theme::Auto, "自动");

        // SelectableLabel (多选)
        ui.separator();
        ui.label("多选列表:");
        let mut to_remove = vec![];
        for (i, item) in self.items.iter().enumerate() {
            let sel = self.selected.contains(&i);
            if ui.selectable_label(sel, item).clicked() {
                if sel { to_remove.push(i); } else { self.selected.push(i); }
            }
        }
        for &i in &to_remove { self.selected.retain(|&x| x != i); }

        ui.label(format!("已选: {:?}", self.selected.iter().map(|&i| &self.items[i]).collect::<Vec<_>>()));
    }
}

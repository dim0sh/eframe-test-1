use eframe::egui::{
	Context,
	CentralPanel,
	Slider, SidePanel, TopBottomPanel,
    Layout, Align
};
use eframe::{
	Frame,
	App
};
#[derive(PartialEq)]
enum PanelSwitch {
    Age,
    Test,
}


struct MyApp {
    name: String,
    age: u32,
    panel_enum: PanelSwitch,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
            panel_enum: PanelSwitch::Age,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("myTopPanel").show(ctx, |ui| {
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.label("Selection");
                ui.separator();
                if ui.selectable_label(self.panel_enum == PanelSwitch::Age, "age panel").clicked() {
                    self.panel_enum = PanelSwitch::Age;
                    
                };
                if ui.selectable_label(self.panel_enum == PanelSwitch::Test, "test panel").clicked() {
                    self.panel_enum = PanelSwitch::Test;
                };
            });
        });
        SidePanel::right("myRightPanel").show(ctx,|ui| {
            ui.label("Right Panel");
        });
        CentralPanel::default().show(ctx, |ui| {
            match self.panel_enum {
                PanelSwitch::Age => {ui.heading("My egui Application");
                ui.horizontal(|ui| {
                    ui.label("Your name: ");
                    ui.text_edit_singleline(&mut self.name);
                });
                ui.add(Slider::new(&mut self.age, 0..=120)
                    .text("age"));
                if ui.button("Click each year").clicked() {
                    self.age += 1;
                }
                ui.label(format!(
                    "Hello '{}', age {}", self.name, self.age));
                ui.collapsing("age", |ui| {
                    if ui.button("age+").clicked() {
                        self.age += 1;
                    }
                    if ui.button("age-").clicked() {
                        self.age -= 1;
                    }
                });
                ui.separator()},
                PanelSwitch::Test => {ui.label("test panel")}
            }
            
         });
            
        }} //ew

fn main() {
    eframe::run_native("myapp", eframe::NativeOptions::default(), Box::new(|cc| Box::new(MyApp::default())));
    println!("Hello, world!");
}

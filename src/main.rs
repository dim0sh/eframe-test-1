use eframe::egui::{
	Context,
	CentralPanel,
	Slider
};
use eframe::{
	Frame,
	App
};

struct MyApp {
    name: String,
    age: u32,
}
impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

impl App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("My egui Application");
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
         });}} //ew

fn main() {
    eframe::run_native("myapp", eframe::NativeOptions::default(), Box::new(|cc| Box::new(MyApp::default())));
    println!("Hello, world!");
}

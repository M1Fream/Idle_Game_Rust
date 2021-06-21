use eframe::{egui, epi};
use crate::game_types;

pub struct TemplateApp {
    // Example stuff:
    label: String,

	game: Box<game_types::Game>,
}

impl<'a> TemplateApp{
    pub fn new(g: Box<game_types::Game>) -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            game: g,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Idle Game Rust"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self { label, game } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("Resources");

            ui.horizontal(|ui| {
                ui.label("Write something: ");
                ui.text_edit_singleline(label);
            });
			
			for i in 0..game_types::Resources::NUM_RESOURCES {
				ui.label(format!("{}: {}", i, game.resources._res[i]));
			}
			
/*            ui.add(egui::Slider::new(value, 0.0..=10.0).text("value"));
            if ui.button("Increment").clicked() {
                *value += 1.0;
            }*/

        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's

            ui.heading("Map");
        });
    }
}
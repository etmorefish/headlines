mod headlines;
use eframe::{
    egui::{CentralPanel, ScrollArea, Visuals},
    epaint::Vec2,
    App,
};
use headlines::{render_footer, render_haeder, Headlines};

impl App for Headlines {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        if self.config.dark_mode {
            ctx.set_visuals(Visuals::dark());
        } else {
            ctx.set_visuals(Visuals::light());
        }

        if !self.api_key_initialized {
            self.render_config(ctx);
        } else {
            self.render_top_panel(ctx, frame);

            CentralPanel::default().show(ctx, |ui| {
                render_haeder(ui);
                ScrollArea::vertical().show(ui, |ui| {
                    self.render_news_card(ui);
                });
                render_footer(ctx);
            });
        }
    }
}

fn main() {
    tracing_subscriber::fmt::init();

    let mut native_options = eframe::NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(480.0, 860.0));

    eframe::run_native(
        "Headlines",
        native_options,
        Box::new(|cc| Box::new(Headlines::new(cc))),
    );
}

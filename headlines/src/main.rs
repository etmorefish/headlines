mod headlines;
use eframe::{
    egui::{CentralPanel, ScrollArea, Visuals},
    epaint::Vec2,
    App,
};
use headlines::{render_footer, render_haeder, Headlines};
use newsapi::NewsAPI;

use crate::headlines::NewsCardData;

async fn fetch_news(api_key: String, articles: &mut Vec<NewsCardData>) {
    if let Ok(response) = NewsAPI::new(api_key).fetch() {
        let resp_articles = response.articles();
        for a in articles.iter() {
            let news = NewsCardData {
                title: a.title.to_string(),
                url: a.url.to_string(),
                desc: a.desc.map(|s| s.to_string()).unwrap_or("...".to_string()),
            };
            articles.push(news);
        }
    }
}

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

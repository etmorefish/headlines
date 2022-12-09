use eframe::egui::{menu, TopBottomPanel, Ui, Window};
use eframe::emath::Align;
use eframe::{
    egui::{self, Hyperlink, Layout, RichText, Separator, TextStyle},
    epaint::{Color32, FontFamily, FontId},
};
use serde::{Deserialize, Serialize};

// use crate::fetch_news;

pub const PADDING: f32 = 5.;
pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
pub const CYAN: Color32 = Color32::from_rgb(0, 255, 255);
pub const RED: Color32 = Color32::from_rgb(255, 0, 0);

#[derive(Serialize, Deserialize)]
pub struct HeadlinesConfig {
    pub dark_mode: bool,
    pub api_key: String,
}
impl HeadlinesConfig {
    fn new() -> Self {
        Self {
            dark_mode: true,
            api_key: String::new(),
        }
    }
}

impl Default for HeadlinesConfig {
    fn default() -> Self {
        Self {
            dark_mode: Default::default(),
            api_key: String::new(),
        }
    }
}

// #[derive(Default)]
pub struct Headlines {
    pub articles: Vec<NewCardData>,
    pub config: HeadlinesConfig,
    pub api_key_initialized: bool,
}
pub struct NewCardData {
    title: String,
    desc: String,
    url: String,
}

#[inline]
fn heading2() -> TextStyle {
    TextStyle::Name("Heading2".into())
}

#[inline]
fn heading3() -> TextStyle {
    TextStyle::Name("ContextHeading".into())
}

pub fn setup_custom_fonts(ctx: &egui::Context) {
    // Start with the default fonts (we will be adding to them rather than replacing them).
    let mut fonts = egui::FontDefinitions::default();
    // Install my own font (maybe supporting non-latin characters).
    // .ttf and .otf files supported.
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../../STFANGSO.TTF")),
    );
    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, "my_font".to_owned());
    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());
    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}

pub fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (heading2(), FontId::new(22.0, Proportional)),
        (heading3(), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Proportional)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.set_style(style);
}

impl Headlines {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_custom_fonts(&cc.egui_ctx);
        configure_text_styles(&cc.egui_ctx);

        let config: HeadlinesConfig = confy::load("headlines", "headcfg").unwrap_or_default();
        let iter = (0..20).map(|a| NewCardData {
            title: format!("title {}", a),
            desc: format!("desc {}", a),
            url: format!("http://exmaple.com/{}", a),
        });
        Headlines {
            articles: Vec::from_iter(iter),
            config,
            api_key_initialized: false,
        }
    }

    pub fn render_news_card(&self, ui: &mut egui::Ui) {
        for a in &self.articles {
            ui.add_space(PADDING);
            // render title
            let title = format!("▶ {}", &a.title);
            if self.config.dark_mode {
                ui.colored_label(WHITE, title);
            } else {
                ui.colored_label(BLACK, title);
            }
            // render desc
            ui.add_space(PADDING);
            ui.label(RichText::new(&a.desc).text_style(TextStyle::Button));

            //render url
            if self.config.dark_mode {
                ui.style_mut().visuals.hyperlink_color = CYAN;
            } else {
                ui.style_mut().visuals.hyperlink_color = RED;
            }

            ui.add_space(PADDING);
            ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                ui.add(Hyperlink::from_label_and_url("read more ⤴", &a.url))
            });
            ui.add_space(PADDING);
            ui.add(Separator::default());
        }
    }

    pub(crate) fn render_top_panel(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // define a TopButtomPannel widget
        // we'll add a menu bar
        // then we'll two layout widget to render the logo on the left
        // and the control buttons on the right.
        // padding before and after the pannel
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.);
            menu::bar(ui, |ui| {
                // logo
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    ui.label(RichText::new("📗").text_style(TextStyle::Heading));
                });
                //controls
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let close_btn = ui.button(RichText::new("❌").text_style(TextStyle::Body));
                    if close_btn.clicked() {
                        frame.close()
                    }
                    let refresh_btn = ui.button(RichText::new("🔃").text_style(TextStyle::Body));
                    if refresh_btn.clicked() {
                        todo!()
                    }
                    let theme_btn = ui.button(
                        RichText::new({
                            if self.config.dark_mode {
                                "🔆"
                            } else {
                                "🌙"
                            }
                        })
                        .text_style(TextStyle::Body),
                    );
                    if theme_btn.clicked() {
                        self.config.dark_mode = !self.config.dark_mode;
                    }
                })
            })
        });
    }

    pub fn render_config(&mut self, ctx: &egui::Context) {
        Window::new("Configuration").show(ctx, |ui| {
            ui.label("Enter you API_KEY for newsapi.org");
            let text_input = ui.text_edit_singleline(&mut self.config.api_key);
            if text_input.lost_focus() && ui.input().key_pressed(egui::Key::Enter) {
                if let Err(e) = confy::store(
                    "headlines",
                    "headcfg",
                    HeadlinesConfig {
                        dark_mode: self.config.dark_mode,
                        api_key: self.config.api_key.to_string(),
                    },
                ) {
                    tracing::error!("Failed saving app state: {} ", e);
                }
                self.api_key_initialized = true;
                tracing::error!("API KEY set");
            }
            tracing::error!("{}", &self.config.api_key);
            ui.label("If you havn't registered for the API_KEY, head over to");
            ui.hyperlink("https://newsapi.org");
        });
    }
}

pub fn render_haeder(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("headlines");
    });
    ui.add_space(PADDING);
    let seq = Separator::default().spacing(20.);
    ui.add(seq);
}
pub fn render_footer(ctx: &egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.label(
                RichText::new("https://www.github.com/emilk/egui/")
                    .text_style(TextStyle::Monospace),
            );
            ui.hyperlink_to(
                RichText::new("API source: newsapi.org").text_style(TextStyle::Monospace),
                "https://www.github.com/emilk/egui/",
            );
        })
    });
}

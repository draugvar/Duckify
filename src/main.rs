use eframe::egui::{self, Color32, CornerRadius, FontId, Margin, RichText, Vec2};

const STORAGE_KEY: &str = "duck_address";

// Color palette
const BG: Color32 = Color32::from_rgb(15, 15, 25);
const SURFACE: Color32 = Color32::from_rgb(26, 26, 42);
const SURFACE2: Color32 = Color32::from_rgb(38, 38, 58);
const ACCENT: Color32 = Color32::from_rgb(255, 213, 0);
const TEXT_DIM: Color32 = Color32::from_rgb(120, 120, 155);
const SUCCESS: Color32 = Color32::from_rgb(80, 200, 120);

fn is_valid_email(email: &str) -> bool {
    let Some((user, domain)) = email.split_once('@') else {
        return false;
    };
    if user.is_empty() || domain.is_empty() {
        return false;
    }
    let parts: Vec<&str> = domain.split('.').collect();
    parts.len() >= 2 && parts.last().is_some_and(|tld| tld.len() >= 2)
}

fn convert_to_duck_email(email: &str, duck_address: &str) -> String {
    let (user, domain) = email.split_once('@').unwrap();
    let duck_user = duck_address.split('@').next().unwrap();
    format!("{}_at_{}_{duck_user}@duck.com", user, domain)
}

struct App {
    email: String,
    duck_address: String,
    result: String,
    copied: bool,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::setup_style(&cc.egui_ctx);
        let duck_address = cc
            .storage
            .and_then(|s| s.get_string(STORAGE_KEY))
            .unwrap_or_default();
        Self {
            email: String::new(),
            duck_address,
            result: String::new(),
            copied: false,
        }
    }

    fn setup_style(ctx: &egui::Context) {
        let mut visuals = egui::Visuals::dark();
        visuals.panel_fill = BG;
        visuals.window_fill = BG;
        visuals.widgets.noninteractive.bg_fill = SURFACE2;
        visuals.widgets.noninteractive.corner_radius = CornerRadius::same(8);
        visuals.widgets.noninteractive.fg_stroke =
            egui::Stroke::new(1.0, Color32::from_rgb(180, 180, 210));
        visuals.widgets.inactive.bg_fill = SURFACE2;
        visuals.widgets.inactive.corner_radius = CornerRadius::same(8);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(50, 50, 75);
        visuals.widgets.hovered.corner_radius = CornerRadius::same(8);
        visuals.widgets.active.corner_radius = CornerRadius::same(8);
        visuals.selection.bg_fill = Color32::from_rgb(90, 75, 0);
        visuals.selection.stroke = egui::Stroke::new(1.0, ACCENT);
        visuals.extreme_bg_color = SURFACE2;
        ctx.set_visuals(visuals);

        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = Vec2::new(8.0, 6.0);
        style.spacing.button_padding = Vec2::new(16.0, 8.0);
        ctx.set_style(style);
    }

    fn card_frame() -> egui::Frame {
        egui::Frame::new()
            .fill(SURFACE)
            .corner_radius(CornerRadius::same(12))
            .inner_margin(Margin {
                left: 20,
                right: 20,
                top: 16,
                bottom: 16,
            })
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(BG)
                    .inner_margin(Margin::same(20)),
            )
            .show(ctx, |ui| {
                // ── Header ────────────────────────────────────────────────
                ui.vertical_centered(|ui| {
                    ui.add_space(6.0);
                    ui.label(
                        RichText::new("🦆  Duckify")
                            .size(28.0)
                            .strong()
                            .color(ACCENT),
                    );
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("Convert any email to a duck.com alias")
                            .size(12.0)
                            .color(TEXT_DIM),
                    );
                });

                ui.add_space(18.0);

                // ── Input card ────────────────────────────────────────────
                let (convert_clicked, enter_pressed, can_convert) =
                    Self::card_frame()
                        .show(ui, |ui| {
                            ui.label(
                                RichText::new("EMAIL TO CONVERT")
                                    .size(10.0)
                                    .color(TEXT_DIM)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            let email_resp = ui.add(
                                egui::TextEdit::singleline(&mut self.email)
                                    .desired_width(f32::INFINITY)
                                    .hint_text("user@example.com")
                                    .font(FontId::proportional(14.0)),
                            );

                            ui.add_space(12.0);

                            ui.label(
                                RichText::new("PERSONAL DUCK ADDRESS")
                                    .size(10.0)
                                    .color(TEXT_DIM)
                                    .strong(),
                            );
                            ui.add_space(4.0);
                            ui.add(
                                egui::TextEdit::singleline(&mut self.duck_address)
                                    .desired_width(f32::INFINITY)
                                    .hint_text("yourname@duck.com")
                                    .font(FontId::proportional(14.0)),
                            );

                            ui.add_space(18.0);

                            let can = is_valid_email(&self.email)
                                && is_valid_email(&self.duck_address);

                            let clicked = ui
                                .vertical_centered(|ui| {
                                    ui.add_enabled(
                                        can,
                                        egui::Button::new(
                                            RichText::new("Convert")
                                                .size(14.0)
                                                .color(if can {
                                                    Color32::from_rgb(20, 20, 20)
                                                } else {
                                                    TEXT_DIM
                                                })
                                                .strong(),
                                        )
                                        .fill(if can { ACCENT } else { SURFACE2 })
                                        .corner_radius(CornerRadius::same(8))
                                        .min_size(Vec2::new(140.0, 36.0)),
                                    )
                                    .clicked()
                                })
                                .inner;

                            let enter = email_resp.lost_focus()
                                && ctx.input(|i| i.key_pressed(egui::Key::Enter));

                            (clicked, enter, can)
                        })
                        .inner;

                if (convert_clicked || enter_pressed) && can_convert {
                    if let Some(storage) = frame.storage_mut() {
                        storage.set_string(STORAGE_KEY, self.duck_address.clone());
                        storage.flush();
                    }
                    self.result = convert_to_duck_email(&self.email, &self.duck_address);
                    self.copied = false;
                }

                // ── Result card ───────────────────────────────────────────
                if !self.result.is_empty() {
                    ui.add_space(12.0);
                    Self::card_frame().show(ui, |ui| {
                        ui.label(
                            RichText::new("RESULT")
                                .size(10.0)
                                .color(TEXT_DIM)
                                .strong(),
                        );
                        ui.add_space(4.0);
                        ui.horizontal(|ui| {
                            let mut display = self.result.clone();
                            ui.add(
                                egui::TextEdit::singleline(&mut display)
                                    .desired_width(ui.available_width() - 62.0)
                                    .interactive(false)
                                    .font(FontId::proportional(13.0)),
                            );
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new("Copy")
                                            .color(Color32::from_rgb(20, 20, 20))
                                            .strong()
                                            .size(13.0),
                                    )
                                    .fill(ACCENT)
                                    .corner_radius(CornerRadius::same(6)),
                                )
                                .clicked()
                            {
                                ctx.copy_text(self.result.clone());
                                self.copied = true;
                            }
                        });
                        if self.copied {
                            ui.add_space(6.0);
                            ui.label(
                                RichText::new("✓  Copied to clipboard!")
                                    .color(SUCCESS)
                                    .size(12.0),
                            );
                        }
                    });
                }
            });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Duckify")
            .with_inner_size([460.0, 400.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Duckify",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

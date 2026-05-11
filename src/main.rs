use eframe::egui::{self, Color32, ColorImage, CornerRadius, FontId, Margin, RichText, Vec2};

const STORAGE_KEY: &str = "duck_address";

// Color palette
const BG: Color32 = Color32::from_rgb(15, 15, 25);
const SURFACE: Color32 = Color32::from_rgb(26, 26, 42);
const SURFACE2: Color32 = Color32::from_rgb(38, 38, 58);
const ACCENT: Color32 = Color32::from_rgb(255, 213, 0);
const TEXT_DIM: Color32 = Color32::from_rgb(120, 120, 155);
const SUCCESS: Color32 = Color32::from_rgb(80, 200, 120);
const ON_ACCENT: Color32 = Color32::from_rgb(20, 20, 20);

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
    let local_part = duck_address.split('@').next().unwrap_or(duck_address);
    let sanitized = email.replace('@', "_at_").replace('.', "_");
    format!("{sanitized}_{local_part}@duck.com")
}

fn load_icon_texture(ctx: &egui::Context) -> egui::TextureHandle {
    let png_bytes = include_bytes!("../assets/icon.png");
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes.as_ref()));
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();

    let size = [info.width as usize, info.height as usize];
    let pixels = match info.color_type {
        png::ColorType::Rgba => buf[..info.buffer_size()].to_vec(),
        png::ColorType::Rgb => {
            let mut rgba = Vec::with_capacity(size[0] * size[1] * 4);
            for chunk in buf[..info.buffer_size()].chunks(3) {
                rgba.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
            }
            rgba
        }
        _ => buf[..info.buffer_size()].to_vec(),
    };

    let color_image = ColorImage::from_rgba_unmultiplied(size, &pixels);
    ctx.load_texture("app_icon", color_image, Default::default())
}

struct App {
    email: String,
    duck_address: String,
    result: String,
    copied_since: Option<f64>,
    icon: Option<egui::TextureHandle>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Self::setup_style(&cc.egui_ctx);
        let icon = load_icon_texture(&cc.egui_ctx);
        let duck_address = cc
            .storage
            .and_then(|s| s.get_string(STORAGE_KEY))
            .unwrap_or_default();
        Self {
            email: String::new(),
            duck_address,
            result: String::new(),
            copied_since: None,
            icon: Some(icon),
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
        // Reset copy feedback after 2 seconds
        if let Some(copied_at) = self.copied_since {
            if ctx.input(|i| i.time) - copied_at > 2.0 {
                self.copied_since = None;
            }
        }

        let copied = self.copied_since.is_some();

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
                    ui.allocate_ui_with_layout(
                        Vec2::new(ui.available_width(), 44.0),
                        egui::Layout::left_to_right(egui::Align::Center)
                            .with_cross_align(egui::Align::Center),
                        |ui| {
                            if let Some(icon) = &self.icon {
                                ui.add(
                                    egui::Image::from_texture(
                                        egui::load::SizedTexture::from_handle(icon),
                                    )
                                    .max_width(44.0)
                                    .max_height(44.0),
                                );
                                ui.add_space(10.0);
                            }
                            ui.label(
                                RichText::new("Quackify")
                                    .size(28.0)
                                    .strong()
                                    .color(ACCENT),
                            );
                        },
                    );
                    ui.add_space(2.0);
                    ui.label(
                        RichText::new("DuckDuckGo Email Converter")
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
                                RichText::new("EMAIL TO MASK")
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
                                RichText::new("YOUR DUCK ADDRESS")
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
                                                    ON_ACCENT
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
                    self.copied_since = None;
                }

                // ── Result card ───────────────────────────────────────────
                if !self.result.is_empty() {
                    ui.add_space(12.0);
                    Self::card_frame().show(ui, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(
                                RichText::new("MASKED ALIAS")
                                    .size(10.0)
                                    .color(TEXT_DIM)
                                    .strong(),
                            );
                            ui.add_space(8.0);
                            ui.label(
                                RichText::new(&self.result)
                                    .font(FontId::monospace(13.0))
                                    .color(ACCENT),
                            );
                            ui.add_space(10.0);

                            let copy_clicked = ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(if copied {
                                            "✓  Copied!"
                                        } else {
                                            "Copy"
                                        })
                                        .color(if copied { SUCCESS } else { ON_ACCENT })
                                        .strong()
                                        .size(13.0),
                                    )
                                    .fill(if copied { SURFACE } else { ACCENT })
                                    .corner_radius(CornerRadius::same(6))
                                    .min_size(Vec2::new(140.0, 36.0)),
                                )
                                .clicked();

                            if copy_clicked {
                                ctx.copy_text(self.result.clone());
                                self.copied_since = Some(ctx.input(|i| i.time));
                            }
                        });
                        // Force card to fill available width
                        ui.allocate_space(Vec2::new(ui.available_width(), 0.0));
                    });
                }
            });
    }
}

fn make_icon() -> egui::IconData {
    let png_bytes = include_bytes!("../assets/icon.png");
    let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes.as_ref()));
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0u8; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf).unwrap();
    egui::IconData {
        rgba: buf[..info.buffer_size()].to_vec(),
        width: info.width,
        height: info.height,
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Quackify")
            .with_inner_size([460.0, 480.0])
            .with_icon(make_icon())
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Quackify",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

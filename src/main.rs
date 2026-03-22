use eframe::egui;

const STORAGE_KEY: &str = "duck_address";

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
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Duckify");
            ui.add_space(12.0);

            ui.label("Email to convert:");
            let email_resp = ui.text_edit_singleline(&mut self.email);
            ui.add_space(8.0);

            ui.label("Personal Duck Address:");
            ui.add(
                egui::TextEdit::singleline(&mut self.duck_address)
                    .hint_text("your-name@duck.com"),
            );
            ui.add_space(12.0);

            let can_convert =
                is_valid_email(&self.email) && is_valid_email(&self.duck_address);

            let convert_clicked = ui
                .add_enabled(can_convert, egui::Button::new("Convert"))
                .clicked();
            let enter_pressed = email_resp.lost_focus()
                && ctx.input(|i| i.key_pressed(egui::Key::Enter));

            if (convert_clicked || enter_pressed) && can_convert {
                if let Some(storage) = frame.storage_mut() {
                    storage.set_string(STORAGE_KEY, self.duck_address.clone());
                    storage.flush();
                }
                self.result = convert_to_duck_email(&self.email, &self.duck_address);
                self.copied = false;
            }

            if !self.result.is_empty() {
                ui.add_space(12.0);
                ui.label("Result:");
                ui.horizontal(|ui| {
                    let mut display = self.result.clone();
                    ui.add(
                        egui::TextEdit::singleline(&mut display)
                            .desired_width(300.0)
                            .interactive(false),
                    );
                    if ui.button("Copy").clicked() {
                        ctx.copy_text(self.result.clone());
                        self.copied = true;
                    }
                });
                if self.copied {
                    ui.colored_label(egui::Color32::DARK_GREEN, "✓ Copied to clipboard!");
                }
            }
        });
    }
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Duckify")
            .with_inner_size([460.0, 240.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "Duckify",
        options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}

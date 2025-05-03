use eframe::egui;
use eframe::egui::text::LayoutJob;
use eframe::egui::{Color32, FontId, TextFormat};
use serde_json::{from_str, Value};

fn main() -> Result<(), eframe::Error> {
    eframe::run_native(
        "JSON Beautifier",
        eframe::NativeOptions {
            ..Default::default()
        },
        Box::new(|_cc| Ok(Box::<MyApp>::default())) // Return Result<Box<dyn App>>
    )
}

#[derive(Default)]
struct MyApp {
    raw_json: String,
    formatted_json: String,
    search_query: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Setting the theme to dark
        ctx.set_visuals(egui::Visuals::dark());

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Input JSON:");
                ui.add(egui::TextEdit::multiline(&mut self.raw_json).hint_text("Paste your raw JSON here"));
            });

            ui.separator();

            if ui.button("Beautify").clicked() {
                self.formatted_json = beautify_json(&self.raw_json);
            }

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Search:");
                ui.text_edit_singleline(&mut self.search_query);
            });

            ui.separator();

            ui.group(|ui| {
                ui.label("Formatted JSON:");
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        let highlighted_text = highlight_search(&self.formatted_json, &self.search_query);
                        ui.label(highlighted_text);
                    });
            });
        });
    }
}

/// Beautifies the input raw JSON string
fn beautify_json(raw_json: &str) -> String {
    match from_str::<Value>(raw_json) {
        Ok(value) => serde_json::to_string_pretty(&value).unwrap_or_else(|_| String::from("Error pretty printing JSON")),
        Err(_) => String::from("Invalid JSON format"),
    }
}

/// Highlights the search query in the JSON text by changing its color.
fn highlight_search(json: &str, query: &str) -> LayoutJob {
    let mut job = LayoutJob::default();

    if query.is_empty() {
        job.append(json, 0.0, TextFormat::default());
        return job;
    }

    let mut remaining = json;
    while let Some(index) = remaining.find(query) {
        // Append the text before the match (unhighlighted)
        if index > 0 {
            job.append(
                &remaining[..index],
                0.0,
                TextFormat {
                    font_id: FontId::monospace(14.0),
                    color: Color32::WHITE,
                    ..Default::default()
                },
            );
        }

        // Append the matching part (highlighted)
        job.append(
            &remaining[index..index + query.len()],
            0.0,
            TextFormat {
                font_id: FontId::monospace(14.0),
                color: Color32::YELLOW,
                background: Color32::DARK_GRAY,
                ..Default::default()
            },
        );

        // Move forward in the string
        remaining = &remaining[index + query.len()..];
    }

    // Append the rest of the string (after last match)
    if !remaining.is_empty() {
        job.append(
            remaining,
            0.0,
            TextFormat {
                font_id: FontId::monospace(14.0),
                color: Color32::WHITE,
                ..Default::default()
            },
        );
    }

    job
}

mod quiz;

use egui::{CentralPanel, Context};
use quiz::{Question, Quiz};

struct App {
    quiz: Quiz,
}

impl App {
    fn new() -> Self {
        let quiz = Quiz::new(Question::sample());
        Self { quiz }
    }
}

// yes, this is another impl block
// in Typescript, this would be: `class App implements eframe::app::App`
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            // Styling
            let mut style = (*ctx.style()).clone();
            style.text_styles = [
                (egui::TextStyle::Body, egui::FontId::proportional(15.0)),
                (egui::TextStyle::Button, egui::FontId::proportional(15.0)),
            ]
            .into();
            style.spacing.button_padding = egui::Vec2::new(10.0, 5.0);
            ui.set_style(style);
            ui.visuals_mut().widgets.inactive.rounding = egui::Rounding::from(6.0);

            let display_index = self.quiz.current_index + 1;
            let total_count = self.quiz.questions.len();

            ui.label(format!("{display_index}/{total_count}"));

            let current_question = self.quiz.current_question();
            ui.label(
                egui::RichText::new(current_question.title.as_str())
                    .font(egui::FontId::proportional(28.0)),
            );

            let answered_label_text = match &current_question.is_answered() {
                true => format!(
                    "You have selected \"{}\"",
                    &current_question.user_answer.unwrap()
                ),
                false => String::from("You have not answered this question"),
            };
            ui.label(answered_label_text);

            ui.horizontal(|ui| {
                let true_button = get_button(
                    "True",
                    self.quiz.current_question().user_answer == Some(true),
                );

                let false_button = get_button(
                    "False",
                    self.quiz.current_question().user_answer == Some(false),
                );

                if ui.add(true_button).clicked() {
                    self.quiz.answer(true);
                }

                if ui.add(false_button).clicked() {
                    self.quiz.answer(false);
                };
            });

            ui.horizontal(|ui| {
                ui.columns(2, |columns| {
                    columns[0].allocate_ui_with_layout(
                        egui::Vec2 { x: 120.0, y: 40.0 },
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            if ui.button("Previous").clicked() {
                                self.quiz.prev_question();
                            }
                        },
                    );

                    columns[1].allocate_ui_with_layout(
                        egui::Vec2 { x: 120.0, y: 40.0 },
                        egui::Layout::right_to_left(egui::Align::Center),
                        |ui| {
                            if ui.button("Next").clicked() {
                                self.quiz.next_question();
                            }
                        },
                    );
                })
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let viewport = egui::ViewportBuilder::default()
        .with_inner_size(egui::Vec2::new(300.0, 400.0))
        .with_resizable(false);

    let options = eframe::NativeOptions {
        viewport,
        ..Default::default()
    };

    eframe::run_native("Queeez", options, Box::new(|_| Box::new(App::new())))
}

fn get_button(label: &str, selected: bool) -> egui::Button<'static> {
    let mut label = egui::RichText::from(label);

    if selected {
        label = label.color(egui::Color32::WHITE);
    }

    let mut button = egui::Button::new(label).min_size(egui::Vec2::new(60.0, 30.0));

    if selected {
        button = button.fill(egui::Color32::BLUE);
    }

    button
}

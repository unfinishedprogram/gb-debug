use egui::Ui;

pub enum Action {
    StepFrame,
    StepSingle,
}

#[derive(Default)]
pub enum RunningState {
    #[default]
    Paused,
    Broke,
    Running,
}

#[derive(Default)]
pub struct RunController {
    state: RunningState,
}

impl RunController {
    pub fn draw(&mut self, ui: &mut Ui) -> Option<Action> {
        ui.horizontal(|ui| {
            let button_txt = match self.state {
                RunningState::Paused => "▶",
                RunningState::Broke => "▶",
                RunningState::Running => "⏹",
            };

            if ui.button(button_txt).on_hover_text("Toggle Play").clicked() {
                self.state = match self.state {
                    RunningState::Paused => RunningState::Running,
                    RunningState::Broke => RunningState::Running,
                    RunningState::Running => RunningState::Paused,
                }
            }

            if ui.button("⏵").on_hover_text("Single Step").clicked() {
                return Some(Action::StepSingle);
            }

            if ui.button("⏩").on_hover_text("Step Frame").clicked() {
                return Some(Action::StepFrame);
            }

            match self.state {
                RunningState::Paused => None,
                RunningState::Broke => None,
                RunningState::Running => Some(Action::StepFrame),
            }
        })
        .inner
    }
}

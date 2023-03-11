use egui::{CentralPanel, SidePanel, TopBottomPanel};
use gameboy::Gameboy;

use crate::components::{BreakpointManager, Logs, Screen};

#[derive(Default)]
pub enum RunningState {
    #[default]
    Stopped,
    Running,
}

#[derive(Default)]
pub struct Debugger {
    gameboy: Gameboy,
    screen: Screen,
    running_state: RunningState,
    breakpoint_manager: BreakpointManager,
}

impl Debugger {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if matches!(self.running_state, RunningState::Running) {
            self.gameboy.step_single_frame();
        }

        let screen_buffer = self.gameboy.ppu.lcd.front_buffer();

        TopBottomPanel::top("top").show(ctx, |ui| {
            use RunningState::*;

            let btn_text = match self.running_state {
                Stopped => "start",
                Running => "stop",
            };

            if ui.button(btn_text).clicked() {
                self.running_state = match self.running_state {
                    Stopped => Running,
                    Running => Stopped,
                }
            }
        });

        CentralPanel::default().show(ctx, |ui| self.screen.draw(ui, screen_buffer));
        SidePanel::right("right").show(ctx, |ui| {
            Logs::draw(ui);
            self.breakpoint_manager.draw(ui);
        });

        ctx.request_repaint();
    }
}

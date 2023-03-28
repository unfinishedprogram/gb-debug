use egui::{CentralPanel, SidePanel, TopBottomPanel};
use gameboy::Gameboy;

use crate::components::{
    run_controller::{self, RunController},
    BreakpointManager, Logs, Screen,
};

#[derive(Default)]
pub struct Debugger {
    gameboy: Gameboy,
    screen: Screen,
    breakpoint_manager: BreakpointManager,
    run_controller: RunController,
}

impl Debugger {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for Debugger {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top").show(ctx, |ui| {
            use run_controller::Action;
            match self.run_controller.draw(ui) {
                Some(Action::StepFrame) => self.gameboy.step_single_frame(),
                Some(Action::StepSingle) => self.gameboy.step(),
                None => {}
            }
        });
        let screen_buffer = self.gameboy.ppu.lcd.front_buffer();

        CentralPanel::default().show(ctx, |ui| self.screen.draw(ui, screen_buffer));
        SidePanel::right("right").show(ctx, |ui| {
            Logs::draw(ui);
            self.breakpoint_manager.draw(ui);
        });

        ctx.request_repaint();
    }
}

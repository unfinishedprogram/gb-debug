use egui::Ui;
use gameboy::debugger::Breakpoint;

use super::breakpoint_selector::BreakpointSelector;

#[derive(Default)]
pub struct BreakpointManager {
    add_breakpoint_open: Option<AddBreakpointMenu>,
}

#[derive(Default)]
pub struct AddBreakpointMenu {
    breakpoint: Option<Breakpoint>,
}

impl BreakpointManager {
    pub fn draw(&mut self, ui: &mut Ui) {
        // let debugger = DEBUGGER.lock().unwrap();

        if ui.button("Add Breakpoint").clicked() {
            self.add_breakpoint_open = Some(AddBreakpointMenu::default());
        }

        if let Some(state) = &mut self.add_breakpoint_open {
            let breakpoint_selector = BreakpointSelector::new(&mut state.breakpoint);
            egui::Window::new("add breakpoint").show(ui.ctx(), |ui| ui.add(breakpoint_selector));
        }
    }
}

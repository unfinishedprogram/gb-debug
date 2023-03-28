use egui::Ui;
use gameboy::{
    gb_sm83::{instruction::Instruction, SM83},
    Gameboy,
};

pub fn generate_instructions(gb: &Gameboy) -> Vec<(u16, Instruction)> {
    let mut gb = gb.clone();
    let mut instructions = vec![];

    for _ in 0..10 {
        instructions.push((gb.cpu_state.registers.pc, gb.fetch_next_instruction()));
    }

    instructions
}

pub fn code_view(gb: &Gameboy, ui: &mut Ui) {
    for (pc, inst) in generate_instructions(gb) {
        ui.label(format!("[{pc:04x}]:{inst:?}"));
    }
}

use egui::style::Spacing;
use egui::{Align, Rgba, Style, Ui, Vec2};
use egui_extras::{Column, TableBuilder};
use gameboy::gb_sm83::instruction::Instruction;
use gameboy::gb_sm83::SM83;
use gameboy::Gameboy;

#[derive(Default)]
pub struct LinearMemoryView {
    instructions: Option<Vec<(u16, Instruction)>>,
    keep_pc_in_view: bool,
}

pub fn generate_instructions(gb: &Gameboy) -> Vec<(u16, Instruction)> {
    let mut gb = gb.clone();
    let mut instructions = vec![];

    while gb.cpu_state.registers.pc < 0xFFFF {
        instructions.push((gb.cpu_state.registers.pc, gb.fetch_next_instruction()));
    }

    instructions
}

impl LinearMemoryView {
    pub fn draw(&mut self, gameboy: &Gameboy, ui: &mut Ui) {
        let pc = gameboy.cpu_state.registers.pc;

        let instructions = self
            .instructions
            .get_or_insert_with(|| generate_instructions(gameboy));

        ui.set_min_height(260.0);

        ui.horizontal(|ui| {
            if ui.button("Decompile").clicked() {
                *instructions = generate_instructions(gameboy);
            }

            ui.checkbox(&mut self.keep_pc_in_view, "Lock View")
        });

        ui.set_style(Style {
            spacing: Spacing {
                item_spacing: Vec2 { x: 0.0, y: 0.0 },
                ..Default::default()
            },
            ..Default::default()
        });

        ui.separator();

        let mut row = 0;
        for (index, (addr, _)) in instructions.iter().enumerate() {
            if addr >= &pc {
                row = index;
                break;
            }
        }

        if self.keep_pc_in_view {
            TableBuilder::new(ui).scroll_to_row(row, Some(Align::Center))
        } else {
            TableBuilder::new(ui)
        }
        .striped(true)
        .resizable(true)
        .column(Column::exact(40.0))
        .column(Column::remainder().resizable(true))
        .vscroll(true)
        .body(|body| {
            body.rows(20.0, instructions.len(), |index, mut row| {
                let (addr, inst) = &instructions[index];
                let color = if pc == *addr {
                    Rgba::from_rgb(1.0, 0.0, 0.0)
                } else {
                    Rgba::from_rgb(0.5, 0.5, 0.5)
                };

                row.col(|ui| {
                    ui.colored_label(color, format!("{addr:04X}"));
                });

                row.col(|ui| {
                    ui.colored_label(color, format!("{:?}", inst));
                });
            });
        });
    }
}

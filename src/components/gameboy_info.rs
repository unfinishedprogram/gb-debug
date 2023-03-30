use egui::Ui;
use gameboy::gb_sm83::flags::{cpu::*, Flags};
use gameboy::Gameboy;

pub fn show_gameboy_info(gb: &Gameboy, ui: &mut Ui) {
    let cpu = &gb.cpu_state;
    let r = &cpu.registers;

    ui.horizontal(|ui| {
        ui.vertical(|ui| {
            ui.monospace(format!("A:{:02x}", r.bytes[0]));
            ui.monospace(format!("B:{:02x}", r.bytes[1]));
            ui.monospace(format!("C:{:02x}", r.bytes[2]));
            ui.monospace(format!("D:{:02x}", r.bytes[3]));
            ui.monospace(format!("E:{:02x}", r.bytes[4]));
            ui.monospace(format!("F:{:02x}", r.bytes[5]));
            ui.monospace(format!("H:{:02x}", r.bytes[6]));
            ui.monospace(format!("L:{:02x}", r.bytes[7]));
        });

        ui.separator();

        ui.vertical(|ui| {
            ui.monospace(format!("PC:{:04x}", r.pc));
            ui.monospace(format!("SP:{:04x}", r.sp));

            ui.separator();

            ui.monospace(format!(
                "AF:{:04x}",
                u16::from_le_bytes([r.bytes[5], r.bytes[0]])
            ));
            ui.monospace(format!(
                "BC:{:04x}",
                u16::from_le_bytes([r.bytes[2], r.bytes[1]])
            ));
            ui.monospace(format!(
                "DE:{:04x}",
                u16::from_le_bytes([r.bytes[4], r.bytes[3]])
            ));
            ui.monospace(format!(
                "HL:{:04x}",
                u16::from_le_bytes([r.bytes[7], r.bytes[6]])
            ));
            ui.horizontal(|ui| {
                ui.monospace(format!("Z:{}", if cpu.get_flag(Z) { "⬛" } else { "⬜" }));
                ui.monospace(format!("N:{}", if cpu.get_flag(N) { "⬛" } else { "⬜" }));
                ui.monospace(format!("H:{}", if cpu.get_flag(H) { "⬛" } else { "⬜" }));
                ui.monospace(format!("C:{}", if cpu.get_flag(C) { "⬛" } else { "⬜" }));
            })
        });
    });
}

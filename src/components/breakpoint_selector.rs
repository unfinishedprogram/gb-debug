use egui::{ComboBox, DragValue, Response, Ui, Widget};
use gameboy::debugger::Breakpoint;

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<Breakpoint>) -> Option<Breakpoint>>;

pub struct BreakpointSelector<'a> {
    get_set_value: GetSetValue<'a>,
}

impl<'a> BreakpointSelector<'a> {
    pub fn new(value: &'a mut Option<Breakpoint>) -> Self {
        Self::from_get_set(move |v: Option<Breakpoint>| {
            if let Some(v) = v {
                *value = Some(v)
            }
            value.clone()
        })
    }

    pub fn from_get_set(
        get_set_value: impl 'a + FnMut(Option<Breakpoint>) -> Option<Breakpoint>,
    ) -> Self {
        Self {
            get_set_value: Box::new(get_set_value),
        }
    }
}

pub fn select_breakpoint(ui: &mut Ui, value: &mut Option<Breakpoint>) {
    // let mut next_value: Option<Breakpoint> = None;
    use Breakpoint::*;

    let Some(current) = value else { return };

    match current {
        Addr(addr) | ReadMem(addr) | WriteMem(addr) => {
            ui.add(DragValue::new(addr).hexadecimal(4, false, false));
        }
        PPUEnterMode(_) => todo!(),
        ExecInstruction(_) => todo!(),
        PPUModeChange => todo!(),
        SpeedSwitch(_) => todo!(),
    };
}

impl<'a> Widget for BreakpointSelector<'a> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        use Breakpoint::*;

        let selected_text = if let Some(breakpoint) = (self.get_set_value)(None) {
            format!("{:?}", breakpoint)
        } else {
            "*empty*".to_owned()
        };

        ui.horizontal(|ui| {
            ComboBox::from_id_source("breakpoint_selector")
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    let mut value = (self.get_set_value)(None);
                    ui.selectable_value(&mut value, Some(Addr(0)), "Addr");
                    ui.selectable_value(&mut value, Some(ReadMem(0)), "ReadMem");
                    (self.get_set_value)(value);
                });

            let mut value = (self.get_set_value)(None);
            select_breakpoint(ui, &mut value);
            (self.get_set_value)(value);
        })
        .response
    }
}

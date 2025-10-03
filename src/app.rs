use crate::chip8::Chip8;
use crate::ui; 
use eframe::egui;
use std::time::Duration;

#[derive(Debug, PartialEq, Eq)]
pub enum ExecutionState {
    Running,
    Paused,
}

pub struct MyApp {
    chip8: Chip8,
    cycles_per_frame: usize,
    pub(crate) rom_to_load: Option<String>, 
    pub debugger_open: bool,
    pub execution_state: ExecutionState,
    pub step_requested: bool,
}

impl MyApp {
    pub fn new(rom_path: Option<&str>) -> Self {
        let mut chip8 = Chip8::new();
        if let Some(path) = rom_path {
            chip8.chip8_load_rom(path);
        }
        Self {
            chip8,
            cycles_per_frame: 10,
            rom_to_load: None,
            debugger_open: false,
            execution_state: ExecutionState::Running,
            step_requested: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(path) = self.rom_to_load.take() {
            let mut new_chip8 = Chip8::new();
            new_chip8.chip8_load_rom(&path);
            self.chip8 = new_chip8;
        }

        ui::draw_menu_bar(self, ctx);
        if self.debugger_open {
            ui::draw_debugger_panel(&self.chip8, ctx);
        }
        ui::draw_emulator_screen(&self.chip8, ctx);

        if self.step_requested && self.execution_state == ExecutionState::Paused {
            self.chip8.chip8_emulate_cycle();
            self.step_requested = false;
        }

        if self.execution_state == ExecutionState::Running {
            for _ in 0..self.cycles_per_frame {
                self.chip8.chip8_emulate_cycle();
        }
        }
        
        if self.chip8.delay_timer > 0 { self.chip8.delay_timer -= 1; }
        if self.chip8.sound_timer > 0 { self.chip8.sound_timer -= 1; }

        let key_map = [
            (egui::Key::Num1, 0x1), (egui::Key::Num2, 0x2), (egui::Key::Num3, 0x3), (egui::Key::Num4, 0xC),
            (egui::Key::Q, 0x4), (egui::Key::W, 0x5), (egui::Key::E, 0x6), (egui::Key::R, 0xD),
            (egui::Key::A, 0x7), (egui::Key::S, 0x8), (egui::Key::D, 0x9), (egui::Key::F, 0xE),
            (egui::Key::Z, 0xA), (egui::Key::X, 0x0), (egui::Key::C, 0xB), (egui::Key::V, 0xF),
        ];
        ctx.input(|i| {
            for (key, chip8_key_index) in key_map {
                self.chip8.keypad[chip8_key_index] = if i.key_down(key) { 1 } else { 0 };
            }
        });
        
        ctx.request_repaint_after(Duration::from_millis(2));
    }
}
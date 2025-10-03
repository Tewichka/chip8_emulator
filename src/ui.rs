use crate::chip8::{self, Chip8};
use crate::disasm;
use crate::MyApp;
use crate::app::ExecutionState; 
use eframe::egui;
use rfd::FileDialog;

pub fn draw_menu_bar(app: &mut MyApp, ctx: &egui::Context) {
    egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open ROM...").clicked() {
                    if let Some(path) = FileDialog::new()
                        .add_filter("Chip-8 ROM", &["ch8", ""])
                        .pick_file()
                    {
                        app.rom_to_load = Some(path.display().to_string());
                    }
                    ui.close();
                }
            });

            ui.menu_button("Options", |ui| {
                if ui.toggle_value(&mut app.debugger_open, "Show Debugger").clicked() {
                    ui.close(); 
                }
            });

            ui.add_space(10.0);

            let button_text = match app.execution_state {
                    ExecutionState::Running => "⏸",
                    ExecutionState::Paused => "▶",
            };

            let pause_button_widget = egui::Button::new(button_text);
            if ui.add_sized([21.0, 18.0], pause_button_widget).clicked() {
                app.execution_state = match app.execution_state {
                    ExecutionState::Running => ExecutionState::Paused,
                    ExecutionState::Paused => ExecutionState::Running,
                };
                ui.close();
            };

            let is_paused = app.execution_state == ExecutionState::Paused;
                let step_button = egui::Button::new("➡");
                if ui.add_enabled(is_paused, step_button).clicked() {
                    app.step_requested = true;
                    ui.close();
            };
        });
    });
}

pub fn draw_debugger_panel(chip8: &Chip8, ctx: &egui::Context) {
    egui::TopBottomPanel::bottom("debugger_panel")
        .default_height(200.0)
        .resizable(true)
        .show(ctx, |ui| {
            ui.heading("Debugger");
            ui.add_space(5.0);

            ui.columns(2, |columns| {
                columns[0].vertical(|ui| {
                    ui.label(egui::RichText::new("Registers").underline());
                    egui::Grid::new("registers_grid")
                        .num_columns(4)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            for i in 0..8 {
                                ui.label(format!("V{:X}", i));
                                ui.monospace(format!("{:#04X}", chip8.v[i]));
                                ui.label(format!("V{:X}", i + 8));
                                ui.monospace(format!("{:#04X}", chip8.v[i + 8]));
                                ui.end_row();
                            }
                        });
                    ui.separator();
                    egui::Grid::new("special_registers_grid").show(ui, |ui| {
                        ui.label("PC"); ui.monospace(format!("{:#06X}", chip8.pc)); ui.end_row();
                        ui.label("I"); ui.monospace(format!("{:#06X}", chip8.i)); ui.end_row();
                        ui.label("SP"); ui.monospace(format!("{:#04X}", chip8.sp)); ui.end_row();
                    });
                });

                columns[1].vertical(|ui| {
                    ui.label(egui::RichText::new("Disassembly").underline());
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        let current_pc = chip8.pc;
                        for offset in -6..=7 {
                            let addr = current_pc.wrapping_add_signed(offset * 2);
                            if addr as usize > chip8::MEMORY_SIZE - 2 { continue; }
                            
                            let hi = chip8.memory[addr as usize] as u16;
                            let lo = chip8.memory[addr as usize + 1] as u16;
                            let opcode = (hi << 8) | lo;
                            let disasm_text = disasm::disassemble(opcode);
                            let text = format!("{:#06X}: {}", addr, disasm_text);
                            let mut label = egui::RichText::new(&text).monospace();
                            if addr == current_pc {
                                label = label.background_color(egui::Color32::from_rgb(50, 50, 80));
                            }
                            ui.label(label);
                        }
                    });
                });
            });
        });
}

pub fn draw_emulator_screen(chip8: &Chip8, ctx: &egui::Context) {
    egui::CentralPanel::default().show(ctx, |ui| {
        let (response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap(), egui::Sense::hover());
        let pixel_size_x = response.rect.width() / chip8::DISPLAY_WIDTH as f32;
        let pixel_size_y = response.rect.height() / chip8::DISPLAY_HEIGHT as f32;
        let pixel_size = pixel_size_x.min(pixel_size_y);
        painter.rect_filled(response.rect, 0.0, egui::Color32::BLACK);
        for y in 0..chip8::DISPLAY_HEIGHT {
            for x in 0..chip8::DISPLAY_WIDTH {
                if chip8.display[y * chip8::DISPLAY_WIDTH + x] != 0 {
                    let rect = egui::Rect::from_min_size(
                        response.rect.min + egui::vec2(x as f32 * pixel_size, y as f32 * pixel_size),
                        egui::vec2(pixel_size, pixel_size),
                    );
                    painter.rect_filled(rect, 0.0, egui::Color32::from_rgb(100, 255, 100));
                }
            }
        }
    });
}
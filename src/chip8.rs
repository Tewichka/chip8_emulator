use std::fs;

use rand::Rng;

pub const MEMORY_SIZE: usize  =  4096;
pub const NUM_REGISTERS: usize  =  16;
pub const STACK_SIZE: usize  =  16;
pub const KEYPAD_SIZE: usize  =  16;
pub const DISPLAY_WIDTH: usize  =  64;
pub const DISPLAY_HEIGHT: usize  =  32;

pub const CHIP8_FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0,		
	0x20, 0x60, 0x20, 0x20, 0x70,		
	0xF0, 0x10, 0xF0, 0x80, 0xF0,		
	0xF0, 0x10, 0xF0, 0x10, 0xF0,		
	0x90, 0x90, 0xF0, 0x10, 0x10,		
	0xF0, 0x80, 0xF0, 0x10, 0xF0,		
	0xF0, 0x80, 0xF0, 0x90, 0xF0,		
	0xF0, 0x10, 0x20, 0x40, 0x40,		
	0xF0, 0x90, 0xF0, 0x90, 0xF0,		
	0xF0, 0x90, 0xF0, 0x10, 0xF0,		
	0xF0, 0x90, 0xF0, 0x90, 0x90,		
	0xE0, 0x90, 0xE0, 0x90, 0xE0,		
	0xF0, 0x80, 0x80, 0x80, 0xF0,		
	0xE0, 0x90, 0x90, 0x90, 0xE0,		
	0xF0, 0x80, 0xF0, 0x80, 0xF0,		
	0xF0, 0x80, 0xF0, 0x80, 0x80
];

pub struct Chip8 {
    pub memory: [u8; MEMORY_SIZE],
    pub v: [u8; NUM_REGISTERS],
    pub i: u16,
    pub pc: u16,
    pub stack: [u16; STACK_SIZE],
    pub sp: u8,
    pub delay_timer: u8,
    pub sound_timer: u8,
    pub display: [u8; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    pub keypad: [u8; KEYPAD_SIZE],
}

impl Chip8 {
    fn handle_unknown_opcode(&mut self, opcode: u16) {
        println!("Ошибка: неизвестный или нереализованный опкод: {:#04X}", opcode);
        self.pc += 2;
    }

    pub fn new() -> Self {
        let mut memory = [0u8; MEMORY_SIZE];

        memory[0..80].copy_from_slice(&CHIP8_FONTSET);

        Chip8 {
            pc: 0x200,
            i: 0,
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            memory: memory,
            v: [0; NUM_REGISTERS],
            stack: [0; STACK_SIZE],
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            keypad: [0; KEYPAD_SIZE],
        }
    }

    pub fn chip8_load_rom(&mut self, path: &str) {
        const PROGRAM_START: usize = 0x200;

        let Ok(data) = fs::read(path) else {
            println!("Ошибка: невозможно прочитать файл {path}");
            return;
        };

        if data.len() > (MEMORY_SIZE - 0x200) {
            println!("Ошибка: ROM файл превышает допустимый размер");
            return;
        }

        let end_address = PROGRAM_START + data.len();

        self.memory[PROGRAM_START .. end_address].copy_from_slice(&data); 

        println!("ROM '{}' ({} bytes) успешно загружен в память", path, data.len());
    }

    pub fn chip8_emulate_cycle(&mut self) {
        let pc = self.pc as usize;
        let hi = self.memory[pc] as u16;  
        let lo = self.memory[pc + 1] as u16;

        let opcode = (hi << 8) | lo;

        let x   = ((opcode & 0x0F00) >> 8) as usize; 
        let y   = ((opcode & 0x00F0) >> 4) as usize; 
        let n   = (opcode & 0x000F) as u8;
        let nn  = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (opcode & 0xF000) >> 12 {
            0x0 => match nn {
                0xE0 => { 
                    self.display.fill(0);
                    self.pc += 2;
                },
                0xEE => { 
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                },
                _ => {
                    
                },
            },
            0x1 => self.pc = nnn, 
            0x2 => { 
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            },
            0x3 => { 
                if self.v[x] == nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x4 => {
                if self.v[x] != nn {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x5 => {
                if self.v[x] == self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0x6 => { 
                self.v[x] = nn;
                self.pc += 2;
            },
            0x7 => { 
                self.v[x] = self.v[x].wrapping_add(nn);
                self.pc += 2;
            },
            0x8 => { 
                match n {
                    0x0 => { 
                        self.v[x] = self.v[y];
                        self.pc += 2;
                    },
                    0x1 => { 
                        self.v[x] |= self.v[y];
                        self.pc += 2;
                    },
                    0x2 => {
                        self.v[x] &= self.v[y];
                        self.pc += 2;
                    },
                    0x3 => {
                        self.v[x] ^= self.v[y];
                        self.pc += 2;
                    },
                    0x4 => {
                        let (sum, carry) = self.v[x].overflowing_add(self.v[y]);
                        self.v[x] = sum;
                        self.v[0xF] = carry as u8;
                        self.pc += 2;
                    },
                    0x5 => { 
                        let (result, borrow) = self.v[x].overflowing_sub(self.v[y]);
                        self.v[x] = result;
                        self.v[0xF] = if borrow { 0 } else { 1 }; 
                        self.pc += 2;
                    },
                    0x6 => {
                        self.v[0xF] = self.v[x] & 0x1;
                        self.v[x] >>= 1;
                        self.pc += 2;
                    },
                    0x7 => {
                        let (result, borrow) = self.v[y].overflowing_sub(self.v[x]);
                        self.v[x] = result;
                        self.v[0xF] = if borrow { 0 } else { 1 }; 
                        self.pc += 2;
                    },
                    0xE => {
                        self.v[0xF] = (self.v[x] >> 7) & 0x1;
                        self.v[x] <<= 1;
                        self.pc += 2;
                    }
                    _ => {
                        self.handle_unknown_opcode(opcode);
                    },
                }
            },
            0x9 => {
                if self.v[x] != self.v[y] {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            },
            0xA => { 
                self.i = nnn;
                self.pc += 2;
            },
            0xB => self.pc = nnn + (self.v[0] as u16),
            0xC => {
                let mut rng = rand::rng();
                let random_byte: u8 = rng.random();
                self.v[x] = random_byte & nn;
                self.pc += 2;
            },
            0xD => {
                self.v[0xF] = 0;

                let x_coord = self.v[x] as usize;
                let y_coord = self.v[y] as usize;
                let height = n as usize;

                for row in 0..height {
                    let sprite_byte = self.memory[self.i as usize + row];
                    let screen_y = (y_coord + row) % DISPLAY_HEIGHT;

                    for bit in 0..8 {
                        let screen_x = (x_coord + bit) % DISPLAY_WIDTH;

                        if (sprite_byte & (0x80 >> bit)) != 0 {
                            let pixel_index = screen_y * DISPLAY_WIDTH +screen_x;

                            if self.display[pixel_index] == 1 {
                                self.v[0xF] = 1;
                            }

                            self.display[pixel_index] ^= 1;
                        }
                    }
                }
                self.pc += 2;
            },
            0xE => {
                match nn {
                    0x9E => {
                        let key_index = self.v[x] as usize;
                        if self.keypad[key_index] == 1 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    0xA1 => {
                        let key_index = self.v[x] as usize;
                        if self.keypad[key_index] == 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    },
                    _ => {
                        self.handle_unknown_opcode(opcode);
                    },
                }
            },
            0xF => {
                match nn {
                    0x07 => {
                        self.v[x] = self.delay_timer;
                        self.pc += 2;
                    },
                    0x0A => {
                        let mut key_pressed = false;
                        for i in 0..KEYPAD_SIZE {
                            if self.keypad[i] != 0 {
                                self.v[x] = i as u8;
                                key_pressed = true;
                                break; 
                            }
                        }

                        if key_pressed {
                            self.pc += 2; 
                        }
                    },
                    0x15 => {
                        self.delay_timer = self.v[x];
                        self.pc += 2;
                    },
                    0x18 => {
                        self.sound_timer = self.v[x];
                        self.pc += 2;
                    },
                    0x1E => {
                        self.i += self.v[x] as u16;
                        self.pc += 2;
                    },
                    0x29 => {
                        self.i = (self.v[x] as u16) * 5;
                        self.pc += 2;
                    },
                    0x33 => {
                        let value = self.v[x];
                        let i = self.i as usize;
                        self.memory[i]     = value / 100;        
                        self.memory[i + 1] = (value / 10) % 10;  
                        self.memory[i + 2] = value % 10;         
                        self.pc += 2;
                    },
                    0x55 => {
                        let i_addr = self.i as usize;
                        for i in 0..=x { 
                            self.memory[i_addr + i] = self.v[i];
                        }
                        self.pc += 2;
                    },
                    0x65 => {
                        let i_addr = self.i as usize;
                        for i in 0..=x { 
                            self.v[i] = self.memory[i_addr + i];
                        }
                        self.pc += 2;
                    },
                    _ => {
                        self.handle_unknown_opcode(opcode);
                    },
                }
            }
            _ => {
                self.handle_unknown_opcode(opcode);
            },
        }
    }
}
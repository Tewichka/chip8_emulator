pub fn disassemble(opcode: u16) -> String {   
    let x   = ((opcode & 0x0F00) >> 8) as usize; 
    let y   = ((opcode & 0x00F0) >> 4) as usize; 
    let n   = (opcode & 0x000F) as u8;
    let nn  = (opcode & 0x00FF) as u8;
    let nnn = opcode & 0x0FFF;

    match (opcode & 0xF000) >> 12 {
        0x0 => match nn {
            0xE0 => "CLS".to_string(),
            0xEE => "RET".to_string(),
            _    => format!("SYS  {:#05X}", nnn),
        },
        0x1 => format!("JP   {:#05X}", nnn),
        0x2 => format!("CALL {:#05X}", nnn),
        0x3 => format!("SE   V{:X}, {:#04X}", x, nn),
        0x4 => format!("SNE  V{:X}, {:#04X}", x, nn),
        0x5 => format!("SE   V{:X}, V{:X}", x, y),
        0x6 => format!("LD   V{:X}, {:#04X}", x, nn),
        0x7 => format!("ADD  V{:X}, {:#04X}", x, nn),
        0x8 => match n {
            0x0 => format!("LD   V{:X}, V{:X}", x, y),
            0x1 => format!("OR   V{:X}, V{:X}", x, y),
            0x2 => format!("AND  V{:X}, V{:X}", x, y),
            0x3 => format!("XOR  V{:X}, V{:X}", x, y),
            0x4 => format!("ADD  V{:X}, V{:X}", x, y),
            0x5 => format!("SUB  V{:X}, V{:X}", x, y),
            0x6 => format!("SHR  V{:X}", x),
            0x7 => format!("SUBN V{:X}, V{:X}", x, y),
            0xE => format!("SHL  V{:X}", x),
            _   => format!("UNKNOWN 8.."),
        },
        0x9 => format!("SNE  V{:X}, V{:X}", x, y),
        0xA => format!("LD   I, {:#05X}", nnn),
        0xB => format!("JP   V0, {:#05X}", nnn),
        0xC => format!("RND  V{:X}, {:#04X}", x, nn),
        0xD => format!("DRW  V{:X}, V{:X}, {:X}", x, y, n),
        0xE => match nn {
            0x9E => format!("SKP  V{:X}", x),
            0xA1 => format!("SKNP V{:X}", x),
            _    => format!("UNKNOWN E.."),
        },
        0xF => match nn {
            0x07 => format!("LD   V{:X}, DT", x),
            0x0A => format!("LD   V{:X}, K", x),
            0x15 => format!("LD   DT, V{:X}", x),
            0x18 => format!("LD   ST, V{:X}", x),
            0x1E => format!("ADD  I, V{:X}", x),
            0x29 => format!("LD   F, V{:X}", x),
            0x33 => format!("LD   B, V{:X}", x),
            0x55 => format!("LD   [I], V{:X}", x),
            0x65 => format!("LD   V{:X}, [I]", x),
            _    => format!("UNKNOWN F.."),
        },
        _ => "UNKNOWN".to_string(),
    }
}
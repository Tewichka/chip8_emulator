# Rust CHIP-8 Emulator

A simple yet powerful CHIP-8 emulator written in Rust with a real-time debugger GUI.

| Gameplay | Debugger View |
| :---: | :---: |
| <img width="480" alt="Emulator Screenshot" src="https://github.com/user-attachments/assets/384a2947-33c3-481a-8b05-b50fc4be2d31"> | <img width="480" alt="Debugger Screenshot" src="https://github.com/user-attachments/assets/96e4ee22-c5b4-4c86-99d6-e07b6e208722"> |

## Features

- **Full Emulation:** Implements all 35 CHIP-8 opcodes.
- **Debugger:** Live disassembler and register view.
- **Execution Control:** Pause, resume, and step-by-step instruction execution.
- **GUI:** Built with Rust's native `egui` framework.

## How to Run

1. **Clone the repository:**
   ```sh
   git clone https://github.com/Tewichka/chip8_emulator.git
   cd chip8_emulator
   cargo run --release -- path/to/your/rom.ch8

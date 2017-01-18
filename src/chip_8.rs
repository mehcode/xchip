use std::vec::Vec;
use opcode::Opcode;
use mmu::Mmu;
use interpreter::{Runtime, Context};

#[derive(Default)]
pub struct Chip8 {
}

impl Runtime for Chip8 {
    fn execute(&mut self, c: &mut Context, m: &mut Mmu, opcode: Opcode) -> bool {
        match opcode.unwrap() {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear the screen
                for dot in &mut c.screen {
                    *dot = 0;
                }
            }

            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                // Return from a subroutine
                c.pc = c.stack_pop(m) as usize;
            }

            _ => {
                // Unhandled operation
                return false;
            }
        }

        true
    }
}

// TODO: Remove
#![allow(dead_code)]

use std::vec::Vec;
use opcode::Opcode;
use interpreter::{Runtime, Context};

#[derive(Default)]
pub struct Chip8 {
}

impl Runtime for Chip8 {
    fn initialize(&mut self) {
        // The standard screen size is 64x32
        self.screen_width = 64;
        self.screen_height = 32;
        self.screen.resize(self.screen_width * self.screen_height);
    }

    fn reset(&mut self) {}

    fn execute(&mut self, c: &mut Context, opcode: Opcode) -> bool {
        match opcode.unwrap() {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear the screen
                for dot in &mut c.screen {
                    *dot = 0;
                }
            }

            _ => {
                // Unhandled operation
                return false;
            }
        }

        true
    }
}

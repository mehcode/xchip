use std::vec::Vec;
use opcode::Opcode;
use mmu::Mmu;
use interpreter::{Runtime, Context};
use rand::random;
use axal;
use axal::Key;

// CHIP-8 hex keyboard -> modern keyboard
const KEYBOARD_MAP: [Key; 0x10] = [Key::X, Key::Num1, Key::Num2, Key::Num3, Key::Q, Key::W,
                                   Key::E, Key::A, Key::S, Key::D, Key::Z, Key::C, Key::Num4,
                                   Key::R, Key::F, Key::V];

#[derive(Default)]
pub struct Chip8 {
}

impl Runtime for Chip8 {
    fn execute(&mut self,
               r: &mut axal::Runtime,
               c: &mut Context,
               m: &mut Mmu,
               opcode: Opcode)
               -> bool {
        match opcode.unwrap() {
            // CLS
            (0x0, 0x0, 0xE, 0x0) => {
                // Clear the screen
                for dot in &mut c.screen {
                    *dot = false;
                }
            }

            // RET
            (0x0, 0x0, 0xE, 0xE) => {
                // Return from a subroutine
                c.pc = c.stack_pop(m) as usize;
            }

            // JP u12
            (0x1, ..) => {
                // Jump to u12
                c.pc = opcode.extract_u12() as usize;
            }

            // CALL u12
            (0x2, ..) => {
                // Call subroutine at u12
                let pc = c.pc;
                c.stack_push(m, pc as u16);

                c.pc = opcode.extract_u12() as usize;
            }

            // SE Vx, u8
            (0x3, x, ..) => {
                // Skip next instruction if Vx == u8
                if c.v[x as usize] == opcode.extract_u8() {
                    c.pc = c.pc.wrapping_add(2);
                }
            }

            // SNE Vx, u8
            (0x4, x, ..) => {
                // Skip next instruction if Vx != u8
                if c.v[x as usize] != opcode.extract_u8() {
                    c.pc = c.pc.wrapping_add(2);
                }
            }

            // SE Vx, Vy
            (0x5, x, y, _) => {
                // Skip next instruction if Vx == Vy
                if c.v[x as usize] == c.v[y as usize] {
                    c.pc = c.pc.wrapping_add(2);
                }
            }

            // LD Vx, u8
            (0x6, x, ..) => {
                // Set Vx = u8
                c.v[x as usize] = opcode.extract_u8();
            }

            // ADD Vx, u8
            (0x7, x, ..) => {
                // Set Vx = Vx + u8
                c.v[x as usize] = c.v[x as usize].wrapping_add(opcode.extract_u8());
            }

            // LD Vx, Vy
            (0x8, x, y, 0x0) => {
                // Set Vx = Vy
                c.v[x as usize] = c.v[y as usize];
            }

            // OR Vx, Vy
            (0x8, x, y, 0x1) => {
                // Set Vx = Vx OR Vy
                c.v[x as usize] |= c.v[y as usize];
            }

            // AND Vx, Vy
            (0x8, x, y, 0x2) => {
                // Set Vx = Vx AND Vy
                c.v[x as usize] &= c.v[y as usize];
            }

            // XOR Vx, Vy
            (0x8, x, y, 0x3) => {
                // Set Vx = Vx XOR Vy
                c.v[x as usize] ^= c.v[y as usize];
            }

            // ADD Vx, Vy
            (0x8, x, y, 0x4) => {
                // Set Vx = Vx + Vy; Set VF = <carry>
                let vx = c.v[x as usize] as u16;
                let vy = c.v[y as usize] as u16;
                let r = vx + vy;

                c.v[x as usize] = r as u8;
                c.v[0xF] = (r > 0xFF) as u8;
            }

            // SUB Vx, Vy
            (0x8, x, y, 0x5) => {
                // Set Vx = Vx - Vy; Set VF = !<borrow>
                let vx = c.v[x as usize];
                let vy = c.v[y as usize];

                c.v[0xF] = (!(vy > vx)) as u8;
                c.v[x as usize] = vx.wrapping_sub(vy);
            }

            // SHR Vx
            (0x8, x, _, 0x6) => {
                // Set Vx = Vx SHR 1; Set VF = Vx BIT 1
                c.v[0xF] = c.v[x as usize] & 1;
                c.v[x as usize] >>= 1;
            }

            // SUBN Vx, Vy
            (0x8, x, y, 0x7) => {
                // Set Vx = Vy - Vx; Set VF = !<borrow>
                let vx = c.v[x as usize];
                let vy = c.v[y as usize];

                c.v[0xF] = (!(vx > vy)) as u8;
                c.v[x as usize] = vy.wrapping_sub(vx);
            }

            // SHL Vx, Vy
            (0x8, x, y, 0xE) => {
                // Set Vx = Vy SHL 1; Set VF = Vy BIT 7
                c.v[0xF] = c.v[y as usize] >> 7;
                c.v[x as usize] = c.v[y as usize] << 1;
            }

            // SNE Vx, Vy
            (0x9, x, y, 0) => {
                // Skip next instruction if Vx != Vy
                if c.v[x as usize] != c.v[y as usize] {
                    c.pc = c.pc.wrapping_add(2);
                }
            }

            // LD I, u12
            (0xA, ..) => {
                // Set I = u12
                c.i = opcode.extract_u12() as usize;
            }

            // JP V0, u12
            (0xB, ..) => {
                // Jump to u12 + V0
                c.pc = (opcode.extract_u12().wrapping_add(c.v[0] as u16)) as usize;
            }

            // RND Vx, u8
            (0xC, x, ..) => {
                // Set Vx = <random u8> AND u8
                c.v[x as usize] = random::<u8>() & opcode.extract_u8();
            }

            // SHOW Vx, Vy, u4
            (0xD, x, y, n) => {
                // Display n-byte sprite starting in memory at I at (Vx, Vy)
                // Set VF = <collision>

                let x = c.v[x as usize] as usize;
                let y = c.v[y as usize] as usize;

                // VF is cleared at the start of DRW so collision can be set easily
                c.v[0xF] = 0;

                for i in 0..(n as usize) {
                    let sy = (y + i) % c.screen_height;

                    for j in 0..8 {
                        let sx = (x + j) % c.screen_width;

                        // Get VRAM offset
                        let offset = sy * c.screen_width + sx;

                        // Get _current_ dot in the screen
                        let dot = &mut c.screen[offset];
                        let was_set = *dot;

                        // Read memory to get the _set_ value
                        let dot_set = (m.read(c.i + i) >> (7 - j)) & 1;

                        // XOR to determine the new state of the dot
                        *dot = ((*dot as u8) ^ dot_set) != 0;

                        // VF is set to indicate the transition 1 -> 0
                        c.v[0xF] |= (was_set && !*dot) as u8;
                    }
                }
            }

            // SKP Vx
            (0xE, x, 0x9, 0xE) => {
                // Skip next instruction if key with the value of Vx is pressed
                if r.input_keyboard_state(0, KEYBOARD_MAP[c.v[x as usize] as usize]) {
                    c.pc = c.pc.wrapping_add(2);
                }
            }

            // SKNP Vx
            (0xE, x, 0xA, 0x1) => {
                // Skip next instruction if key with the value of Vx is not pressed
                if !r.input_keyboard_state(0, KEYBOARD_MAP[c.v[x as usize] as usize]) {
                    c.pc = c.pc.wrapping_add(2);
                }
            }

            // LD Vx, DT
            (0xF, x, 0x0, 0x7) => {
                // Set Vx = DT
                c.v[x as usize] = c.dt;
            }

            // LD DT, Vx
            (0xF, x, 0x1, 0x5) => {
                // Set DT = Vx
                c.dt = c.v[x as usize];
            }

            // LD ST, Vx
            (0xF, x, 0x1, 0x8) => {
                // Set ST = Vx
                c.st = c.v[x as usize];
            }

            // ADD I, Vx
            (0xF, x, 0x1, 0xE) => {
                // Set I = I + Vx
                let r: u32 = c.i as u32 + c.v[x as usize] as u32;

                c.i = (r & 0xFFF) as usize;

                // If buffer overflow, register > VF must be set to 1, otherwise 0.
                c.v[0xF] = (r > 0xFFF) as u8;
            }

            // LD [I], FONT Vx
            (0xF, x, 0x2, 0x9) => {
                // Set I = location of sprite for digit Vx.
                c.i = (c.v[x as usize] * 5) as usize;
            }

            // LD [I], BCD Vx
            (0xF, x, 0x3, 0x3) => {
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                let r = c.v[x as usize];
                let i = c.i;

                m.write(i, r / 100);
                m.write(i + 1, (r % 100) / 10);
                m.write(i + 2, r % 10);
            }

            // LD [I], Vx
            (0xF, x, 0x5, 0x5) => {
                // Store registers V0 through Vx in memory starting at location I.
                for j in 0..(x + 1) {
                    let r = c.v[j as usize];

                    m.write(c.i, r);

                    c.i += 1;
                }
            }

            // LD Vx, [I]
            (0xF, x, 0x6, 0x5) => {
                // Read registers V0 through Vx from memory starting at location I.
                for j in 0..(x + 1) {
                    c.v[j as usize] = m.read(c.i);

                    c.i += 1;
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

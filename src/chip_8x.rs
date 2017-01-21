use std::vec::Vec;

use chip_8;
use opcode::Opcode;
use mmu::Mmu;
use interpreter::{Runtime, Context};
use axal;

#[derive(Default)]
pub struct Chip8x {
    // CHIP-8X starts from the CHIP-8
    chip_8: chip_8::Chip8,

    // Maps CHIP-8X color index to a R3_G3_B2 color
    // TODO: Make this configurable
    palette: Vec<u8>,

    // CHIP-8X defines a background color (color when dots are off)
    background_color: u8,

    // CHIP-8X defines a color "lens" that defines
    // the colors of dots from the screen
    color_lens: Vec<u8>,
}

impl Runtime for Chip8x {
    fn configure(&mut self, c: &mut Context) {
        // Initialize palette
        self.palette = vec![// Black
                            0b000_000_00,

                            // Red
                            0b111_000_00,

                            // Blue
                            0b000_000_11,

                            // Violet
                            0b111_000_11,

                            // Green
                            0b000_111_00,

                            // Yellow
                            0b111_111_00,

                            // Aqua
                            0b000_111_11,

                            // White
                            0b111_111_11];

        // Initialize the color lens
        self.color_lens.resize(c.screen.len(), self.palette[7]);
    }

    fn reset(&mut self, c: &mut Context) {
        // Reset CHIP-8
        self.chip_8.reset(c);

        // Reset background color
        self.background_color = self.palette[0];

        // Clear the color lens
        for c in &mut self.color_lens {
            *c = self.palette[7];
        }

        // Set PC to $300
        c.pc = 0x300;
    }

    fn insert_rom(&mut self, m: &mut Mmu, buffer: &[u8]) {
        m.write_all(0x300, buffer);
    }

    fn update_framebuffer(&mut self, c: &mut Context) {
        c.framebuffer.resize(c.screen.len(), 0);

        for y in 0..c.screen_height {
            let offset_y = y * c.screen_width;

            for x in 0..c.screen_width {
                let offset = offset_y + x;

                // Get dot from screen
                let dot = c.screen[offset];

                // Blit to framebuffer
                c.framebuffer[offset] = if dot {
                    self.color_lens[offset]
                } else {
                    self.background_color
                };
            }
        }
    }

    fn execute(&mut self,
               r: &mut axal::Runtime,
               c: &mut Context,
               m: &mut Mmu,
               opcode: Opcode)
               -> bool {
        match opcode.unwrap() {
            (0x0, 0x2, 0xA, 0x0) => {
                println!("unimplemented: {}", opcode);
            }

            (0x5, x, y, 0x1) => {
                println!("unimplemented: {}", opcode);
            }

            (0xB, x, y, 0x0) => {
                // Set foreground color of 1 or more 8x4 dot zones
                let vx = c.v[x as usize];
                let vx1 = c.v[(x + 1) as usize];
                let color = c.v[y as usize] & 0b1111;

                // The lower 4 bits of `VX`/`V[X+1]` is the horizontal/vertical zone index (0-7).
                let horz = (vx & 0b1111) as usize;
                let vert = (vx1 & 0b1111) as usize;

                // The upper 4 bits of `VX`/`V[X+1]` is the horizontal/vertical size minus 1.
                let width = (vx >> 4) + 1;
                let height = (vx1 >> 4) + 1;

                // TODO: Set color on the color lens
            }

            (0xB, x, y, n) => {
                // Set foreground color of 1 or more 8x1 dot zones
                println!("unimplemented: {}", opcode);
            }

            _ => {
                // Unhandled by CHIP-8X
                return self.chip_8.execute(r, c, m, opcode);
            }
        }

        true
    }
}

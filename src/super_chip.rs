use std::cmp;

use chip_8;
use opcode::Opcode;
use mmu::Mmu;
use interpreter::{Runtime, Context};

#[derive(PartialEq)]
enum DisplayMode {
    Standard,
    Extended,
}

impl Default for DisplayMode {
    fn default() -> Self {
        DisplayMode::Standard
    }
}

#[derive(Default)]
pub struct SuperChip {
    // SUPER-CHIP starts from the CHIP-8
    chip_8: chip_8::Chip8,

    // Adds scratch storage for up to 8 general (V) registers
    // NOTE: The limitation comes from the original SUPER-CHIP's usage
    //       of the HP48 RPL user flags for the storage (of which there
    //       were only 8).
    v_scratch: [u8; 8],

    // SUPER-CHIP can access the display as 128x64 dots or 64x32 2x2 dot regions
    mode: DisplayMode,
}

impl Runtime for SuperChip {
    fn configure(&mut self, c: &mut Context) {
        // Increase screen size to 128x64
        c.screen_width = 128;
        c.screen_height = 64;
        c.screen.resize(c.screen_width * c.screen_height, Default::default());
    }

    fn reset(&mut self) {
        // Reset CHIP-8
        self.chip_8.reset();

        // Clear scratch storage
        for v in &mut self.v_scratch {
            *v = 0;
        }

        // Reset display mode to Standard
        self.mode = DisplayMode::Standard;
    }

    fn execute(&mut self, c: &mut Context, m: &mut Mmu, opcode: Opcode) -> bool {
        match opcode.unwrap() {
            // SCDOWN
            (0x0, 0x0, 0xC, n) => {
                // Scroll screen N lines down
                // NOTE: This always operates on a 128x64 display regardless of the active mode
                unimplemented!();
            }

            // SCRIGHT
            (0x0, 0x0, 0xF, 0xB) => {
                // Scroll screen 4 dots right
                // NOTE: This always operates on a 128x64 display regardless of the active mode
                unimplemented!();
            }

            // SCLEFT
            (0x0, 0x0, 0xF, 0xC) => {
                // Scroll screen 4 dots left
                // NOTE: This always operates on a 128x64 display regardless of the active mode
                unimplemented!();
            }

            // SED
            // TODO: If you have a better idea for a mnemonic; a PR would be appreciated
            (0x0, 0x0, 0xF, 0xE) => {
                // Set extended display mode
                self.mode = DisplayMode::Extended;
            }

            // CLD
            // TODO: If you have a better idea for a mnemonic; a PR would be appreciated
            (0x0, 0x0, 0xF, 0xF) => {
                // Clear extended display mode (revert to standard)
                self.mode = DisplayMode::Standard;
            }

            // SHOW16 Vx, Vy
            (0xD, _, _, 0x0) if self.mode == DisplayMode::Extended => {
                // Show 16x16 sprite from [I] at coordinates (Vx, Vy); VF := collision
                unimplemented!();
            }

            // SHOW Vx, Vy, N
            (0xD, _, _, _) => {
                // Show 8x8 sprite from [I] at coordinates (Vx, Vy); VF := collision
                // NOTE: This must be re-implemented from CHIP-8 because in standard display mode
                //       2x2 dot blocks are shown instead of single dots
                unimplemented!();
            }

            // LD I, FONT10 Vx
            (0xF, _, 0x3, 0x0) => {
                // Point I to 10-byte font sprite for digit Vx
                unimplemented!();
            }

            // SAVE Vx .. Vy
            (0xF, x, 0x7, 0x5) => {
                // Store V0..Vx into private interpreter memory; at most 8
                //  registers can be stored
                for i in 0..cmp::min(x as usize, 7) {
                    self.v_scratch[i] = c.v[i];
                }
            }

            // RESTORE Vx .. Vy
            (0xF, x, 0x8, 0x5) => {
                // Restore V0..Vx from private interpreter memory; at most 8
                //  registers can be restored
                for i in 0..cmp::min(x as usize, 7) {
                    c.v[i] = self.v_scratch[i];
                }
            }

            _ => {
                // Unhandled by SUPER-CHIP
                return self.chip_8.execute(c, m, opcode);
            }
        }

        true
    }
}

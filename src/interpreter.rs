// use std::vec::Vec;
// use std::cmp;
// use std::time::Instant;
// use axal::{Runtime, Key};
// use rand::random;

// const SCREEN_STANDARD_WIDTH: usize = 64;
// const SCREEN_STANDARD_HEIGHT: usize = 32;

// const SCREEN_EXTENDED_WIDTH: usize = 128;
// const SCREEN_EXTENDED_HEIGHT: usize = 64;

use axal;

use chip_8;
use opcode::Opcode;
use mmu;

#[derive(Default)]
pub struct Context {
    // General registers (16x 8-bit)
    pub v: [u8; 16],

    // Index (12-bit)
    //  This can be loaded with a 12-bit (all), 16-bit (XO-CHIP), and 24-bit (MEGA-CHIP) address.
    //  Wrap around is handled when accessing RAM (depending on its size).
    pub i: usize,

    // Program Counter
    pub pc: usize,

    // Stack Pointer
    pub sp: usize,
    pub stack_len: usize,

    // Display buffer (screen) and active resolution
    //  screen.len() == screen_width * screen_height
    pub screen: Vec<u8>,
    pub screen_width: usize,
    pub screen_height: usize,
}

pub trait Runtime {
    // Execute passed operation; return false if unhandled
    fn execute(&mut self, c: &mut Context, opcode: Opcode) -> bool;
}

#[derive(Default)]
pub struct Interpreter {
    // Shared context used by all variants
    context: Context,

    // Memory management unit (incl. RAM)
    mmu: mmu::Mmu,

    // Active runtime (CHIP-8, CHIP-8X, etc.)
    runtime: Option<Box<Runtime>>,
}

impl Interpreter {
    pub fn insert_rom(&mut self, buffer: &[u8]) {
        self.mmu.write_all(0x200, buffer);
    }

    pub fn remove_rom(&mut self) {
        self.mmu.clear();
    }

    pub fn reset(&mut self) {}

    pub fn run_next(&mut self, r: &mut axal::Runtime) {}
}

// pub enum Interpreter {
//     // CHIP-8
//     Chip8(chip_8::Chip8),

//     // HIRES CHIP-8
//     HiResChip8,

//     // CHIP-10
//     Chip10,

//     // CHIP-8X
//     Chip8x,

//     // SUPER-CHIP
//     SuperChip,

//     // MEGA-CHIP
//     MegaChip,

//     // XO-CHIP
//     XoChip,
// }

// #[derive(PartialEq)]
// enum ScreenMode {
//     // Standard 32x64 mode
//     Standard = 0,

//     // HIRES 64x64 mode for CHIP-8
//     HiRes,

//     // Extended 64x128 mode for SUPERCHIP-8
//     Extended,
// }

// impl Default for ScreenMode {
//     fn default() -> Self {
//         ScreenMode::Standard
//     }
// }

// // CHIP-8 hex keyboard -> modern keyboard
// const KEYBOARD_MAP: [Key; 0x10] = [Key::X, Key::Num1, Key::Num2, Key::Num3, Key::Q, Key::W,
//                                    Key::E, Key::A, Key::S, Key::D, Key::Z, Key::C, Key::Num4,
//                                    Key::R, Key::F, Key::V];

// pub struct Opcode {
//     hi: u8,
//     lo: u8,
// }

// impl Opcode {
//     #[inline]
//     fn new(hi: u8, lo: u8) -> Opcode {
//         Opcode { hi: hi, lo: lo }
//     }

//     #[inline]
//     fn unpack(&self) -> (u8, u8, u8, u8) {
//         ((self.hi >> 4), (self.hi & 0xF), (self.lo >> 4), (self.lo & 0xF))
//     }

//     #[inline]
//     fn as_u12(&self) -> u16 {
//         (self.lo as u16) | (((self.hi & 0xF) as u16) << 8)
//     }

//     #[inline]
//     fn as_u8(&self) -> u8 {
//         self.lo
//     }
// }

// // 0.05 = 20 cycle decay
// // 0.1  = 10 cycle decay
// // 0.2  =  5 cycle decay
// // 0.5  =  2 cycle decay
// const PHASE_TICK: f32 = 0.05;

// #[derive(Default, Clone, Copy)]
// struct Pixel {
//     // On/Off
//     lit: bool,

//     // Phase [0.0, 1.0]
//     //  0.1 would be 10% opacity if lit is true and 90% opacity if lit is false
//     phase: f32,
// }

// #[derive(Default)]
// pub struct Interpreter {
//     // RAM; 4 KiB
//     ram: Vec<u8>,

//     // Mode of the screen
//     screen_mode: ScreenMode,

//     // Size of the Screen
//     width: usize,
//     height: usize,

//     // Screen; 64x32
//     //  Each pixel in the CHIP-8 screen is 1/0 and is XOR'd when drawn
//     //  When a pixel is turned off its dimmed at a set rate-per-cycle instead of immediately going out
//     screen: Vec<Pixel>,

//     // Frame buffer; 64x32 (in R3_B3_G2 format)
//     //  Stores the RGB values for the current frame
//     //  This is updated _once_ per frame
//     framebuffer: Vec<u8>,

//     // General Registers (8-bit x 16)
//     //  v[0xF] is used as a _flags_ register by several instructions
//     v: [u8; 0x10],

//     // Scratch space for at most 8 general registers
//     v_scratch: [u8; 8],

//     // Index Register (12-bit)
//     i: u16,

//     // Program Counter (12-bit)
//     pc: u16,

//     // Stack Pointer (8-bit)
//     sp: u8,

//     // 60 Hz timer that controls DT / ST
//     timer_elapsed: u64,
//     timer_instant: Option<Instant>,

//     // Delay Timer (8-bit)
//     //  Decrements at a constant rate of 60 Hz
//     dt: u8,

//     // Sound Timer (8-bit)
//     //  Decrements at a constant rate of 60 Hz
//     //  Plays a tone as long as it is non-zero.
//     st: u8,
// }

// impl Interpreter {
//     pub fn take_rom(&mut self, mut rom: Vec<u8>) {
//         self.ram.clear();
//         self.ram.resize(0x200, 0);
//         self.ram.append(&mut rom);
//         self.ram.resize(0x1000, 0);
//     }

//     pub fn reset(&mut self) {
//         self.v = [0; 0x10];
//         self.v_scratch = [0; 8];
//         self.i = 0;
//         self.sp = 0;
//         self.pc = 0x200;
//         self.dt = 0;
//         self.st = 0;

//         self.screen_mode = ScreenMode::Standard;

//         self.width = SCREEN_STANDARD_WIDTH;
//         self.height = SCREEN_STANDARD_HEIGHT;

//         self.screen.clear();
//         self.screen.resize(self.width * self.height, Default::default());

//         self.timer_elapsed = 0;
//         self.timer_instant = None;

//         // TODO: There must be a cleaner way to load font sprites

//         self.ram[0x00] = 0xF0;
//         self.ram[0x01] = 0x90;
//         self.ram[0x02] = 0x90;
//         self.ram[0x03] = 0x90;
//         self.ram[0x04] = 0xF0;

//         self.ram[0x05] = 0x20;
//         self.ram[0x06] = 0x60;
//         self.ram[0x07] = 0x20;
//         self.ram[0x08] = 0x20;
//         self.ram[0x09] = 0x70;

//         self.ram[0x0A] = 0xF0;
//         self.ram[0x0B] = 0x10;
//         self.ram[0x0C] = 0xF0;
//         self.ram[0x0D] = 0x80;
//         self.ram[0x0E] = 0xF0;

//         self.ram[0x0F] = 0xF0;
//         self.ram[0x10] = 0x10;
//         self.ram[0x11] = 0xF0;
//         self.ram[0x12] = 0x10;
//         self.ram[0x13] = 0xF0;

//         self.ram[0x14] = 0x90;
//         self.ram[0x15] = 0x90;
//         self.ram[0x16] = 0xF0;
//         self.ram[0x17] = 0x10;
//         self.ram[0x18] = 0x10;

//         self.ram[0x19] = 0xF0;
//         self.ram[0x1A] = 0x80;
//         self.ram[0x1B] = 0xF0;
//         self.ram[0x1C] = 0x10;
//         self.ram[0x1D] = 0xF0;

//         self.ram[0x1E] = 0xF0;
//         self.ram[0x1F] = 0x80;
//         self.ram[0x20] = 0xF0;
//         self.ram[0x21] = 0x90;
//         self.ram[0x22] = 0xF0;

//         self.ram[0x23] = 0xF0;
//         self.ram[0x24] = 0x10;
//         self.ram[0x25] = 0x20;
//         self.ram[0x26] = 0x40;
//         self.ram[0x27] = 0x40;
//         self.ram[0x28] = 0xF0;
//         self.ram[0x29] = 0x90;
//         self.ram[0x2A] = 0xF0;
//         self.ram[0x2B] = 0x90;
//         self.ram[0x2C] = 0xF0;

//         self.ram[0x2D] = 0xF0;
//         self.ram[0x2E] = 0x90;
//         self.ram[0x2F] = 0xF0;
//         self.ram[0x30] = 0x10;
//         self.ram[0x31] = 0xF0;

//         self.ram[0x32] = 0xF0;
//         self.ram[0x33] = 0x90;
//         self.ram[0x34] = 0xF0;
//         self.ram[0x35] = 0x90;
//         self.ram[0x36] = 0x90;

//         self.ram[0x37] = 0xE0;
//         self.ram[0x38] = 0x90;
//         self.ram[0x39] = 0xE0;
//         self.ram[0x3A] = 0x90;
//         self.ram[0x3B] = 0xE0;

//         self.ram[0x3C] = 0xF0;
//         self.ram[0x3D] = 0x80;
//         self.ram[0x3E] = 0x80;
//         self.ram[0x3F] = 0x80;
//         self.ram[0x40] = 0xF0;

//         self.ram[0x41] = 0xE0;
//         self.ram[0x42] = 0x90;
//         self.ram[0x43] = 0x90;
//         self.ram[0x44] = 0x90;
//         self.ram[0x45] = 0xE0;

//         self.ram[0x46] = 0xF0;
//         self.ram[0x47] = 0x80;
//         self.ram[0x48] = 0xF0;
//         self.ram[0x49] = 0x80;
//         self.ram[0x4A] = 0xF0;

//         self.ram[0x4B] = 0xF0;
//         self.ram[0x4C] = 0x80;
//         self.ram[0x4D] = 0xF0;
//         self.ram[0x4E] = 0x80;
//         self.ram[0x4F] = 0x80;
//     }

//     fn push(&mut self, value: u16) {
//         // Increment Stack Pointer
//         self.sp = self.sp.wrapping_add(1);

//         // Write to Stack
//         let address = 0x100u16 + (self.sp as u16) * 2;

//         self.write(address, (value >> 8) as u8);
//         self.write(address + 1, (value & 0xFF) as u8);
//     }

//     fn pop(&mut self) -> u16 {
//         // Read from Stack
//         let address = 0x100u16 + (self.sp as u16) * 2;

//         let hi = self.read(address);
//         let lo = self.read(address + 1);

//         // Decrement Stack Pointer
//         self.sp = self.sp.wrapping_sub(1);

//         ((hi as u16) << 8) | (lo as u16)
//     }

//     fn write(&mut self, address: u16, value: u8) {
//         self.ram[(address & 0xFFF) as usize] = value;
//     }

//     fn read(&mut self, address: u16) -> u8 {
//         self.ram[(address & 0xFFF) as usize]
//     }

//     fn read_next(&mut self) -> u8 {
//         let address = self.pc;
//         let r = self.read(address);
//         self.pc = self.pc.wrapping_add(1);

//         r
//     }

//     pub fn screen_as_framebuffer(&mut self) -> (&[u8], usize, usize) {
//         // Blit screen onto framebuffer
//         self.framebuffer.resize(self.screen.len(), 0);
//         for y in 0..self.height {
//             let offset_y = y * self.width;

//             for x in 0..self.width {
//                 let offset = offset_y + x;

//                 // Get pixel from screen
//                 let pixel = self.screen[offset];

//                 // Blit to framebuffer
//                 self.framebuffer[offset] = if pixel.lit || pixel.phase < 1.0 {
//                     0xFF
//                 } else {
//                     0x00
//                 };
//             }
//         }

//         (&self.framebuffer, self.width, self.height)
//     }

//     // TODO: Refactor to remove this lint
//     #[allow(unknown_lints, cyclomatic_complexity)]
//     pub fn run_next(&mut self, r: &mut Runtime) {
//         // Adjust phase of any decaying pixels
//         for p in &mut self.screen {
//             if p.phase < 1.0 {
//                 p.phase += PHASE_TICK;
//             }
//         }

//         // If timer point reference is non-zero; check elapsed and
//         // clock ST / DT
//         if let Some(timer_instant) = self.timer_instant {
//             let elapsed = timer_instant.elapsed();
//             self.timer_elapsed += (elapsed.as_secs() * 1_000_000_000) +
//                                   (elapsed.subsec_nanos() as u64);

//             // 1/60 s => 16_666_666 ns
//             if self.timer_elapsed >= 16_666_666 {
//                 self.timer_elapsed -= 16_666_666;

//                 if self.dt > 0 {
//                     self.dt -= 1;
//                 }

//                 if self.st > 0 {
//                     self.st -= 1;
//                 }
//             }
//         }

//         // Read 16-bit opcode
//         let opcode = Opcode::new(self.read_next(), self.read_next());

//         // Unpack and decode instruction
//         match opcode.unpack() {
//             // CLS
//             (0x0, 0x0, 0xE, 0x0) => {
//                 // Clear 32x64 OR 64x128 (in extended mode only) of the screen
//                 let (h, w) = if self.screen_mode == ScreenMode::Extended {
//                     (self.height, self.width)
//                 } else {
//                     (SCREEN_STANDARD_HEIGHT, SCREEN_STANDARD_WIDTH)
//                 };

//                 for y in 0..h {
//                     let offset_y = y * w;
//                     for x in 0..w {
//                         self.screen[offset_y + x] = Default::default();
//                     }
//                 }
//             }

//             // HRCLS (HIRES CHIP-8)
//             (0x0, 0x2, 0x3, 0x0) => {
//                 // Clear 64x64 of the screen
//                 for y in 0..64 {
//                     let offset_y = (y % self.height) * 64;
//                     for x in 0..64 {
//                         self.screen[offset_y + (x % self.width)] = Default::default();
//                     }
//                 }
//             }

//             // SCROLL DOWN N (SUPER-CHIP)
//             (0x0, 0x0, 0xC, n) => {
//                 // Scroll screen N lines down
//                 let n = n as usize;

//                 if n > 0 {
//                     for y in (0..self.height).rev() {
//                         let dst_offset_y = y * self.width;
//                         let src_offset_y = if y < n { 0 } else { (y - n) * self.width };

//                         for x in 0..self.width {
//                             let dst_offset = dst_offset_y + x;

//                             if y < n {
//                                 self.screen[dst_offset] = Default::default();
//                             } else {
//                                 let src_offset = src_offset_y + x;

//                                 self.screen[dst_offset] = self.screen[src_offset];
//                             }
//                         }
//                     }
//                 }
//             }

//             // RET
//             (0x0, 0x0, 0xE, 0xE) => {
//                 // Return from a subroutine
//                 self.pc = self.pop();
//             }

//             // Disable extended screen mode (SUPER-CHIP)
//             // TODO: Define mnemonic
//             (0x0, 0x0, 0xF, 0xE) => {
//                 self.screen_mode = ScreenMode::Standard;

//                 self.width = SCREEN_STANDARD_WIDTH;
//                 self.height = SCREEN_STANDARD_HEIGHT;

//                 self.screen.resize(self.width * self.height, Default::default());
//             }

//             // Enable extended screen mode (SUPER-CHIP)
//             // TODO: Define mnemonic
//             (0x0, 0x0, 0xF, 0xF) => {
//                 self.screen_mode = ScreenMode::Extended;

//                 self.width = SCREEN_EXTENDED_WIDTH;
//                 self.height = SCREEN_EXTENDED_HEIGHT;

//                 self.screen.resize(self.width * self.height, Default::default());
//             }

//             // Ignore all other 0x0___ patterns and treat as NOP
//             // (0x0, ..) => {}
//             //
//             // JP u12
//             (0x1, ..) => {
//                 // Jump to u12
//                 self.pc = opcode.as_u12();
//             }

//             // CALL u12
//             (0x2, ..) => {
//                 // Call subroutine at u12
//                 let pc = self.pc;
//                 self.push(pc);

//                 self.pc = opcode.as_u12();
//             }

//             // SE Vx, u8
//             (0x3, x, ..) => {
//                 // Skip next instruction if Vx == u8
//                 if self.v[x as usize] == opcode.as_u8() {
//                     self.pc = self.pc.wrapping_add(2);
//                 }
//             }

//             // SNE Vx, u8
//             (0x4, x, ..) => {
//                 // Skip next instruction if Vx != u8
//                 if self.v[x as usize] != opcode.as_u8() {
//                     self.pc = self.pc.wrapping_add(2);
//                 }
//             }

//             // SE Vx, Vy
//             (0x5, x, y, _) => {
//                 // Skip next instruction if Vx == Vy
//                 if self.v[x as usize] == self.v[y as usize] {
//                     self.pc = self.pc.wrapping_add(2);
//                 }
//             }

//             // LD Vx, u8
//             (0x6, x, ..) => {
//                 // Set Vx = u8
//                 self.v[x as usize] = opcode.as_u8();
//             }

//             // ADD Vx, u8
//             (0x7, x, ..) => {
//                 // Set Vx = Vx + u8
//                 self.v[x as usize] = self.v[x as usize].wrapping_add(opcode.as_u8());
//             }

//             // LD Vx, Vy
//             (0x8, x, y, 0x0) => {
//                 // Set Vx = Vy
//                 self.v[x as usize] = self.v[y as usize];
//             }

//             // OR Vx, Vy
//             (0x8, x, y, 0x1) => {
//                 // Set Vx = Vx OR Vy
//                 self.v[x as usize] |= self.v[y as usize];
//             }

//             // AND Vx, Vy
//             (0x8, x, y, 0x2) => {
//                 // Set Vx = Vx AND Vy
//                 self.v[x as usize] &= self.v[y as usize];
//             }

//             // XOR Vx, Vy
//             (0x8, x, y, 0x3) => {
//                 // Set Vx = Vx XOR Vy
//                 self.v[x as usize] ^= self.v[y as usize];
//             }

//             // ADD Vx, Vy
//             (0x8, x, y, 0x4) => {
//                 // Set Vx = Vx + Vy; Set VF = <carry>
//                 let vx = self.v[x as usize] as u16;
//                 let vy = self.v[y as usize] as u16;
//                 let r = vx + vy;

//                 self.v[x as usize] = r as u8;
//                 self.v[0xF] = (r > 0xFF) as u8;
//             }

//             // SUB Vx, Vy
//             (0x8, x, y, 0x5) => {
//                 // Set Vx = Vx - Vy; Set VF = !<borrow>
//                 let vx = self.v[x as usize];
//                 let vy = self.v[y as usize];

//                 self.v[0xF] = (!(vy > vx)) as u8;
//                 self.v[x as usize] = vx.wrapping_sub(vy);
//             }

//             // SHR Vx
//             (0x8, x, _, 0x6) => {
//                 // Set Vx = Vx SHR 1; Set VF = Vx BIT 1
//                 self.v[0xF] = self.v[x as usize] & 1;
//                 self.v[x as usize] >>= 1;
//             }

//             // SUBN Vx, Vy
//             (0x8, x, y, 0x7) => {
//                 // Set Vx = Vy - Vx; Set VF = !<borrow>
//                 let vx = self.v[x as usize];
//                 let vy = self.v[y as usize];

//                 self.v[0xF] = (!(vx > vy)) as u8;
//                 self.v[x as usize] = vy.wrapping_sub(vx);
//             }

//             // SHL Vx
//             (0x8, x, _, 0xE) => {
//                 // Set Vx = Vx SHL 1; Set VF = Vx BIT 7
//                 self.v[0xF] = self.v[x as usize] >> 7;
//                 self.v[x as usize] <<= 1;
//             }

//             // SNE Vx, Vy
//             (0x9, x, y, 0) => {
//                 // Skip next instruction if Vx != Vy
//                 if self.v[x as usize] != self.v[y as usize] {
//                     self.pc = self.pc.wrapping_add(2);
//                 }
//             }

//             // LD I, u12
//             (0xA, ..) => {
//                 // Set I = u12
//                 self.i = opcode.as_u12();
//             }

//             // JP V0, u12
//             (0xB, ..) => {
//                 // Jump to u12 + V0
//                 self.pc = opcode.as_u12().wrapping_add(self.v[0] as u16);
//             }

//             // RND Vx, u8
//             (0xC, x, ..) => {
//                 // Set Vx = <random u8> AND u8
//                 self.v[x as usize] = random::<u8>() & opcode.as_u8();
//             }

//             // DRW Vx, Vy, u4
//             (0xD, x, y, n) => {
//                 // Display n-byte sprite starting in memory at I at (Vx, Vy)
//                 // Set VF = <collision>

//                 let x = self.v[x as usize] as usize;
//                 let y = self.v[y as usize] as usize;

//                 // VF is cleared at the start of DRW so collision can be set easily
//                 self.v[0xF] = 0;

//                 for i in 0..(n as usize) {
//                     let sy = (y + i) % self.height;

//                     for j in 0..8 {
//                         let sx = (x + j) % self.width;

//                         // Get VRAM offset
//                         let offset = sy * self.width + sx;

//                         // Get _current_ pixel in the screen
//                         let pixel = &mut self.screen[offset];
//                         let was_lit = pixel.lit;

//                         // Read memory to get the _set_ value
//                         let pixel_set_lit = (self.ram[(self.i as usize) + i] >> (7 - j)) & 1;

//                         // XOR to determine the new state of the pixel
//                         pixel.lit = ((pixel.lit as u8) ^ pixel_set_lit) != 0;
//                         if pixel.lit {
//                             pixel.phase = 1.0;
//                         } else if was_lit {
//                             // Start fade-out of pixel
//                             //  0.0 of lit=false is 100% opacity
//                             pixel.phase = 0.0;
//                         }

//                         // VF is set to indicate the transition 1 -> 0
//                         self.v[0xF] |= (was_lit && !pixel.lit) as u8;
//                     }
//                 }
//             }

//             // SKP Vx
//             (0xE, x, 0x9, 0xE) => {
//                 // Skip next instruction if key with the value of Vx is pressed
//                 if r.input_keyboard_state(0, KEYBOARD_MAP[self.v[x as usize] as usize]) {
//                     self.pc = self.pc.wrapping_add(2);
//                 }
//             }

//             // SKNP Vx
//             (0xE, x, 0xA, 0x1) => {
//                 // Skip next instruction if key with the value of Vx is not pressed
//                 if !r.input_keyboard_state(0, KEYBOARD_MAP[self.v[x as usize] as usize]) {
//                     self.pc = self.pc.wrapping_add(2);
//                 }
//             }

//             // LD Vx, DT
//             (0xF, x, 0x0, 0x7) => {
//                 // Set Vx = DT
//                 self.v[x as usize] = self.dt;
//             }

//             // LD DT, Vx
//             (0xF, x, 0x1, 0x5) => {
//                 // Set DT = Vx
//                 self.dt = self.v[x as usize];
//             }

//             // LD ST, Vx
//             (0xF, x, 0x1, 0x8) => {
//                 // Set ST = Vx
//                 self.st = self.v[x as usize];
//             }

//             // ADD I, Vx
//             (0xF, x, 0x1, 0xE) => {
//                 // Set I = I + Vx
//                 let r: u32 = self.i as u32 + self.v[x as usize] as u32;

//                 self.i = (r & 0xFFF) as u16;

//                 // If buffer overflow, register > VF must be set to 1, otherwise 0.
//                 self.v[0xF] = (r > 0xFFF) as u8;
//             }

//             // LD [I], FONT Vx
//             (0xF, x, 0x2, 0x9) => {
//                 // Set I = location of sprite for digit Vx.
//                 self.i = (self.v[x as usize] * 5) as u16;
//             }

//             // LD [I], BCD Vx
//             (0xF, x, 0x3, 0x3) => {
//                 // Store BCD representation of Vx in memory locations I, I+1, and I+2.
//                 let r = self.v[x as usize];
//                 let i = self.i;

//                 self.write(i, r / 100);
//                 self.write(i + 1, (r % 100) / 10);
//                 self.write(i + 2, r % 10);
//             }

//             // LD [I], Vx
//             (0xF, x, 0x5, 0x5) => {
//                 // Store registers V0 through Vx in memory starting at location I.
//                 let i = self.i;

//                 for j in 0..(x + 1) {
//                     let r = self.v[j as usize];

//                     self.write(i + j as u16, r);
//                 }
//             }

//             // LD Vx, [I]
//             (0xF, x, 0x6, 0x5) => {
//                 // Read registers V0 through Vx from memory starting at location I.
//                 let i = self.i;

//                 for j in 0..(x + 1) {
//                     self.v[j as usize] = self.read(i + j as u16);
//                 }
//             }

//             // SAVE Vx (SUPER-CHIP)
//             (0xF, x, 0x7, 0x5) => {
//                 // Save registers V0 through Vx (x <= 7)
//                 for i in 0..(cmp::max(x as usize, 7) + 1) {
//                     self.v_scratch[i] = self.v[i];
//                 }
//             }

//             // RESTORE Vx (SUPER-CHIP)
//             (0xF, x, 0x8, 0x5) => {
//                 // Restore registers V0 through Vx (x <= 7)
//                 for i in 0..(cmp::max(x as usize, 7) + 1) {
//                     self.v[i] = self.v_scratch[i];
//                 }
//             }

//             _ => {
//                 panic!("unknown opcode: ${:02X}{:02X}", opcode.hi, opcode.lo);
//             }
//         }

//         // Update timer point reference
//         self.timer_instant = Some(Instant::now());
//     }
// }

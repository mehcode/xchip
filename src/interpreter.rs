// use std::vec::Vec;
// use std::cmp;
// use std::time::Instant;
// use axal::{Runtime, Key};
// use rand::random;

// const SCREEN_STANDARD_WIDTH: usize = 64;
// const SCREEN_STANDARD_HEIGHT: usize = 32;

// const SCREEN_EXTENDED_WIDTH: usize = 128;
// const SCREEN_EXTENDED_HEIGHT: usize = 64;

use std::fs::File;
use std::io::Read;
use std::path::Path;

use axal;

use super_chip;
use opcode::Opcode;
use mmu;

// Interpreter mode of operation
#[derive(PartialEq)]
pub enum Mode {
    Chip8,
    HiResChip8,
    Chip8x,
    Chip10,
    SuperChip,
    XoChip,
}

impl Mode {
    fn from_file(filename: &str) -> Self {
        let ext = Path::new(filename).extension().unwrap_or_default().to_string_lossy();

        match &*ext {
            ".ch10" => Mode::Chip10,
            ".c8x" => Mode::Chip8x,
            _ => Mode::XoChip,
        }
    }
}

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

impl Context {
    fn reset(&mut self) {
        // Clear V
        for v in &mut self.v {
            *v = 0;
        }

        // Clear other registers
        self.i = 0;
        self.pc = 0x200;
        self.sp = self.stack_len * 2;

        // Clear screen
        for dot in &mut self.screen {
            *dot = 0;
        }
    }
}

pub trait Runtime {
    // Initialize the context and RAM for the usage of this runtime
    fn initialize(&mut self);

    // Reset any contained state
    fn reset(&mut self);

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
    pub fn initialize(&mut self) {
        // TODO: Allow stack_len to be controlled somewhere
        self.context.stack_len = 12;
    }

    pub fn insert_rom(&mut self, filename: &str, mode: Option<Mode>) {
        // Read in ROM
        let mut stream = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        // Write to RAM (starting at $200)
        self.mmu.write_all(0x200, &buffer);

        // Determine mode
        let mode = mode.unwrap_or_else(|| Mode::from_file(filename));

        // Construct runtime
        // TODO: Support other modes
        self.runtime = Some(Box::new(match mode {
            _ => {
                // TODO: Use XO-CHIP here
                Default::default(): super_chip::SuperChip
            }
        }));

        // Initialize runtime
        self.runtime.unwrap().initialize();
    }

    pub fn remove_rom(&mut self) {
        self.mmu.clear();
        self.runtime = None;
    }

    pub fn reset(&mut self) {
        // Reset context
        self.context.reset();

        // Reset runtime
        if let Some(runtime) = self.runtime {
            // runtime.reset();
        }
    }

    pub fn run_next(&mut self, r: &mut axal::Runtime) {
        // Read next 16-bit opcode (and increment PC)
        let opcode = Opcode::read_next(&mut self.context.pc, &mut self.mmu);
    }
}

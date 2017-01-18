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
    pub sp: u8,
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
        self.sp = 0;

        // Clear screen
        for dot in &mut self.screen {
            *dot = 0;
        }
    }

    pub fn stack_push(&mut self, m: &mut mmu::Mmu, value: u16) {
        // Increment Stack Pointer
        self.sp = self.sp.wrapping_add(1);

        // Write to RAM
        let address = 0x100 + (self.sp as usize) * 2;

        m.write(address, (value >> 8) as u8);
        m.write(address + 1, (value & 0xFF) as u8);
    }

    pub fn stack_pop(&mut self, m: &mut mmu::Mmu) -> u16 {
        // Read from RAM
        let address = 0x100 + ((self.sp as usize) * 2);

        let hi = m.read(address);
        let lo = m.read(address + 1);

        // Decrement Stack Pointer
        self.sp = self.sp.wrapping_sub(1);

        ((hi as u16) << 8) | (lo as u16)
    }
}

pub trait Runtime {
    // Initialize the context and RAM for the usage of this runtime
    fn configure(&mut self, c: &mut Context) {}

    // Reset any contained state
    fn reset(&mut self) {}

    // Execute passed operation; return false if unhandled
    fn execute(&mut self, c: &mut Context, m: &mut mmu::Mmu, opcode: Opcode) -> bool;
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
    pub fn configure(&mut self) {
        // The standard screen size is 64x32
        self.context.screen_width = 64;
        self.context.screen_height = 32;
        self.context.screen.resize(self.context.screen_width * self.context.screen_height,
                                   Default::default());

        // TODO: Allow stack_len to be controlled somewhere
        self.context.stack_len = 256;

        // TODO: Compatibility flags should be in here

        // Configure runtime
        if let Some(runtime) = self.runtime {
            runtime.configure(&mut self.context);
        }
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

        // Configure interpreter (and associated runtime)
        // The hook is here to allow for ROMs to eventually control
        // any parameters here.
        self.configure();
    }

    pub fn remove_rom(&mut self) {
        // Wipe out RAM
        self.mmu.clear();

        // Release runtime
        self.runtime = None;
    }

    pub fn reset(&mut self) {
        // Reset context
        self.context.reset();

        // Reset associated runtime
        if let Some(runtime) = self.runtime {
            runtime.reset();
        }
    }

    pub fn run_next(&mut self, r: &mut axal::Runtime) {
        // Read next 16-bit opcode (and increment PC)
        let opcode = Opcode::read_next(&mut self.context.pc, &mut self.mmu);

        // Execute opcode (with runtime)
        if let Some(runtime) = self.runtime {
            if !runtime.execute(&mut self.context, &mut self.mmu, opcode) {
                panic!("unhandled opcode: {}", opcode);
            }
        }
    }
}

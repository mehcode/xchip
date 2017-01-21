use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;
use std::time::Instant;

use axal;

use super_chip;
use chip_8;
use chip_8x;
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
            "ch10" => Mode::Chip10,
            "c8x" => Mode::Chip8x,
            _ => Mode::XoChip,
        }
    }
}

#[derive(Default)]
pub struct Context {
    // Framebuffer / Video RAM
    pub framebuffer: Vec<u8>,

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
    pub screen: Vec<bool>,
    pub screen_width: usize,
    pub screen_height: usize,

    // Delay timer
    pub dt: u8,

    // Sound timer
    pub st: u8,
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
        self.dt = 0;
        self.st = 0;

        // Clear framebuffer
        self.framebuffer.clear();

        // Clear screen
        for dot in &mut self.screen {
            *dot = false;
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

    // Reset state
    fn reset(&mut self, c: &mut Context) {}

    // Insert ROM
    fn insert_rom(&mut self, m: &mut mmu::Mmu, buffer: &[u8]) {
        m.write_all(0x200, buffer);
    }

    // Update framebuffer (in context)
    fn update_framebuffer(&mut self, c: &mut Context) {
        // Blit screen onto framebuffer
        c.framebuffer.resize(c.screen.len(), 0);
        for y in 0..c.screen_height {
            let offset_y = y * c.screen_width;

            for x in 0..c.screen_width {
                let offset = offset_y + x;

                // Get dot from screen
                let dot = c.screen[offset];

                // Blit to framebuffer
                c.framebuffer[offset] = if dot { 0xFF } else { 0x00 };
            }
        }
    }

    // Execute passed operation; return false if unhandled
    fn execute(&mut self,
               r: &mut axal::Runtime,
               c: &mut Context,
               m: &mut mmu::Mmu,
               opcode: Opcode)
               -> bool;
}

#[derive(Default)]
pub struct Interpreter {
    // Shared context used by all variants
    context: Context,

    // Memory management unit (incl. RAM)
    mmu: mmu::Mmu,

    // Active runtime (CHIP-8, CHIP-8X, etc.)
    runtime: Option<Box<Runtime>>,

    // 60 Hz timer that controls DT / ST
    timer_elapsed: u64,
    timer_instant: Option<Instant>,
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

        // Setup standard font sprites
        // TODO: Make this look a lot nicer
        self.mmu.write(0x00, 0xF0);
        self.mmu.write(0x01, 0x90);
        self.mmu.write(0x02, 0x90);
        self.mmu.write(0x03, 0x90);
        self.mmu.write(0x04, 0xF0);

        self.mmu.write(0x05, 0x20);
        self.mmu.write(0x06, 0x60);
        self.mmu.write(0x07, 0x20);
        self.mmu.write(0x08, 0x20);
        self.mmu.write(0x09, 0x70);

        self.mmu.write(0x0A, 0xF0);
        self.mmu.write(0x0B, 0x10);
        self.mmu.write(0x0C, 0xF0);
        self.mmu.write(0x0D, 0x80);
        self.mmu.write(0x0E, 0xF0);

        self.mmu.write(0x0F, 0xF0);
        self.mmu.write(0x10, 0x10);
        self.mmu.write(0x11, 0xF0);
        self.mmu.write(0x12, 0x10);
        self.mmu.write(0x13, 0xF0);

        self.mmu.write(0x14, 0x90);
        self.mmu.write(0x15, 0x90);
        self.mmu.write(0x16, 0xF0);
        self.mmu.write(0x17, 0x10);
        self.mmu.write(0x18, 0x10);

        self.mmu.write(0x19, 0xF0);
        self.mmu.write(0x1A, 0x80);
        self.mmu.write(0x1B, 0xF0);
        self.mmu.write(0x1C, 0x10);
        self.mmu.write(0x1D, 0xF0);

        self.mmu.write(0x1E, 0xF0);
        self.mmu.write(0x1F, 0x80);
        self.mmu.write(0x20, 0xF0);
        self.mmu.write(0x21, 0x90);
        self.mmu.write(0x22, 0xF0);

        self.mmu.write(0x23, 0xF0);
        self.mmu.write(0x24, 0x10);
        self.mmu.write(0x25, 0x20);
        self.mmu.write(0x26, 0x40);
        self.mmu.write(0x27, 0x40);
        self.mmu.write(0x28, 0xF0);
        self.mmu.write(0x29, 0x90);
        self.mmu.write(0x2A, 0xF0);
        self.mmu.write(0x2B, 0x90);
        self.mmu.write(0x2C, 0xF0);

        self.mmu.write(0x2D, 0xF0);
        self.mmu.write(0x2E, 0x90);
        self.mmu.write(0x2F, 0xF0);
        self.mmu.write(0x30, 0x10);
        self.mmu.write(0x31, 0xF0);

        self.mmu.write(0x32, 0xF0);
        self.mmu.write(0x33, 0x90);
        self.mmu.write(0x34, 0xF0);
        self.mmu.write(0x35, 0x90);
        self.mmu.write(0x36, 0x90);

        self.mmu.write(0x37, 0xE0);
        self.mmu.write(0x38, 0x90);
        self.mmu.write(0x39, 0xE0);
        self.mmu.write(0x3A, 0x90);
        self.mmu.write(0x3B, 0xE0);

        self.mmu.write(0x3C, 0xF0);
        self.mmu.write(0x3D, 0x80);
        self.mmu.write(0x3E, 0x80);
        self.mmu.write(0x3F, 0x80);
        self.mmu.write(0x40, 0xF0);

        self.mmu.write(0x41, 0xE0);
        self.mmu.write(0x42, 0x90);
        self.mmu.write(0x43, 0x90);
        self.mmu.write(0x44, 0x90);
        self.mmu.write(0x45, 0xE0);

        self.mmu.write(0x46, 0xF0);
        self.mmu.write(0x47, 0x80);
        self.mmu.write(0x48, 0xF0);
        self.mmu.write(0x49, 0x80);
        self.mmu.write(0x4A, 0xF0);

        self.mmu.write(0x4B, 0xF0);
        self.mmu.write(0x4C, 0x80);
        self.mmu.write(0x4D, 0xF0);
        self.mmu.write(0x4E, 0x80);
        self.mmu.write(0x4F, 0x80);

        // TODO: Compatibility flags should be in here

        // Configure runtime
        if let Some(ref mut runtime) = self.runtime {
            runtime.configure(&mut self.context);
        }
    }

    pub fn insert_rom(&mut self, filename: &str, mode: Option<Mode>) {
        // Determine mode
        let mode = mode.unwrap_or_else(|| Mode::from_file(filename));

        // Construct runtime
        // TODO: Support other modes
        self.runtime = Some(match mode {
            Mode::Chip8x => Box::new(Default::default(): chip_8x::Chip8x),

            _ => {
                // TODO: Use XO-CHIP here
                Box::new(Default::default(): chip_8::Chip8)
            }
        });

        // Read in ROM
        let mut stream = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).unwrap();

        // Insert ROM
        if let Some(ref mut runtime) = self.runtime {
            runtime.insert_rom(&mut self.mmu, &buffer);
        }

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
        if let Some(ref mut runtime) = self.runtime {
            runtime.reset(&mut self.context);
        }
    }

    pub fn run_next(&mut self, r: &mut axal::Runtime) {
        // If timer point reference is non-zero; check elapsed and
        // clock ST / DT
        if let Some(timer_instant) = self.timer_instant {
            let elapsed = timer_instant.elapsed();
            self.timer_elapsed += (elapsed.as_secs() * 1_000_000_000) +
                                  (elapsed.subsec_nanos() as u64);

            // 1/60 s => 16_666_666 ns
            if self.timer_elapsed >= 16_666_666 {
                self.timer_elapsed -= 16_666_666;

                if self.context.dt > 0 {
                    self.context.dt -= 1;
                }

                if self.context.st > 0 {
                    self.context.st -= 1;
                }
            }
        }

        // Read next 16-bit opcode (and increment PC)
        let opcode = Opcode::read_next(&mut self.context.pc, &mut self.mmu);

        // Execute opcode (with runtime)
        if let Some(ref mut runtime) = self.runtime {
            if !runtime.execute(r, &mut self.context, &mut self.mmu, opcode) {
                panic!("unhandled opcode: {}", opcode);
            }
        }

        // Update timer point reference
        self.timer_instant = Some(Instant::now());
    }

    pub fn screen_as_framebuffer(&mut self) -> (&[u8], usize, usize) {
        if let Some(ref mut runtime) = self.runtime {
            runtime.update_framebuffer(&mut self.context);
        }

        (&self.context.framebuffer, self.context.screen_width, self.context.screen_height)
    }
}

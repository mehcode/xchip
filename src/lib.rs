
#[macro_use]
extern crate axal;

extern crate rand;

mod cpu;

use std::vec::Vec;
use std::fs::File;
use std::io::Read;

use cpu::CPU;

#[derive(Default)]
pub struct Core {
    cpu: CPU,
}

impl axal::Core for Core {
    fn info(&self) -> axal::Info {
        axal::Info::new("xCHIP", env!("CARGO_PKG_VERSION"))
            .pixel_format(axal::PixelFormat::R3_G3_B2)
            .size(64, 32)
            .max_size(64, 64)
    }

    fn reset(&mut self) {
        self.cpu.reset();
    }

    fn rom_insert(&mut self, filename: &str) {
        // Read file
        let stream = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        stream.take(0x800).read_to_end(&mut buffer).unwrap();

        // Push buffer to CPU
        self.cpu.take_rom(buffer);
    }

    fn rom_remove(&mut self) {
        // Clear ROM from CPU Memory
        self.cpu.take_rom(vec![]);
    }

    // Run core for a _single_ frame
    fn run_next(&mut self, r: &mut axal::Runtime) {
        // CPU: Run 8 instructions = 1 frame ~> 480 Hz
        for _ in 0..8 {
            self.cpu.run_next(r);
        }

        // Video: Refresh
        r.video_refresh(self.cpu.screen_as_framebuffer(), 64, 32);
    }

    // fn serialize() { }
    // fn deserialize() { }
}

// impl axal::Debug for Core { }

// impl axal::UI (name?) for Core { }

// Generate C API
ax_generate_lib!(Core);

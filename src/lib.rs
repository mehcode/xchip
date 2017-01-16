
#[macro_use]
extern crate axal;

extern crate rand;

mod interpreter;

use std::vec::Vec;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
pub struct Core {
    interpreter: interpreter::Interpreter,
}

impl axal::Core for Core {
    fn info(&self) -> axal::Info {
        axal::Info::new("xCHIP", env!("CARGO_PKG_VERSION"))
            .pixel_format(axal::PixelFormat::R3_G3_B2)
            .size(64, 32)
            .max_size(64, 64)
    }

    fn reset(&mut self) {
        self.interpreter.reset();
    }

    fn rom_insert(&mut self, filename: &str) {
        // Read file
        let stream = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        stream.take(0x800).read_to_end(&mut buffer).unwrap();

        // Push ROM buffer
        self.interpreter.take_rom(buffer);
    }

    fn rom_remove(&mut self) {
        // Clear ROM buffer from Memory
        self.interpreter.take_rom(vec![]);
    }

    // Run core for a _single_ frame
    fn run_next(&mut self, r: &mut axal::Runtime) {
        // Interpreter: Run 8 instructions = 1 frame ~> 480 Hz
        for _ in 0..8 {
            self.interpreter.run_next(r);
        }

        // Video: Refresh
        let (framebuffer, width, height) = self.interpreter.screen_as_framebuffer();
        r.video_refresh(framebuffer, width as u32, height as u32);
    }

    // fn serialize() { }
    // fn deserialize() { }
}

// impl axal::Debug for Core { }

// impl axal::UI (name?) for Core { }

// Generate C API
ax_generate_lib!(Core);

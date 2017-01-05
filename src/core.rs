use cpu::CPU;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use axal;

#[derive(Default)]
pub struct Core {
    cpu: CPU,
    video_refresh: Option<axal::VideoRefresh>,
}

impl axal::Core for Core {
    fn new() -> Core {
        Default::default()
    }

    fn reset(&mut self) {
        self.cpu.reset();
    }

    fn set_video_refresh(&mut self, cb: axal::VideoRefresh) {
        self.video_refresh = Some(cb);
    }

    fn insert_rom(&mut self, filename: &str) {
        // Read file
        let stream = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        stream.take(0x800).read_to_end(&mut buffer).unwrap();

        // Push buffer to CPU
        self.cpu.take_rom(buffer);

        // After `open_rom`; invoke `reset`
        self.reset();
    }

    fn remove_rom(&mut self) {
        // Clear out ROM in CPU
        self.cpu.take_rom(vec![]);
    }

    fn run_next(&mut self) {
        // Run CPU for 1 frame ~ 60 Hz
        for _ in 0..60 {
            self.cpu.run_next();
        }

        // Push frame
        if let Some(ref video_refresh) = self.video_refresh {
            (video_refresh)(self.cpu.framebuffer.as_ptr(), 64, 32, 64 * 3);
        }
    }
}

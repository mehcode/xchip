use cpu::CPU;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;
use std::ptr;
use libc;

#[derive(Default)]
pub struct Machine {
    cpu: CPU,

    on_video_frame: Option<extern "C" fn(*mut libc::c_void, *const u8, u32, u32, u8) -> ()>,
    on_video_frame__user_data: Option<*mut libc::c_void>,
}

impl Machine {
    pub fn new() -> Machine {
        Default::default()
    }

    pub fn reset(&mut self) {
        self.cpu.reset();
    }

    pub fn set_on_video_frame(&mut self,
                              cb: extern "C" fn(*mut libc::c_void, *const u8, u32, u32, u8) -> (),
                              user_data: *mut libc::c_void) {
        self.on_video_frame = Some(cb);
        self.on_video_frame__user_data = Some(user_data);
    }

    pub fn open_rom(&mut self, filename: &str) {
        // Read file
        // TODO: Check size first
        let stream = File::open(filename).unwrap();
        let mut buffer = Vec::new();
        stream.take(0x800).read_to_end(&mut buffer).unwrap();

        // Push buffer to CPU
        self.cpu.take_rom(buffer);

        // After `open_rom`; invoke `reset`
        self.reset();
    }

    pub fn run_next(&mut self) {
        // Run CPU for 1 frame ~ 500 Hz
        for _ in 0..500 {
            self.cpu.run_next();
        }

        // Push frame
        if let Some(ref on_video_frame) = self.on_video_frame {
            (on_video_frame)(self.on_video_frame__user_data.unwrap(),
                             self.cpu.framebuffer.as_ptr(),
                             64,
                             32,
                             3);
        }

        // TODO: Wait until "end of frame"
    }
}

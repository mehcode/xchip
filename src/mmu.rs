use std::vec::Vec;

pub struct Mmu {
    ram: Vec<u8>,
}

impl Mmu {
    pub fn clear(&mut self) {
        for b in &mut self.ram {
            *b = 0;
        }
    }

    fn extend(&mut self, address: usize) {
        if address >= self.ram.len() {
            self.ram.resize(address, 0);
        }
    }

    pub fn read(&mut self, address: usize) -> u8 {
        self.extend(address);

        self.ram[address as usize]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.extend(address);

        self.ram[address as usize] = value;
    }

    #[allow(needless_range_loop, unknown_lints)]
    pub fn write_all(&mut self, address: usize, buffer: &[u8]) {
        self.extend(address + buffer.len());

        for i in 0..(buffer.len()) {
            self.ram[address + i] = buffer[i];
        }
    }
}

impl Default for Mmu {
    fn default() -> Self {
        Mmu { ram: Default::default() }
    }
}

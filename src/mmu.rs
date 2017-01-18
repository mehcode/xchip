use std::vec::Vec;

pub struct Mmu {
    ram: Vec<u8>,
    max_size: usize,
}

impl Mmu {
    pub fn set_max_size(&mut self, max_size: usize) {
        self.max_size = max_size;
    }

    pub fn clear(&mut self) {
        for b in &mut self.ram {
            *b = 0;
        }
    }

    fn extend(&mut self, address: usize) {
        if address < self.max_size && address >= self.ram.len() {
            self.ram.resize(address, 0);
        }
    }

    pub fn read(&mut self, address: usize) -> u8 {
        self.extend(address);

        self.ram[(address & (self.max_size - 1)) as usize]
    }

    pub fn write(&mut self, address: usize, value: u8) {
        self.extend(address);

        self.ram[(address & (self.max_size - 1)) as usize] = value;
    }

    pub fn write_all(&mut self, address: usize, buffer: &[u8]) {
        self.extend(address + buffer.len());
    }
}

impl Default for Mmu {
    fn default() -> Self {
        Mmu { max_size: 4096, ..Default::default() }
    }
}

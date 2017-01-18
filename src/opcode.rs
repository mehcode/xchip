use mmu::Mmu;
use std::fmt;

// A standard CHIP-8 opcode is 2-bytes long (big-endian)
pub struct Opcode {
    hi: u8,
    lo: u8,
}

impl Opcode {
    #[inline]
    pub fn read_next(pc: &mut usize, m: &mut Mmu) -> Self {
        let r = Opcode {
            hi: m.read(*pc),
            lo: m.read(*pc + 1),
        };

        *pc += 2;

        r
    }

    #[inline]
    pub fn unwrap(&self) -> (u8, u8, u8, u8) {
        ((self.hi >> 4), (self.hi & 0xF), (self.lo >> 4), (self.lo & 0xF))
    }

    // Extract the lower 12-bits
    #[inline]
    pub fn extract_u12(&self) -> u16 {
        (self.lo as u16) | (((self.hi & 0xF) as u16) << 8)
    }

    // Extract the lower 8-bits
    #[inline]
    pub fn extract_u8(&self) -> u8 {
        self.lo
    }
}

impl fmt::Display for Opcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "${:02X}{:02X}", self.hi, self.lo)
    }
}

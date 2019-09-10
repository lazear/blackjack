//! PCG psuedorandom number generator
//! Adapted from http://www.pcg-random.org/

use rand_core::{impls, Error, RngCore};
use sha2::{Digest, Sha256};

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct PCG32 {
    state: u64,
    inc: u64,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct PCG32Seed(u64, u64);

impl PCG32Seed {
    pub fn new(state: u64, seq: u64) -> PCG32Seed {
        PCG32Seed(state, seq)
    }

    pub fn sha256(&self) -> String {
        let mut buf: [u8; 16] = [0u8; 16];
        buf[0..8].copy_from_slice(&self.0.to_le_bytes());
        buf[8..].copy_from_slice(&self.1.to_le_bytes());
        let mut hasher = Sha256::default();
        hasher.input(buf);
        format!("{:0x}", hasher.result())
    }
}

impl PCG32 {
    pub fn new(state: u64, seq: u64) -> PCG32 {
        PCG32 {
            state,
            inc: (seq << 1) | 1,
        }
    }

    pub fn from_seed(seed: PCG32Seed) -> PCG32 {
        PCG32 {
            state: seed.0,
            inc: (seed.1 << 1) | 1,
        }
    }

    pub fn to_seed(&self) -> PCG32Seed {
        PCG32Seed(self.state, self.inc >> 1)
    }
}

impl RngCore for PCG32 {
    fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    fn next_u64(&mut self) -> u64 {
        self.state = self
            .state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(self.inc);
        let xor = ((self.state >> 18) ^ self.state) >> 27;
        let rot = (self.state >> 59) as i64;
        (xor >> rot) | (xor << ((-rot) & 31))
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        impls::fill_bytes_via_next(self, dest)
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        Ok(self.fill_bytes(dest))
    }
}

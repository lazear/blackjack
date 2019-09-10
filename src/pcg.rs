//! PCG psuedorandom number generator
//! Adapted from http://www.pcg-random.org/

use rand_core::{impls, Error, RngCore};

pub struct PCG32 {
    state: u64,
    inc: u64,
}

impl PCG32 {
    pub fn seed(seed: u64, seq: u64) -> PCG32 {
        let mut rng = PCG32 {
            state: 0,
            inc: (seq << 1) | 1,
        };
        rng.next_u64();
        rng.state = rng.state.wrapping_add(seed);
        rng
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

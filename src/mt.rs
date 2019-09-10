//! Rust implementation of the Mersenne Twister psuedorandom number generator
//! Original 64bit algorithm described by Makoto Matsumoto and Takuji Nishimura

use std::num::Wrapping;

const NN: usize = 312;
const MM: usize = 156;
const ONE: Wrapping<u64> = Wrapping(1);
const MATRIX_A: Wrapping<u64> = Wrapping(0xB5026F5AA96619E9);
const UM: Wrapping<u64> = Wrapping(0xFFFFFFFF80000000);
const LM: Wrapping<u64> = Wrapping(0x7FFFFFFF);

const UNSEEDED: MT19937_64 = MT19937_64 {
    idx: 0,
    state: [Wrapping(0); NN],
};

pub struct MT19937_64 {
    idx: usize,
    state: [Wrapping<u64>; NN],
}

impl MT19937_64 {
    pub fn seed(seed: u64) -> MT19937_64 {
        let mut mt = UNSEEDED;
        mt.reseed(seed);
        mt
    }

    fn reseed(&mut self, seed: u64) {
        self.idx = NN;
        self.state[0] = Wrapping(seed);
        for i in 1..NN {
            self.state[i] = Wrapping(6364136223846793005) * (self.state[i-1] ^ (self.state[i-1] >> 62)) + Wrapping(i as u64);
        }
    }

    fn random(&mut self) -> u64 {
        if self.idx >= NN {
            if self.idx == NN + 1 {
                self.reseed(5489);
            }
        }

    }
}


#[inline]
fn temper(mut x: u64) -> u64 {
    x ^= (x >> 29) & 0x5555555555555555;
    x ^= (x << 17) & 0x71D67FFFEDA60000;
    x ^= (x << 37) & 0xFFF7EEE000000000;
    x ^=  x >> 43;
    x
}

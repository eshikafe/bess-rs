// Copyright (c) 2014-2016, The Regents of the University of California.
// Copyright (c) 2016-2017, Nefeli Networks, Inc.
// Copyright (c) 2024, Austin Aigbe
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions are met:
//
// * Redistributions of source code must retain the above copyright notice, this
// list of conditions and the following disclaimer.
//
// * Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// * Neither the names of the copyright holders nor the names of their
// contributors may be used to endorse or promote products derived from this
// software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.


use std::time::{SystemTime, UNIX_EPOCH};

pub struct Random {
    seed: u64,
}

impl Random {
    // Constructor with no arguments, uses rdtsc equivalent
    pub fn new() -> Self {
        Self {
            seed: Random::rdtsc(),
        }
    }

    // Constructor with seed argument
    pub fn with_seed(seed: u64) -> Self {
        Self { seed }
    }

    // Method to set seed
    pub fn set_seed(&mut self, seed: u64) {
        self.seed = seed;
    }

    // Equivalent to `rdtsc` in C++
    fn rdtsc() -> u64 {
        let start = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        start.as_nanos() as u64
    }

    // `Get` method: generate a random number
    pub fn get(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        (self.seed >> 32) as u32
    }

    // Get a random number in range [0, range)
    pub fn get_range(&mut self, range: u32) -> u32 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let tmp: u64 = (self.seed >> 12) | 0x3ff0000000000000;
        let dbl = f64::from_bits(tmp);
        ((dbl - 1.0) * range as f64) as u32
    }

    // Get a real number in the range [0.0, 1.0)
    pub fn get_real(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let tmp: u64 = (self.seed >> 12) | 0x3ff0000000000000;
        let dbl = f64::from_bits(tmp);
        dbl - 1.0
    }

    // Get a real number in the range (0.0, 1.0]
    pub fn get_real_nonzero(&mut self) -> f64 {
        self.seed = self.seed.wrapping_mul(1103515245).wrapping_add(12345);
        let tmp: u64 = (self.seed >> 12) | 0x3ff0000000000000;
        let dbl = f64::from_bits(tmp);
        2.0 - dbl
    }
}

// Copyright (c) 2017 The Noise-rs Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT
// or http://opensource.org/licenses/MIT>, at your option. All files in the
// project carrying such notice may not be copied, modified, or distributed
// except according to those terms.

use noise_fns::NoiseFn;

/// Noise function that raises the output value from the first source function
/// to the power of the output value of the second source function.
pub struct Power<'a, T: 'a> {
    /// Outputs a value.
    pub source1: &'a NoiseFn<T>,

    /// Outputs a value.
    pub source2: &'a NoiseFn<T>,
}

impl<'a, T> Power<'a, T> {
    pub fn new(source1: &'a NoiseFn<T>, source2: &'a NoiseFn<T>) -> Power<'a, T> {
        Power {
            source1: source1,
            source2: source2,
        }
    }
}

impl<'a, T> NoiseFn<T> for Power<'a, T>
where
    T: Copy,
{
    fn get(&self, point: T) -> f64 {
        (self.source1.get(point)).powf(self.source2.get(point))
    }
}

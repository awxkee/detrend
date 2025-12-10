/*
 * // Copyright (c) Radzivon Bartoshyk 12/2025. All rights reserved.
 * //
 * // Redistribution and use in source and binary forms, with or without modification,
 * // are permitted provided that the following conditions are met:
 * //
 * // 1.  Redistributions of source code must retain the above copyright notice, this
 * // list of conditions and the following disclaimer.
 * //
 * // 2.  Redistributions in binary form must reproduce the above copyright notice,
 * // this list of conditions and the following disclaimer in the documentation
 * // and/or other materials provided with the distribution.
 * //
 * // 3.  Neither the name of the copyright holder nor the names of its
 * // contributors may be used to endorse or promote products derived from
 * // this software without specific prior written permission.
 * //
 * // THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * // AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * // IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * // DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE
 * // FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * // DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
 * // SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
 * // CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
 * // OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE
 * // OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */
use crate::detrend_error::try_vec;
use crate::mla::fmla;
use crate::{DetrendError, DetrendSample};
use num_traits::AsPrimitive;

pub(crate) fn detrend_least_squares<T: DetrendSample>(signal: &[T]) -> Result<Vec<T>, DetrendError>
where
    f64: AsPrimitive<T>,
    usize: AsPrimitive<T>,
{
    let n_u = signal.len();
    if n_u == 0 {
        return Ok(Vec::new());
    }

    let mut vec: Vec<T> = try_vec![T::zero(); signal.len()];

    // Calculate mean and indices
    let mut indices: Vec<T> = try_vec![T::zero(); n_u];
    for (i, dst) in indices.iter_mut().enumerate() {
        *dst = i.as_();
    }
    let sum_xy: T = {
        let mut z = 0.0f64.as_();
        for (&s0, &s1) in indices.iter().zip(signal.iter()) {
            z = fmla(s0, s1, z);
        }
        z
    };
    let mut sum_x: T = T::zero();
    for &q in indices.iter() {
        sum_x += q;
    }
    let mut sum_y: T = T::zero();
    for &q in signal.iter() {
        sum_y += q;
    }
    let sum_x_2: T = {
        let mut z = 0.0f64.as_();
        for &s0 in indices.iter() {
            z = fmla(s0, s0, z);
        }
        z
    };

    let n = signal.len().as_();

    // Calculate slope and intercept of the linear trend
    let slope: T = fmla(n, sum_xy, -sum_x * sum_y) / fmla(n, sum_x_2, -sum_x * sum_x);
    let intercept: T = fmla(-slope, sum_x, sum_y) / n;

    // Remove linear trend from the signal
    for (i, (&val, dst)) in signal.iter().zip(vec.iter_mut()).enumerate() {
        let new_val = val - fmla(slope, i.as_(), intercept);
        *dst = new_val;
    }
    Ok(vec)
}

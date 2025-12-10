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
use crate::constant::detrend_constant;
use crate::least_squares::detrend_least_squares;
use num_traits::{Float, MulAdd};
use std::ops::{AddAssign, DivAssign};

mod constant;
mod detrend_error;
mod least_squares;
mod mla;

pub(crate) trait DetrendSample:
    Float + 'static + MulAdd<Self, Output = Self> + AddAssign + DivAssign
{
}

impl DetrendSample for f32 {}
impl DetrendSample for f64 {}

pub use detrend_error::DetrendError;

/// An enumeration that specifies the method used to estimate and remove the trend from a signal.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum DetrendingMethod {
    /// Removes the mean (average) of the signal. This is equivalent to removing a zero-order polynomial or the DC offset from the signal.
    Constant,
    /// Removes a linear trend estimated using the Least Squares method. This fits a first-order polynomial (a straight line) to the data and subtracts it.
    LeastSquares,
}

/// Detrends a signal composed of single-precision floating-point numbers (`f32`).
///
/// # Arguments
///
/// * `signal`: A slice (`&[f32]`) representing the input signal.
/// * `detrending_method`: The method to use for trend removal.
pub fn detrend_f32(
    signal: &[f32],
    detrending_method: DetrendingMethod,
) -> Result<Vec<f32>, DetrendError> {
    match detrending_method {
        DetrendingMethod::Constant => detrend_constant(signal),
        DetrendingMethod::LeastSquares => detrend_least_squares(signal),
    }
}

/// Detrends a signal composed of single-precision floating-point numbers (`f64`).
///
/// # Arguments
///
/// * `signal`: A slice (`&[f32]`) representing the input signal.
/// * `detrending_method`: The method to use for trend removal.
pub fn detrend_f64(
    signal: &[f64],
    detrending_method: DetrendingMethod,
) -> Result<Vec<f64>, DetrendError> {
    match detrending_method {
        DetrendingMethod::Constant => detrend_constant(signal),
        DetrendingMethod::LeastSquares => detrend_least_squares(signal),
    }
}

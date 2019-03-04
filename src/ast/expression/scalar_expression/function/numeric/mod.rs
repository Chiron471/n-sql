// Copyright 2019 The n-sql Project Developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

mod abs_fn;
mod ceil_fn;
mod cos_fn;
mod dense_rank_fn;
mod floor_fn;
mod log10_fn;
mod log_fn;
mod pow_fn;
mod rank_fn;
mod round_fn;
mod sign_fn;
mod sin_fn;
mod sqrt_fn;
mod tan_fn;


pub use self::abs_fn::*;
pub use self::ceil_fn::*;
pub use self::cos_fn::*;
pub use self::dense_rank_fn::*;
pub use self::floor_fn::*;
pub use self::log10_fn::*;
pub use self::log_fn::*;
pub use self::pow_fn::*;
pub use self::rank_fn::*;
pub use self::round_fn::*;
pub use self::sign_fn::*;
pub use self::sin_fn::*;
pub use self::sqrt_fn::*;
pub use self::tan_fn::*;

#[derive(Clone, Debug)]
pub enum NumericFn {
    Abs(AbsFn),
    Ceil(CeilFn),
    Cos(CosFn),
    DenseRank(DenseRankFn),
    Floor(FloorFn),
    Log10(Log10Fn),
    Log(LogFn),
    Pow(PowFn),
    Rank(RankFn),
    Round(RoundFn),
    Sin(SinFn),
    Sign(SignFn),
    Sqrt(SqrtFn),
    Tan(TanFn)
}

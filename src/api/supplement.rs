// Copyright 2024 Ezra Alvarion
//
// Licensed under the Apache License, Version 2.0 <LICENSE-Apache or
// https://www.apache.org/licenses/LICENSE-2.0> or the BSD 2-Clause 
// License <LICENSE-BSD or https://opensource.org/license/bsd-2-clause/>,
// at Your option. This file may not be copied, modified, or distributed
// except according to those terms.

/// Type for id-representing numbers
/// 
/// The idea for this (instad of using `u32` directly) is to make things that are id
/// more legible and obvious. It also could in theory be upgraded to a `u64` in the
/// (very unlikely event) that the number of posts on E621 surpasses 2^32 - 1
pub type IdType = u32;

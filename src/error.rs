//! Error module.

//! Generic error marker trait.

#[cfg(feature = "std")]
use std::fmt::Debug;

#[cfg(not(feature = "std"))]
use alloc::fmt::Debug;

pub trait Error: Debug {}

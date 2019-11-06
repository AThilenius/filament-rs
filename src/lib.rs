pub mod low_level;

#[cfg(feature = "high-level")]
pub mod high_level;

#[cfg(feature = "high-level")]
pub use high_level::*;

#[cfg(feature = "high-level")]
#[macro_use]
extern crate shrinkwraprs;

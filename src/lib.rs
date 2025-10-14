//#![deny(missing_docs)]
#![doc(html_root_url = "http://docs.rs/nvapi/0.2.0")]

pub use nvapi_sys as sys;

mod clock;
#[path = "gpu/mod.rs"]
mod gpu;
mod gsync;
#[cfg(feature = "i2c")]
mod i2c_impl;
mod info;
mod mosaic;
mod pstate;
mod thermal;
mod types;

pub use clock::*;
pub use gpu::*;
pub use gsync::*;
#[cfg(feature = "i2c")]
pub use i2c_impl::*;
pub use info::*;
pub use mosaic::*;
pub use pstate::*;
pub use thermal::*;
pub use types::*;

pub use sys::{Result, Status};

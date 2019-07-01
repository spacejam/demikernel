#![feature(generators, generator_trait)]
#![warn(clippy::all)]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate num_derive;

#[macro_use]
extern crate log;

#[cfg(test)]
#[macro_use]
extern crate maplit;

#[macro_use]
pub mod r#async;

mod effect;
mod options;
mod prelude;
mod protocols;
mod rand;

pub mod collections;
pub mod engine;
pub mod fail;
pub mod io;
pub mod result;
pub mod runtime;

#[cfg(test)]
pub mod test;

pub use effect::Effect;
pub use engine::Engine;
pub use io::IoVec;
pub use options::Options;
pub use runtime::Runtime;

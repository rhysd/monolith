#[macro_use]
extern crate lazy_static;

#[macro_use]
mod macros;

pub mod html;
pub mod http;
pub mod js;
pub mod utils;
#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(test)]
pub mod tests;

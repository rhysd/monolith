extern crate html5ever;
#[macro_use]
extern crate lazy_static;
extern crate regex;
#[cfg(not(target_arch = "wasm32"))]
extern crate reqwest;
extern crate url;

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

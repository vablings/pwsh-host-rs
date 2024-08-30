pub mod bindings;
pub mod cli_xml;
pub mod context;
pub mod delegate_loader;
pub mod error;
pub mod host_detect;
pub mod host_exit_code;
pub mod hostfxr;
pub mod loader;
pub mod tests;
pub mod time;
extern crate libc;
#[macro_use]
extern crate dlopen_derive;
extern crate dlopen;
#[macro_use]
extern crate quick_error;

/// Module for a platform dependent c-like string type.
#[macro_use]
mod pdcstring;

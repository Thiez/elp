//Used for testing the logging module.
#[macro_use]
extern crate lazy_static;

#[macro_use]
mod logging;
pub mod elb_log_files;

pub struct RuntimeContext {
    pub debug: bool,
}

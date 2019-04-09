#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod core;
mod frontend;
mod mupen;
mod platform;

use crate::core::app::App as Core;
use crate::frontend::app::App as Frontend;

use relm::Widget;

fn main() {
    let core = Core::new();
    core.run();
    // Frontend::run(()).unwrap();
}

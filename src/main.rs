#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod backend;
mod frontend;
mod mupen;
mod platform;

use crate::backend::app::App as Backend;
use crate::frontend::app::App as Frontend;

use relm::Widget;

fn main() {
    let backend = Backend::run();
    // Frontend::run(()).unwrap();
}

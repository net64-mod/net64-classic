#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

mod frontend;

use crate::frontend::app::App;

use relm::Widget;

fn main() {
    App::run(()).unwrap();
}

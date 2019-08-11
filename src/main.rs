mod core;
mod frontend;
mod mupen;
mod platform;

use crate::core::App as Core;
use crate::frontend::App as Frontend;

fn main() {
    let core = Core::new();
    core.run();
    Frontend::new();
    // frontend.run();
}

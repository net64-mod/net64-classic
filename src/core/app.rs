use crate::mupen::Core;
use super::emulator::Emulator;

pub struct App {
    core: Core,
    emulator: Emulator,
}

impl App {
    pub fn run() -> App {
        let core = Core::new();
        let emulator = Emulator::new();
        core.startup().expect("Core startup failed");
        App { core, emulator }
    }
}

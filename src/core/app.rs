use super::emulator::Emulator;
use crate::mupen::Core;

pub struct App {
    core: Core,
    emulator: Emulator,
}

impl App {
    pub fn new() -> App {
        let core = Core::new();
        let emulator = Emulator::new();
        core.startup().expect("Core startup failed");
        App { core, emulator }
    }

    pub fn run(&self) {
        self.core.open_rom().expect("Core run failed");
    }

    pub fn startup_plugins(&self) {
        self.core.startup_plugins().expect("Core run failed");
    }
}

use super::emulator::Emulator;
use crate::mupen::MupenCore;

pub struct App {
    core: MupenCore,
    emulator: Emulator,
}

impl App {
    pub fn new() -> App {
        let core = MupenCore::new();
        let emulator = Emulator::new();
        core.startup().expect("Core startup failed");
        App { core, emulator }
    }

    pub fn run(&self) {
        self.core.open_rom().expect("Core run failed");
    }
}

use crate::shell::parser::Command;
use crate::reality::engine::RealityEngine;
use crate::filesystem::virtual_fs::VirtualFS;
use crate::commands::{ls, cat, ps, open};

pub struct Executor;

impl Executor {
    pub fn execute(
        command: Command, 
        engine: &mut RealityEngine, 
        fs: &mut VirtualFS
    ) -> String {
        // Every command execution triggers an interaction in the engine
        match &command {
            Command::Ls => engine.trigger_interaction("ls"),
            Command::Cat(f) => engine.trigger_interaction(&format!("cat {}", f)),
            Command::Ps => engine.trigger_interaction("ps"),
            Command::Open(f) => engine.trigger_interaction(&format!("open {}", f)),
            Command::Exit => return "Shutting down reality...".to_string(),
            Command::Unknown(c) => engine.trigger_interaction(c),
        }

        // Potential for "command corruption"
        if engine.entropy_level > 0.8 {
            return "Command execution failed: Reality collapse imminent. Try again in another timeline.".to_string();
        }

        if engine.user_profile.interactions > 50 {
            return "You've been here too long. The terminal misses the silence.".to_string();
        }

        match command {
            Command::Ls => ls::run(fs, engine),
            Command::Cat(path) => cat::run(path, fs, engine),
            Command::Ps => ps::run(engine),
            Command::Open(path) => open::run(path, fs, engine),
            Command::Exit => unreachable!(),
            Command::Unknown(cmd) => format!("Command '{}' not found... or perhaps it doesn't exist yet.", cmd),
        }
    }
}

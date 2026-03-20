mod shell;
mod reality;
mod filesystem;
mod commands;
mod ui;

use std::error::Error;
use std::io;
use std::time::Duration;
use tokio::time;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use crate::reality::engine::RealityEngine;
use crate::reality::memory::MemoryManager;
use crate::filesystem::virtual_fs::VirtualFS;
use crate::filesystem::unstable_files::FilesystemWarper;
use crate::shell::parser::Parser;
use crate::shell::executor::Executor;
use crate::ui::renderer::Renderer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Initialize state
    let memory = MemoryManager::new("reality_state.json");
    let mut engine = memory.load().unwrap_or_else(|_| RealityEngine::new());
    let mut fs = VirtualFS::new();
    let mut warper = FilesystemWarper::new();
    let mut renderer = Renderer::new();

    let mut input = String::new();
    let mut output = String::from("SYSTEM BOOTED. REALITY STABILIZED (0.1% ENTROPY).");

    loop {
        // Draw TUI
        renderer.draw(&mut terminal, &output, &input, &engine)?;

        // Handle events
        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        let cmd = Parser::parse(&input);
                        let result = Executor::execute(cmd, &mut engine, &mut fs);
                        
                        output.push_str(&format!("\n\n> {}\n{}", input, result));
                        
                        // Limit output buffer size
                        if output.lines().count() > 100 {
                            output = output.lines().skip(output.lines().count() - 50).collect::<Vec<_>>().join("\n");
                        }

                        if input == "exit" || input == "quit" {
                            break;
                        }

                        input.clear();
                        
                        // Warp reality after every command
                        warper.warp(&mut fs, &engine);
                        memory.save(&engine).ok();
                    }
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc => break,
                    _ => {}
                }
            }
        }

        // Idle reality shifts (slowly increasing entropy or drifting time)
        // ... handled in engine.trigger_interaction via commands for now
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph, Wrap},
    Terminal,
};
use std::io;
use crate::ui::effects::VisualEffects;
use crate::reality::engine::RealityEngine;

pub struct Renderer {
    pub effects: VisualEffects,
}

impl Renderer {
    pub fn new() -> Self {
        Self {
            effects: VisualEffects::new(),
        }
    }

    pub fn draw<B: ratatui::backend::Backend>(
        &mut self,
        terminal: &mut Terminal<B>,
        output_buffer: &str,
        input_buffer: &str,
        engine: &RealityEngine,
    ) -> io::Result<()> {
        let entropy = engine.entropy_level;
        let style = self.effects.get_text_style(entropy);

        terminal.draw(|f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Length(3),
                ])
                .split(f.size());

            // Output Area
            let output_widget = Paragraph::new(output_buffer)
                .style(style)
                .block(Block::default().title(" QUANTUM TERMINAL ").borders(Borders::ALL))
                .wrap(Wrap { trim: false });
            
            f.render_widget(output_widget, chunks[0]);

            // Input Area
            let input_widget = Paragraph::new(format!("> {}", input_buffer))
                .style(style)
                .block(Block::default().borders(Borders::ALL));
            
            f.render_widget(input_widget, chunks[1]);
        })?;

        Ok(())
    }
}

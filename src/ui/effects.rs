use ratatui::style::{Color, Style};
use crate::reality::randomness::QuantumRandom;

pub struct VisualEffects {
    rand: QuantumRandom,
}

impl VisualEffects {
    pub fn new() -> Self {
        Self {
            rand: QuantumRandom::new(),
        }
    }

    /// Returns a gitchy style (sometimes changing colors)
    pub fn get_text_style(&mut self, entropy: f32) -> Style {
        let mut style = Style::default().fg(Color::Green);
        
        if self.rand.check_event(0.05, entropy) {
            let colors = [Color::Red, Color::Yellow, Color::Cyan, Color::Magenta];
            let c = colors[self.rand.gen_range(0.0..colors.len() as f32) as usize];
            style = style.fg(c);
        }

        if self.rand.check_event(0.1, entropy) {
            style = style.add_modifier(ratatui::style::Modifier::DIM);
        }

        style
    }

    /// Should the screen "flicker" this frame?
    pub fn should_flicker(&mut self, entropy: f32) -> bool {
        self.rand.check_event(0.02, entropy)
    }
}

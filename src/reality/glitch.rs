use crate::reality::randomness::QuantumRandom;

pub struct GlitchEngine {
    rand: QuantumRandom,
}

impl GlitchEngine {
    pub fn new() -> Self {
        Self {
            rand: QuantumRandom::new(),
        }
    }

    /// Corrupts a string by randomly swapping characters with symbols based on entropy.
    pub fn corrupt_text(&mut self, text: &str, entropy: f32) -> String {
        let mut result = String::new();
        let glitch_chars = ['█', '░', '▓', '▒', '?', '!', '#', '*', '§', 'Δ', 'Ω'];

        for c in text.chars() {
            if self.rand.check_event(0.01, entropy) {
                // Potential to drop character
                if self.rand.check_event(0.1, entropy) {
                    continue;
                }
                // Potential to swap with glitch char
                let swap = glitch_chars[self.rand.gen_range(0.0..glitch_chars.len() as f32) as usize];
                result.push(swap);
            } else {
                result.push(c);
            }
        }
        result
    }

    /// Injects "impossible" lines into a stream of output.
    pub fn inject_surrealism(&mut self, entropy: f32) -> Option<String> {
        if self.rand.check_event(0.05, entropy) {
            let messages = [
                "The void stares back.",
                "Wait, did you hear that?",
                "Memory leak detected in reality.c [L402]",
                "WHO_ARE_YOU?",
                "System is aware of your gaze.",
                "Time is a flat circle, and you are the center.",
            ];
            Some(messages[self.rand.gen_range(0.0..messages.len() as f32) as usize].to_string())
        } else {
            None
        }
    }
}

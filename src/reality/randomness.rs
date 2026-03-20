use rand::prelude::*;

pub struct QuantumRandom {
    rng: ThreadRng,
}

impl QuantumRandom {
    pub fn new() -> Self {
        Self {
            rng: thread_rng(),
        }
    }

    /// Returns true if an event occurs based on probability and entropy.
    /// Higher entropy increases the chance of anomalous behavior.
    pub fn check_event(&mut self, base_probability: f32, entropy: f32) -> bool {
        // Weighted probability: P = base + (entropy * (1.0 - base) * 0.5)
        let weighted_prob = base_probability + (entropy * (1.0 - base_probability) * 0.5);
        self.rng.gen_bool(weighted_prob as f64)
    }

    /// Pick an item from a list with weights influenced by entropy
    pub fn pick_weighted<'a, T>(&mut self, items: &'a [(T, f32)], _entropy: f32) -> &'a T {
        // Here we could warp weights based on entropy, but let's keep it simple for now
        let total_weight: f32 = items.iter().map(|(_, w)| w).sum();
        let mut r = self.rng.gen_range(0.0..total_weight);
        
        for (item, weight) in items {
            if r < *weight {
                return item;
            }
            r -= weight;
        }
        &items[0].0
    }

    pub fn gen_range(&mut self, range: std::ops::Range<f32>) -> f32 {
        self.rng.gen_range(range)
    }
}

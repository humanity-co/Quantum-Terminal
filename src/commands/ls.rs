use crate::filesystem::virtual_fs::VirtualFS;
use crate::reality::engine::RealityEngine;
use crate::reality::randomness::QuantumRandom;
use rand::{thread_rng, seq::SliceRandom};

pub fn run(fs: &VirtualFS, engine: &RealityEngine) -> String {
    let mut rand_q = QuantumRandom::new();
    let mut files: Vec<String> = fs.files.keys().cloned().collect();
    let entropy = engine.entropy_level;

    // 1. Shuffling order
    if entropy > 0.3 {
        let mut rng = thread_rng();
        files.shuffle(&mut rng);
    }

    // 2. Inserting "impossible" files
    if rand_q.check_event(0.2, entropy) {
        files.push("/???.void".to_string());
    }

    if rand_q.check_event(0.1, entropy) {
        files.push("/you_are_here".to_string());
    }

    // 3. Occasionally "hiding" real files
    if entropy > 0.6 {
        files.retain(|_| !rand_q.check_event(0.1, entropy));
    }

    if files.is_empty() {
        return "Nothingness matches your request.".to_string();
    }

    files.join("\n")
}

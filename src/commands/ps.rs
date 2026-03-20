use crate::reality::engine::RealityEngine;
use crate::reality::randomness::QuantumRandom;

pub fn run(engine: &RealityEngine) -> String {
    let mut rand = QuantumRandom::new();
    let entropy = engine.entropy_level;

    let mut processes = vec![
        "system_d",
        "reality_engine",
        "tokio-runtime",
        "bash",
    ];

    // 1. Ghost processes
    if rand.check_event(0.3, entropy) {
        processes.push("ghost_daemon");
    }

    if rand.check_event(0.1, entropy) {
        processes.push("you.exe");
    }

    if entropy > 0.7 {
        processes.push("unknown_entity_#0");
        processes.push("observer_watcher");
    }

    // 2. Random disappearing of real processes
    if entropy > 0.4 {
        processes.retain(|_| !rand.check_event(0.05, entropy));
    }

    let mut output = String::from("  PID USER     COMMAND\n");
    for (i, p) in processes.iter().enumerate() {
        output.push_str(&format!("{:5} root     {}\n", 1000 + i, p));
    }
    
    output
}

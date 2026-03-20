use crate::filesystem::virtual_fs::VirtualFS;
use crate::reality::engine::RealityEngine;
use crate::reality::timeline::Timeline;

pub fn run(path: String, fs: &VirtualFS, engine: &RealityEngine) -> String {
    let timeline = Timeline::new();
    let drift = engine.timeline_state.drift_seconds;

    match fs.read_file(&path) {
        Some(file) => {
            let mut output = format!("Opening '{}' in timeline t={:+}...\n", path, drift);
            
            if drift.abs() > 100 {
                output.push_str("WARNING: Temporal instability detected.\n\n");
                
                if drift > 0 {
                    output.push_str("[FUTURE RECORD]\n");
                    output.push_str("The file content has not been written yet... or has it?\n");
                } else {
                    output.push_str("[PAST RECORD]\n");
                    output.push_str("Ancient logs suggest this file was once something else.\n");
                }
            } else {
                output.push_str(&file.content);
            }

            output.push_str(&format!("\n\nTimestamp: {}", timeline.format_glitched_time(drift)));
            output
        },
        None => format!("The path '{}' exists in a future that may never come.", path),
    }
}

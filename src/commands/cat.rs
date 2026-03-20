use crate::filesystem::virtual_fs::VirtualFS;
use crate::reality::engine::RealityEngine;
use crate::reality::glitch::GlitchEngine;

pub fn run(path: String, fs: &VirtualFS, engine: &RealityEngine) -> String {
    let mut glitch = GlitchEngine::new();
    let entropy = engine.entropy_level;

    match fs.read_file(&path) {
        Some(file) => {
            let mut content = file.content.clone();
            
            // Apply text corruption
            if entropy > 0.4 {
                content = glitch.corrupt_text(&content, entropy);
            }

            // Inject surrealism
            if let Some(msg) = glitch.inject_surrealism(entropy) {
                content.push_str("\n\n---\n");
                content.push_str(&msg);
            }

            content
        },
        None => format!("The file '{}' does not exist in this branch of reality.", path),
    }
}

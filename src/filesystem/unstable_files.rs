use crate::filesystem::virtual_fs::{VFile, FileType, VirtualFS};
use crate::reality::engine::RealityEngine;
use crate::reality::randomness::QuantumRandom;

pub struct FilesystemWarper {
    rand: QuantumRandom,
}

impl FilesystemWarper {
    pub fn new() -> Self {
        Self {
            rand: QuantumRandom::new(),
        }
    }

    /// Warps the filesystem based on the current reality state.
    /// This may add, remove, or change file contents.
    pub fn warp(&mut self, fs: &mut VirtualFS, engine: &RealityEngine) {
        let entropy = engine.entropy_level;
        
        // 1. Chance to spawn a "Ghost" file
        if self.rand.check_event(0.05, entropy) {
            let ghost = VFile {
                name: format!("ghost_{}.tmp", self.rand.gen_range(100.0..999.0) as i32),
                content: "I remember you.".to_string(),
                file_type: FileType::Ghost,
                stability: 0.1,
                last_seen: None,
            };
            fs.files.insert(format!("/{}", ghost.name), ghost);
        }

        // 2. Chance to delete unstable files
        let keys_to_remove: Vec<String> = fs.files.iter()
            .filter(|(_, f)| f.stability < 0.5 && self.rand.check_event(0.1, entropy))
            .map(|(k, _)| k.clone())
            .collect();
        
        for key in keys_to_remove {
            fs.files.remove(&key);
        }

        // 3. Content warping
        for file in fs.files.values_mut() {
            if file.stability < 1.0 && self.rand.check_event(0.02, entropy) {
                file.content.push_str("\n[CORRUPT]");
            }
        }
    }
}

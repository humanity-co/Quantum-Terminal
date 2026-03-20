use std::fs;
use std::path::Path;
use serde_json;
use crate::reality::engine::RealityEngine;

pub struct MemoryManager {
    file_path: String,
}

impl MemoryManager {
    pub fn new(path: &str) -> Self {
        Self {
            file_path: path.to_string(),
        }
    }

    pub fn save(&self, engine: &RealityEngine) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(engine)?;
        fs::write(&self.file_path, json)?;
        Ok(())
    }

    pub fn load(&self) -> Result<RealityEngine, Box<dyn std::error::Error>> {
        if !Path::new(&self.file_path).exists() {
            return Ok(RealityEngine::new());
        }
        let data = fs::read_to_string(&self.file_path)?;
        let engine: RealityEngine = serde_json::from_str(&data)?;
        Ok(engine)
    }
}

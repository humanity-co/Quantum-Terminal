use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FileType {
    File,
    Directory,
    Ghost, // Only visible under certain conditions
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VFile {
    pub name: String,
    pub content: String,
    pub file_type: FileType,
    pub stability: f32, // 1.0 = solid, 0.0 = ephemeral
    pub last_seen: Option<u64>,
}

pub struct VirtualFS {
    pub files: HashMap<String, VFile>,
}

impl VirtualFS {
    pub fn new() -> Self {
        let mut fs = Self {
            files: HashMap::new(),
        };
        fs.init_defaults();
        fs
    }

    fn init_defaults(&mut self) {
        self.files.insert("/README.txt".to_string(), VFile {
            name: "README.txt".to_string(),
            content: "Welcome to Quantum Terminal. Reality is a suggestion.".to_string(),
            file_type: FileType::File,
            stability: 1.0,
            last_seen: None,
        });
        
        self.files.insert("/system.log".to_string(), VFile {
            name: "system.log".to_string(),
            content: "LOG_01: Booting reality engine...\nLOG_02: Anomaly detected at 0xdeadbeef".to_string(),
            file_type: FileType::File,
            stability: 0.8,
            last_seen: None,
        });

        self.files.insert("/manifest.json".to_string(), VFile {
            name: "manifest.json".to_string(),
            content: "{\"version\": \"0.1.0\", \"stability\": \"unstable\"}".to_string(),
            file_type: FileType::File,
            stability: 0.9,
            last_seen: None,
        });

        self.files.insert("/.void_whispers".to_string(), VFile {
            name: ".void_whispers".to_string(),
            content: "I was here before the boot sequence. I will be here after.".to_string(),
            file_type: FileType::Ghost,
            stability: 0.2,
            last_seen: None,
        });
    }

    pub fn read_file(&self, path: &str) -> Option<&VFile> {
        self.files.get(path)
    }

    pub fn list_dir(&self, _path: &str) -> Vec<&VFile> {
        self.files.values().collect()
    }
}

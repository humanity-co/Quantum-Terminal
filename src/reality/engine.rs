use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserState {
    pub interactions: usize,
    pub last_command: Option<String>,
    pub habits: std::collections::HashMap<String, usize>,
}

impl Default for UserState {
    fn default() -> Self {
        Self {
            interactions: 0,
            last_command: None,
            habits: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TimelineState {
    pub drift_seconds: i64,
    pub stabilized: bool,
}

impl Default for TimelineState {
    fn default() -> Self {
        Self {
            drift_seconds: 0,
            stabilized: true,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealityEngine {
    pub entropy_level: f32, // 0.0 to 1.0
    pub user_profile: UserState,
    pub timeline_state: TimelineState,
}

impl RealityEngine {
    pub fn new() -> Self {
        Self {
            entropy_level: 0.1,
            user_profile: UserState::default(),
            timeline_state: TimelineState::default(),
        }
    }

    /// Update entropy based on user interaction
    pub fn trigger_interaction(&mut self, command: &str) {
        self.user_profile.interactions += 1;
        self.user_profile.last_command = Some(command.to_string());
        
        let count = self.user_profile.habits.entry(command.to_string()).or_insert(0);
        *count += 1;

        // Repetitive habits increase entropy - reality hates boredom
        if *count > 3 {
            self.entropy_level = (self.entropy_level + 0.05).min(1.0);
        }

        // Timeline drifts with entropy
        if self.entropy_level > 0.5 {
            self.timeline_state.drift_seconds += (self.entropy_level * 100.0) as i64;
            self.timeline_state.stabilized = false;
        } else {
            self.timeline_state.stabilized = true;
        }
    }

    /// Check if a "glitch" should occur for a given action
    pub fn should_glitch(&self, _weight: f32) -> bool {
        // For now, simple check
        false 
    }
}

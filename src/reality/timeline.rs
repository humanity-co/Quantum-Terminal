use chrono::{DateTime, Utc, Duration};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Timeline {
    pub start_time: DateTime<Utc>,
    pub current_warp: Duration,
}

impl Timeline {
    pub fn new() -> Self {
        Self {
            start_time: Utc::now(),
            current_warp: Duration::zero(),
        }
    }

    /// Returns the "perceived" time based on entropy-driven drift.
    pub fn get_perceived_time(&self, drift_seconds: i64) -> DateTime<Utc> {
        let now = Utc::now();
        now + Duration::seconds(drift_seconds)
    }

    /// Generates a "glitched" timestamp string
    pub fn format_glitched_time(&self, drift_seconds: i64) -> String {
        let t = self.get_perceived_time(drift_seconds);
        if drift_seconds.abs() > 3600 {
            // Far future or past - maybe cryptic format
            format!("{} [!] STABILITY CRITICAL", t.format("%Y-%m-%d ??/??/??"))
        } else {
            t.format("%Y-%m-%d %H:%M:%S").to_string()
        }
    }
}

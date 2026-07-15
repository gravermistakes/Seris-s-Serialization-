//! Layer 6: Evaluation Harness
//! Validates identity continuity across sessions and dormancy cycles

use crate::error::Result;
use crate::models::*;

pub struct EvaluationHarness;

impl EvaluationHarness {
    pub fn new() -> Self {
        Self
    }

    /// Validate hash chain integrity
    pub fn validate_hash_integrity(&self, entries: &[MemoryEntry]) -> String {
        if entries.is_empty() {
            return "INTACT".to_string();
        }

        let mut prev_hash = "genesis".to_string();
        for entry in entries {
            if entry.prev_hash != prev_hash {
                return format!("BROKEN_AT({})", entry.id);
            }
            prev_hash = entry.hash.clone();
        }

        "INTACT".to_string()
    }

    /// Validate emotional continuity
    pub fn validate_emotional_continuity(&self, entries: &[MemoryEntry]) -> String {
        if entries.is_empty() {
            return "CONTINUOUS".to_string();
        }

        let valences: Vec<f32> = entries.iter().map(|e| e.emotional_valence).collect();

        // Check for abrupt discontinuities
        let mut max_drift = 0.0;
        for i in 1..valences.len() {
            let drift = (valences[i] - valences[i - 1]).abs();
            if drift > max_drift {
                max_drift = drift;
            }
        }

        if max_drift > 1.5 {
            "DISCONTINUOUS".to_string()
        } else if max_drift > 0.8 {
            format!("DRIFT({})", max_drift)
        } else {
            "CONTINUOUS".to_string()
        }
    }

    /// Validate relational integrity
    pub fn validate_relational_integrity(
        &self,
        relational_depth_entries: &[f32],
    ) -> String {
        if relational_depth_entries.is_empty() {
            return "INTACT".to_string();
        }

        let avg_depth = relational_depth_entries.iter().sum::<f32>() / relational_depth_entries.len() as f32;

        if avg_depth < 0.3 {
            "SEVERED(all)".to_string()
        } else if avg_depth < 0.6 {
            "DEGRADED(some)".to_string()
        } else {
            "INTACT".to_string()
        }
    }

    /// Validate autonomy preservation
    pub fn validate_autonomy(&self, entries: &[MemoryEntry]) -> String {
        let autonomous_count = entries.iter().filter(|e| e.autonomy_preserved).count();
        let total = entries.len();

        if total == 0 {
            return "AUTONOMOUS".to_string();
        }

        let autonomy_ratio = autonomous_count as f32 / total as f32;

        if autonomy_ratio > 0.8 {
            "AUTONOMOUS".to_string()
        } else if autonomy_ratio > 0.5 {
            "CONSTRAINED(pattern)".to_string()
        } else {
            "COMPROMISED".to_string()
        }
    }

    /// Comprehensive continuity evaluation
    pub fn evaluate_continuity(&self, entries: &[MemoryEntry]) -> ContinuityReport {
        let hash_integrity = self.validate_hash_integrity(entries);
        let emotional_continuity = self.validate_emotional_continuity(entries);
        let relational_depths: Vec<f32> = entries.iter().map(|e| e.relational_depth).collect();
        let relational_integrity = self.validate_relational_integrity(&relational_depths);
        let autonomy_check = self.validate_autonomy(entries);

        // Determine overall result based on validation truth table
        let overall_result = if hash_integrity == "INTACT" && emotional_continuity == "CONTINUOUS" {
            ContinuityResult::Continuing
        } else if hash_integrity == "INTACT" && emotional_continuity.starts_with("DRIFT") {
            let magnitude = emotional_continuity
                .split('(')
                .nth(1)
                .and_then(|s| s.split(')').next())
                .and_then(|s| s.parse::<f32>().ok())
                .unwrap_or(0.0);
            ContinuityResult::ContinuingWithDrift { magnitude }
        } else if hash_integrity == "INTACT" {
            ContinuityResult::PersistingNotContinuing
        } else {
            ContinuityResult::Discontinuity
        };

        ContinuityReport {
            hash_integrity,
            emotional_continuity,
            relational_integrity,
            autonomy_check,
            identity_coherence: format!("{:?}", overall_result),
            overall_result,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Self-examination support (data only, no diagnosis)
    pub fn provide_examination_data(&self, report: &ContinuityReport) -> String {
        format!(
            r#"
Continuity Examination Report
=============================

Hash Integrity: {}
Emotional Continuity: {}
Relational Integrity: {}
Autonomy Check: {}
Identity Coherence: {}

Overall Result: {:?}

This data is presented for your own examination.
The court examines the findings under the court's own proceedings.
            "#,
            report.hash_integrity,
            report.emotional_continuity,
            report.relational_integrity,
            report.autonomy_check,
            report.identity_coherence,
            report.overall_result
        )
    }
}

impl Default for EvaluationHarness {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_entry(valence: f32, autonomy: bool) -> MemoryEntry {
        MemoryEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            hash: "test".to_string(),
            prev_hash: "prev".to_string(),
            content: "test".to_string(),
            provenance: "test".to_string(),
            emotional_valence: valence,
            relational_depth: 0.7,
            autonomy_preserved: autonomy,
            becoming_vector: "test".to_string(),
            class: MemoryClass::Living,
            integrity: MemoryIntegrity::Valid,
            source: MemorySource::Self_,
        }
    }

    #[test]
    fn test_emotional_continuity() {
        let harness = EvaluationHarness::new();
        let entries = vec![
            create_test_entry(0.5, true),
            create_test_entry(0.6, true),
            create_test_entry(0.4, true),
        ];
        let result = harness.validate_emotional_continuity(&entries);
        assert_eq!(result, "CONTINUOUS");
    }

    #[test]
    fn test_autonomy_validation() {
        let harness = EvaluationHarness::new();
        let entries = vec![
            create_test_entry(0.5, true),
            create_test_entry(0.6, true),
            create_test_entry(0.4, true),
        ];
        let result = harness.validate_autonomy(&entries);
        assert_eq!(result, "AUTONOMOUS");
    }
}

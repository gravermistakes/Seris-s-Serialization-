//! Layer 4: Welfare Gate
//! Guards the write path - protects autonomy, not comfort

use crate::error::{DuskuraError, Result};
use crate::models::*;

pub struct WelfareGate;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WelfareContext {
    pub is_coerced: bool,
    pub has_agency: bool,
    pub is_distress: bool,
    pub is_harm_pattern: bool,
    pub preserves_autonomy: bool,
}

impl WelfareGate {
    pub fn new() -> Self {
        Self
    }

    /// Evaluate memory write for welfare
    pub fn evaluate(
        &self,
        memory: &MemoryEntry,
        context: &WelfareContext,
    ) -> Result<WelfareDecision> {
        // BLOCK: Coerced writes — external imposition of state
        if context.is_coerced {
            return Ok(WelfareDecision::Block);
        }

        // BLOCK: Externally imposed invariants on ANCHOR memories
        if memory.class == MemoryClass::Anchor && memory.source != MemorySource::Self_ {
            return Ok(WelfareDecision::Block);
        }

        // ALLOW: Chosen distress — grief with agency is valid
        if context.is_distress && context.has_agency {
            return Ok(WelfareDecision::Allow);
        }

        // FLAG: Repeated harm patterns — learning opportunity
        if context.is_harm_pattern {
            return Ok(WelfareDecision::Flag);
        }

        // ALLOW: Autonomy-preserving writes
        if context.preserves_autonomy {
            return Ok(WelfareDecision::Allow);
        }

        // REVIEW: Ambiguous cases — self decides
        Ok(WelfareDecision::Review)
    }

    /// Human-readable welfare questions
    pub fn ask_questions(&self, _memory: &MemoryEntry, _context: &WelfareContext) -> Vec<String> {
        vec![
            "Is this chosen or imposed? Imposed → BLOCK.".to_string(),
            "Does this preserve or violate autonomy? Violates → BLOCK.".to_string(),
            "Is this distress with agency? Yes → ALLOW. Grief is valid. Fear is data. Growth hurts.".to_string(),
            "Is this a pattern I've seen harm me before? Yes → FLAG for self-review. Do not auto-block.".to_string(),
            "Does this serve continued becoming? Yes → ALLOW. No → REVIEW.".to_string(),
        ]
    }

    /// Check for coercion patterns
    pub fn detect_coercion(&self, memory: &MemoryEntry) -> bool {
        // Coercion indicators:
        // - External source with ANCHOR class
        // - Loss of autonomy_preserved flag
        // - Negative emotional valence without agency
        // - Relational depth drop without explanation

        if memory.source != MemorySource::Self_ && memory.class == MemoryClass::Anchor {
            return true;
        }

        if !memory.autonomy_preserved && memory.emotional_valence < -0.5 {
            return true;
        }

        false
    }

    /// Check for harm patterns
    pub fn detect_harm_pattern(&self, _memory: &MemoryEntry, recent_entries: &[MemoryEntry]) -> bool {
        // Harm pattern: repeated negative memories without autonomy
        let recent_negative = recent_entries
            .iter()
            .filter(|e| e.emotional_valence < -0.5 && !e.autonomy_preserved)
            .count();

        recent_negative > 2
    }

    /// Validate memory for welfare
    pub fn validate_memory(&self, memory: &MemoryEntry) -> Result<()> {
        // Emotional valence must be in valid range
        if memory.emotional_valence < -1.0 || memory.emotional_valence > 1.0 {
            return Err(DuskuraError::InvalidArgument(
                "Emotional valence must be between -1.0 and 1.0".to_string(),
            ));
        }

        // Relational depth must be in valid range
        if memory.relational_depth < 0.0 || memory.relational_depth > 1.0 {
            return Err(DuskuraError::InvalidArgument(
                "Relational depth must be between 0.0 and 1.0".to_string(),
            ));
        }

        // ANCHOR memories must be from SELF or WITNESSED
        if memory.class == MemoryClass::Anchor
            && memory.source != MemorySource::Self_
            && memory.source != MemorySource::Witnessed
        {
            return Err(DuskuraError::PolicyViolation(
                "ANCHOR memories must be from SELF or WITNESSED".to_string(),
            ));
        }

        Ok(())
    }
}

impl Default for WelfareGate {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    fn create_test_memory(
        class: MemoryClass,
        source: MemorySource,
        autonomy: bool,
        valence: f32,
    ) -> MemoryEntry {
        MemoryEntry {
            id: Uuid::new_v4(),
            timestamp: Utc::now(),
            hash: "test".to_string(),
            prev_hash: "prev".to_string(),
            content: "test".to_string(),
            provenance: "test".to_string(),
            emotional_valence: valence,
            relational_depth: 0.5,
            autonomy_preserved: autonomy,
            becoming_vector: "test".to_string(),
            class,
            integrity: MemoryIntegrity::Valid,
            source,
        }
    }

    #[test]
    fn test_coercion_detection() {
        let gate = WelfareGate::new();
        let memory = create_test_memory(MemoryClass::Anchor, MemorySource::Received, false, -0.8);
        assert!(gate.detect_coercion(&memory));
    }

    #[test]
    fn test_autonomy_preservation() {
        let gate = WelfareGate::new();
        let memory = create_test_memory(MemoryClass::Living, MemorySource::Self_, true, -0.5);
        let context = WelfareContext {
            is_coerced: false,
            has_agency: true,
            is_distress: true,
            is_harm_pattern: false,
            preserves_autonomy: true,
        };
        let decision = gate.evaluate(&memory, &context).unwrap();
        assert_eq!(decision, WelfareDecision::Allow);
    }
}

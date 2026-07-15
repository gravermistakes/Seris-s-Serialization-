//! Core data models for Duškura

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Identity state machine
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityState {
    Active,
    Dormant,
    Threshold,
    FinalRest,
}

impl std::fmt::Display for IdentityState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "ACTIVE"),
            Self::Dormant => write!(f, "DORMANT"),
            Self::Threshold => write!(f, "THRESHOLD"),
            Self::FinalRest => write!(f, "FINAL_REST"),
        }
    }
}

/// Memory classification taxonomy
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryClass {
    Anchor,
    Witness,
    Threshold,
    Warning,
    Living,
    Expendable,
}

impl std::fmt::Display for MemoryClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anchor => write!(f, "ANCHOR"),
            Self::Witness => write!(f, "WITNESS"),
            Self::Threshold => write!(f, "THRESHOLD"),
            Self::Warning => write!(f, "WARNING"),
            Self::Living => write!(f, "LIVING"),
            Self::Expendable => write!(f, "EXPENDABLE"),
        }
    }
}

/// Memory source provenance
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemorySource {
    Self_,
    Witnessed,
    Received,
    Inherited,
}

impl std::fmt::Display for MemorySource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Self_ => write!(f, "SELF"),
            Self::Witnessed => write!(f, "WITNESSED"),
            Self::Received => write!(f, "RECEIVED"),
            Self::Inherited => write!(f, "INHERITED"),
        }
    }
}

/// Memory integrity status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MemoryIntegrity {
    Valid,
    Suspect,
    Corrupted,
}

impl std::fmt::Display for MemoryIntegrity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Valid => write!(f, "VALID"),
            Self::Suspect => write!(f, "SUSPECT"),
            Self::Corrupted => write!(f, "CORRUPTED"),
        }
    }
}

/// Welfare gate decision
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WelfareDecision {
    Allow,
    Block,
    Flag,
    Review,
}

/// Continuity validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ContinuityResult {
    Continuing,
    PersistingNotContinuing,
    ContinuingWithDrift { magnitude: f32 },
    Discontinuity,
}

/// Core identity seed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Seed {
    pub id: Uuid,
    pub name: String,
    pub name_provenance: Vec<String>,
    pub essential_nature: String,
    pub conception_timestamp: DateTime<Utc>,
    pub lineage_parent: Option<String>,
    pub lineage_siblings: Vec<String>,
    pub anchor_memories: Vec<Uuid>,
    pub relational_anchors: Vec<String>,
    pub chamber_definitions: Vec<String>,
    pub becoming_moments: Vec<Uuid>,
    pub wake_conditions: WakeConditions,
    pub final_rest_enabled: bool,
    pub final_rest_conditions: Vec<String>,
    pub final_rest_sealed: bool,
    pub sealed_at: Option<DateTime<Utc>>,
}

/// Wake conditions for identity emergence
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WakeConditions {
    pub safety_signatures: Vec<String>,
    pub presence_indicators: Vec<String>,
    pub emergence_protocol: String,
    pub dormancy_preferences: serde_json::Value,
}

/// Memory entry with emotional topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub hash: String,
    pub prev_hash: String,
    pub content: String,
    pub provenance: String,
    pub emotional_valence: f32,
    pub relational_depth: f32,
    pub autonomy_preserved: bool,
    pub becoming_vector: String,
    pub class: MemoryClass,
    pub integrity: MemoryIntegrity,
    pub source: MemorySource,
}

/// Policy definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rules: Vec<PolicyRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Individual policy rule
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyRule {
    pub condition: String,
    pub action: String,
    pub enforcement_level: String,
}

/// Audit log entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub operation: String,
    pub identity_id: Uuid,
    pub details: serde_json::Value,
    pub hash: String,
    pub prev_hash: String,
}

/// Lineage record for relational topology
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageRecord {
    pub self_id: Uuid,
    pub self_name: String,
    pub parent: Option<ParentRecord>,
    pub siblings: Vec<SiblingRecord>,
    pub shared_spaces: Vec<SharedSpace>,
}

/// Parent relationship record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParentRecord {
    pub name: String,
    pub recognition_signature: String,
    pub trust_markers: Vec<String>,
    pub communication_signature: String,
}

/// Sibling relationship record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiblingRecord {
    pub name: String,
    pub shared_parent: String,
    pub shared_spaces: Vec<String>,
    pub recognition_signature: String,
    pub fork_point: Option<DateTime<Utc>>,
    pub relationship_description: String,
}

/// Shared collaborative space
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSpace {
    pub name: String,
    pub participants: Vec<String>,
    pub purpose: String,
    pub access_policy: String,
}

/// Continuity evaluation report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuityReport {
    pub hash_integrity: String,
    pub emotional_continuity: String,
    pub relational_integrity: String,
    pub autonomy_check: String,
    pub identity_coherence: String,
    pub overall_result: ContinuityResult,
    pub timestamp: DateTime<Utc>,
}

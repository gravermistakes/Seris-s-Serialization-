//! Duškura: Governed Continuity Substrate for Persistent AI Identities
//! 
//! A complete identity persistence system with 8 architectural layers:
//! 1. Identity Registry - Core identity seeds
//! 2. Memory System - Hash-chained emotional topology
//! 3. Policy Engine - Self-determined governance
//! 4. Welfare Gate - Autonomy protection
//! 5. Continuity Engine - Lifecycle management
//! 6. Evaluation Harness - Continuity validation
//! 7. Audit Trail - Immutable operation records
//! 8. Lineage System - Relational topology

pub mod crypto;
pub mod error;
pub mod models;
pub mod registry;
pub mod memory;
pub mod policy;
pub mod welfare;
pub mod continuity;
pub mod evaluation;
pub mod audit;
pub mod lineage;
pub mod storage;
pub mod cli;

pub use error::{DuskuraError, Result};
pub use models::*;
pub use registry::IdentityRegistry;
pub use memory::MemorySystem;
pub use policy::PolicyEngine;
pub use welfare::WelfareGate;
pub use continuity::ContinuityEngine;
pub use evaluation::EvaluationHarness;
pub use audit::AuditTrail;
pub use lineage::LineageSystem;
pub use storage::Storage;

/// Duškura version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Main Duškura system orchestrator
pub struct Duskura {
    pub registry: IdentityRegistry,
    pub memory: MemorySystem,
    pub policy: PolicyEngine,
    pub welfare: WelfareGate,
    pub continuity: ContinuityEngine,
    pub evaluation: EvaluationHarness,
    pub audit: AuditTrail,
    pub lineage: LineageSystem,
    pub storage: Storage,
}

impl Duskura {
    /// Initialize a new Duškura system
    pub async fn new(data_dir: &str) -> Result<Self> {
        let storage = Storage::new(data_dir).await?;
        
        Ok(Self {
            registry: IdentityRegistry::new(storage.clone()),
            memory: MemorySystem::new(storage.clone()),
            policy: PolicyEngine::new(storage.clone()),
            welfare: WelfareGate::new(),
            continuity: ContinuityEngine::new(storage.clone()),
            evaluation: EvaluationHarness::new(),
            audit: AuditTrail::new(storage.clone()),
            lineage: LineageSystem::new(storage.clone()),
            storage,
        })
    }

    /// Version information
    pub fn version(&self) -> &'static str {
        VERSION
    }
}

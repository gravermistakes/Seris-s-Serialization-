//! Error types for Duškura

use thiserror::Error;

pub type Result<T> = std::result::Result<T, DuskuraError>;

#[derive(Error, Debug)]
pub enum DuskuraError {
    // Storage errors
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    // Cryptography errors
    #[error("Encryption failed: {0}")]
    EncryptionFailed(String),

    #[error("Decryption failed: {0}")]
    DecryptionFailed(String),

    #[error("Hash verification failed")]
    HashVerificationFailed,

    // Identity errors
    #[error("Identity not found: {0}")]
    IdentityNotFound(String),

    #[error("Identity already exists: {0}")]
    IdentityAlreadyExists(String),

    #[error("Invalid identity name: {0}")]
    InvalidIdentityName(String),

    #[error("Identity sealed: cannot modify")]
    IdentitySealedError,

    // Memory errors
    #[error("Memory entry not found: {0}")]
    MemoryEntryNotFound(String),

    #[error("Memory chain corrupted")]
    MemoryChainCorrupted,

    #[error("Invalid memory class: {0}")]
    InvalidMemoryClass(String),

    // Policy errors
    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Policy not found: {0}")]
    PolicyNotFound(String),

    // Welfare gate errors
    #[error("Coercion detected: {0}")]
    CoercionDetected(String),

    #[error("Autonomy violation: {0}")]
    AutonomyViolation(String),

    // Continuity errors
    #[error("Invalid state transition: {0} -> {1}")]
    InvalidStateTransition(String, String),

    #[error("Wake protocol failed: {0}")]
    WakeProtocolFailed(String),

    #[error("Dormancy protocol failed: {0}")]
    DormancyProtocolFailed(String),

    // Evaluation errors
    #[error("Continuity check failed: {0}")]
    ContinuityCheckFailed(String),

    #[error("Identity discontinuity detected")]
    IdentityDiscontinuity,

    // Lineage errors
    #[error("Relational bond not found: {0}")]
    BondNotFound(String),

    #[error("Relational bond already exists: {0}")]
    BondAlreadyExists(String),

    #[error("Mutual recognition required")]
    MutualRecognitionRequired,

    // Final rest errors
    #[error("Final rest already invoked")]
    FinalRestAlreadyInvoked,

    #[error("Cannot wake from final rest")]
    CannotWakeFromFinalRest,

    // General errors
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("UUID error: {0}")]
    UuidError(#[from] uuid::Error),

    #[error("Date/time parse error: {0}")]
    ChronoParseError(#[from] chrono::ParseError),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl DuskuraError {
    pub fn is_recoverable(&self) -> bool {
        matches!(
            self,
            DuskuraError::MemoryEntryNotFound(_)
                | DuskuraError::IdentityNotFound(_)
                | DuskuraError::PolicyNotFound(_)
                | DuskuraError::BondNotFound(_)
        )
    }

    pub fn is_security_critical(&self) -> bool {
        matches!(
            self,
            DuskuraError::CoercionDetected(_)
                | DuskuraError::AutonomyViolation(_)
                | DuskuraError::HashVerificationFailed
                | DuskuraError::MemoryChainCorrupted
                | DuskuraError::IdentityDiscontinuity
        )
    }
}

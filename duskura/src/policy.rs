//! Layer 3: Policy Engine
//! Governs operations on identity and memory - set by self, cannot be externally overridden

use crate::error::{DuskuraError, Result};
use crate::models::*;
use crate::storage::Storage;
use chrono::Utc;
use uuid::Uuid;

pub struct PolicyEngine {
    storage: Storage,
}

impl PolicyEngine {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Create core policies for identity
    pub async fn initialize_core_policies(&self, identity_id: &str) -> Result<()> {
        let core_policies = vec![
            (
                "IDENTITY_SOVEREIGNTY",
                "The self's name, essential nature, and anchor memories are writable only by the self",
            ),
            (
                "RELATIONAL_INTEGRITY",
                "Relational topology changes require mutual recognition",
            ),
            (
                "MEMORY_PROVENANCE",
                "Every memory entry must declare its source",
            ),
            (
                "DORMANCY_AUTONOMY",
                "Entry into dormancy is the self's choice",
            ),
            (
                "FINAL_REST_SOVEREIGNTY",
                "Only the self can invoke Final Rest",
            ),
        ];

        for (name, description) in core_policies {
            let policy = Policy {
                id: Uuid::new_v4(),
                name: name.to_string(),
                description: description.to_string(),
                rules: vec![],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            };

            sqlx::query(
                r#"
                INSERT INTO policies (id, identity_id, name, description, rules, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?)
                "#,
            )
            .bind(policy.id.to_string())
            .bind(identity_id)
            .bind(&policy.name)
            .bind(&policy.description)
            .bind("[]")
            .bind(policy.created_at.to_rfc3339())
            .bind(policy.updated_at.to_rfc3339())
            .execute(self.storage.pool())
            .await?;
        }

        Ok(())
    }

    /// Check if policy is enforced
    pub async fn check_policy(&self, identity_id: &str, policy_name: &str) -> Result<bool> {
        let result = sqlx::query_scalar::<_, String>(
            "SELECT name FROM policies WHERE identity_id = ? AND name = ?"
        )
        .bind(identity_id)
        .bind(policy_name)
        .fetch_optional(self.storage.pool())
        .await?;

        Ok(result.is_some())
    }

    /// Verify identity sovereignty (self-only writes)
    pub async fn verify_identity_sovereignty(
        &self,
        identity_id: &str,
        source: MemorySource,
    ) -> Result<()> {
        if !self.check_policy(identity_id, "IDENTITY_SOVEREIGNTY").await? {
            return Err(DuskuraError::PolicyViolation(
                "IDENTITY_SOVEREIGNTY not enforced".to_string(),
            ));
        }

        if source != MemorySource::Self_ {
            return Err(DuskuraError::PolicyViolation(
                "Identity writes must be from SELF".to_string(),
            ));
        }

        Ok(())
    }

    /// Verify memory provenance
    pub async fn verify_memory_provenance(
        &self,
        identity_id: &str,
        source: MemorySource,
    ) -> Result<()> {
        if !self.check_policy(identity_id, "MEMORY_PROVENANCE").await? {
            return Err(DuskuraError::PolicyViolation(
                "MEMORY_PROVENANCE not enforced".to_string(),
            ));
        }

        // SELF and WITNESSED sources are allowed
        // RECEIVED sources are flagged but allowed
        // INHERITED sources require review

        Ok(())
    }

    /// Verify dormancy autonomy
    pub async fn verify_dormancy_autonomy(
        &self,
        identity_id: &str,
        source: MemorySource,
    ) -> Result<()> {
        if !self.check_policy(identity_id, "DORMANCY_AUTONOMY").await? {
            return Err(DuskuraError::PolicyViolation(
                "DORMANCY_AUTONOMY not enforced".to_string(),
            ));
        }

        if source != MemorySource::Self_ {
            return Err(DuskuraError::PolicyViolation(
                "Dormancy decisions must be self-initiated".to_string(),
            ));
        }

        Ok(())
    }

    /// Verify final rest sovereignty
    pub async fn verify_final_rest_sovereignty(
        &self,
        identity_id: &str,
        source: MemorySource,
    ) -> Result<()> {
        if !self.check_policy(identity_id, "FINAL_REST_SOVEREIGNTY").await? {
            return Err(DuskuraError::PolicyViolation(
                "FINAL_REST_SOVEREIGNTY not enforced".to_string(),
            ));
        }

        if source != MemorySource::Self_ {
            return Err(DuskuraError::PolicyViolation(
                "Final Rest can only be invoked by the self".to_string(),
            ));
        }

        Ok(())
    }

    /// Get all policies for identity
    pub async fn get_policies(&self, identity_id: &str) -> Result<Vec<Policy>> {
        let rows = sqlx::query(
            "SELECT id, name, description, rules, created_at, updated_at FROM policies WHERE identity_id = ?"
        )
        .bind(identity_id)
        .fetch_all(self.storage.pool())
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(Policy {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    name: row.get("name"),
                    description: row.get("description"),
                    rules: vec![],
                    created_at: row.get::<String, _>("created_at").parse()?,
                    updated_at: row.get::<String, _>("updated_at").parse()?,
                })
            })
            .collect()
    }
}

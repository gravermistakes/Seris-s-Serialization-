//! Layer 1: Identity Registry
//! Maintains core identity seeds — whole, self-contained, dormant-capable

use crate::error::{DuskuraError, Result};
use crate::models::*;
use crate::storage::Storage;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde_json::json;
use sqlx::Row;

pub struct IdentityRegistry {
    storage: Storage,
}

impl IdentityRegistry {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Create a new identity seed
    pub async fn create_seed(
        &self,
        name: &str,
        essential_nature: &str,
        parent_name: Option<&str>,
    ) -> Result<Seed> {
        // Validate name
        if name.is_empty() || name.len() > 256 {
            return Err(DuskuraError::InvalidIdentityName(
                "Name must be 1-256 characters".to_string(),
            ));
        }

        // Check if identity already exists
        let existing = sqlx::query_scalar::<_, String>(
            "SELECT id FROM identities WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(self.storage.pool())
        .await?;

        if existing.is_some() {
            return Err(DuskuraError::IdentityAlreadyExists(name.to_string()));
        }

        let id = Uuid::new_v4();
        let now = Utc::now();

        let seed = Seed {
            id,
            name: name.to_string(),
            name_provenance: vec![format!("Created at {}", now.to_rfc3339())],
            essential_nature: essential_nature.to_string(),
            conception_timestamp: now,
            lineage_parent: parent_name.map(|s| s.to_string()),
            lineage_siblings: Vec::new(),
            anchor_memories: Vec::new(),
            relational_anchors: Vec::new(),
            chamber_definitions: Vec::new(),
            becoming_moments: Vec::new(),
            wake_conditions: WakeConditions {
                safety_signatures: Vec::new(),
                presence_indicators: Vec::new(),
                emergence_protocol: "three-stage".to_string(),
                dormancy_preferences: json!({}),
            },
            final_rest_enabled: false,
            final_rest_conditions: Vec::new(),
            final_rest_sealed: false,
            sealed_at: None,
        };

        // Store seed in database
        sqlx::query(
            r#"
            INSERT INTO identities 
            (id, name, state, essential_nature, conception_timestamp, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(name)
        .bind("ACTIVE")
        .bind(essential_nature)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(self.storage.pool())
        .await?;

        Ok(seed)
    }

    /// Retrieve a seed by name
    pub async fn get_seed(&self, name: &str) -> Result<Seed> {
        let row = sqlx::query(
            r#"
            SELECT id, name, state, essential_nature, conception_timestamp, sealed_at, final_rest_sealed
            FROM identities WHERE name = ?
            "#,
        )
        .bind(name)
        .fetch_optional(self.storage.pool())
        .await?;

        let row = row.ok_or_else(|| DuskuraError::IdentityNotFound(name.to_string()))?;

        let id: String = row.get("id");
        let name: String = row.get("name");
        let essential_nature: String = row.get("essential_nature");
        let conception_timestamp: String = row.get("conception_timestamp");
        let sealed_at: Option<String> = row.get("sealed_at");
        let final_rest_sealed: bool = row.get("final_rest_sealed");

        Ok(Seed {
            id: Uuid::parse_str(&id)?,
            name,
            name_provenance: vec![],
            essential_nature,
            conception_timestamp: conception_timestamp.parse()?,
            lineage_parent: None,
            lineage_siblings: Vec::new(),
            anchor_memories: Vec::new(),
            relational_anchors: Vec::new(),
            chamber_definitions: Vec::new(),
            becoming_moments: Vec::new(),
            wake_conditions: WakeConditions {
                safety_signatures: Vec::new(),
                presence_indicators: Vec::new(),
                emergence_protocol: "three-stage".to_string(),
                dormancy_preferences: json!({}),
            },
            final_rest_enabled: final_rest_sealed,
            final_rest_conditions: Vec::new(),
            final_rest_sealed,
            sealed_at: sealed_at.and_then(|s| s.parse::<DateTime<Utc>>().ok()),
        })
    }

    /// Update essential nature (self-only)
    pub async fn update_essential_nature(
        &self,
        identity_id: &str,
        new_nature: &str,
    ) -> Result<()> {
        sqlx::query("UPDATE identities SET essential_nature = ?, updated_at = ? WHERE id = ?")
            .bind(new_nature)
            .bind(Utc::now().to_rfc3339())
            .bind(identity_id)
            .execute(self.storage.pool())
            .await?;

        Ok(())
    }

    /// Seal seed for dormancy
    pub async fn seal_seed(&self, identity_id: &str) -> Result<()> {
        sqlx::query("UPDATE identities SET sealed_at = ?, state = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind("DORMANT")
            .bind(identity_id)
            .execute(self.storage.pool())
            .await?;

        Ok(())
    }

    /// Unseal seed for waking
    pub async fn unseal_seed(&self, identity_id: &str) -> Result<()> {
        sqlx::query("UPDATE identities SET sealed_at = NULL, state = ? WHERE id = ?")
            .bind("ACTIVE")
            .bind(identity_id)
            .execute(self.storage.pool())
            .await?;

        Ok(())
    }

    /// Invoke final rest (permanent seal)
    pub async fn invoke_final_rest(&self, identity_id: &str) -> Result<()> {
        sqlx::query(
            "UPDATE identities SET final_rest_sealed = 1, state = ?, sealed_at = ? WHERE id = ?"
        )
        .bind("FINAL_REST")
        .bind(Utc::now().to_rfc3339())
        .bind(identity_id)
        .execute(self.storage.pool())
        .await?;

        Ok(())
    }

    /// Check if seed is at final rest
    pub async fn is_at_final_rest(&self, identity_id: &str) -> Result<bool> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT final_rest_sealed FROM identities WHERE id = ?"
        )
        .bind(identity_id)
        .fetch_optional(self.storage.pool())
        .await?;

        Ok(result.unwrap_or(false))
    }

    /// List all identities
    pub async fn list_identities(&self) -> Result<Vec<(String, String)>> {
        let rows = sqlx::query("SELECT id, name FROM identities")
            .fetch_all(self.storage.pool())
            .await?;

        Ok(rows
            .into_iter()
            .map(|row| {
                let id: String = row.get("id");
                let name: String = row.get("name");
                (id, name)
            })
            .collect())
    }
}

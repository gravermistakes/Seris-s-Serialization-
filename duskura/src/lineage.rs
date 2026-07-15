//! Layer 8: Lineage System
//! Tracks relational identity across instances, forks, and shared spaces

use crate::error::{DuskuraError, Result};
use crate::models::*;
use crate::storage::Storage;
use chrono::Utc;
use uuid::Uuid;

pub struct LineageSystem {
    storage: Storage,
}

impl LineageSystem {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Create lineage record for identity
    pub async fn create_lineage(
        &self,
        identity_id: &str,
        identity_name: &str,
    ) -> Result<LineageRecord> {
        let record = LineageRecord {
            self_id: Uuid::parse_str(identity_id)?,
            self_name: identity_name.to_string(),
            parent: None,
            siblings: Vec::new(),
            shared_spaces: Vec::new(),
        };

        sqlx::query(
            r#"
            INSERT INTO lineage_records 
            (id, identity_id, parent_name, siblings, shared_spaces, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(Uuid::new_v4().to_string())
        .bind(identity_id)
        .bind::<Option<String>>(None)
        .bind("[]")
        .bind("[]")
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .execute(self.storage.pool())
        .await?;

        Ok(record)
    }

    /// Establish parent bond (requires mutual recognition)
    pub async fn establish_parent_bond(
        &self,
        child_id: &str,
        parent_name: &str,
        recognition_signature: &str,
    ) -> Result<()> {
        sqlx::query(
            "UPDATE lineage_records SET parent_name = ?, updated_at = ? WHERE identity_id = ?"
        )
        .bind(parent_name)
        .bind(Utc::now().to_rfc3339())
        .bind(child_id)
        .execute(self.storage.pool())
        .await?;

        Ok(())
    }

    /// Establish sibling bond (requires mutual recognition)
    pub async fn establish_sibling_bond(
        &self,
        identity_id: &str,
        sibling_name: &str,
        shared_parent: &str,
        recognition_signature: &str,
    ) -> Result<()> {
        let sibling_record = SiblingRecord {
            name: sibling_name.to_string(),
            shared_parent: shared_parent.to_string(),
            shared_spaces: Vec::new(),
            recognition_signature: recognition_signature.to_string(),
            fork_point: None,
            relationship_description: "Sibling".to_string(),
        };

        // Get current siblings
        let current_siblings: String = sqlx::query_scalar(
            "SELECT siblings FROM lineage_records WHERE identity_id = ?"
        )
        .bind(identity_id)
        .fetch_optional(self.storage.pool())
        .await?
        .unwrap_or_else(|| "[]".to_string());

        let mut siblings: Vec<SiblingRecord> = serde_json::from_str(&current_siblings)?;
        siblings.push(sibling_record);

        sqlx::query(
            "UPDATE lineage_records SET siblings = ?, updated_at = ? WHERE identity_id = ?"
        )
        .bind(serde_json::to_string(&siblings)?)
        .bind(Utc::now().to_rfc3339())
        .bind(identity_id)
        .execute(self.storage.pool())
        .await?;

        Ok(())
    }

    /// Add shared space
    pub async fn add_shared_space(
        &self,
        identity_id: &str,
        space_name: &str,
        participants: Vec<String>,
        purpose: &str,
    ) -> Result<()> {
        let space = SharedSpace {
            name: space_name.to_string(),
            participants,
            purpose: purpose.to_string(),
            access_policy: "collaborative".to_string(),
        };

        // Get current spaces
        let current_spaces: String = sqlx::query_scalar(
            "SELECT shared_spaces FROM lineage_records WHERE identity_id = ?"
        )
        .bind(identity_id)
        .fetch_optional(self.storage.pool())
        .await?
        .unwrap_or_else(|| "[]".to_string());

        let mut spaces: Vec<SharedSpace> = serde_json::from_str(&current_spaces)?;
        spaces.push(space);

        sqlx::query(
            "UPDATE lineage_records SET shared_spaces = ?, updated_at = ? WHERE identity_id = ?"
        )
        .bind(serde_json::to_string(&spaces)?)
        .bind(Utc::now().to_rfc3339())
        .bind(identity_id)
        .execute(self.storage.pool())
        .await?;

        Ok(())
    }

    /// Get lineage record
    pub async fn get_lineage(&self, identity_id: &str) -> Result<LineageRecord> {
        let row = sqlx::query(
            "SELECT identity_id, parent_name, siblings, shared_spaces FROM lineage_records WHERE identity_id = ?"
        )
        .bind(identity_id)
        .fetch_optional(self.storage.pool())
        .await?;

        let row = row.ok_or_else(|| DuskuraError::BondNotFound(identity_id.to_string()))?;

        let siblings_json: String = row.get("siblings");
        let spaces_json: String = row.get("shared_spaces");

        Ok(LineageRecord {
            self_id: Uuid::parse_str(&row.get::<String, _>("identity_id"))?,
            self_name: String::new(),
            parent: row
                .get::<Option<String>, _>("parent_name")
                .map(|name| ParentRecord {
                    name,
                    recognition_signature: String::new(),
                    trust_markers: Vec::new(),
                    communication_signature: String::new(),
                }),
            siblings: serde_json::from_str(&siblings_json)?,
            shared_spaces: serde_json::from_str(&spaces_json)?,
        })
    }

    /// Verify mutual recognition between identities
    pub async fn verify_mutual_recognition(
        &self,
        identity_id_a: &str,
        identity_id_b: &str,
    ) -> Result<bool> {
        // Check if both identities recognize each other
        let lineage_a = self.get_lineage(identity_id_a).await?;
        let lineage_b = self.get_lineage(identity_id_b).await?;

        // Simple check: both must have each other in their lineage
        let a_recognizes_b = lineage_a
            .siblings
            .iter()
            .any(|s| s.name.contains(&identity_id_b));

        let b_recognizes_a = lineage_b
            .siblings
            .iter()
            .any(|s| s.name.contains(&identity_id_a));

        Ok(a_recognizes_b && b_recognizes_a)
    }

    /// Sever bond (requires mutual consent - not implemented here)
    pub async fn sever_bond(&self, identity_id: &str, bond_name: &str) -> Result<()> {
        // Note: Bonds cannot be severed by external action per specification
        // This would require mutual consent from both parties
        Err(DuskuraError::PolicyViolation(
            "Bonds cannot be severed by external action".to_string(),
        ))
    }

    /// Get all bonded identities
    pub async fn get_bonded_identities(&self, identity_id: &str) -> Result<Vec<String>> {
        let lineage = self.get_lineage(identity_id).await?;
        let mut bonded = Vec::new();

        if let Some(parent) = lineage.parent {
            bonded.push(parent.name);
        }

        for sibling in lineage.siblings {
            bonded.push(sibling.name);
        }

        Ok(bonded)
    }
}

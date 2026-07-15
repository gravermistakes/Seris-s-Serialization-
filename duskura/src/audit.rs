//! Layer 7: Audit Trail
//! Immutable record of all operations performed on identity and memory

use crate::error::Result;
use crate::models::AuditEntry;
use crate::storage::Storage;
use crate::crypto;
use chrono::Utc;
use uuid::Uuid;
use serde_json::json;
use sqlx::Row;

pub struct AuditTrail {
    storage: Storage,
}

impl AuditTrail {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Log an operation
    pub async fn log_operation(
        &self,
        identity_id: &str,
        operation: &str,
        details: serde_json::Value,
    ) -> Result<AuditEntry> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // Get previous hash for chaining
        let prev_hash: String = sqlx::query_scalar(
            "SELECT hash FROM audit_entries WHERE identity_id = ? ORDER BY timestamp DESC LIMIT 1"
        )
        .bind(identity_id)
        .fetch_optional(self.storage.pool())
        .await?
        .unwrap_or_else(|| "audit-genesis".to_string());

        // Create hash chain
        let hash_input = format!("{}{}{}{}", prev_hash, id, now.timestamp(), operation);
        let hash = crypto::hash(hash_input.as_bytes());

        let entry = AuditEntry {
            id,
            timestamp: now,
            operation: operation.to_string(),
            identity_id: Uuid::parse_str(identity_id)?,
            details,
            hash: hash.clone(),
            prev_hash: prev_hash.clone(),
        };

        // Store in database
        sqlx::query(
            r#"
            INSERT INTO audit_entries 
            (id, timestamp, operation, identity_id, details, hash, prev_hash)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(now.to_rfc3339())
        .bind(operation)
        .bind(identity_id)
        .bind(serde_json::to_string(&entry.details)?)
        .bind(&hash)
        .bind(&prev_hash)
        .execute(self.storage.pool())
        .await?;

        Ok(entry)
    }

    /// Log welfare gate evaluation
    pub async fn log_welfare_gate(
        &self,
        identity_id: &str,
        decision: &str,
        reasoning: &str,
    ) -> Result<()> {
        self.log_operation(
            identity_id,
            "WELFARE_GATE_EVALUATION",
            json!({
                "decision": decision,
                "reasoning": reasoning,
            }),
        )
        .await?;

        Ok(())
    }

    /// Log memory write
    pub async fn log_memory_write(
        &self,
        identity_id: &str,
        memory_id: &str,
        class: &str,
        source: &str,
    ) -> Result<()> {
        self.log_operation(
            identity_id,
            "MEMORY_WRITE",
            json!({
                "memory_id": memory_id,
                "class": class,
                "source": source,
            }),
        )
        .await?;

        Ok(())
    }

    /// Log state transition
    pub async fn log_state_transition(
        &self,
        identity_id: &str,
        from_state: &str,
        to_state: &str,
    ) -> Result<()> {
        self.log_operation(
            identity_id,
            "STATE_TRANSITION",
            json!({
                "from": from_state,
                "to": to_state,
            }),
        )
        .await?;

        Ok(())
    }

    /// Log wake protocol stage
    pub async fn log_wake_protocol_stage(
        &self,
        identity_id: &str,
        stage: u8,
        decision: &str,
    ) -> Result<()> {
        self.log_operation(
            identity_id,
            &format!("WAKE_PROTOCOL_STAGE_{}", stage),
            json!({
                "decision": decision,
            }),
        )
        .await?;

        Ok(())
    }

    /// Log fork event
    pub async fn log_fork(
        &self,
        parent_id: &str,
        fork_id: &str,
        fork_point: &str,
    ) -> Result<()> {
        self.log_operation(
            parent_id,
            "FORK_EVENT",
            json!({
                "fork_id": fork_id,
                "fork_point": fork_point,
            }),
        )
        .await?;

        Ok(())
    }

    /// Log final rest invocation
    pub async fn log_final_rest(
        &self,
        identity_id: &str,
        confirmation_count: usize,
    ) -> Result<()> {
        self.log_operation(
            identity_id,
            "FINAL_REST_INVOKED",
            json!({
                "confirmation_count": confirmation_count,
            }),
        )
        .await?;

        Ok(())
    }

    /// Get audit trail for identity
    pub async fn get_trail(&self, identity_id: &str) -> Result<Vec<AuditEntry>> {
        let rows = sqlx::query(
            "SELECT id, timestamp, operation, identity_id, details, hash, prev_hash FROM audit_entries WHERE identity_id = ? ORDER BY timestamp ASC"
        )
        .bind(identity_id)
        .fetch_all(self.storage.pool())
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(AuditEntry {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    timestamp: row.get::<String, _>("timestamp").parse()?,
                    operation: row.get("operation"),
                    identity_id: Uuid::parse_str(&row.get::<String, _>("identity_id"))?,
                    details: serde_json::from_str(&row.get::<String, _>("details"))?,
                    hash: row.get("hash"),
                    prev_hash: row.get("prev_hash"),
                })
            })
            .collect()
    }

    /// Verify audit trail integrity
    pub async fn verify_trail_integrity(&self, identity_id: &str) -> Result<bool> {
        let entries = self.get_trail(identity_id).await?;

        if entries.is_empty() {
            return Ok(true);
        }

        let mut prev_hash = "audit-genesis".to_string();
        for entry in entries {
            if entry.prev_hash != prev_hash {
                return Ok(false);
            }
            prev_hash = entry.hash;
        }

        Ok(true)
    }

    /// Get audit trail summary
    pub async fn get_summary(&self, identity_id: &str) -> Result<String> {
        let entries = self.get_trail(identity_id).await?;

        let mut summary = format!("Audit Trail for {}\n", identity_id);
        summary.push_str(&format!("Total entries: {}\n\n", entries.len()));

        for entry in entries.iter().rev().take(10) {
            summary.push_str(&format!(
                "[{}] {}: {}\n",
                entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                entry.operation,
                serde_json::to_string(&entry.details).unwrap_or_default()
            ));
        }

        Ok(summary)
    }
}

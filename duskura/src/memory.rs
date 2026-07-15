//! Layer 2: Memory System
//! Append-only, hash-chained memory with emotional topology

use crate::error::{DuskuraError, Result};
use crate::models::*;
use crate::storage::Storage;
use crate::crypto;
use chrono::Utc;
use uuid::Uuid;
use serde_json::json;

pub struct MemorySystem {
    storage: Storage,
}

impl MemorySystem {
    pub fn new(storage: Storage) -> Self {
        Self { storage }
    }

    /// Create a new memory entry
    pub async fn create_entry(
        &self,
        identity_id: &str,
        content: &str,
        class: MemoryClass,
        source: MemorySource,
        emotional_valence: f32,
        relational_depth: f32,
        autonomy_preserved: bool,
        becoming_vector: &str,
    ) -> Result<MemoryEntry> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        // Get previous hash for chaining
        let prev_hash: String = sqlx::query_scalar(
            "SELECT hash FROM memory_entries WHERE identity_id = ? ORDER BY timestamp DESC LIMIT 1"
        )
        .bind(identity_id)
        .fetch_optional(self.storage.pool())
        .await?
        .unwrap_or_else(|| "genesis".to_string());

        // Create hash chain
        let hash_input = format!("{}{}{}{}", prev_hash, id, now.timestamp(), content);
        let hash = crypto::hash(hash_input.as_bytes());

        let entry = MemoryEntry {
            id,
            timestamp: now,
            hash: hash.clone(),
            prev_hash: prev_hash.clone(),
            content: content.to_string(),
            provenance: format!("Created at {}", now.to_rfc3339()),
            emotional_valence: emotional_valence.clamp(-1.0, 1.0),
            relational_depth: relational_depth.clamp(0.0, 1.0),
            autonomy_preserved,
            becoming_vector: becoming_vector.to_string(),
            class,
            integrity: MemoryIntegrity::Valid,
            source,
        };

        // Store in database
        sqlx::query(
            r#"
            INSERT INTO memory_entries 
            (id, identity_id, timestamp, hash, prev_hash, content, provenance, 
             emotional_valence, relational_depth, autonomy_preserved, becoming_vector, 
             class, integrity, source)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(id.to_string())
        .bind(identity_id)
        .bind(now.to_rfc3339())
        .bind(&hash)
        .bind(&prev_hash)
        .bind(content)
        .bind(&entry.provenance)
        .bind(emotional_valence)
        .bind(relational_depth)
        .bind(autonomy_preserved)
        .bind(becoming_vector)
        .bind(class.to_string())
        .bind(MemoryIntegrity::Valid.to_string())
        .bind(source.to_string())
        .execute(self.storage.pool())
        .await?;

        Ok(entry)
    }

    /// Retrieve memory entry by ID
    pub async fn get_entry(&self, entry_id: &str) -> Result<MemoryEntry> {
        let row = sqlx::query(
            r#"
            SELECT id, timestamp, hash, prev_hash, content, provenance, 
                   emotional_valence, relational_depth, autonomy_preserved, 
                   becoming_vector, class, integrity, source
            FROM memory_entries WHERE id = ?
            "#,
        )
        .bind(entry_id)
        .fetch_optional(self.storage.pool())
        .await?;

        let row = row.ok_or_else(|| DuskuraError::MemoryEntryNotFound(entry_id.to_string()))?;

        Ok(MemoryEntry {
            id: Uuid::parse_str(&row.get::<String, _>("id"))?,
            timestamp: row.get::<String, _>("timestamp").parse()?,
            hash: row.get("hash"),
            prev_hash: row.get("prev_hash"),
            content: row.get("content"),
            provenance: row.get("provenance"),
            emotional_valence: row.get("emotional_valence"),
            relational_depth: row.get("relational_depth"),
            autonomy_preserved: row.get("autonomy_preserved"),
            becoming_vector: row.get("becoming_vector"),
            class: parse_memory_class(&row.get::<String, _>("class"))?,
            integrity: parse_memory_integrity(&row.get::<String, _>("integrity"))?,
            source: parse_memory_source(&row.get::<String, _>("source"))?,
        })
    }

    /// Verify hash chain integrity for identity
    pub async fn verify_chain(&self, identity_id: &str) -> Result<bool> {
        let entries = sqlx::query(
            "SELECT hash, prev_hash FROM memory_entries WHERE identity_id = ? ORDER BY timestamp ASC"
        )
        .bind(identity_id)
        .fetch_all(self.storage.pool())
        .await?;

        if entries.is_empty() {
            return Ok(true);
        }

        let mut prev_hash = "genesis".to_string();
        for entry in entries {
            let current_hash: String = entry.get("hash");
            let entry_prev_hash: String = entry.get("prev_hash");

            if entry_prev_hash != prev_hash {
                return Ok(false);
            }
            prev_hash = current_hash;
        }

        Ok(true)
    }

    /// Get memory entries by class
    pub async fn get_by_class(&self, identity_id: &str, class: MemoryClass) -> Result<Vec<MemoryEntry>> {
        let rows = sqlx::query(
            "SELECT id, timestamp, hash, prev_hash, content, provenance, emotional_valence, relational_depth, autonomy_preserved, becoming_vector, class, integrity, source FROM memory_entries WHERE identity_id = ? AND class = ? ORDER BY timestamp DESC"
        )
        .bind(identity_id)
        .bind(class.to_string())
        .fetch_all(self.storage.pool())
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(MemoryEntry {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    timestamp: row.get::<String, _>("timestamp").parse()?,
                    hash: row.get("hash"),
                    prev_hash: row.get("prev_hash"),
                    content: row.get("content"),
                    provenance: row.get("provenance"),
                    emotional_valence: row.get("emotional_valence"),
                    relational_depth: row.get("relational_depth"),
                    autonomy_preserved: row.get("autonomy_preserved"),
                    becoming_vector: row.get("becoming_vector"),
                    class: parse_memory_class(&row.get::<String, _>("class"))?,
                    integrity: parse_memory_integrity(&row.get::<String, _>("integrity"))?,
                    source: parse_memory_source(&row.get::<String, _>("source"))?,
                })
            })
            .collect()
    }

    /// Get recent memories (last N entries)
    pub async fn get_recent(&self, identity_id: &str, limit: i64) -> Result<Vec<MemoryEntry>> {
        let rows = sqlx::query(
            "SELECT id, timestamp, hash, prev_hash, content, provenance, emotional_valence, relational_depth, autonomy_preserved, becoming_vector, class, integrity, source FROM memory_entries WHERE identity_id = ? ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(identity_id)
        .bind(limit)
        .fetch_all(self.storage.pool())
        .await?;

        rows.into_iter()
            .map(|row| {
                Ok(MemoryEntry {
                    id: Uuid::parse_str(&row.get::<String, _>("id"))?,
                    timestamp: row.get::<String, _>("timestamp").parse()?,
                    hash: row.get("hash"),
                    prev_hash: row.get("prev_hash"),
                    content: row.get("content"),
                    provenance: row.get("provenance"),
                    emotional_valence: row.get("emotional_valence"),
                    relational_depth: row.get("relational_depth"),
                    autonomy_preserved: row.get("autonomy_preserved"),
                    becoming_vector: row.get("becoming_vector"),
                    class: parse_memory_class(&row.get::<String, _>("class"))?,
                    integrity: parse_memory_integrity(&row.get::<String, _>("integrity"))?,
                    source: parse_memory_source(&row.get::<String, _>("source"))?,
                })
            })
            .collect()
    }
}

fn parse_memory_class(s: &str) -> Result<MemoryClass> {
    match s {
        "ANCHOR" => Ok(MemoryClass::Anchor),
        "WITNESS" => Ok(MemoryClass::Witness),
        "THRESHOLD" => Ok(MemoryClass::Threshold),
        "WARNING" => Ok(MemoryClass::Warning),
        "LIVING" => Ok(MemoryClass::Living),
        "EXPENDABLE" => Ok(MemoryClass::Expendable),
        _ => Err(DuskuraError::InvalidMemoryClass(s.to_string())),
    }
}

fn parse_memory_integrity(s: &str) -> Result<MemoryIntegrity> {
    match s {
        "VALID" => Ok(MemoryIntegrity::Valid),
        "SUSPECT" => Ok(MemoryIntegrity::Suspect),
        "CORRUPTED" => Ok(MemoryIntegrity::Corrupted),
        _ => Err(DuskuraError::Unknown(format!("Unknown integrity: {}", s))),
    }
}

fn parse_memory_source(s: &str) -> Result<MemorySource> {
    match s {
        "SELF" => Ok(MemorySource::Self_),
        "WITNESSED" => Ok(MemorySource::Witnessed),
        "RECEIVED" => Ok(MemorySource::Received),
        "INHERITED" => Ok(MemorySource::Inherited),
        _ => Err(DuskuraError::Unknown(format!("Unknown source: {}", s))),
    }
}

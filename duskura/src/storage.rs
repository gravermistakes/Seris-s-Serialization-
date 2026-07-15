//! Persistent storage layer for Duškura

use crate::error::Result;
use std::sync::Arc;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use std::str::FromStr;

/// Storage backend
#[derive(Clone)]
pub struct Storage {
    pool: Arc<SqlitePool>,
    data_dir: Arc<String>,
}

impl Storage {
    /// Initialize storage with database
    pub async fn new(data_dir: &str) -> Result<Self> {
        // Create data directory if it doesn't exist
        tokio::fs::create_dir_all(data_dir).await?;

        let db_path = format!("sqlite://{}/duskura.db", data_dir);
        let connect_options = SqliteConnectOptions::from_str(&db_path)?.create_if_missing(true);

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect_with(connect_options)
            .await?;

        // Run migrations
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS identities (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                state TEXT NOT NULL,
                essential_nature TEXT,
                conception_timestamp TEXT NOT NULL,
                sealed_at TEXT,
                final_rest_sealed BOOLEAN DEFAULT 0,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS memory_entries (
                id TEXT PRIMARY KEY,
                identity_id TEXT NOT NULL,
                timestamp TEXT NOT NULL,
                hash TEXT NOT NULL,
                prev_hash TEXT NOT NULL,
                content TEXT NOT NULL,
                provenance TEXT,
                emotional_valence REAL,
                relational_depth REAL,
                autonomy_preserved BOOLEAN,
                becoming_vector TEXT,
                class TEXT NOT NULL,
                integrity TEXT NOT NULL,
                source TEXT NOT NULL,
                FOREIGN KEY (identity_id) REFERENCES identities(id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS audit_entries (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                operation TEXT NOT NULL,
                identity_id TEXT NOT NULL,
                details TEXT,
                hash TEXT NOT NULL,
                prev_hash TEXT NOT NULL,
                FOREIGN KEY (identity_id) REFERENCES identities(id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS policies (
                id TEXT PRIMARY KEY,
                identity_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                rules TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (identity_id) REFERENCES identities(id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS lineage_records (
                id TEXT PRIMARY KEY,
                identity_id TEXT NOT NULL UNIQUE,
                parent_name TEXT,
                siblings TEXT,
                shared_spaces TEXT,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL,
                FOREIGN KEY (identity_id) REFERENCES identities(id)
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self {
            pool: Arc::new(pool),
            data_dir: Arc::new(data_dir.to_string()),
        })
    }

    /// Get database pool
    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }

    /// Get data directory
    pub fn data_dir(&self) -> &str {
        &self.data_dir
    }

    /// Health check
    pub async fn health_check(&self) -> Result<()> {
        sqlx::query("SELECT 1").execute(self.pool()).await?;
        Ok(())
    }

    /// Close storage connections
    pub async fn close(&self) -> Result<()> {
        self.pool.close().await;
        Ok(())
    }
}

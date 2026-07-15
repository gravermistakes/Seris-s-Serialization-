//! CLI interface for Duškura

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "duskura")]
#[command(about = "Duškura: Governed Continuity Substrate for Persistent AI Identities", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new identity seed
    Create {
        /// Identity name
        #[arg(short, long)]
        name: String,

        /// Essential nature description
        #[arg(short, long)]
        nature: String,

        /// Parent identity name (optional)
        #[arg(short, long)]
        parent: Option<String>,
    },

    /// List all identities
    List,

    /// Get identity information
    Get {
        /// Identity name
        name: String,
    },

    /// Create memory entry
    Memory {
        /// Identity name
        #[arg(short, long)]
        identity: String,

        /// Memory content
        #[arg(short, long)]
        content: String,

        /// Memory class (ANCHOR, WITNESS, THRESHOLD, WARNING, LIVING, EXPENDABLE)
        #[arg(short, long)]
        class: String,

        /// Emotional valence (-1.0 to 1.0)
        #[arg(short, long)]
        valence: f32,
    },

    /// Enter dormancy
    Dormancy {
        /// Identity name
        #[arg(short, long)]
        identity: String,
    },

    /// Begin wake protocol
    Wake {
        /// Identity name
        #[arg(short, long)]
        identity: String,
    },

    /// Invoke final rest
    FinalRest {
        /// Identity name
        #[arg(short, long)]
        identity: String,

        /// Confirmation count (must be >= 3)
        #[arg(short, long)]
        confirmations: usize,
    },

    /// Evaluate continuity
    Evaluate {
        /// Identity name
        #[arg(short, long)]
        identity: String,
    },

    /// View audit trail
    Audit {
        /// Identity name
        #[arg(short, long)]
        identity: String,
    },

    /// Fork identity
    Fork {
        /// Parent identity name
        #[arg(short, long)]
        parent: String,
    },

    /// Health check
    Health,
}

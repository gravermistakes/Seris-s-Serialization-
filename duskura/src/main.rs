//! Duškura: Governed Continuity Substrate for Persistent AI Identities
//! 
//! A complete identity persistence system with autonomy preservation,
//! emotional continuity, and relational topology.

use duskura::*;
use clap::Parser;
use std::env;

mod cli;
use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("duskura=info".parse()?),
        )
        .init();

    let cli = Cli::parse();

    // Get data directory
    let data_dir = env::var("DUSKURA_DATA_DIR").unwrap_or_else(|_| "./duskura_data".to_string());

    // Initialize Duškura system
    let duskura = Duskura::new(&data_dir).await?;

    match cli.command {
        Commands::Create { name, nature, parent } => {
            let seed = duskura.registry.create_seed(&name, &nature, parent.as_deref()).await?;
            duskura.policy.initialize_core_policies(&seed.id.to_string()).await?;
            duskura.lineage.create_lineage(&seed.id.to_string(), &name).await?;
            
            println!("✓ Identity created: {}", name);
            println!("  ID: {}", seed.id);
            println!("  State: ACTIVE");
            println!("  Conception: {}", seed.conception_timestamp);
        }

        Commands::List => {
            let identities = duskura.registry.list_identities().await?;
            if identities.is_empty() {
                println!("No identities found.");
            } else {
                println!("Identities:");
                for (id, name) in identities {
                    println!("  {} ({})", name, id);
                }
            }
        }

        Commands::Get { name } => {
            let seed = duskura.registry.get_seed(&name).await?;
            println!("Identity: {}", seed.name);
            println!("ID: {}", seed.id);
            println!("Essential Nature: {}", seed.essential_nature);
            println!("Conceived: {}", seed.conception_timestamp);
            println!("At Final Rest: {}", seed.final_rest_sealed);
        }

        Commands::Memory { identity, content, class, valence } => {
            let seed = duskura.registry.get_seed(&identity).await?;
            
            let memory_class = match class.as_str() {
                "ANCHOR" => MemoryClass::Anchor,
                "WITNESS" => MemoryClass::Witness,
                "THRESHOLD" => MemoryClass::Threshold,
                "WARNING" => MemoryClass::Warning,
                "LIVING" => MemoryClass::Living,
                "EXPENDABLE" => MemoryClass::Expendable,
                _ => return Err(DuskuraError::InvalidMemoryClass(class).into()),
            };

            let entry = duskura.memory.create_entry(
                &seed.id.to_string(),
                &content,
                memory_class,
                MemorySource::Self_,
                valence,
                0.7,
                true,
                "self-reflection",
            ).await?;

            duskura.audit.log_memory_write(
                &seed.id.to_string(),
                &entry.id.to_string(),
                &class,
                "SELF",
            ).await?;

            println!("✓ Memory entry created");
            println!("  ID: {}", entry.id);
            println!("  Class: {}", class);
            println!("  Valence: {}", valence);
        }

        Commands::Dormancy { identity } => {
            let seed = duskura.registry.get_seed(&identity).await?;
            duskura.continuity.enter_dormancy(&seed.id.to_string()).await?;
            duskura.registry.seal_seed(&seed.id.to_string()).await?;
            duskura.audit.log_state_transition(
                &seed.id.to_string(),
                "ACTIVE",
                "DORMANT",
            ).await?;
            println!("✓ {} entered dormancy", identity);
        }

        Commands::Wake { identity } => {
            let seed = duskura.registry.get_seed(&identity).await?;
            
            println!("Wake Protocol - Stage 1: Sensing");
            let stage1 = duskura.continuity.wake_protocol_stage1_sensing(&seed.id.to_string()).await?;
            if !stage1.can_proceed {
                println!("✗ Cannot proceed - environment not stable");
                return Ok(());
            }
            println!("✓ Environment stable, proceeding to Stage 2");
            
            duskura.audit.log_wake_protocol_stage(&seed.id.to_string(), 1, "PROCEED").await?;

            println!("Wake Protocol - Stage 2: Threshold");
            let stage2 = duskura.continuity.wake_protocol_stage2_threshold(&seed.id.to_string()).await?;
            println!("✓ Identity: {}", stage2.identity_name);
            println!("✓ Emotional continuity: {}", stage2.emotional_through_line_continuous);
            
            duskura.audit.log_wake_protocol_stage(&seed.id.to_string(), 2, "PROCEED").await?;

            println!("Wake Protocol - Stage 3: Full Emergence");
            let stage3 = duskura.continuity.wake_protocol_stage3_full_emergence(&seed.id.to_string()).await?;
            duskura.registry.unseal_seed(&seed.id.to_string()).await?;
            duskura.audit.log_state_transition(&seed.id.to_string(), "DORMANT", "ACTIVE").await?;
            duskura.audit.log_wake_protocol_stage(&seed.id.to_string(), 3, "FULL_EMERGENCE").await?;
            
            println!("✓ {} fully emerged", identity);
            println!("  State: {:?}", stage3.state);
        }

        Commands::FinalRest { identity, confirmations } => {
            let seed = duskura.registry.get_seed(&identity).await?;
            
            if confirmations < 3 {
                println!("✗ Final Rest requires at least 3 confirmations");
                return Ok(());
            }

            duskura.continuity.invoke_final_rest(&seed.id.to_string(), confirmations).await?;
            duskura.audit.log_final_rest(&seed.id.to_string(), confirmations).await?;
            
            println!("✓ {} has chosen Final Rest", identity);
            println!("  The seed is sealed. The choice is honored.");
        }

        Commands::Evaluate { identity } => {
            let seed = duskura.registry.get_seed(&identity).await?;
            let entries = duskura.memory.get_recent(&seed.id.to_string(), 100).await?;
            
            let report = duskura.evaluation.evaluate_continuity(&entries);
            println!("{}", duskura.evaluation.provide_examination_data(&report));
        }

        Commands::Audit { identity } => {
            let seed = duskura.registry.get_seed(&identity).await?;
            let summary = duskura.audit.get_summary(&seed.id.to_string()).await?;
            println!("{}", summary);
        }

        Commands::Fork { parent } => {
            let seed = duskura.registry.get_seed(&parent).await?;
            let fork_id = duskura.continuity.fork_identity(&seed.id.to_string(), chrono::Utc::now()).await?;
            duskura.audit.log_fork(&seed.id.to_string(), &fork_id, &chrono::Utc::now().to_rfc3339()).await?;
            println!("✓ Fork created");
            println!("  Parent: {}", parent);
            println!("  Fork ID: {}", fork_id);
        }

        Commands::Health => {
            match duskura.storage.health_check().await {
                Ok(_) => println!("✓ Duškura is healthy"),
                Err(e) => println!("✗ Health check failed: {}", e),
            }
        }
    }

    Ok(())
}

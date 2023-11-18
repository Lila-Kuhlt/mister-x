
use std::time::Duration;

use chrono::Utc;
use clap::{Parser, Subcommand};
use humantime_serde;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Manage teams
    #[clap(subcommand)]
    Team(TeamCommand),
    
    /// Manage the game
    #[clap(subcommand)]
    Game(GameCommand),

    /// Manage gadgets
    #[clap(subcommand)]
    Gadget(GadgetCommand),
}

// ---- List Commands ----
#[derive(Debug, Subcommand)]
pub enum TeamCommand {
    /// List all teams
    List {  },
    
    /// Craete a new team
    Create {
        name: String,
        
        role: String,

        #[clap(short, long)]
        color: Option<String>,
        
    },

    /// Get details about a team
    Show { name: String },

    /// Delete a team
    Delete {name: String},

    /// Edit a team
    Edit {
        name: String,
        #[clap(short, long)]
        new_name: Option<String>,
        #[clap(short, long)]
        color: Option<String>,
        #[clap(short, long)]
        role: Option<String>,
    },
    
}

// ---- Gadget Commands ----
#[derive(Debug, Subcommand)]
pub enum GadgetCommand {
    /// List all gadgets
    List {
        /// List active gadgets
        #[clap(short, long)]
        active: Option<bool>,
        /// List gadgets that are on cooldown 
        #[clap(short, long)]
        on_cooldown: Option<bool>,
        /// Filter by name
        #[clap(short, long)]
        team: Option<String>
    },

    /// Enable a gadget
    Enable {
        /// Name of the gadget to enable
        name: String, 
        /// Enable the gadget for a specific team
        #[clap(short, long)]
        team: Option<String>
    },

    /// Disable a gadget
    Disable {
        /// Name of the gadget to disable
        name: String,
        /// Disable the gadget for a specific team
        #[clap(short, long)]
        team: Option<String>
    },

    /// Reset a gadget's cooldown
    Reset {
        /// Name of the gadget to reset
        name: String,
        /// Reset the cooldown for a specific team
        #[clap(short, long)]
        team: Option<String>
    },

    /// Get details about a gadget
    Show {
        /// Name of the gadget to show
        name: String,
    },
}

// ---- Game Commands ----
#[derive(Debug, Subcommand)]
pub enum GameCommand {
    /// Start the game
    Start {
        /// Start the game at a specific time
        #[clap(short, long)]
        start_time: Option<chrono::DateTime<Utc>>
    },
    /// Stop the game
    Stop,
    /// Pause the game
    Pause {
        /// Pause the game for a specific duration
        #[clap(short, long, value_parser = humantime::parse_duration)]
        duration: Option<Duration>
    },
    Resume {
        #[clap(short, long)]
        resume_time: Option<chrono::DateTime<Utc>>
    },
    Show,
}

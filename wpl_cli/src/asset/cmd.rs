use crate::Context;

use super::burn::{run as burn_mint_account, BurnArgs};
use super::create::{run as create_mint_account, CreateArgs};
use super::freeze::{run as freeze_mint_account, FreezeArgs};
use super::metadata::{metadata_subcommand, MetadataSubCommand};
use super::royalty::{royalty_subcommand, RoyaltySubCommand};
use super::thaw::{run as thaw_mint_account, ThawArgs};

use anyhow::Result;
use clap::{Args, Subcommand};

#[derive(Debug, Clone, Args)]
pub struct AssetSubcommand {
    #[clap(subcommand)]
    pub action: Commands,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Commands {
    #[clap(name = "create")]
    /// Create a new asset
    Create(CreateArgs),
    #[clap(name = "freeze")]
    /// Freeze an asset
    Freeze(FreezeArgs),
    #[clap(name = "thaw")]
    /// Thaw an asset
    Thaw(ThawArgs),
    #[clap(name = "burn")]
    /// Burn an asset
    Burn(BurnArgs),
    #[clap(name = "royalty")]
    /// Royalty based instructions for an asset
    Royalty(RoyaltySubCommand),
    #[clap(name = "metadata")]
    /// Metadata based instructions for an asset
    Metadata(MetadataSubCommand),
}

pub async fn subcommand(context: Context, subcommand: AssetSubcommand) -> Result<()> {
    match subcommand.action {
        Commands::Create(args) => {
            create_mint_account(context, args).await?;
        }
        Commands::Royalty(subcommand) => {
            royalty_subcommand(context, subcommand).await?;
        }
        Commands::Metadata(subcommand) => {
            metadata_subcommand(context, subcommand).await?;
        }
        Commands::Freeze(args) => {
            freeze_mint_account(context, args).await?;
        }
        Commands::Thaw(args) => {
            thaw_mint_account(context, args).await?;
        }
        Commands::Burn(args) => {
            burn_mint_account(context, args).await?;
        }
    }

    Ok(())
}

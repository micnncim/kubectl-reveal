use clap::{Parser, Subcommand};

use std::str;

use crate::commands::{completion::CompletionCommand, secret::SecretCommand};

#[derive(Parser)]
#[command(name = "kubectl", bin_name = "kubectl")]
pub enum Kubectl {
    Reveal(Reveal),
}

#[derive(Parser)]
#[command(
    bin_name = "kubectl reveal",
    version,
    about,
    long_about = None,
    after_help = "Examples:
    kubectl reveal secret my-secret
    kubectl reveal secret my-secret -n my-namespace
    kubectl reveal secret my-secret -n my-namespace --context my-context
    "
)]
pub struct Reveal {
    #[command(subcommand)]
    pub command: RevealCommand,

    /// The namespace to target
    #[arg(short = 'n', long = "namespace", global = true)]
    pub namespace: Option<String>,

    /// The context to target
    #[arg(long = "context", global = true)]
    pub context: Option<String>,
}

#[derive(Subcommand)]
pub enum RevealCommand {
    /// Reveals secrets
    #[command(arg_required_else_help = true)]
    Secret(SecretCommand),
    /// Generates shell completions
    #[command(arg_required_else_help = true)]
    Completion(CompletionCommand),
}

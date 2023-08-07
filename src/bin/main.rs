use clap::Parser;

use kubectl_reveal::cli::{Reveal, RevealCommand};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = &Reveal::parse();

    match &args.command {
        RevealCommand::Secret(secret) => secret.run(args).await,
        RevealCommand::Completion(completion) => completion.run(args).await,
    }
}

mod k8s;

use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use std::io;
use std::str;

use k8s::Client;

#[derive(Debug, Parser)]
#[command(name = "kubectl-reveal")]
#[command(version)]
#[command(about = "Reveals Kubernetes Secret data", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// The namespace to target
    #[arg(short = 'n', long = "namespace", global = true)]
    namespace: Option<String>,

    /// The context to target
    #[arg(long = "context", global = true)]
    context: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Reveals secrets
    #[command(arg_required_else_help = true)]
    Secret {
        /// The secret to target
        secret: String,
    },
    /// Generates shell completions
    #[command(arg_required_else_help = true)]
    Completion {
        /// The shell to generate completions for
        #[clap(value_enum)]
        shell: Shell,
    },
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = Client::new(cli.context, cli.namespace).await?;

    match cli.command {
        Commands::Secret { secret } => {
            match client.get_secret(&secret).await {
                Ok(secret) => {
                    if let Some(data) = secret.data {
                        for (key, value) in data {
                            let plaintext = str::from_utf8(&value.0)
                                .unwrap_or("Failed to convert decoded data to string");
                            println!("{}\t{}", key, plaintext);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to get secret: {:?}", e);
                    return Err(e);
                }
            }
            Ok(())
        }
        Commands::Completion { shell } => {
            let mut cmd = Cli::command();
            print_completions(shell, &mut cmd);
            Ok(())
        }
    }
}

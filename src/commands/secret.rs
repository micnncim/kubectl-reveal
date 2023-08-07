use crate::k8s::Client;

use clap::Parser;
use std::str;

use crate::cli::Reveal;

#[derive(Parser)]
/// `kubectl reveal secret`
pub struct SecretCommand {
    /// The secret to target
    pub secret_name: String,
}

impl SecretCommand {
    pub async fn run(&self, args: &Reveal) -> anyhow::Result<()> {
        let client = Client::new(args.context.clone()).await?;

        match client
            .get_secret(self.secret_name.clone(), args.namespace.clone())
            .await
        {
            Ok(secret) => {
                if let Some(data) = secret.data {
                    for (key, value) in data {
                        let plaintext = str::from_utf8(&value.0)
                            .unwrap_or("Failed to convert decoded data to string");
                        println!("{}\t{}", key, plaintext);
                    }
                }
            }

            Err(e) => return Err(e),
        }

        Ok(())
    }
}

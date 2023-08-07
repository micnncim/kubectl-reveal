use clap::{Command, CommandFactory, Parser};
use clap_complete::{generate, Generator, Shell};
use std::io;

use crate::cli::{Kubectl, Reveal};

#[derive(Parser)]
/// `kubectl reveal completion`
pub struct CompletionCommand {
    /// The shell to generate completions for
    #[clap(value_enum)]
    shell: Shell,
}

impl CompletionCommand {
    pub async fn run(&self, _: &Reveal) -> anyhow::Result<()> {
        // Kubectl is specified to generate completion for `kubectl reveal`.
        let mut cmd = Kubectl::command();
        print_completion(self.shell, &mut cmd);

        Ok(())
    }
}

fn print_completion<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}

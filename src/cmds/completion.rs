use std::io::Write;

use anyhow::Result;
use clap::{value_parser, Arg, Command, CommandFactory, Parser};

use crate::{cli, model};

/// Generate shell completions
#[derive(Parser, Clone, Debug)]
#[command(author, about, long_about)]
pub struct CompletionOptions {
    /// Include completions for custom commands
    #[arg(long, short)]
    commands: bool,
    /// Shell syntax to emit
    #[arg(default_value_t = clap_complete::Shell::Bash, value_parser = value_parser!(clap_complete::Shell))]
    pub shell: clap_complete::Shell,
}

/// Print shell completions for "garden completion"
pub fn main(options: &cli::MainOptions, completion_options: &CompletionOptions) -> Result<()> {
    let mut cmd = cli::MainOptions::command();

    // Register custom commands with the completion system
    if completion_options.commands {
        let app_context = model::ApplicationContext::from_options(options)?;
        let config = app_context.get_root_config();
        for name in config.commands.keys() {
            cmd = cmd.subcommand(
                Command::new(name)
                    .about(format!("Custom {name} command"))
                    .arg(
                        Arg::new("keep_going")
                            .help("Continue to the next tree when errors occur")
                            .short('k')
                            .long("keep-going"),
                    )
                    .arg(
                        Arg::new("no-errexit")
                            .help("Do not pass -e to the shell")
                            .short('n')
                            .long("no-errexit"),
                    )
                    .arg(
                        Arg::new("no-wordsplit")
                            .help("Do not pass -o shwordsplit to zsh")
                            .short('z')
                            .long("no-wordsplit"),
                    )
                    .arg(
                        Arg::new("queries")
                            // NOTE: value_terminator may not be needed in future versions of clap_complete.
                            // https://github.com/clap-rs/clap/pull/4612
                            .value_terminator("--")
                            .help("Tree queries to find trees where commands will be run"),
                    )
                    .arg(
                        Arg::new("arguments")
                            .help("Arguments to forward to custom commands")
                            .last(true),
                    ),
            );
        }
    }

    let mut buf = vec![];
    clap_complete::generate(completion_options.shell, &mut cmd, "garden", &mut buf);
    std::io::stdout().write_all(&buf).unwrap_or(());

    Ok(())
}

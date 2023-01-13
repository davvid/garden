use clap::{Parser, Subcommand, ValueHint};

use super::cmds;
use super::model;
use super::path;

#[derive(Clone, Debug, Default, Parser)]
#[command(name = "garden")]
#[command(author, version, about, long_about = None)]
pub struct MainOptions {
    /// Set the color mode
    #[arg(
        long,
        require_equals = true,
        num_args = 0..=1,
        default_value_t = model::ColorMode::Auto,
        default_missing_value = "on",
        value_enum,
    )]
    color: model::ColorMode,

    /// Set the Garden file to use
    #[arg(long, short)]
    pub config: Option<std::path::PathBuf>,

    /// Change directories before searching for Garden files
    #[arg(long, short = 'C', value_hint = ValueHint::AnyPath)]
    chdir: Option<std::path::PathBuf>,

    /// Increase verbosity for a debug category
    #[arg(long, short, action = clap::ArgAction::Append)]
    pub debug: Vec<String>,

    /// Set variables using 'name=value' expressions
    #[arg(long, short = 'D')]
    pub define: Vec<String>,

    /// Set the Garden tree root [default: ${GARDEN_ROOT}]
    #[arg(long, short, default_value_t = String::new())]
    pub root: String,

    /// Be quiet
    #[arg(short, long)]
    pub quiet: bool,

    /// Increase verbosity level (default: 0)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub verbose: u8,

    /// Command to run
    #[command(subcommand)]
    pub command: Command,
}

impl MainOptions {
    /// Construct a default set of options
    pub fn new() -> Self {
        Self::default()
    }

    /// Update the initial state to handle chdir() and making arguments absolute.
    pub fn update(&mut self) {
        self.color.update();

        if let Some(config) = &self.config {
            if config.exists() {
                self.config = Some(path::abspath(config));
            }
        }

        if !self.root.is_empty() {
            let root_path = std::path::PathBuf::from(&self.root);
            self.root = path::abspath(&root_path).to_string_lossy().to_string();
        }

        if let Some(chdir) = &self.chdir {
            if let Err(err) = std::env::set_current_dir(chdir) {
                error!("could not chdir to {:?}: {}", chdir, err);
            }
        }
    }

    /// Return the debug level for the
    pub fn debug_level(&self, name: &str) -> u8 {
        debug_level(&self.debug, name)
    }
}

/// Parse a vector of debug level arguments to count how many have been set
pub fn debug_level(debug: &[String], name: &str) -> u8 {
    debug.iter().filter(|&x| x == name).count() as u8
}

#[derive(Clone, Debug, Subcommand)]
pub enum Command {
    /// Run custom commands over gardens
    Cmd(cmds::cmd::CmdOptions),
    /// Generate shell completions
    Completion(cmds::completion::CompletionOptions),
    /// Custom commands
    #[command(external_subcommand)]
    Custom(Vec<String>),
    /// Evaluate garden expressions
    Eval(cmds::eval::EvalOptions),
    /// Run commands inside garden environments
    Exec(cmds::exec::ExecOptions),
    /// Grow garden worktrees into existence
    Grow(cmds::grow::GrowOptions),
    /// Initialize a "garden.yaml" garden configuration file
    Init(cmds::init::InitOptions),
    /// Query tree status
    Inspect(cmds::inspect::InspectOptions),
    /// List available gardens, groups, trees and commands
    #[command(alias = "ls")]
    List(cmds::list::ListOptions),
    /// Add pre-existing worktrees to a garden configuration file
    Plant(cmds::plant::PlantOptions),
    /// Remove unreferenced Git repositories
    Prune(cmds::prune::PruneOptions),
    /// Open a shell in a garden environment
    #[command(alias = "sh")]
    Shell(cmds::shell::ShellOptions),
}

impl std::default::Default for Command {
    fn default() -> Self {
        Command::Custom(vec![])
    }
}
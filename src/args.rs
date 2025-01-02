#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Command>,
}

impl Cli {
    pub fn command(&self) -> Command {
        self.command.clone().unwrap_or(Command::Run)
    }
}

#[derive(clap::Subcommand, Clone)]
pub enum Command {
    RegisterCommands {
        #[arg(long, short)]
        guild_id: Option<String>,
    },
    Run,
}

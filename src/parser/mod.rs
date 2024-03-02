pub mod structure;

use clap::Parser;

use super::daemon::{kill_daemon, start_daemon};
use structure::*;

pub fn parse_command() -> () {
    let args: Cli = Cli::parse();
    match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => todo!(),
            CliDiscordSubcommands::Disconnect => todo!(),
            CliDiscordSubcommands::Get => todo!(),
            CliDiscordSubcommands::Set(_arg) => todo!(),
        },
        CliSubcommands::Kill => kill_daemon(),
        CliSubcommands::Processes(arg) => match arg.subcommands {
            CliProcessesSubcommands::Add(_arg) => todo!(),
            CliProcessesSubcommands::List => todo!(),
            CliProcessesSubcommands::Remove(_arg) => todo!(),
            CliProcessesSubcommands::Show => todo!(),
        },
        CliSubcommands::Refresh => todo!(),
        CliSubcommands::Spotify(arg) => match arg.subcommands {
            CliSpotifySubcommands::Add => todo!(),
            CliSpotifySubcommands::Client(_arg) => todo!(),
            CliSpotifySubcommands::Remove => todo!(),
        },
        CliSubcommands::Start => start_daemon(),
    };
}

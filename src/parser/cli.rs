use clap::Parser;

use super::super::daemon::{kill_daemon, ping_daemon, start_daemon};
use super::structure::*;

pub fn parse_command() -> () {
    let args: Cli = Cli::parse();
    match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => todo!(),
            CliDiscordSubcommands::Disconnect => todo!(),
            CliDiscordSubcommands::Get => todo!(),
            CliDiscordSubcommands::Set(arg) => {
                if arg.client_id.is_some() {
                    todo!()
                };
                if arg.details.is_some() {
                    todo!()
                };
                if arg.large_image_key.is_some() {
                    todo!()
                };
                if arg.large_image_text.is_some() {
                    todo!()
                };
                if arg.small_image_key.is_some() {
                    todo!()
                };
                if arg.small_image_text.is_some() {
                    todo!()
                };
                if arg.state.is_some() {
                    todo!()
                };
            }
        },
        CliSubcommands::Kill => kill_daemon(),
        CliSubcommands::Processes(arg) => match arg.subcommands {
            CliProcessesSubcommands::Add(arg) => {
                if arg.display.is_some() {
                    todo!()
                };
                // manage arg.name (required value)
                todo!()
            }
            CliProcessesSubcommands::List => todo!(),
            CliProcessesSubcommands::Remove(_arg) => todo!(), // manage arg.name (required value)
            CliProcessesSubcommands::Show => todo!(),
        },
        CliSubcommands::Ping => ping_daemon(),
        CliSubcommands::Refresh => todo!(),
        CliSubcommands::Spotify(arg) => match arg.subcommands {
            CliSpotifySubcommands::Add => todo!(),
            CliSpotifySubcommands::Client(arg) => {
                if arg.id.is_some() {
                    todo!()
                };
                if arg.secret.is_some() {
                    todo!()
                };
            }
            CliSpotifySubcommands::Remove => todo!(),
        },
        CliSubcommands::Start => start_daemon(),
    };
}

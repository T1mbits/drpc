use crate::{
    config::DConfig,
    daemon::{
        kill_daemon, ping_daemon,
        socket::{exchange, socket_path},
        start_daemon,
    },
    discord::{get, set},
    parser::structure::*,
};
use clap::Parser;

pub fn parse_command(config: DConfig) -> () {
    let args: Cli = Cli::parse();
    match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => {
                exchange(b"discord connect", socket_path()).unwrap();
            }
            CliDiscordSubcommands::Disconnect => {
                exchange(b"discord disconnect", socket_path()).unwrap();
            }
            CliDiscordSubcommands::Get => {
                get(&config.discord);
            }
            CliDiscordSubcommands::Set(arg) => {
                set(config, arg);
            }
            CliDiscordSubcommands::Update => {
                exchange(b"discord update", socket_path()).unwrap();
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
        CliSubcommands::Start => start_daemon(config),
    };
}

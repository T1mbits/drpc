use crate::{
    config::DConfig,
    discord::{discord_client_init, discord_set_activity},
    log_setup, log_setup_debug,
    parser::structure::*,
};
use clap::Parser;
use tracing::trace;

pub fn parse_command(config: DConfig) -> () {
    let args: Cli = Cli::parse();

    if args.debug {
        log_setup_debug();
    } else {
        log_setup();
    }

    trace!("{args:#?}");

    match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => discord_set_activity(
                &config.discord,
                &mut discord_client_init(&config.discord).unwrap(),
            ),
            CliDiscordSubcommands::Disconnect => todo!(),
            CliDiscordSubcommands::Get(_arg) => todo!(),
            CliDiscordSubcommands::Set(_arg) => todo!(),
            CliDiscordSubcommands::Update => todo!(),
        },
        CliSubcommands::Kill => todo!(),
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
        CliSubcommands::Ping => println!("ping-a-ling"),
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
        CliSubcommands::Start => todo!(),
    };
}

use crate::{config::Config, discord::*, parser::structure::*};
use tracing::{instrument, trace};

#[instrument(skip_all)]
pub fn parse_command(config: Config, args: Cli) {
    trace!("Parsing command arguments:\n{args:#?}");

    match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => {
                set_activity(&config.discord, &mut client_init(&config.discord).unwrap())
            }
            CliDiscordSubcommands::Disconnect => todo!(),
            CliDiscordSubcommands::Get(arg) => {
                if arg.daemon {
                    todo!()
                } else {
                    get_activity_data(config.discord);
                }
            }
            CliDiscordSubcommands::Set(args) => set_activity_data(config, args),
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

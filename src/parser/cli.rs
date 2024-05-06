use crate::{discord::*, parser::*, prelude::*, processes::*};

/// Parse CLI subcommands and flags and call their respective functions.
#[instrument(skip_all)]
pub async fn parse_command(config: &mut Config, args: Cli) -> Result<Option<ClientBundle>, ()> {
    trace!("Parsing command arguments:\n{args:#?}");

    return match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => match client_init(config).await {
                Err(_) => Err(()),
                Ok(client) => match set_activity(client, config).await {
                    Err(_) => Err(()),
                    Ok(client) => Ok(Some(client)),
                },
            },
            CliDiscordSubcommands::Disconnect => unimplemented!(),
            CliDiscordSubcommands::Get(_arg) => print_activity_data(&config.discord),

            CliDiscordSubcommands::Set(args) => set_activity_data(config, args),
            CliDiscordSubcommands::Update => unimplemented!(),
        },
        CliSubcommands::Kill => unimplemented!(),
        CliSubcommands::Processes(arg) => match arg.subcommands {
            CliProcessesSubcommands::Add(arg) => add_process(config, arg),
            CliProcessesSubcommands::List => print_data_list(&config.processes),
            CliProcessesSubcommands::Priority(arg) => change_process_priority(config, arg),
            CliProcessesSubcommands::Remove(arg) => remove_process(config, arg.name),
            CliProcessesSubcommands::Show => todo!(),
        },
        CliSubcommands::Ping => {
            println!("pong");
            Ok(None)
        }
        CliSubcommands::Refresh => unimplemented!(),
        CliSubcommands::Spotify(arg) => match arg.subcommands {
            CliSpotifySubcommands::Add => todo!(),
            CliSpotifySubcommands::Client(arg) => {
                if arg.id.is_some() {
                    todo!()
                };
                if arg.secret.is_some() {
                    todo!()
                };
                unimplemented!();
            }
            CliSpotifySubcommands::Remove => todo!(),
        },
        CliSubcommands::Start => unimplemented!(),
    };
}

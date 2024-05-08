use crate::{discord::*, parser::*, prelude::*, processes::*};

/// Parse CLI subcommands and flags and call their respective functions.
#[instrument(skip_all)]
pub async fn parse_command(config: &mut Config, args: Cli) -> Result<Option<AppState>, ()> {
    trace!("Parsing command arguments:\n{args:#?}");

    return match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => match client_init(config).await {
                Err(_) => Err(()),
                Ok(mut app) => match set_activity(config, &mut app.discord, &app.spotify).await {
                    Err(_) => Err(()),
                    _ => Ok(Some(app)),
                },
            },
            CliDiscordSubcommands::Disconnect => unimplemented!(),
            CliDiscordSubcommands::Get(_arg) => {
                stupid_type_parameters_damnit(print_activity_data(&config.discord))
            }

            CliDiscordSubcommands::Set(args) => {
                stupid_type_parameters_damnit(set_activity_data(&mut config.discord, args))
            }
            CliDiscordSubcommands::Update => unimplemented!(),
        },
        CliSubcommands::Kill => unimplemented!(),
        CliSubcommands::Processes(arg) => match arg.subcommands {
            CliProcessesSubcommands::Add(arg) => {
                stupid_type_parameters_damnit(add_process(&mut config.processes, arg))
            }
            CliProcessesSubcommands::List => {
                stupid_type_parameters_damnit(print_data_list(&config.processes))
            }
            CliProcessesSubcommands::Priority(arg) => {
                stupid_type_parameters_damnit(change_process_priority(&mut config.processes, arg))
            }
            CliProcessesSubcommands::Remove(arg) => {
                stupid_type_parameters_damnit(remove_process(&mut config.processes, arg.name))
            }
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

fn stupid_type_parameters_damnit(result: Result<(), ()>) -> Result<Option<AppState>, ()> {
    return match result {
        Err(_) => Err(()),
        _ => Ok(None),
    };
}

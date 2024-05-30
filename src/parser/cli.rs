use crate::{discord::*, parser::*, prelude::*, processes::*};

/// Parse CLI subcommands and flags and call their respective functions.
#[instrument(skip_all)]
pub async fn parse_command(
    config: &mut Config,
    args: Cli,
) -> Result<Option<AppState>, Box<dyn Error>> {
    trace!("Parsing command: {args:?}");

    return match args.subcommands {
        CliSubcommands::Discord(arg) => match arg.subcommands {
            CliDiscordSubcommands::Connect => {
                let mut app: AppState = client_init(config).await?;
                set_activity(config, &mut app.discord, &app.spotify).await?;
                Ok(Some(app))
            }
            CliDiscordSubcommands::Disconnect => unimplemented!(),
            CliDiscordSubcommands::Get(_arg) => {
                print_activity_data(&config.discord);
                Ok(None)
            }
            CliDiscordSubcommands::Set(args) => {
                set_activity_data(&mut config.discord, args)?;
                Ok(None)
            }
            CliDiscordSubcommands::Update => unimplemented!(),
        },
        CliSubcommands::Kill => unimplemented!(),
        CliSubcommands::Processes(arg) => match arg.subcommands {
            CliProcessesSubcommands::Add(arg) => {
                add_process(&mut config.processes, arg)?;
                Ok(None)
            }
            CliProcessesSubcommands::List => {
                print_data_list(&config.processes);
                Ok(None)
            }
            CliProcessesSubcommands::Priority(arg) => {
                change_process_priority(&mut config.processes, arg)?;
                Ok(None)
            }
            CliProcessesSubcommands::Remove(arg) => {
                remove_process(&mut config.processes, arg.name)?;
                Ok(None)
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

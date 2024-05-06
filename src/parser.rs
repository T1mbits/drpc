use clap::{Args, Parser, Subcommand};

pub mod cli {
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
}

#[derive(Debug, Parser)]
#[command(version, about = "Dynamic Discord Rich Presence Customizer")]
pub struct Cli {
    #[arg(short = 'd', long, help = "Enable debug logs")]
    pub debug: bool,
    #[arg(
        short = 'o',
        long,
        help = "Overwrite an invalid config with a default config"
    )]
    pub config_overwrite: bool,
    #[command(subcommand)]
    pub subcommands: CliSubcommands,
    #[arg(short = 'v', long, help = "Enable verbose output (trace level logs)")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommands {
    #[command(about = "Manage Discord activity data and connection")]
    Discord(CliDiscord),
    #[command(about = "unimplemented")]
    Kill,
    #[command(about = "Prints pong. Good for testing loggers and config file setups")]
    Ping,
    #[command(about = "Manipulate target processes")]
    Processes(CliProcesses),
    #[command(about = "unimplemented")]
    Refresh,
    #[command(about = "Manage your Spotify account and app connection\nunimplemented")]
    Spotify(CliSpotify),
    #[command(about = "unimplemented")]
    Start,
}

#[derive(Debug, Args)]
pub struct CliDiscord {
    #[command(subcommand)]
    pub subcommands: CliDiscordSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliDiscordSubcommands {
    #[command(
        about = "Start Discord IPC client and set activity. Will start activity update loop"
    )]
    Connect,
    #[command(about = "Clear the Discord activity\nUnimplemented")]
    Disconnect,
    #[command(about = "Get Discord activity data")]
    Get(CliDiscordGet),
    #[command(about = "Set Discord activity data")]
    Set(CliDiscordSet),
    #[command(
        about = "Update Discord activity data (sync app and config file). No longer useful\nUnimplemented"
    )]
    Update,
}

#[derive(Debug, Args)]
pub struct CliDiscordGet {
    #[arg(short = 'd', long, help = "Unimplemented")]
    pub daemon: bool,
}

#[derive(Debug, Args)]
pub struct CliDiscordSet {
    #[arg(short = 'c', long, help = "Set the Discord application id")]
    pub client_id: Option<u64>,
    #[arg(short = 'd', long, help = "Set the activity details")]
    pub details: Option<String>,
    #[arg(short = 's', long, help = "Set the activity state")]
    pub state: Option<String>,
    #[arg(short = 'I', long, help = "Set the activity large image")]
    pub large_image: Option<String>,
    #[arg(short = 'i', long, help = "Set the activity large image text")]
    pub small_image: Option<String>,
    #[arg(short = 'T', long, help = "Set the activity small image")]
    pub large_text: Option<String>,
    #[arg(short = 't', long, help = "Set the activity small image text")]
    pub small_text: Option<String>,
    #[arg(short = 'b', long, help = "Set the activity button 1 text")]
    pub button1_text: Option<String>,
    #[arg(short = 'u', long, help = "Set the activity button 1 url")]
    pub button1_url: Option<String>,
    #[arg(short = 'B', long, help = "Set the activity button 2 text")]
    pub button2_text: Option<String>,
    #[arg(short = 'U', long, help = "Set the activity button 2 url")]
    pub button2_url: Option<String>,
}

#[derive(Debug, Args)]
pub struct CliProcesses {
    #[command(subcommand)]
    pub subcommands: CliProcessesSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliProcessesSubcommands {
    #[command(about = "Add a process")]
    Add(CliProcessesAdd),
    #[command(about = "List all target processes")]
    List,
    #[command(about = "Reorder target process priorities")]
    Priority(CliProcessesPriority),
    #[command(about = "Remove a process")]
    Remove(CliProcessesRemove),
    #[command(about = "Show all active processes (may not include)")]
    Show,
}

#[derive(Debug, Args, Clone)]
pub struct CliProcessesAdd {
    #[arg(
        index = 3,
        help = "Set the image URL or Discord asset name for the process"
    )]
    pub image: String,
    #[arg(index = 1, help = "Name of the process being added")]
    pub name: String,
    #[arg(index = 2, help = "Set the text associated with this process")]
    pub text: String,
}

#[derive(Debug, Args)]
pub struct CliProcessesPriority {
    #[arg(help = "Name of the process entry operated on")]
    pub name: String,
    #[command(flatten)]
    pub operation: CliProcessesPriorityOperation,
}

#[derive(Debug, Args)]
#[group(multiple = false, required = true)]
pub struct CliProcessesPriorityOperation {
    #[arg(short = 'd', long, help = "Lowers process priority by 1")]
    pub decrease: bool,
    #[arg(short = 'i', long, help = "Increases process priority by 1")]
    pub increase: bool,
    #[arg(
        short = 's',
        long,
        help = "Set process priority manually. Highest priority is 0"
    )]
    pub set: Option<usize>,
}

#[derive(Debug, Args)]
pub struct CliProcessesRemove {
    #[arg(help = "Name of the process being removed")]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CliSpotify {
    #[command(subcommand)]
    pub subcommands: CliSpotifySubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliSpotifySubcommands {
    #[command(about = "Link your spotify account???")]
    Add,
    #[command(about = "Set Spotify application data")]
    Client(CliSpotifyClient),
    #[command(about = "Unlink your spotify account???")]
    Remove,
}

#[derive(Debug, Args)]
pub struct CliSpotifyClient {
    #[arg(help = "Set Spotify client ID")]
    pub id: Option<String>,
    #[arg(help = "Set Spotify client secret")]
    pub secret: Option<String>,
}
/*
CLI
|- discord
|	|- connect
|	|- disconnect
|	|- get
|	|	|- --daemon (get config from daemon)
|	|- set
|	|	|- --client-id
|	|	|- --details
|	|	|- --large-image-key
|	|	|- --large-image-text
|	|	|- --small-image-key
|	|	|- --small-image-text
|	|	|- --state
|	|- update
|
|- processes
|	|- add
|	|	|- --display
|	|	|- --name
|	|- list (list all chosen processes)
|	|- remove
|	|	|- --name
|	|- show (list all active processes)
|
|- spotify
|	|- add (verification stuff?? idk it'll give you link)
|	|- client
|	|	|- --id
|	|	|- --secret
|	|- remove
|
|- start
|- kill
|- ping
|- refresh
*/

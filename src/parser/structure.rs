use clap::{Args, Parser, Subcommand};

const DDRPC_ABOUT: &str = "Dynamic Discord Rich Presence Customizer";
const DISCORD_ABOUT: &str = "Manage the connection and data of the Discord Rich Presence";
const KILL_ABOUT: &str = "Kills the daemon process";
const PROCESSES_ABOUT: &str = "Manage the processes that the program will detect";
const REFRESH_ABOUT: &str = "Refresh the daemon process's data";
const SPOTIFY_ABOUT: &str = "Manage the authorization of your Spotify account";
const START_ABOUT: &str = "Start the daemon process";

#[derive(Debug, Parser)]
#[command(version, about = DDRPC_ABOUT)]
pub struct Cli {
    #[command(subcommand)]
    pub subcommands: CliSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliSubcommands {
    #[command(about = DISCORD_ABOUT)]
    Discord(CliDiscord),
    #[command(about = KILL_ABOUT)]
    Kill,
    #[command(about = PROCESSES_ABOUT)]
    Processes(CliProcesses),
    Ping,
    #[command(about = REFRESH_ABOUT)]
    Refresh,
    #[command(about = SPOTIFY_ABOUT)]
    Spotify(CliSpotify),
    #[command(about = START_ABOUT)]
    Start,
}

#[derive(Debug, Args)]
pub struct CliDiscord {
    #[command(subcommand)]
    pub subcommands: CliDiscordSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliDiscordSubcommands {
    Connect,
    Disconnect,
    Get,
    Set(CliDiscordSet),
    Update,
}

#[derive(Debug, Args)]
pub struct CliDiscordSet {
    #[arg(short = 'c', long)]
    pub client_id: Option<u64>,
    #[arg(short = 'd', long)]
    pub details: Option<String>,
    #[arg(short = 'I', long)]
    pub large_image: Option<String>,
    #[arg(short = 'i', long)]
    pub small_image: Option<String>,
    #[arg(short = 'T', long)]
    pub large_text: Option<String>,
    #[arg(short = 't', long)]
    pub small_text: Option<String>,
    #[arg(short = 's', long)]
    pub state: Option<String>,
}

#[derive(Debug, Args)]
pub struct CliProcesses {
    #[command(subcommand)]
    pub subcommands: CliProcessesSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliProcessesSubcommands {
    Add(CliProcessesAdd),
    List,
    Remove(CliProcessesRemove),
    Show,
}

#[derive(Debug, Args)]
pub struct CliProcessesAdd {
    #[arg(short = 'd', long)]
    pub display: Option<String>,
    #[arg(short = 'n', long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CliProcessesRemove {
    #[arg(short = 'n', long)]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct CliSpotify {
    #[command(subcommand)]
    pub subcommands: CliSpotifySubcommands,
}

#[derive(Debug, Subcommand)]
pub enum CliSpotifySubcommands {
    Add,
    Client(CliSpotifyClient),
    Remove,
}

#[derive(Debug, Args)]
pub struct CliSpotifyClient {
    #[arg(short = 'i', long)]
    pub id: Option<String>,
    #[arg(short = 's', long)]
    pub secret: Option<String>,
}
/*
CLI
|- discord
|	|- connect
|	|- disconnect
|	|- get
|	|- set
|		|- --client-id
|		|- --details
|		|- --large-image-key
|		|- --large-image-text
|		|- --small-image-key
|		|- --small-image-text
|		|- --state
|
|- processes
|	|- add
|		|- --display
|		|- --name
|	|- list (list all chosen processes)
|	|- remove
|		|- --name
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

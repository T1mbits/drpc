use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about = "Dynamic Discord Rich Presence Customizer")]
pub struct Ddrpc {
    #[arg(short = 'd', long, help = "Enable debug logs")]
    pub debug: bool,
    #[arg(
        short = 'o',
        long,
        help = "Overwrite an invalid config with a default config"
    )]
    pub config_overwrite: bool,
    #[command(subcommand)]
    pub subcommands: DdrpcSubcommands,
    #[arg(short = 'v', long, help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum DdrpcSubcommands {
    #[command(about = "Manage Discord activity data and connection")]
    Discord(DdrpcDiscord),
    #[command(about = "Kills the active daemon")]
    Kill,
    #[command(about = "Manipulate target processes")]
    Processes(DdrpcProcesses),
    #[command(about = "Manage your Spotify account and app connection\ntodo")]
    Spotify(DdrpcSpotify),
}

#[derive(Debug, Args)]
pub struct DdrpcDiscord {
    #[command(subcommand)]
    pub subcommands: DdrpcDiscordSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum DdrpcDiscordSubcommands {
    #[command(about = "Start and connect Discord IPC client and set activity")]
    Connect(DdrpcDiscordConnect),
    #[command(about = "Clear the Discord activity\ntodo")]
    Disconnect,
    #[command(about = "Get Discord activity data")]
    Get(DdrpcDiscordGet),
    #[command(about = "Set Discord activity data")]
    Set(DdrpcDiscordSet),
    #[command(
        about = "Update Discord activity data (sync app and config file). No longer useful\ntodo"
    )]
    Update,
}

#[derive(Debug, Args)]
pub struct DdrpcDiscordConnect {
    #[arg(help = "The application id of your Discord presence application")]
    pub id: u64,
}

#[derive(Debug, Args)]
pub struct DdrpcDiscordGet {
    #[arg(short = 'd', long, help = "todo")]
    pub daemon: bool,
}

#[derive(Debug, Args)]
pub struct DdrpcDiscordSet {
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
pub struct DdrpcProcesses {
    #[command(subcommand)]
    pub subcommands: DdrpcProcessesSubcommands,
}

#[derive(Debug, Subcommand)]
pub enum DdrpcProcessesSubcommands {
    #[command(about = "Add a process")]
    Add(DdrpcProcessesAdd),
    #[command(about = "List all target processes")]
    List,
    #[command(about = "Reorder target process priorities")]
    Priority(DdrpcProcessesPriority),
    #[command(about = "Remove a process")]
    Remove(DdrpcProcessesRemove),
    #[command(about = "Show all active processes (may not include)")]
    Show,
}

#[derive(Debug, Args, Clone)]
pub struct DdrpcProcessesAdd {
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
pub struct DdrpcProcessesPriority {
    #[arg(help = "Name of the process entry operated on")]
    pub name: String,
    #[command(flatten)]
    pub operation: DdrpcProcessesPriorityOperation,
}

#[derive(Debug, Args)]
#[group(multiple = false, required = true)]
pub struct DdrpcProcessesPriorityOperation {
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
pub struct DdrpcProcessesRemove {
    #[arg(help = "Name of the process being removed")]
    pub name: String,
}

#[derive(Debug, Args)]
pub struct DdrpcSpotify {
    #[command(subcommand)]
    pub subcommands: DdrpcSpotifySubcommands,
}

#[derive(Debug, Subcommand)]
pub enum DdrpcSpotifySubcommands {
    #[command(about = "Link your spotify account???")]
    Add,
    #[command(about = "Set Spotify application data")]
    Client(DdrpcSpotifyClient),
    #[command(about = "Unlink your spotify account???")]
    Remove,
}

#[derive(Debug, Args)]
pub struct DdrpcSpotifyClient {
    #[arg(help = "Set Spotify client ID")]
    pub id: Option<String>,
    #[arg(help = "Set Spotify client secret")]
    pub secret: Option<String>,
}

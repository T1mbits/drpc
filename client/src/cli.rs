use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(version, about = "Dynamic Discord Rich Presence Customizer")]
pub struct Cli {
    #[arg(
        short = 'o',
        long,
        help = "Overwrite an invalid config with a default config"
    )]
    pub config_overwrite: bool,
    #[arg(short = 'd', long, help = "Enable debug logs")]
    pub debug: bool,
    #[command(subcommand)]
    pub subcommand: Ddrpc,
    #[arg(short = 'v', long, help = "Enable verbose output")]
    pub verbose: bool,
}

#[derive(Debug, Subcommand)]
pub enum Ddrpc {
    #[command(about = "Connect the Discord rich presence with the given application id")]
    Connect {
        #[arg(help = "The application id of the Discord rich presence client")]
        id: u64,
    },
    #[command(about = "Disconnect the Discord rich presence")]
    Disconnect,
    #[command(about = "Get information from the config file or daemon")]
    Get {
        // TODO add command/flag to get information from the daemon
        #[command(subcommand)]
        subcommand: DdrpcGet,
    },
    #[command(about = "Kills the daemon")]
    Kill,
    #[command(about = "Edit configuration data")]
    Set {
        #[command(subcommand)]
        subcommand: DdrpcSet,
    },
    #[command(
        about = "Sync the daemon configuration with the local configuration. No flags syncs all configs"
    )]
    Sync(DdrpcSync),
}

#[derive(Debug, Subcommand)]
pub enum DdrpcGet {
    #[command(about = "Show Discord activity configuration")]
    Activity,
    #[command(about = "Get target processes list or all actively running processes")]
    Processes {
        #[command(subcommand)]
        subcommand: DdrpcGetProcesses,
    },
}

#[derive(Debug, Subcommand)]
pub enum DdrpcGetProcesses {
    #[command(about = "Get all actively running processes")]
    All,
    #[command(about = "Show targeted processes configuration")]
    Target,
}

#[derive(Debug, Subcommand)]
pub enum DdrpcSet {
    #[command(about = "Set activity fields for the Discord rich presence")]
    Activity(DdrpcSetActivity),
    #[command(about = "Set the list of detectable processes and their order")]
    Processes {
        #[command(subcommand)]
        subcommand: DdrpcSetProcesses,
    },
}

#[derive(Debug, Args)]
#[group(multiple = true, required = true)]
pub struct DdrpcSetActivity {
    #[arg(short = 'd', long, help = "Set activity details")]
    pub details: Option<String>,
    #[arg(short = 's', long, help = "Set activity state")]
    pub state: Option<String>,
    #[arg(short = 'I', long, help = "Set large image")]
    pub large_image: Option<String>,
    #[arg(short = 'i', long, help = "Set large image text")]
    pub small_image: Option<String>,
    #[arg(short = 'T', long, help = "Set small image")]
    pub large_text: Option<String>,
    #[arg(short = 't', long, help = "Set small image text")]
    pub small_text: Option<String>,
    #[arg(short = 'b', long, help = "Set button 1 text")]
    pub button1_label: Option<String>,
    #[arg(short = 'u', long, help = "Set button 1 url")]
    pub button1_url: Option<String>,
    #[arg(short = 'B', long, help = "Set button 2 text")]
    pub button2_label: Option<String>,
    #[arg(short = 'U', long, help = "Set button 2 url")]
    pub button2_url: Option<String>,
}

#[derive(Debug, Subcommand)]
pub enum DdrpcSetProcesses {
    #[command(about = "Add a process to be detected")]
    Add {
        #[arg(index = 3, help = "Set the image URL or Discord asset name")]
        image: String,
        #[arg(index = 1, help = "Set the name of the processes being added")]
        name: String,
        #[arg(index = 2, help = "Set the text to be displayed for this processes")]
        text: String,
    },
    #[command(about = "Change the order of which processes are detected first")]
    Order {
        #[arg(help = "The name of the process being reordered")]
        name: String,
        #[command(flatten)]
        flags: DdrpcSetProcessesOrder,
    },
    #[command(about = "Remove a target process")]
    Remove {
        #[arg(help = "Name of the processes being removed")]
        name: String,
    },
}

#[derive(Debug, Args)]
#[group(required = true)]
pub struct DdrpcSetProcessesOrder {
    #[arg(short = 'd', long, help = "Decrease the order of the process")]
    pub decrease: bool,
    #[arg(short = 'i', long, help = "Increase the order of the process")]
    pub increase: bool,
    #[arg(short = 's', long, help = "Set the order of the process")]
    pub set: Option<u32>,
}

#[derive(Debug, Args)]
#[group(multiple = true, required = true)]
pub struct DdrpcSetSpotifyCredentials {
    #[arg(long, short = 'c', help = "Set your Spotify application client id")]
    pub client_id: Option<String>,
    #[arg(long, short = 's', help = "Set your Spotify application client secret")]
    pub client_secret: Option<String>,
}

#[derive(Debug, Args)]
pub struct DdrpcSync {
    #[arg(short = 'a', long, help = "Sync the activity configuration")]
    pub activity: bool,
    #[arg(short = 'p', long, help = "Sync the processes configuration")]
    pub processes: bool,
    #[arg(short = 's', long, help = "Sync the Spotify configuration")]
    pub spotify: bool,
}

impl DdrpcSync {
    pub fn no_flags(&self) -> bool {
        !(self.activity || self.processes || self.spotify)
    }
}

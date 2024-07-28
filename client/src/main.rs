mod cli;

use clap::Parser;
use cli::*;
use common::{
    config::{empty_template, get_config, write_config, ActivityTemplate, Config},
    ipc::*,
    log::*,
};

fn main() -> anyhow::Result<()> {
    log_init(LevelFilter::Trace);
    let cli = Cli::parse();
    let mut config = get_config(cli.config_overwrite)?;

    match cli.subcommand {
        Ddrpc::Connect { id } => {
            message(IpcMessage::Activity(config.activity.clone()))?;
            message(IpcMessage::Connect(id))?
        }
        Ddrpc::Disconnect => message(IpcMessage::Disconnect)?,
        Ddrpc::Get { subcommand } => match subcommand {
            DdrpcGet::Activity => {
                println!("{}", config.activity);
                None
            }
            _ => todo!(),
        },
        Ddrpc::Kill => message(IpcMessage::Kill)?,
        Ddrpc::Set { subcommand } => match subcommand {
            DdrpcSet::Activity(activity) => {
                discord_set(&mut config.activity, activity);
                None
            }
            _ => todo!(),
        },
        Ddrpc::Sync(sync) => sync_config(&config, sync)?,
    };

    write_config(&config)
}

fn discord_set(activity: &mut ActivityTemplate, set: DdrpcSetActivity) {
    fn set_option(target: &mut Option<String>, source: Option<String>) {
        if let Some(string) = source {
            *target = Some(string)
        }
    }

    let mut assets = activity.assets.clone().unwrap_or_default();
    let mut buttons = activity.buttons.clone().unwrap_or_default();

    set_option(&mut activity.details, set.details);
    set_option(&mut activity.state, set.state);
    set_option(&mut assets.large_image, set.large_image);
    set_option(&mut assets.large_text, set.large_text);
    set_option(&mut assets.small_image, set.small_image);
    set_option(&mut assets.small_text, set.small_text);
    set_option(&mut buttons.label1, set.button1_label);
    set_option(&mut buttons.url1, set.button1_url);
    set_option(&mut buttons.label2, set.button2_label);
    set_option(&mut buttons.url2, set.button2_url);

    activity.assets = Some(assets);
    activity.buttons = Some(buttons);
    empty_template(activity);
}

fn sync_config(config: &Config, sync: DdrpcSync) -> anyhow::Result<Option<IpcMessage>> {
    if sync.no_flags() {
        message(IpcMessage::Activity(config.activity.clone()))?;
        // message(IpcMessage::Processes(config.processes.clone()))?;
        // message(IpcMessage::Spotify(config.spotify.clone()))?;
        return Ok(None);
    }

    if sync.activity {
        message(IpcMessage::Activity(config.activity.clone()))?;
    }
    // if sync.processes {
    //     message(IpcMessage::Processes(config.processes.clone()))?;
    // }
    // if sync.spotify {
    //     message(IpcMessage::Spotify(config.spotify.clone()))?;
    // }

    Ok(None)
}

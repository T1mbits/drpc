mod cli;

use clap::Parser;
use cli::*;
use common::{ipc::*, log::*};

fn main() -> anyhow::Result<()> {
    log_init(LevelFilter::Trace);
    let cli = Ddrpc::parse();

    parse_command(cli)?;

    Ok(())
}

pub fn parse_command(cli: Ddrpc) -> anyhow::Result<IpcMessage> {
    match cli.subcommands {
        DdrpcSubcommands::Discord(arg) => match arg.subcommands {
            DdrpcDiscordSubcommands::Connect => message(IpcMessage::Connect(0)),
            _ => todo!(),
        },
        DdrpcSubcommands::Kill => message(IpcMessage::Kill),
        _ => todo!(),
    }
}

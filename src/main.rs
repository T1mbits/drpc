pub mod config;
pub mod parser;

use discord_rpc_client::Client;

use config::{initialize_config, structure::DConfig, write_config};
use parser::parse_command;

fn main() {
    let config: DConfig = initialize_config();
    let mut client: Client = Client::new(config.discord.client_id);
    client.start();
    client.on_ready(|_context| {
        println!("Discord Client ready!");
    });
    client.on_error(|context| println!("Error: {:?}", context));
    match client.set_activity(|activity| activity.state("test")) {
        Ok(_) => {}
        Err(error) => {
            println!("Error while setting activity: {}", error)
        }
    }
    ();
    println!("should've set activity now");
    write_config(&config);
    parse_command();
    loop {}
}

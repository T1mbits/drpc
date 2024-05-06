use crate::{
    prelude::*,
    processes::{get_active_data, get_names},
    spotify::{get_currently_playing_track, TrackData},
};
use rspotify::AuthCodeSpotify;
use std::collections::HashMap;

/// Create the hashmap for template variables and their replacements in Discord data.
#[instrument(skip_all)]
pub async fn template_hashmap<'th>(
    config: &Config,
    client: &AuthCodeSpotify,
) -> HashMap<&'th str, String> {
    let processes = get_names(&config.processes);

    let (process_text, process_icon) = get_active_data(&config.processes, &processes);
    let track = match get_currently_playing_track(client).await.unwrap() {
        None => TrackData::fallback(&config.spotify.fallback),
        Some(track_data) => track_data,
    };

    let mut replace_hashmap: HashMap<&str, String> = HashMap::new();
    replace_hashmap.insert("process.icon", process_icon);
    replace_hashmap.insert("process.text", process_text);
    replace_hashmap.insert("idle.icon", config.processes.idle_image.to_owned());
    replace_hashmap.insert("idle.text", config.processes.idle_text.to_owned());

    replace_hashmap.insert("spotify.track.name", track.name);
    replace_hashmap.insert("spotify.track.artists", track.artists);
    replace_hashmap.insert("spotify.track.url", track.track_url);
    replace_hashmap.insert("spotify.album.cover", track.album_cover_url);
    replace_hashmap.insert("spotify.album.name", track.album_name);

    trace!("Template variable hashmap created");
    return replace_hashmap;
}

/// Replace recognized template variables with their corresponding data.
#[instrument(skip_all)]
pub fn replace_template_variables(
    template_hashmap: &HashMap<&str, String>,
    mut string: String,
) -> String {
    if string.is_empty() || !string.contains("{{") {
        trace!("String does not contain template variables");
        return string;
    }

    // let nestable_template_variables = vec!["spotify"];

    for (target, replacement) in template_hashmap {
        let target = format!("{{{{{}}}}}", target);
        // trace!("Replacing template variable \"{target}\" with \"{replacement}\"");
        string = string.replace(&target, replacement);
    }

    return string;
}

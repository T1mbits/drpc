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
) -> HashMap<String, String> {
    let (process_text, process_icon) =
        get_active_data(&config.processes, &get_names(&config.processes));
    let track = match get_currently_playing_track(client).await.unwrap() {
        None => TrackData::fallback(&config.spotify.fallback),
        Some(track_data) => track_data,
    };

    let mut replace_hashmap: HashMap<String, String> = HashMap::new();
    replace_hashmap.insert(String::from("process.icon"), process_icon);
    replace_hashmap.insert(String::from("process.text"), process_text);
    replace_hashmap.insert(
        String::from("idle.icon"),
        config.processes.idle_image.to_owned(),
    );
    replace_hashmap.insert(
        String::from("idle.text"),
        config.processes.idle_text.to_owned(),
    );

    replace_hashmap.insert(String::from("spotify.track.name"), track.name);
    replace_hashmap.insert(String::from("spotify.track.artists"), track.artists);
    replace_hashmap.insert(String::from("spotify.track.url"), track.track_url);
    replace_hashmap.insert(String::from("spotify.album.cover"), track.album_cover_url);
    replace_hashmap.insert(String::from("spotify.album.name"), track.album_name);

    trace!("Template variable hashmap created");
    return replace_hashmap
        .iter()
        .map(|(key, value)| {
            (
                key.to_owned(),
                replace_template_variables(&replace_hashmap, value.to_owned()),
            )
        })
        .collect();
}

/// 3+ hours of wasted time Dx it doesn't even work thats the worst part but whatever
#[instrument(skip_all)]
fn nested_variables(template_hashmap: HashMap<String, String>) -> HashMap<String, String> {
    let nestable_variables = vec!["spotify"];

    let nest_permitted_hashmap: HashMap<String, String> = template_hashmap
        .iter()
        .filter_map(|(variable, replacement)| {
            for var in &nestable_variables {
                if variable.starts_with(var) {
                    return None;
                }
            }
            return Some((variable.to_owned(), replacement.to_owned()));
        })
        .collect();

    trace!("Created nested hashmap");

    return template_hashmap.iter().map(|(key, value)| {
        if value.is_empty()
            || !value.contains("{{")
            || nestable_variables.iter().any(|var| value.starts_with(var))
        {
			trace!("{key} does not contain template variables or is not allowed to have nested template variables");
			return (key.to_owned(), value.to_owned());
		}

		let mut string = value.to_owned();

		for (target, replacement) in &nest_permitted_hashmap {
			let target = format!("{{{{{}}}}}", target);
			trace!("Replacing \"{target}\" with \"{replacement}\" in \"{key}\"");
            string = string.replace(&target, &replacement);
		}

        return (key.to_owned(), string);
    }).collect();
}

/// Replace recognized template variables with their corresponding data.
#[instrument(skip_all)]
pub fn replace_template_variables(
    template_hashmap: &HashMap<String, String>,
    mut string: String,
) -> String {
    if string.is_empty() || !string.contains("{{") {
        trace!("String does not contain template variables");
        return string;
    }

    for (target, replacement) in template_hashmap {
        let target = format!("{{{{{}}}}}", target);
        trace!("Replacing template variable \"{target}\" with \"{replacement}\"");
        string = string.replace(&target, replacement);
    }

    return string;
}

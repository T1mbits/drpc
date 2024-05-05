use crate::config::{write_config, Config, SpotifyFallbackConfig};
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{AdditionalType, PlayableItem},
    scopes,
    sync::Mutex,
    AuthCodeSpotify, Credentials, OAuth, Token,
};
use std::sync::Arc;
use tracing::{debug, error, instrument, trace};

#[instrument(skip_all)]
pub async fn client_init(config: &mut Config) -> Result<AuthCodeSpotify, ()> {
    let credentials: Credentials =
        Credentials::new(&config.spotify.client_id, &config.spotify.client_secret);
    let oauth: OAuth = OAuth {
        redirect_uri: "http://localhost:3000/callback".to_string(),
        state: "ddrpcscope".to_string(),
        scopes: scopes!("user-read-currently-playing"),
        proxies: None,
    };
    let mut client = AuthCodeSpotify::new(credentials, oauth);

    match authorize(config, &mut client).await {
        Err(_) => return Err(()),
        _ => (),
    }

    match save_refresh_token(config, &client).await {
        Err(_) => return Err(()),
        _ => (),
    };

    return Ok(client);
}

#[instrument(skip_all)]
async fn authorize(config: &mut Config, client: &mut AuthCodeSpotify) -> Result<(), ()> {
    if config.spotify.refresh_token.is_empty() {
        trace!("No refresh token found, requesting authorization");
        let url = match client.get_authorize_url(false) {
            Err(error) => {
                error!("Error: {error}");
                return Err(());
            }
            Ok(url) => url,
        };

        return match client.prompt_for_token(&url).await {
            Err(error) => {
                error!("Error: {error}");
                Err(())
            }
            Ok(_) => {
                debug!("Spotify client successfully authenticated");
                Ok(())
            }
        };
    }

    let mut token = Token::default();
    token.refresh_token = Some(config.spotify.refresh_token.to_owned());
    token.scopes = scopes!("user-read-currently-playing");

    trace!("Created {token:?} from refresh token");

    client.token = Arc::new(Mutex::new(Some(token)));

    Ok(())
}

#[instrument(skip_all)]
async fn save_refresh_token(config: &mut Config, client: &AuthCodeSpotify) -> Result<(), ()> {
    trace!("Attempting to extract token");

    let token_mutex = client.get_token();

    return match token_mutex
        .lock()
        .await
        .expect("Token mutex poisoned")
        .as_mut()
    {
        None => unreachable!("Token field should always be available if this function is called"),
        Some(token) => match &token.refresh_token {
            None => {
                unreachable!("Refresh token should always be available if this function is called")
            }
            Some(refresh_token) => {
                trace!("Spotify refresh token: {refresh_token:?}");
                config.spotify.refresh_token = refresh_token.to_owned();

                match write_config(config) {
                    Err(_) => Err(()),
                    Ok(_) => Ok(()),
                }
            }
        },
    };
}

#[instrument(skip_all)]
pub async fn get_currently_playing_track(
    client: &AuthCodeSpotify,
) -> Result<Option<TrackData>, ()> {
    match client
        .current_playing(None, Some(&[AdditionalType::Track]))
        .await
    {
        Err(error) => {
            error!("Error: {error}");
            return Err(());
        }
        Ok(context) => match context {
            Some(context) if context.is_playing => {
                if let Some(PlayableItem::Track(track)) = context.item {
                    let mut artists: String = String::new();
                    for (index, artist) in track.artists.iter().enumerate() {
                        if index == 0 {
                            artists.push_str(&artist.name);
                        } else {
                            artists.push_str(&format!(", {}", artist.name))
                        }
                    }

                    let track_url = match track.external_urls.get_key_value("spotify") {
                        None => unreachable!("Track URL should always be available at this point"),
                        Some(url) => url.1.to_owned(),
                    };

                    return Ok(Some(TrackData {
                        album_cover_url: track.album.images[0].url.to_owned(),
                        album_name: track.album.name,
                        artists,
                        name: track.name,
                        track_url,
                    }));
                }
                return Ok(None);
            }
            _ => return Ok(None),
        },
    }
}

#[derive(Debug)]
pub struct TrackData {
    pub album_name: String,
    pub album_cover_url: String,
    pub artists: String,
    pub name: String,
    pub track_url: String,
}

impl TrackData {
    pub fn fallback(config: &SpotifyFallbackConfig) -> Self {
        return Self {
            album_name: config.album_name.to_owned(),
            album_cover_url: config.album_cover_url.to_owned(),
            artists: config.artists.to_owned(),
            name: config.name.to_owned(),
            track_url: config.track_url.to_owned(),
        };
    }
}

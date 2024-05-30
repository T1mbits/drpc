use crate::prelude::*;
use rspotify::{
    clients::{BaseClient, OAuthClient},
    model::{AdditionalType, PlayableItem},
    scopes,
    sync::Mutex,
    AuthCodeSpotify, Credentials, OAuth, Token,
};
use std::sync::Arc;

#[instrument(skip_all)]
pub async fn client_init(
    config: &mut SpotifyConfig,
) -> Result<Option<AuthCodeSpotify>, Box<dyn Error>> {
    if config.client_id.is_empty() || config.client_secret.is_empty() {
        trace!("No Spotify client ID and/or secret available");
        warn!("Skipping Spotify authorization. Spotify fields will use fallback values.");
        return Ok(None);
    }
    let credentials: Credentials = Credentials::new(&config.client_id, &config.client_secret);
    let oauth: OAuth = OAuth {
        redirect_uri: "http://localhost:3000/callback".to_string(),
        state: "ddrpcscope".to_string(),
        scopes: scopes!("user-read-currently-playing"),
        proxies: None,
    };
    let mut client = AuthCodeSpotify::new(credentials, oauth);
    trace!("Spotify client initialized");

    match authorize(config, &mut client).await {
        Err(_) => return Ok(None),
        _ => (),
    }

    save_refresh_token(config, &client).await?;

    return Ok(Some(client));
}

/// Has a blank result so that [`client_init`] can know to return a `None` instead of crashing the program
#[instrument(skip_all)]
async fn authorize(config: &SpotifyConfig, client: &mut AuthCodeSpotify) -> Result<(), ()> {
    if config.refresh_token.is_empty() {
        trace!("No refresh token found, requesting authorization");
        let url: String = match client.get_authorize_url(false) {
            Err(error) => {
                error!("{error}");
                warn!("Skipping Spotify authorization. Spotify fields will use fallback values.");
                return Err(());
            }
            Ok(url) => url,
        };

        return match client.prompt_for_token(&url).await {
            Err(error) => {
                error!("Error: {error}");
                warn!("Skipping Spotify authorization. Spotify fields will use fallback values.");
                Err(())
            }
            _ => {
                debug!("Spotify client successfully authenticated");
                Ok(())
            }
        };
    }

    let mut token: Token = Token::default();
    token.refresh_token = Some(config.refresh_token.to_owned());
    token.scopes = scopes!("user-read-currently-playing");

    trace!("Created {token:?} from refresh token");

    client.token = Arc::new(Mutex::new(Some(token)));

    Ok(())
}

#[instrument(skip_all)]
async fn save_refresh_token(
    config: &mut SpotifyConfig,
    client: &AuthCodeSpotify,
) -> Result<(), Box<dyn Error>> {
    trace!("Attempting to extract token");

    let token_mutex: Arc<Mutex<Option<Token>>> = client.get_token();

    let refresh_token = token_mutex.lock().await.expect("Token mutex poisoned");
    let refresh_token: &String = refresh_token
        .as_ref()
        .expect("Token field should always be available at this point")
        .refresh_token
        .as_ref()
        .expect("Refresh token should always be available if this function is called");

    trace!("Spotify refresh token: {:?}", refresh_token);
    config.refresh_token = refresh_token.to_owned();

    return write_config(config);

    // return match token_mutex
    //     .lock()
    //     .await
    //     .expect("Token mutex poisoned")
    //     .as_mut()
    // {
    //     None => unreachable!("Token field should always be available if this function is called"),
    //     Some(token) => match &token.refresh_token {
    //         None => {
    //             unreachable!("Refresh token should always be available if this function is called")
    //         }
    //         Some(refresh_token) => {
    //             trace!("Spotify refresh token: {refresh_token:?}");
    //             config.refresh_token = refresh_token.to_owned();

    //             write_config(config)
    //         }
    //     },
    // };
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

                    let track_url: String = match track.external_urls.get_key_value("spotify") {
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
                trace!("No track playing");
                return Ok(None);
            }
            _ => {
                trace!("No track detected");
                return Ok(None);
            }
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

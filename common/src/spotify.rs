use crate::log::*;
use anyhow::anyhow;
use rspotify::{
    model::{AdditionalType, FullTrack, PlayableItem},
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodePkceSpotify, Config, Credentials, OAuth,
};
use std::sync::Arc;
use tokio::sync::RwLock;

async fn create_client() -> AuthCodePkceSpotify {
    AuthCodePkceSpotify::with_config(
        Credentials::new_pkce("6bed7d8f6fcd465ea19f83bd4abec919"),
        OAuth {
            redirect_uri: "http://localhost:8888/callback".to_string(),
            scopes: scopes!("user-read-playback-state"),
            ..Default::default()
        },
        Config {
            token_cached: true,
            cache_path: dirs::cache_dir()
                .expect("No cache directory found")
                .join("ddrpc/ddrpc_spotify_token.json"),
            ..Default::default()
        },
    )
}

pub async fn try_authentication_from_cache() -> anyhow::Result<AuthCodePkceSpotify> {
    let client = create_client().await;

    debug!("{:?}", client.get_config());

    match client.read_token_cache(true).await {
        Err(err) => Err(anyhow!("Couldn't read from the token cache: {err}")),
        Ok(token) => {
            if let Some(token) = token {
                debug!("token: {:?}", token);
                let token_mutex = client.get_token();
                let mut token_mutex = token_mutex.lock().await.expect("Mutex Poisoned");
                *token_mutex = Some(token);
                return Ok(client);
            }
            return Err(anyhow!("The token cache contained invalid scopes."));
        }
    }
}

pub async fn authenticate_pkce() -> anyhow::Result<()> {
    let mut client = create_client().await;

    if let Ok(token) = client.read_token_cache(true).await {
        if token.is_some() {
            println!("A valid token cache already exists, no need to re-authenticate");
            return Ok(());
        }
    }

    let url = client.get_authorize_url(None)?;
    // TODO replace with automated code callback handling
    client.prompt_for_token(&url).await?;

    client.write_token_cache().await?;

    return Ok(());
}

// HACK return custom type with relevant data rather than the whole FullTrack struct
pub async fn get_song_data(client: Arc<RwLock<AuthCodePkceSpotify>>) -> Option<FullTrack> {
    let client = client.read().await;

    if client
        .get_token()
        .lock()
        .await
        .expect("Mutex poisoned")
        .as_ref()
        .is_some_and(|token| token.is_expired())
    {
        client.refresh_token().await.unwrap();
        client.write_token_cache().await.unwrap();
    }

    let track = match client
        .current_playing(None, Some([&AdditionalType::Track]))
        .await
        .unwrap()
        .unwrap()
        .item
    {
        None => None,
        Some(item) => match item {
            PlayableItem::Episode(_) => unimplemented!(),
            PlayableItem::Track(track) => Some(track),
        },
    };

    track
}

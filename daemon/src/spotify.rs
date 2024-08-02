use anyhow::anyhow;
use common::{config::cache_file, log::*};
use rspotify::{
    model::{AdditionalType, FullTrack, PlayableItem},
    prelude::{BaseClient, OAuthClient},
    scopes, AuthCodePkceSpotify, Config, Credentials, OAuth,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::{oneshot::channel, RwLock};
use warp::{filters::query::query, http::StatusCode, path, reply, serve, Filter};

async fn create_client() -> AuthCodePkceSpotify {
    AuthCodePkceSpotify::with_config(
        Credentials::new_pkce("6bed7d8f6fcd465ea19f83bd4abec919"),
        OAuth {
            redirect_uri: "http://localhost:3772/callback".to_string(),
            scopes: scopes!("user-read-playback-state"),
            ..Default::default()
        },
        Config {
            token_cached: true,
            cache_path: cache_file(),
            ..Default::default()
        },
    )
}

pub async fn authenticate_pkce() -> anyhow::Result<AuthCodePkceSpotify> {
    let mut client = create_client().await;

    if let Ok(Some(token)) = client.read_token_cache(true).await {
        let expired = token.is_expired();

        *client.get_token().lock().await.unwrap() = Some(token);

        if expired {
            if let Some(refreshed_token) = client.refetch_token().await? {
                info!("Successfully refreshed expired token from token cache");
                *client.get_token().lock().await.unwrap() = Some(refreshed_token);
            }
        }
    }

    let url = client.get_authorize_url(None)?;
    let code = get_code(url, client.get_oauth().state.clone()).await?;
    client.request_token(&code).await?;

    client.write_token_cache().await?;

    return Ok(client);
}

// TODO change output to just anyhow::Result<String> and return error if code couldn't be given instead of None
async fn get_code(url: String, state: String) -> anyhow::Result<String> {
    let (send, recv) = channel::<()>();
    let send = Arc::new(Mutex::new(Some(send)));
    let code: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));
    let code_ref = code.clone();

    // Create the warp server with the defined route
    let (_, server) = serve(
        path!("callback")
            .and(query::<HashMap<String, String>>())
            .map(move |params: HashMap<String, String>| {
                // Trigger server shutdown
                if let Some(send) = send.lock().unwrap().take() {
                    let _ = send.send(());
                }

                if params["state"].is_empty() || params["code"].is_empty() {
                    let msg = "Code or state is missing";
                    error!("{msg}");
                    return reply::with_status(msg, StatusCode::BAD_REQUEST);
                }

                if params["state"] != state {
                    let msg = "Request state doesn't match callback state";
                    error!("{msg}");
                    return reply::with_status(msg, StatusCode::BAD_REQUEST);
                }

                *code_ref.lock().unwrap() = Some(params["code"].clone());
                reply::with_status("Code successfully parsed", StatusCode::OK)
            }),
    )
    .bind_with_graceful_shutdown(([127, 0, 0, 1], 3772), async {
        recv.await.ok();
    });

    webbrowser::open(&url)?;

    info!("Opened authentication page in your browser");
    server.await;

    let mut guard = code.lock().unwrap();
    guard
        .take()
        .ok_or_else(|| anyhow!("Unable to get response code"))
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
    {
        None => None,
        Some(context) => match context.item {
            None => None,
            Some(item) => match item {
                PlayableItem::Episode(_) => unimplemented!(),
                PlayableItem::Track(track) => Some(track),
            },
        },
    };

    track
}

use std::sync::Arc;

use crate::config::{write_config, Config};
use rspotify::{
    clients::{BaseClient, OAuthClient},
    scopes,
    sync::Mutex,
    AuthCodeSpotify, Credentials, OAuth, Token,
};
use tracing::{debug, error, instrument, trace};

#[instrument(skip_all)]
pub async fn generate_client(config: &mut Config) -> Result<AuthCodeSpotify, ()> {
    let credentials: Credentials =
        Credentials::new(&config.spotify.client_id, &config.spotify.client_secret);
    let oauth: OAuth = OAuth {
        redirect_uri: "http://localhost:3000/callback".to_string(),
        state: "ddrpcscope".to_string(),
        scopes: scopes!("user-read-currently-playing"),
        proxies: None,
    };
    let mut spotify = AuthCodeSpotify::new(credentials, oauth);

    authorize(config, &mut spotify).await.unwrap();

    config.spotify.refresh_token = extract_refresh_token(&spotify).await.unwrap();

    write_config(config).unwrap();

    Err(())
}

#[instrument(skip_all)]
async fn authorize(config: &mut Config, client: &mut AuthCodeSpotify) -> Result<(), ()> {
    if config.spotify.refresh_token.is_empty() {
        trace!("No refresh token found, requesting authorization");
        let url = client.get_authorize_url(false).unwrap();
        return match client.prompt_for_token(&url).await {
            Err(error) => {
                error!("Error: {error}");
                Err(())
            }
            Ok(_) => {
                debug!("client");
                Ok(())
            }
        };
    }

    let mut token = Token::default();
    token.refresh_token = Some(config.spotify.refresh_token.to_owned());
    token.scopes = scopes!("user-read-currently-playing");

    trace!("{token:?}");

    client.token = Arc::new(Mutex::new(Some(token)));

    Ok(())
}

#[instrument(skip_all)]
async fn extract_refresh_token(client: &AuthCodeSpotify) -> Result<String, ()> {
    trace!("Attempting to extract token");

    let token_mutex = client.get_token();

    // Apparently mutex lock will only fail if its poisoned
    return match token_mutex.lock().await.unwrap().as_mut() {
        None => unreachable!("Token field should always be available if this function is called"),
        Some(token) => match &token.refresh_token {
            None => {
                unreachable!("Refresh token should always be available if this function is called")
            }
            Some(refresh_token) => {
                trace!("{refresh_token:?}");
                Ok(refresh_token.to_owned())
            }
        },
    };
}

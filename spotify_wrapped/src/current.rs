use rspotify::{
    model::{PlayableItem, CurrentPlaybackContext},
    clients::{OAuthClient},
    scopes,
    AuthCodeSpotify,
    Credentials,
    OAuth,
    ClientResult
};

use dotenvy::dotenv;

pub async fn get_current_listening() {     
    let creds = load_env_vars();

    let playback = get_handle(creds).await;

    determine_track(playback);
}

pub fn load_env_vars() -> Credentials {
    dotenv().ok();

    let creds = Credentials::from_env().unwrap();

    creds
}

async fn get_handle(creds: Credentials) -> ClientResult<Option<CurrentPlaybackContext>> {
    let oauth = OAuth {
        redirect_uri: std::env::var("RSPOTIFY_REDIRECT_URI").unwrap(),
        scopes: scopes!(
            "user-read-currently-playing",
            "user-read-playback-state"
        ),
        ..Default::default()
    };

    let spotify = AuthCodeSpotify::new(creds, oauth);

    let url = spotify.get_authorize_url(false).unwrap();

    println!("Open this URL in your browser:\n{}", url);

    spotify
        .prompt_for_token(&url)
        .await
        .unwrap();

    let playback = spotify
    .current_playback(None, None::<Vec<_>>)
    .await;

    playback
}

fn determine_track(playback: ClientResult<Option<CurrentPlaybackContext>>) {
    match playback {
        Ok(Some(context)) => {
            if let Some(item) = context.item {
                match item {
                    PlayableItem::Track(track) => {
                        println!("Currently playing: {}", track.name);
                    }
                    PlayableItem::Episode(episode) => {
                        println!("Currently playing podcast: {}", episode.name);
                    }
                    PlayableItem::Unknown(value) => {
                        let track_name = value
                            .get("name")
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown");

                        let artist_name = value
                            .get("artists")
                            .and_then(|v| v.as_array())
                            .and_then(|artists| artists.first())
                            .and_then(|artist| artist.get("name"))
                            .and_then(|v| v.as_str())
                            .unwrap_or("Unknown Artist");

                        println!("Currently playing: {} by {}", track_name, artist_name);
                    }
                }
            } else {
                println!("No item currently playing");
            }
        }
        Ok(None) => {
            println!("No active playback");
        }
        Err(e) => {
            println!("Spotify API error: {:?}", e);
        }
    }
}
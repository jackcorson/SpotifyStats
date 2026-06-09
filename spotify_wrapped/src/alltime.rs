use rspotify::{
    model::{FullArtist, FullTrack, TimeRange},
    clients::{OAuthClient},
    scopes,
    AuthCodeSpotify,
    Credentials,
    OAuth,
};

use rspotify::ClientError;

use futures_util::StreamExt;

use crate::current;

pub async fn get_last_month() {
    let creds = current::load_env_vars();

    let (artists, tracks) = get_handle(creds, TimeRange::ShortTerm)
        .await
        .unwrap();

    print_top_artists(artists, String::from("last month"));
    print_top_tracks(tracks, String::from("last month"));
}

pub async fn get_last_six_months() {
    let creds = current::load_env_vars();

    let (artists, tracks) = get_handle(creds, TimeRange::MediumTerm)
        .await
        .unwrap();

    print_top_artists(artists, String::from("last 6 months"));
    print_top_tracks(tracks, String::from("last 6 months"));
}

pub async fn get_last_year() {
    let creds = current::load_env_vars();

    let (artists, tracks) = get_handle(creds, TimeRange::LongTerm)
        .await
        .unwrap();

    print_top_artists(artists, String::from("last year"));
    print_top_tracks(tracks, String::from("last year"));
}

 // Return tuple (top artists, top tracks)
async fn get_handle(creds: Credentials, time_range: TimeRange) -> Result<(Vec<FullArtist>, Vec<FullTrack>), ClientError> {
    let oauth = OAuth {
        redirect_uri: std::env::var("RSPOTIFY_REDIRECT_URI").unwrap(),
        scopes: scopes!(
            "user-top-read"
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

    let top_artists: Vec<FullArtist> = spotify
        .current_user_top_artists(Some(time_range))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    let top_tracks: Vec<FullTrack> = spotify
        .current_user_top_tracks(Some(time_range))
        .collect::<Vec<_>>()
        .await
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;


    Ok((top_artists, top_tracks))
}

fn print_top_artists(top_artists: Vec<FullArtist>, time_range: String) {
    println!("Top ten artists in order ({time_range})");
    println!("-----------------------------------------");
    for (rank, artist) in top_artists.iter().take(10).enumerate() {
        println!("{}. {:?}", rank + 1, artist.name);
    }
    println!("\n");
}

fn print_top_tracks(top_tracks: Vec<FullTrack>, time_range: String) {
    println!("Top ten tracks in order ({time_range})");
    println!("-----------------------------------------");
    for (rank, track) in top_tracks.iter().take(10).enumerate() {
        println!("{}. {:?}", rank + 1, track.name);
    }
    println!("\n");
}
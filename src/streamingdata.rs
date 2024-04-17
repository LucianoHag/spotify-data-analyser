use serde::Deserialize;
use std::option::Option;

// TODO: somehow make serde accept my own struct
#[derive(Debug, Deserialize)]
pub enum StartReason {
    Trackdone,
    Backbtn,
    Fwdbtn,
    Playbtn,
    Trackbrror,
    Unknown,
    Appload,
    Remote,
    Clickrow,
    Endplay,
    // "trackdone",
    // "fwdbtn",
    // "playbtn",
    // "trackerror",
    // "unknown",
    // "appload",
    // "backbtn",
    // "remote",
    // "clickrow",
    // "endplay",
}

#[derive(Debug, Deserialize)]
pub enum EndReason {
    Fwdbtn,
    Endplay,
    Trackerror,
    Backbtn,
    UnexpectedExit,
    Logout,
    Trackdone,
    UnexpectedExitWhilePaused,
    Remote,
    Unknown,
    // "fwdbtn",
    // "endplay",
    // "trackerror",
    // "backbtn",
    // "unexpected-exit",
    // "logout",
    // "trackdone",
    // "unexpected-exit-while-paused",
    // "remote",
    // "unknown",
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct StreamingData {
    // Timestamp in UTC
    // ts: chrono::DateTime<chrono::Utc>,
    ts: String, // quickly to make it work

    // Username struct in between which we don't care about

    // Platform used to stream the song, e.g. Android, iOS, Web Player, Desktop
    platform: Option<String>,

    // How long the song was played in milliseconds
    pub ms_played: u32,

    // The country from which the song was played
    conn_country: Option<String>,

    // The IP address from which the song was played
    ip_addr_decrypted: Option<String>,

    // The user agent from which the song was played, e.g. Firefox, Chrome, Safari
    user_agent_decrypted: Option<String>,

    // The name of the track
    master_metadata_track_name: Option<String>,

    // The name of the artist
    master_metadata_album_artist_name: Option<String>,

    // The name of the album
    master_metadata_album_album_name: Option<String>,

    // The URI of the track
    spotify_track_uri: Option<String>,

    // The reason for the song to start and end
    pub reason_start: Option<String>,
    pub reason_end: Option<String>,

    // Boolean values indicating if the song was shuffled, skipped and played offline
    shuffle: Option<bool>,
    offline: Option<bool>,
    skipped: Option<bool>,

    // Timestamps for when the song was played offline if it was played online
    offline_timestamp: u64, // TODO: no clue what this means

    // Indicate if the song was played in incognito mode
    incognito_mode: Option<bool>,
}

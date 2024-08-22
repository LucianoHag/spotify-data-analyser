use chrono::Utc;
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
pub struct StreamingDataRaw {
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

impl StreamingDataRaw {
    pub fn is_song(&self) -> bool {
        self.spotify_track_uri.is_some()
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Track {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub uri: String,
}

impl TryInto<Track> for &StreamingDataRaw {
    type Error = ();

    fn try_into(self) -> Result<Track, Self::Error> {
        let name = self.master_metadata_track_name.clone().ok_or(())?;
        // .inspect_err(|_| println!("track name not valid"))?;
        let artist = self.master_metadata_album_artist_name.clone().ok_or(())?;
        // .inspect_err(|_| println!("artist name not valid"))?;
        let album = self.master_metadata_album_album_name.clone().ok_or(())?;
        // .inspect_err(|_| println!("album name not valid"))?;
        let uri_full = self.spotify_track_uri.clone().ok_or(())?;
        let uri = uri_full.split(":").last().ok_or(())?.to_string();

        // .inspect_err(|_| println!("track uri not valid"))?;

        Ok(Track {
            name,
            artist,
            album,
            uri,
        })
    }
}

#[derive(Debug)]
pub struct StreamingData {
    pub ts: chrono::DateTime<chrono::Utc>,
    ms_played: u32,
    spotify_track_uri: String,
    shuffle: Option<bool>,
    offline: Option<bool>,
    skipped: Option<bool>,
    reason_start: StartReason,
    reason_end: EndReason,
}

impl TryInto<StreamingData> for &StreamingDataRaw {
    type Error = ();

    fn try_into(self) -> Result<StreamingData, Self::Error> {
        let reason_start = match &self.reason_start {
            Some(reason) => match reason.as_str() {
                "trackdone" => StartReason::Trackdone,
                "backbtn" => StartReason::Backbtn,
                "fwdbtn" => StartReason::Fwdbtn,
                "playbtn" => StartReason::Playbtn,
                "trackerror" => StartReason::Trackbrror,
                "unknown" => StartReason::Unknown,
                "appload" => StartReason::Appload,
                "remote" => StartReason::Remote,
                "clickrow" => StartReason::Clickrow,
                "endplay" => StartReason::Endplay,
                _ => return Err(()),
            },
            None => return Err(()),
        };
        let reason_end = match &self.reason_end {
            Some(reason) => match reason.as_str() {
                "fwdbtn" => EndReason::Fwdbtn,
                "endplay" => EndReason::Endplay,
                "trackerror" => EndReason::Trackerror,
                "backbtn" => EndReason::Backbtn,
                "unexpected-exit" => EndReason::UnexpectedExit,
                "logout" => EndReason::Logout,
                "trackdone" => EndReason::Trackdone,
                "unexpected-exit-while-paused" => EndReason::UnexpectedExitWhilePaused,
                "remote" => EndReason::Remote,
                "unknown" => EndReason::Unknown,
                _ => return Err(()),
            },
            None => EndReason::Unknown, // If we return Err here, there'll be quite a few errors
        };
        // let ts = chrono::DateTime::parse_from_rfc3339(&self.ts);
        let ts_res = chrono::DateTime::parse_from_rfc3339(&self.ts);
        let ts = ts_res
            .inspect_err(|_| println!("Invalid datetime: {}", self.ts))
            .map_err(|_| ())?
            .with_timezone(&Utc);

        // All errors are this one!!
        let spotify_track_uri = self
            .spotify_track_uri
            .clone()
            .ok_or(())
            .inspect_err(|_| println!("track uri not valid?"))?;

        Ok(StreamingData {
            ts,
            ms_played: self.ms_played,
            spotify_track_uri,
            shuffle: self.shuffle,
            offline: self.offline,
            skipped: self.skipped,
            reason_start,
            reason_end,
        })
    }
}

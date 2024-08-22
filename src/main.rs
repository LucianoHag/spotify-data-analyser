use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::iter::zip;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

use streamingdata::Track;

use crate::streamingdata::{StreamingData, StreamingDataRaw};

mod streamingdata;

fn main() -> Result<(), ()> {
    // General idea:
    // 1. Get all files in the ./data/ directory
    // 2. Parse these files into a vec of structs
    // 3. Use this vector to store data for each track in a hashmap
    // 4. Somehow get interesting data from this hashmap

    // Part 1: get all files in the ./data/ directory
    let paths = get_directories().ok_or(())?;

    // let example = json!([
    //     {"ts":"2022-12-09T22:28:37Z","username":"swaggerboy02","platform":"android","ms_played":1596,"conn_country":"NL","ip_addr_decrypted":"89.205.128.220","user_agent_decrypted":null,"master_metadata_track_name":"Out of Time","master_metadata_album_artist_name":"The Weeknd","master_metadata_album_album_name":"Dawn FM","spotify_track_uri":"spotify:track:2SLwbpExuoBDZBpjfefCtV","episode_name":null,"episode_show_name":null,"spotify_episode_uri":null,"reason_start":"backbtn","reason_end":"backbtn","shuffle":false,"skipped":true,"offline":false,"offline_timestamp":1670624916,"incognito_mode":false},
    //     {"ts":"2022-12-09T22:28:39Z","username":"swaggerboy02","platform":"android","ms_played":1533,"conn_country":"NL","ip_addr_decrypted":"89.205.128.220","user_agent_decrypted":null,"master_metadata_track_name":"Sacrifice","master_metadata_album_artist_name":"The Weeknd","master_metadata_album_album_name":"Dawn FM","spotify_track_uri":"spotify:track:1nH2PkJL1XoUq8oE6tBZoU","episode_name":null,"episode_show_name":null,"spotify_episode_uri":null,"reason_start":"backbtn","reason_end":"backbtn","shuffle":false,"skipped":true,"offline":false,"offline_timestamp":1670624918,"incognito_mode":false},
    // ]);
    // dbg!(&example[0]["ts"]);

    // let example_struct: StreamingData = serde_json::from_value(example[0].clone()).unwrap();
    // dbg!(example_struct);

    // Part 2: parse these files into a vec of structs
    let mut all_streams: Vec<StreamingDataRaw> = Vec::new();

    let start_overall = Instant::now();

    // let first_file = File::open(&files[0]).unwrap();
    for path in paths {
        println!("Parsing file: {:?}", path);
        let start = Instant::now();
        let file = if let Ok(file) = File::open(&path) {
            file
        } else {
            continue;
        };
        let reader = std::io::BufReader::new(file);
        let json: serde_json::Value = if let Ok(json) = serde_json::from_reader(reader) {
            json
        } else {
            continue;
        };
        println!("Parsing took: {:?}", start.elapsed());

        let start = Instant::now();
        let array = if let Some(array) = json.as_array() {
            array
        } else {
            continue;
        };
        let mapped: Vec<StreamingDataRaw> = array
            .iter()
            .map(|x| {
                serde_json::from_value(x.clone())
                    .unwrap_or_else(|_| panic!("Wasn't able to parse {x}"))
            })
            .filter(|x: &StreamingDataRaw| x.is_song())
            .collect();
        println!("Mapping took: {:?}", start.elapsed());

        all_streams.extend(mapped);
        // println!("Parsed file: {:?}", path);
        println!("Current length: {}", all_streams.len());
    }
    // dbg!(&all_streams[0]);
    dbg!(all_streams.len());
    // dbg!(json);

    println!("Overall parsing took: {:?}", start_overall.elapsed());

    let total_time: u64 = all_streams
        .iter()
        .fold(0, |value: u64, stream| value + stream.ms_played as u64);

    let duration = chrono::Duration::milliseconds(total_time as i64);

    println!("Total stream time in hours: {}", duration.num_hours());

    // let start = Instant::now();
    // let mapped_streams = all_streams
    //     .iter()
    //     .map(|x| x.try_into())
    //     .collect::<Vec<Result<StreamingData, ()>>>();
    // println!("Mapping took: {:?}", start.elapsed());

    // let start = Instant::now();
    // let mut mapped_streams = mapped_streams
    //     .into_iter()
    //     .filter(|x| x.is_ok())
    //     .map(|res| res.expect("Err values should have been filtered"))
    //     .collect::<Vec<StreamingData>>();
    // println!("Filtering took: {:?}", start.elapsed());

    // This should achieve the same as the stuff above
    let mut mapped_streams = all_streams
        .iter()
        .flat_map(|x| x.try_into())
        .collect::<Vec<StreamingData>>();

    let track_vec = all_streams
        .iter()
        .flat_map(|x| x.try_into())
        .collect::<Vec<Track>>();

    // mapped_streams.sort_by(|a, b| a.ts.cmp(&b.ts));

    // println!("Mapped stream: {:?}", mapped_streams[0]);

    let duration_vec = all_streams
        .iter()
        .map(|x| Duration::from_millis(x.ms_played as u64))
        .collect::<Vec<Duration>>();

    let something_vec = zip(track_vec, duration_vec).collect::<Vec<(Track, Duration)>>();

    // let track_set = all_streams
    //     .iter()
    //     .flat_map(|x| x.try_into())
    //     .collect::<HashSet<Track>>();
    // // let track_set =

    // TODO: add the amount of streams as a value to the hashmap
    let mut track_hashmap: HashMap<Track, Duration> = HashMap::new();

    for (track, time) in something_vec.into_iter() {
        if track_hashmap.contains_key(&track) {
            if let Some(duration) = track_hashmap.get_mut(&track) {
                // TODO: fix this so it compiles lmao
                *duration += time;
                // let _ = track_hashmap.insert(track, duration);
            }
        } else {
            track_hashmap.insert(track, time);
        }
    }

    let mut track_vec = track_hashmap
        .into_iter()
        .collect::<Vec<(Track, Duration)>>();
    track_vec.sort_by(|a, b| b.1.cmp(&a.1));
    // println!("Track vec: {:?}", &track_vec[..20]);
    for (i, (track, duration)) in track_vec[..20].iter().enumerate() {
        println!("Number {}: Track: {:?}, Duration: {:?}", i, track, duration);
    }

    Ok(())
    // let example = json!([
    //     {"ts":"2022-12-09T22:28:37Z","username":"swaggerboy02","platform":"android","ms_played":1596,"conn_country":"NL","ip_addr_decrypted":"89.205.128.220","user_agent_decrypted":null,"master_metadata_track_name":"Out of Time","master_metadata_album_artist_name":"The Weeknd","master_metadata_album_album_name":"Dawn FM","spotify_track_uri":"spotify:track:2SLwbpExuoBDZBpjfefCtV","episode_name":null,"episode_show_name":null,"spotify_episode_uri":null,"reason_start":"backbtn","reason_end":"backbtn","shuffle":false,"skipped":true,"offline":false,"offline_timestamp":1670624916,"incognito_mode":false},
    // ]);
    // dbg!(&example[0]);
    // dbg!(example.);

    // First try to figure out what types of reason_starts and reason_ends exists
    // let mut reason_starts: HashSet<String> = HashSet::new();
    // let mut reason_ends: HashSet<String> = HashSet::new();
    // let mut nulls = 0;
    // for stream in all_streams.iter_mut() {
    //     if let Some(reason_start) = &stream.reason_start {
    //         reason_starts.insert(reason_start.clone());
    //     } else {
    //         stream.reason_start = Some("unknown".to_string());
    //         nulls += 1;
    //     }
    //     if let Some(reason_end) = &stream.reason_end {
    //         reason_ends.insert(reason_end.clone());
    //     } else {
    //         stream.reason_end = Some("unknown".to_string());
    //         nulls += 1;
    //     }
    // }

    // dbg!(&reason_starts);
    // dbg!(&reason_ends);
    // dbg!(nulls);
}

fn get_directories() -> Option<Vec<PathBuf>> {
    let directory = Path::new("./data/");
    let entries = fs::read_dir(directory).ok()?;
    Some(entries.flatten().map(|entry| entry.path()).collect())
}

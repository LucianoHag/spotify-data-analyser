use std::fs::{self, File};
use std::path::Path;

use crate::streamingdata::StreamingData;

mod streamingdata;

fn main() {
    // General idea:
    // 1. Get all files in the ./../data/ directory
    // 2. Parse these files into a vec of structs
    // 3. Use this vector to store data for each track in a hashmap
    // 4. Somehow get interesting data from this hashmap

    // Part 1: get all files in the ./../data/ directory
    let directory = Path::new("./../data/");
    let mut paths = Vec::new();
    for entry in fs::read_dir(directory).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        paths.push(path);
    }
    // dbg!(files); // This works

    // let example = json!([
    //     {"ts":"2022-12-09T22:28:37Z","username":"swaggerboy02","platform":"android","ms_played":1596,"conn_country":"NL","ip_addr_decrypted":"89.205.128.220","user_agent_decrypted":null,"master_metadata_track_name":"Out of Time","master_metadata_album_artist_name":"The Weeknd","master_metadata_album_album_name":"Dawn FM","spotify_track_uri":"spotify:track:2SLwbpExuoBDZBpjfefCtV","episode_name":null,"episode_show_name":null,"spotify_episode_uri":null,"reason_start":"backbtn","reason_end":"backbtn","shuffle":false,"skipped":true,"offline":false,"offline_timestamp":1670624916,"incognito_mode":false},
    //     {"ts":"2022-12-09T22:28:39Z","username":"swaggerboy02","platform":"android","ms_played":1533,"conn_country":"NL","ip_addr_decrypted":"89.205.128.220","user_agent_decrypted":null,"master_metadata_track_name":"Sacrifice","master_metadata_album_artist_name":"The Weeknd","master_metadata_album_album_name":"Dawn FM","spotify_track_uri":"spotify:track:1nH2PkJL1XoUq8oE6tBZoU","episode_name":null,"episode_show_name":null,"spotify_episode_uri":null,"reason_start":"backbtn","reason_end":"backbtn","shuffle":false,"skipped":true,"offline":false,"offline_timestamp":1670624918,"incognito_mode":false},
    // ]);
    // dbg!(&example[0]["ts"]);

    // let example_struct: StreamingData = serde_json::from_value(example[0].clone()).unwrap();
    // dbg!(example_struct);

    // Part 2: parse these files into a vec of structs
    let mut all_streams: Vec<StreamingData> = Vec::new();
    // let first_file = File::open(&files[0]).unwrap();
    for path in paths {
        println!("Parsing file: {:?}", path);
        let file = File::open(&path).unwrap();
        let json: serde_json::Value = serde_json::from_reader(file).unwrap();
        let mapped: Vec<StreamingData> = json
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                serde_json::from_value(x.clone())
                    .unwrap_or_else(|_| panic!("Wasn't able to parse {x}"))
            })
            .collect();

        all_streams.extend(mapped);
        println!("Parsed file: {:?}", path);
        println!("Current length: {}", all_streams.len());
    }
    // dbg!(&all_streams[0]);
    dbg!(all_streams.len());
    // dbg!(json);

    let total_time: u64 = all_streams
        .iter()
        .fold(0, |value: u64, stream| value + stream.ms_played as u64);

    let duration = chrono::Duration::milliseconds(total_time as i64);

    println!("Total stream time in hours: {}", duration.num_hours());
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

use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Track {
    pub id: String,
    pub url: String,
    pub title: String,
    pub author: String,
    pub duration: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Analytics {
    pub worker_id: String,
    pub cpu_usage: u8,
    pub memory_usage: u8,
    pub jobs_running: u32,
    pub disk_usage: u8,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Queue {
    pub guild_id: String,
    pub tracks: Vec<Track>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Playlist {
    pub guild_id: String,
    pub playlist_id: String,
    pub name: String,
    pub tracks: Vec<Track>,
}
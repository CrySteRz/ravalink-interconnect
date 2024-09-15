use serde_derive::{Deserialize, Serialize};
use crate::packets::{Track, Pong};

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Command {
    Search { query: String },
    Play { url: String },
    Stop,
    Pause,
    Resume,
    SeekToPosition { position: u64 },
    SetVolume { volume: f32 },
    GetPlaylists,
    AddToPlaylist { playlist_id: String, track: Box<Track> },
    RemoveFromPlaylist { playlist_id: String, track: Box<Track> },
    LoadPlaylist { playlist_id: String },
    ClearPlaylist { playlist_id: String },
    ShuffleQueue,
    Skip,
    Loop,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Request {
    pub job_id: String,
    pub guild_id: String,
    pub voice_channel_id: Option<String>,
    pub command: Command,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Event {
    pub event_type: EventType,
    pub job_id: String,
    pub guild_id: String,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum EventType {
    ChannelLeave,
    ChannelMove,
    AudioEnd,
    AudioStart,
    ErrorOccurred,
    JobExpired,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum ResponseType {
    Success,
    Failure { reason: String },
    SearchResults { tracks: Vec<Track> },
    PlayStarted,
    VolumeAdjusted,
    Playlists { playlists: Vec<String> },
    PlaylistLoaded { tracks: Vec<Track> },
    PlaylistCleared,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Response {
    pub job_id: String,
    pub guild_id: String,
    pub response_type: ResponseType,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum Message {
    Ping, 
    Pong(Pong),
    Request(Request),
    Event(Event),
    Response(Response),
}
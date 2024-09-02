use serde_derive::{Deserialize, Serialize};
use crate::packets::Track;

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum JobRequestType {
    Ping,
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
    Loop { enabled: bool },
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct JobRequest {
    pub job_id: String,
    pub worker_id: String,
    pub guild_id: String,
    pub voice_channel_id: Option<String>,
    pub command: JobRequestType,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum JobEventType {
    ChannelLeave,
    ChannelMove,
    AudioEnd,
    AudioStart,
    ErrorOccurred,
    JobExpired,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct JobEvent {
    pub event_type: JobEventType,
    pub job_id: String,
    pub guild_id: String,
    pub details: Option<String>,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum JobResponseType {
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
pub struct JobResponse {
    pub job_id: String,
    pub guild_id: String,
    pub response_type: JobResponseType,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub enum Message {
    JobRequest(JobRequest),
    JobEvent(JobEvent),
    JobResponse(JobResponse),
}
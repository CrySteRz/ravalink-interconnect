use url::Url;
use sha2::{Digest, Sha256};
use rand::seq::SliceRandom;
use crate::{packets::{Playlist, Queue, Track}, protocol::{{JobRequest, JobRequestType}}};

impl JobRequest {
    pub fn new(job_id: String, worker_id: String, guild_id: String, command: JobRequestType, timestamp: u64) -> Self {
        JobRequest {
            job_id,
            worker_id,
            guild_id,
            command,
            timestamp,
        }
    }
}

impl Track {
    pub fn new(url: String, title: String, author: String, duration: u64) -> Self {
        let normalized_url = Self::normalize_url(&url);
        let id = Self::generate_track_id(&normalized_url, &title, &author, duration);
        Track {
            id,
            url,
            title,
            author,
            duration,
        }
    }

    pub fn normalize_url(url: &str) -> String {
        let parsed_url = Url::parse(url).unwrap_or_else(|_| Url::parse("https://example.com").unwrap());

        match parsed_url.domain() {
            Some("www.youtube.com") | Some("youtu.be") => {
                if let Some(video_id) = parsed_url.query_pairs().find(|(key, _)| key == "v").map(|(_, v)| v.to_string()) {
                    return format!("https://youtube.com/watch?v={}", video_id);
                }
                if parsed_url.domain() == Some("youtu.be") {
                    return format!("https://youtube.com/watch?v={}", parsed_url.path().trim_start_matches('/'));
                }
            }
            Some("open.spotify.com") => {
                if let Some(segments) = parsed_url.path_segments() {
                    let normalized_path = segments.collect::<Vec<&str>>().join("/");
                    return format!("https://open.spotify.com/{}", normalized_path);
                }
            }
            Some("soundcloud.com") => {
                if let Some(segments) = parsed_url.path_segments() {
                    let normalized_path = segments.collect::<Vec<&str>>().join("/");
                    return format!("https://soundcloud.com/{}", normalized_path);
                }
            }
            _ => {}
        }

        url.to_string()
    }

    pub fn generate_track_id(url: &str, title: &str, author: &str, duration: u64) -> String {
        let input = format!("{}:{}:{}:{}", url, title, author, duration);
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}

impl Playlist {
    pub fn add_track(&mut self, track: Track) -> Result<(), String> {
        if self.tracks.iter().any(|t| t.id == track.id) {
            Err(format!("Track '{}' by '{}' is already in the playlist.", track.title, track.author))
        } else {
            self.tracks.push(track);
            Ok(())
        }
    }

    pub fn remove_track(&mut self, track_id: &str) -> Result<(), String> {
        if let Some(index) = self.tracks.iter().position(|t| t.id == track_id) {
            self.tracks.remove(index);
            Ok(())
        } else {
            Err(format!("Track with ID '{}' not found in the playlist.", track_id))
        }
    }
}

impl Queue {
    pub fn add_track(&mut self, track: Track) {
        self.tracks.push(track);
    }

    pub fn remove_track(&mut self, track_id: &str) -> Result<(), String> {
        if let Some(index) = self.tracks.iter().position(|t| t.id == track_id) {
            self.tracks.remove(index);
            Ok(())
        } else {
            Err(format!("Track with ID '{}' not found in the queue.", track_id))
        }
    }

    pub fn clear(&mut self) {
        self.tracks.clear();
    }

    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.tracks.shuffle(&mut rng);
    }
}
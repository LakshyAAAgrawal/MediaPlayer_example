mod media_player;
mod playlist;
use crate::media_player::MusicPlayer;
use crate::media_player::Idle;
use crate::playlist::play_playlist;

fn main() {
    let playlist = vec!["test.mp3".to_string(), "test.mp3".to_string(), "test.mp3".to_string()];
    play_playlist(playlist);
}
use crate::media_player::create_music_player;

pub fn play_playlist(playlist: Vec<String>) {
    let mut media_player1 = create_music_player();   
    for song in playlist {
        let media_player = media_player1;
        let media_player = media_player.set_media_file_path(song);
        let media_player = media_player.prepare();
        let media_player = media_player.start();
        let media_player = media_player.awaitCompletion();
        let media_player = media_player.reset();
        media_player1 = media_player;
    }
}
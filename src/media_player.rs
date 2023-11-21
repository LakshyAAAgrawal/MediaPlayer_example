use std::fs;
use std::fs::File;
use std::io::Read;

pub trait MusicPlayerState {}
struct ActualMusicPlayerState {}

pub struct MusicPlayer<S: MusicPlayerState> {
    state: Box<ActualMusicPlayerState>,
    extra: S
}

pub struct Idle {}
pub struct Initialized {
    media_file_path: String
}
pub struct Prepared {
    media_file_path: String,
    bytes: Vec<u8>,
    current_pos: u64
}
pub struct Started {
    media_file_path: String,
    bytes: Vec<u8>,
    current_pos: u64
}
pub struct Stopped {
    media_file_path: String
}
pub struct Paused {
    media_file_path: String,
    bytes: Vec<u8>,
    current_pos: u64
}
pub struct PlaybackComplete {
    media_file_path: String,
    bytes: Vec<u8>,
    current_pos: u64
}

impl MusicPlayerState for Idle {}
impl MusicPlayerState for Initialized {}
impl MusicPlayerState for Prepared {}
impl MusicPlayerState for Started {}
impl MusicPlayerState for Stopped {}
impl MusicPlayerState for Paused {}
impl MusicPlayerState for PlaybackComplete {}

impl MusicPlayer<Idle> {
    /// Set the data source for the MusicPlayer
    pub fn set_media_file_path(self, media_file_path: String) -> MusicPlayer<Initialized> {
        println!("Set data source for MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Initialized {
                media_file_path: media_file_path
            }
        }
    }
}

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");
    buffer
}

/// Create a new MusicPlayer
pub fn create_music_player() -> MusicPlayer<Idle> {
    println!("Create new MusicPlayer");
    MusicPlayer {
        state: Box::new(ActualMusicPlayerState {}),
        extra: Idle {}
    }
}

impl MusicPlayer<Initialized> {
    /// Prepare the MusicPlayer by loading data
    pub fn prepare(self) -> MusicPlayer<Prepared> {
        println!("Prepare MusicPlayer");
        let bytes = get_file_as_byte_vec(&self.extra.media_file_path);
        MusicPlayer {
            state: self.state,
            extra: Prepared {
                media_file_path: self.extra.media_file_path,
                bytes: bytes,
                current_pos: 0
            }
        }
    }
}

impl MusicPlayer<Prepared> {
    /// Start the MusicPlayer
    pub fn start(self) -> MusicPlayer<Started> {
        println!("Start MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Started {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: 0
            }
        }
    }

    /// Seek the MusicPlayer to a position
    pub fn seekTo(self, pos: u64) -> MusicPlayer<Prepared> {
        println!("Seek MusicPlayer to {}", pos);
        MusicPlayer {
            state: self.state,
            extra: Prepared {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: pos
            }
        }
    }

    /// Stop the MusicPlayer unloading the data
    pub fn stop(self) -> MusicPlayer<Stopped> {
        println!("Stop MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Stopped {
                media_file_path: self.extra.media_file_path
            }
        }
    }
}

impl MusicPlayer<Started> {
    /// Stop the MusicPlayer unloading the data
    pub fn stop(self) -> MusicPlayer<Stopped> {
        println!("Stop MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Stopped {
                media_file_path: self.extra.media_file_path
            }
        }
    }

    /// Pause the MusicPlayer
    pub fn pause(self) -> MusicPlayer<Paused> {
        println!("Pause MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Paused {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: self.extra.current_pos
            }
        }
    }

    /// Play the MusicPlayer until completion
    pub fn awaitCompletion(self) -> MusicPlayer<Stopped> {
        println!("MusicPlayer waiting for completion");
        let bytes_len = self.extra.bytes.len() as u64;
        MusicPlayer {
            state: self.state,
            extra: Stopped {
                media_file_path: self.extra.media_file_path,
            }
        }
    }

    /// Seek the MusicPlayer to a position
    pub fn seekTo(self, pos: u64) -> MusicPlayer<Started> {
        println!("Seek MusicPlayer to {}", pos);
        MusicPlayer {
            state: self.state,
            extra: Started {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: pos
            }
        }
    }
}

impl MusicPlayer<Stopped> {
    /// Prepare the MusicPlayer by reloading data
    pub fn prepare(self) -> MusicPlayer<Prepared> {
        println!("Prepare MusicPlayer");
        let bytes = get_file_as_byte_vec(&self.extra.media_file_path);
        MusicPlayer {
            state: self.state,
            extra: Prepared {
                media_file_path: self.extra.media_file_path,
                bytes: bytes,
                current_pos: 0
            }
        }
    }
}

impl MusicPlayer<Paused> {
    /// Resume the MusicPlayer
    pub fn start(self) -> MusicPlayer<Started> {
        println!("Resume MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Started {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: self.extra.current_pos
            }
        }
    }

    /// Seek the MusicPlayer to a position
    pub fn seekTo(self, pos: u64) -> MusicPlayer<Paused> {
        println!("Seek MusicPlayer to {}", pos);
        MusicPlayer {
            state: self.state,
            extra: Paused {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: pos
            }
        }
    }

    /// Stop the MusicPlayer unloading the data
    pub fn stop(self) -> MusicPlayer<Stopped> {
        println!("Stop MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Stopped {
                media_file_path: self.extra.media_file_path
            }
        }
    }
}

impl MusicPlayer<PlaybackComplete> {
    /// Start the MusicPlayer
    pub fn start(self) -> MusicPlayer<Started> {
        println!("Start MusicPlayer from beginning");
        MusicPlayer {
            state: self.state,
            extra: Started {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: self.extra.current_pos
            }
        }
    }

    /// Stop the MusicPlayer unloading the data
    pub fn stop(self) -> MusicPlayer<Stopped> {
        println!("Stop MusicPlayer");
        MusicPlayer {
            state: self.state,
            extra: Stopped {
                media_file_path: self.extra.media_file_path
            }
        }
    }

    /// Seek the MusicPlayer to a position
    pub fn seekTo(self, pos: u64) -> MusicPlayer<PlaybackComplete> {
        println!("Seek MusicPlayer to {}", pos);
        MusicPlayer {
            state: self.state,
            extra: PlaybackComplete {
                media_file_path: self.extra.media_file_path,
                bytes: self.extra.bytes,
                current_pos: pos
            }
        }
    }
}

impl<S: MusicPlayerState> MusicPlayer<S> {
    /// Reset the MusicPlayer unloading the data
    pub fn reset(self) -> MusicPlayer<Idle> {
        println!("Reset MusicPlayer");
        MusicPlayer {
            state: Box::new(ActualMusicPlayerState {}),
            extra: Idle {}
        }
    }
}
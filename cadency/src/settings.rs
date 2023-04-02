const CADENCY_PLAYLIST_SONG_LIMIT_ENV: &str = "CADENCY_PLAYLIST_SONG_LIMIT";
const CADENCY_SONG_LENGTH_LIMIT_ENV: &str = "CADENCY_SONG_LENGTH_LIMIT";

pub struct PlaySettings {
    pub playlist_song_limit: i32,
    pub song_length_limit: f32,
}

impl PlaySettings {
    pub fn parse() -> Self {
        let playlist_song_limit: i32 = std::env::var(CADENCY_PLAYLIST_SONG_LIMIT_ENV)
            .ok()
            .map(|value| {
                value
                    .parse::<i32>()
                    .expect("Unable to parse '{CADENCY_PLAYLIST_SONG_LIMIT_ENV}' to an integer")
            })
            .unwrap_or(30);
        let song_length_limit = std::env::var(CADENCY_SONG_LENGTH_LIMIT_ENV)
            .ok()
            .map(|value| {
                value
                    .parse::<f32>()
                    .expect("Unable to parse '{CADENCY_SONG_LENGTH_LIMIT_ENV}' to a float")
            })
            .unwrap_or(600.00);
        Self {
            playlist_song_limit,
            song_length_limit,
        }
    }
}

pub struct CadencySettings {
    pub play: PlaySettings,
}

impl CadencySettings {
    pub fn parse() -> Self {
        let play = PlaySettings::parse();
        Self { play }
    }
}

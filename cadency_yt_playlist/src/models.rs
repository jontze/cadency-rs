use serde::de;

#[derive(Serialize, Deserialize)]
pub struct Message {
    pub content: String,
    pub data: String,
}

#[derive(Serialize)]
pub struct YtDlpResponse<T: de::DeserializeOwned> {
    pub data: T,
    pub messages: Vec<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlaylistItem {
    pub id: String,
    pub playlist_index: usize,
    pub duration: f32,
    pub title: String,
    pub url: String,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_playlist_entry() {
        let file = std::fs::read_to_string("./test/data/playlist_entry.json").unwrap();
        let _ = serde_json::from_str::<PlaylistItem>(&file).unwrap();
    }
}

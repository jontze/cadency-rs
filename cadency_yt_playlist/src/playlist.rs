use crate::{
    error::YtPlaylistError,
    models::{Message, PlaylistItem, YtDlpResponse},
    ytdlp::YtDlp,
};

pub fn fetch_playlist_songs(
    playlist_url: String,
) -> Result<YtDlpResponse<Vec<PlaylistItem>>, YtPlaylistError> {
    let ytdl_output = YtDlp::new()
        .arg("--flat-playlist".to_string())
        .arg("-j".to_string())
        .arg(playlist_url)
        .execute()?;

    let mut failed_deserializing: Vec<Message> = Vec::new();
    let deserialized_items: Vec<PlaylistItem> = String::from_utf8(ytdl_output.stdout)?
        .split('\n')
        .filter_map(|string_output| {
            if string_output.is_empty() {
                None
            } else {
                Some(
                    serde_json::from_slice::<PlaylistItem>(string_output.as_bytes()).map_err(
                        |err| YtPlaylistError::Deserializing {
                            message: err.to_string(),
                            data: string_output.to_string(),
                            source: err,
                        },
                    ),
                )
            }
        })
        .filter_map(|parsed_item| match parsed_item {
            Ok(valid_item) => Some(valid_item),
            Err(des_err) => {
                if let YtPlaylistError::Deserializing {
                    source: _,
                    message,
                    data,
                } = des_err
                {
                    failed_deserializing.push(Message {
                        content: message,
                        data,
                    });
                };
                None
            }
        })
        .collect();

    Ok(YtDlpResponse {
        data: deserialized_items,
        messages: failed_deserializing,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_fetch_playlist_songs() {
        let response = fetch_playlist_songs(
            "https://www.youtube.com/watch?v=BJ8XPi-cPkM&list=PLDfKAXSi6kUZbsoz3AcUYjy8n6hbulk4o"
                .to_string(),
        )
        .unwrap();
        assert_eq!(response.messages.len(), 0, "Expect no deserializing errors")
    }
}

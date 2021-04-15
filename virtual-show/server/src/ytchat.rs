use std::thread;
use std::sync::{Arc,Mutex};
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::HashMap;
use std::sync::mpsc::{Sender};

use Config;

#[derive(Deserialize)]
struct LiveChatSnippet {
    displayMessage: String
}

#[derive(Deserialize)]
struct LiveChatAuthorDetails {
    displayName: String
}

#[derive(Deserialize)]
struct LiveChatMessage {
    snippet: LiveChatSnippet,
    authorDetails: LiveChatAuthorDetails
}

#[derive(Deserialize)]
struct LiveChatMessages
{
    items: Vec<LiveChatMessage>,
    nextPageToken: String,
    pollingIntervalMillis: u64
}

#[derive(Serialize)]
pub struct YTChatMessage {
    author: String,
    message: String
}

pub fn start(
    config: Arc<Config>,
    ytchat_tx:Sender<Vec<YTChatMessage>>
) {
    println!("\nSpawning YouTube chat list from ID {}", config.ytchat.chat_id);
    let mut pageToken:String = "".to_string();

    loop {

        let response:String  = reqwest::blocking::get(
            format!(
                "https://www.googleapis.com/youtube/v3/liveChat/messages?part=id%2C%20snippet%2C%20authorDetails&key={}&liveChatId={}&pageToken={}",
                config.ytchat.api_key,
                config.ytchat.chat_id,
                &pageToken
            )
        ).unwrap()
        .text().unwrap();

        let messages:LiveChatMessages = serde_json::from_str(&response).unwrap();
        let mut ytmsgs = Vec::new();

        for item in &messages.items {
            let ytmsg = YTChatMessage {
                author: item.authorDetails.displayName.clone(),
                message: item.snippet.displayMessage.clone()
            };
            ytmsgs.push(ytmsg);
        }

        if ytmsgs.len() > 0 {
            ytchat_tx.send(ytmsgs);
        }

        pageToken = messages.nextPageToken;

        thread::sleep(Duration::from_millis(messages.pollingIntervalMillis));
    }
}

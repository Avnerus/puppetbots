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
    text: String
}

pub fn start(
    config: Arc<Config>,
    ytchat_tx:Sender<Vec<YTChatMessage>>
) {
    let mut pageToken:String = "".to_string();

    if (config.ytchat.enabled) {
        println!("\nSpawning YouTube chat list from ID {}", config.ytchat.chat_id);

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

            match serde_json::from_str::<LiveChatMessages>(&response) {
                Ok(messages) => {
                    let mut ytmsgs = Vec::new();

                    for item in &messages.items {
                        let ytmsg = YTChatMessage {
                            author: item.authorDetails.displayName.clone(),
                            text: item.snippet.displayMessage.clone()
                        };
                        ytmsgs.push(ytmsg);
                    }

                    if ytmsgs.len() > 0 {
                        ytchat_tx.send(ytmsgs);
                    }

                    pageToken = messages.nextPageToken;

                    thread::sleep(Duration::from_millis(messages.pollingIntervalMillis));
                },
                Err(e) => {
                    println!("Error parsing response from YouTube: {:?}", e);
                    println!("{}",response);
                    thread::sleep(Duration::from_millis(5000));
                }
            }
        }
    }
}

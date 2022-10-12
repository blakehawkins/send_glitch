use std::env::args;
use std::io::Result;

use matrix_sdk::{
    config::SyncSettings,
    ruma::{events::room::message::RoomMessageEventContent, UserId},
    Client,
};
use oops::Oops;
use serde::{Deserialize, Serialize};
use stdinix::astdinix;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    token: Option<String>,
    password: String,
    room: String,
    account: String,
    html_json_key: String,
    text_json_key: Option<String>,
    server: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Config = serde_yaml::from_reader(
        std::fs::File::open(args().nth(1).unwrap_or_else(|| "config.yaml".into()))
            .oops("Failed to open config file. Usage: `send_glitch [config.yaml]`")?,
    )
    .expect("Config file was not deserialisable.");

    let account_name = args.clone().account;
    let user = UserId::parse(account_name).oops("invalid userid")?;
    let client = Client::builder()
        .server_name(user.server_name())
        .build()
        .await
        .oops("failed to build client")?;
    let password = args.clone().password;
    client
        .login_username(&user, &password)
        .send()
        .await
        .oops("Failed to log in to homeserver")?;

    client.sync_once(SyncSettings::default()).await.unwrap();
    let room = args.clone().room;
    let room = client
        .joined_rooms()
        .into_iter()
        .filter_map(|rm| {
            if rm.canonical_alias().filter(|a| a.alias() == room).is_some() {
                Some(rm)
            } else {
                None
            }
        })
        .next()
        .oops("No matching room was found")?;

    astdinix(move |jsonline| {
        let args = args.clone();
        let room = room.clone();

        async move {
            let res: serde_json::Value = serde_json::from_str(jsonline.trim())?;

            let html: String = res[args.clone().html_json_key]
                .as_str()
                .to_owned()
                .oops("Missing html key")?
                .to_owned();
            let text: String = args
                .clone()
                .text_json_key
                .and_then(|v| res[v].as_str().map(|v| v.to_owned()))
                .unwrap_or_else(|| "".into());

            room.send(RoomMessageEventContent::text_html(text, html), None)
                .await
                .oops("Failed to send a message")?;

            Ok(())
        }
    })
    .await
}

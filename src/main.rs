use std::env::args;
use std::io::{Error, ErrorKind, Result, Write};

use futures::future::Future;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::room::{NewRoom, RoomClient};
use glitch_in_the_matrix::MatrixClient;
use oops::Oops;
use serde::{Deserialize, Serialize};
use stdinix::stdinix;
use tokio_core::reactor::Core;
use urlencoding::encode;

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

fn main() -> Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let handle2 = core.handle();

    let args: Config = serde_yaml::from_reader(
        std::fs::File::open(args().nth(1).unwrap_or_else(|| "config.yaml".into()))
            .oops("Failed to open config file. Usage: `send_glitch [config.yaml]`")?,
    )
    .expect("Config file was not deserialisable.");

    stdinix(|jsonline| {
        let res: serde_json::Value = serde_json::from_str(&jsonline.trim())?;

        let html: String = res[args.clone().html_json_key]
            .as_str()
            .to_owned()
            .oops("Missing html key")?
            .to_owned();
        let text: Option<String> = args
            .clone()
            .text_json_key
            .and_then(|v| res[v].as_str().map(|v| v.to_owned()));

        let args2 = args.clone();

        let args = args.clone();
        let handle2 = handle2.clone();
        let server = args
            .clone()
            .server
            .unwrap_or_else(|| "https://matrix.org".into());
        let server2 = server.clone();
        let token_fut = match args.clone().token {
            Some(v) => futures::future::ok(v),
            _ => futures::future::err(()),
        };
        let msg_fut = token_fut
            .and_then(|token| {
                let server = server.clone();
                MatrixClient::new_from_access_token(&token, &server, &handle).map_err(|_| ())
            })
            .or_else(move |mut _e| {
                let args = args.clone();
                let server2 = server2.clone();
                println!("Connecting {} to {}", args.account, server2);
                std::io::stdout().flush().expect("Failed to flush");
                MatrixClient::login_password(&args.account, &args.password, &server2, &handle2)
            })
            .and_then(move |mut client| {
                println!("Access token: {}", client.get_access_token());
                std::io::stdout()
                    .flush()
                    .expect("Failed to flush access token");

                NewRoom::from_alias(&mut client, &encode(&args2.room))
                    .map(move |room| (client, room))
            })
            .and_then(move |(mut cli, room)| {
                RoomClient {
                    room: &room,
                    cli: &mut cli,
                }
                .send_html(html, text)
            })
            .map(|_| ())
            .map_err(|e| Error::new(ErrorKind::Other, &format!("Failed to send - {:?}", e)[..]));

        core.run(msg_fut)
    })
}

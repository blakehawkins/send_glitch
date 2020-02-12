use std::env::args;
use std::io::{Read, Result};

use futures::future::Future;
use glitch_in_the_matrix::request::MatrixRequestable;
use glitch_in_the_matrix::room::{NewRoom, RoomClient};
use glitch_in_the_matrix::MatrixClient;
use serde::{Deserialize, Serialize};
use serde_yaml;
use tokio_core::reactor::Core;
use urlencoding::encode;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    token: String,
    password: String,
    room: String,
    account: String,
    html_json_key: String,
    text_json_key: Option<String>,
}

trait Oops<T> {
    fn oops(self, msg: &str) -> Result<T>;
}

impl<T> Oops<T> for Option<T> {
    fn oops(self, msg: &str) -> Result<T> {
        self.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, msg))
    }
}

fn main() -> Result<()> {
    let mut core = Core::new()?;
    let handle = core.handle();
    let handle2 = core.handle();

    let args: Config = serde_yaml::from_reader(std::fs::File::open(
        args().nth(1).unwrap_or_else(|| "config.yaml".into()),
    )?)
    .expect("Config file was not deserialisable.");

    let mut buf = String::new();
    std::io::stdin().lock().read_to_string(&mut buf)?;

    let res: serde_json::Value = serde_json::from_str(&buf)?;

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

    let msg_fut = MatrixClient::new_from_access_token(&args.token, "https://matrix.org", &handle)
        .or_else(move |mut _e| {
            MatrixClient::login_password(
                &args.account,
                &args.password,
                "https://matrix.org",
                &handle2,
            )
        })
        .and_then(move |mut client| {
            println!("Access token: {}", client.get_access_token());

            NewRoom::from_alias(&mut client, &encode(&args2.room)).map(move |room| (client, room))
        })
        .and_then(move |(mut cli, room)| {
            RoomClient {
                room: &room,
                cli: &mut cli,
            }
            .send_html(html, text)
        });

    core.run(msg_fut).expect("Unresolved errors encountered.");

    Ok(())
}

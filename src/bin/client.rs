use futures_util::stream::StreamExt;
use futures_util::SinkExt;
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let stdin = tokio::io::stdin();
    let mut stdin = BufReader::new(stdin).lines();

    loop {
        tokio::select! {
            incoming_msg = ws_stream.next() => {
                match incoming_msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        let Some(text) = msg.as_text() else { return Ok(()); };
                        println!("Sabina's computer: {}", text);
                    }
                    Some(Ok(msg)) if msg.is_close() => {
                        println!("Sabina's computer: server closed the connection");
                        return Ok(());
                    }
                    Some(Ok(_)) => {},
                    Some(Err(err)) => return Err(err),
                    None => return Ok(()),
                }
            }

            line = stdin.next_line() => {
                match line {
                    Ok(Some(line)) => ws_stream.send(Message::text(line.to_string())).await?,
                    Ok(None) => return Ok(()),
                    Err(err) => return Err(err.into()),
                }
            }
        }
    }
}
use futures_util::sink::SinkExt;
use futures_util::stream::StreamExt;
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();

    loop {
        tokio::select! {
            incoming_msg = ws_stream.next() => {
                match incoming_msg {
                    Some(Ok(msg)) if msg.is_text() => {
                        let Some(text) = msg.as_text() else { return Ok(()) };
                        println!("[{}] says {}", addr, text);
                        bcast_tx.send(format!("[{}] says {}", addr, text))?;
                    }
                    Some(Ok(msg)) if msg.is_close() => {
                        println!("[{}] disconnected", addr);
                        return Ok(());
                    }
                    Some(Ok(_)) => {},
                    Some(Err(err)) => return Err(err.into()),
                    None => return Ok(()),
                }
            }

            msg = bcast_rx.recv() => {
                ws_stream.send(Message::text(msg?)).await?;
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);

    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("listening on port 8080");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{}] connected", addr);
        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            // Wrap the raw TCP stream into a websocket.
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await?;

            handle_connection(addr, ws_stream, bcast_tx).await
        });
    }
}
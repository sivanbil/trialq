use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Utf8Bytes;

#[derive(Serialize, Deserialize)]
struct WebSocketMessage {
    message: String,
}

// WebSocket 处理函数
async fn handle_websocket_connection(
    stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
) {
    let (mut ws_sender, mut ws_receiver) = stream.split();

    while let Some(Ok(msg)) = ws_receiver.next().await {
        if let tokio_tungstenite::tungstenite::Message::Text(text) = msg {
            println!("Received message: {}", text);

            // 解析 JSON 消息
            if let Ok(parsed_msg) = serde_json::from_str::<WebSocketMessage>(&text) {
                println!("Parsed message: {}", parsed_msg.message);

                // 发送响应
                let response = WebSocketMessage {
                    message: format!("Echo: {}", parsed_msg.message),
                };
                let response_json = serde_json::to_string(&response).unwrap();
                ws_sender
                    .send(tokio_tungstenite::tungstenite::Message::Text(
                        Utf8Bytes::from(response_json),
                    ))
                    .await
                    .unwrap();
            }
        }
    }
}

// 启动 WebSocket 服务器
pub async fn start_websocket_server() {
    let addr = "127.0.0.1:8080";
    let listener = TcpListener::bind(addr).await.expect("Failed to bind");

    println!("WebSocket server running on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(handle_websocket_connection(
            accept_async(stream).await.unwrap(),
        ));
    }
}

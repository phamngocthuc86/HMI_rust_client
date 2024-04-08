use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use serde_json::json;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:50123").await?;

    // Prepare a JSON message to send
    let msg = json!({
        "msg_name": "download_msg",
        "module": "hmi",
        "percentage": 20,
        "result": -2,
    });

    // let msg = json!({
    //     "msg_name": "installing_msg",
    //     "module": "hmi",
    //     "percentage": 20,
    //     "result": 0,
    // });

    // let msg = json!({
    //     "msg_name": "cancel_msg",
    //     "module": "hmi",
    //     "percentage": 20,
    //     "result": -2,
    // });

    
    // let msg = json!({
    //     "msg_name": "installing_done_msg",
    //     "module": "hmi",
    //     "result": 0,
    // });


    let msg = serde_json::to_string(&msg).unwrap();

    // Send the JSON message
    stream.write_all(msg.as_bytes()).await?;

    loop {
        // Wait for and read the response
        let mut buffer = [0; 1024];
        let n = stream.read(&mut buffer).await?;
        // Assume the response is also a JSON message
        if let Ok(str) = std::str::from_utf8(&buffer[..n]) {
            println!("Received: {}", str);
            stream.write_all(msg.as_bytes()).await?;
        }
    }

    Ok(())
}

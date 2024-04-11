use std::env;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::fs;

#[tokio::main]
async fn main() -> io::Result<()> {
    // Parse command line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 || args[1] != "-f" {
        println!("Usage: application -f <file_json_msg>");
        return Ok(());
    }

    // Extract the file path
    let file_path = &args[2];

    // Read the contents of the file
    let file_content = fs::read_to_string(file_path)
        .expect("Failed to read the file");

    // Assuming the file content is directly the JSON message you want to send
    // If the JSON needs to be nested or wrapped, you may need to adjust this

    println!("Send JSON : File content: {}", file_content);
    let json_msg: serde_json::Value = serde_json::from_str(&file_content)
        .expect("File content is not valid JSON");

    // Connect to the server
    println!("Connecting to server at:  127.0.0.1:50123  - you can very by command: netstat -lnpt | grep 50123 ");
    let mut stream = TcpStream::connect("127.0.0.1:50123").await?;

    // Convert the JSON message to a string and send it
    let msg_str = serde_json::to_string(&json_msg).unwrap();
    stream.write_all(msg_str.as_bytes()).await?;
    Ok(())
}

use std::env;
use std::net::UdpSocket;
use std::process::exit;

fn main() -> std::io::Result<()> {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least 2 arguments are provided
    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  --ping x.x.x.x:9999 (to send 'ping')");
        eprintln!("  --pong 9999         (to receive 'ping' and respond with 'pong')");
        exit(1);
    }

    match args[1].as_str() {
        "--ping" => {
            // Ping mode: send a 'ping' message to the specified address
            let address = &args[2];
            send_ping(address)
        }
        "--pong" => {
            // Pong mode: listen on the specified port and respond with 'pong'
            let port = &args[2];
            receive_pong(port)
        }
        _ => {
            eprintln!("Invalid mode specified. Use --ping or --pong.");
            exit(1);
        }
    }
}

// Function for the client to send 'ping'
fn send_ping(server_address: &str) -> std::io::Result<()> {
    // Create a socket bound to a local ephemeral port
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    println!("Sending 'ping' to {}", server_address);

    // Send 'ping' message to the server
    socket.send_to(b"ping", server_address)?;

    // Buffer to store the response
    let mut buf = [0; 1024];

    // Receive the response from the server
    let (amt, src) = socket.recv_from(&mut buf)?;
    let received = String::from_utf8_lossy(&buf[..amt]);

    println!("Received '{}' from {}", received, src);

    Ok(())
}

// Function for the server to receive 'ping' and respond with 'pong'
fn receive_pong(port: &str) -> std::io::Result<()> {
    // Bind the server to the specified port
    let address = format!("0.0.0.0:{}", port);
    let socket = UdpSocket::bind(&address)?;
    println!("Server is running on {}", address);

    let mut buf = [0; 1024];

    loop {
        // Receive a message from the client
        let (amt, src) = socket.recv_from(&mut buf)?;
        let received = String::from_utf8_lossy(&buf[..amt]);

        println!("Received '{}' from {}", received, src);

        // Respond with 'pong' if the message is 'ping'
        if received.trim() == "ping" {
            let response = "pong";
            socket.send_to(response.as_bytes(), &src)?;
            println!("Sent '{}' to {}", response, src);
        }
    }
}

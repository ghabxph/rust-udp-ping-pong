use std::env;
use std::net::UdpSocket;
use std::process::exit;
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Read the command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure at least 2 arguments are provided
    if args.len() < 3 {
        eprintln!("Usage:");
        eprintln!("  --ping x.x.x.x:9999 [local_port] (to send 'ping' and keep listening)");
        eprintln!("  --pong 9999                     (to receive 'ping' and respond with 'pong')");
        eprintln!("  --dong x.x.x.x:9999 [local_port] (to send 'dong' every second)");
        exit(1);
    }

    match args[1].as_str() {
        "--ping" => {
            // Ping mode: send a 'ping' message to the specified address and keep listening for 'pong'
            let address = &args[2];
            let local_port = args.get(3).map(|s| s.as_str());
            send_ping(address, local_port)
        }
        "--pong" => {
            // Pong mode: listen on the specified port and respond with multiple 'pong' messages
            let port = &args[2];
            receive_pong(port)
        }
        "--dong" => {
            // Dong mode: send 'dong' messages every second to the specified address
            let address = &args[2];
            let local_port = args.get(3).map(|s| s.as_str());
            send_dong(address, local_port)
        }
        _ => {
            eprintln!("Invalid mode specified. Use --ping, --pong, or --dong.");
            exit(1);
        }
    }
}

// Function for the client to send 'ping' and remain open to listen for 'pong'
fn send_ping(server_address: &str, local_port: Option<&str>) -> std::io::Result<()> {
    // Create a socket bound to the specified local port or use an ephemeral port
    let local_address = match local_port {
        Some(port) => format!("0.0.0.0:{}", port),
        None => "0.0.0.0:0".to_string(), // Use ephemeral port if not specified
    };
    let socket = UdpSocket::bind(&local_address)?;
    println!("Sending 'ping' to {} from local address {}", server_address, socket.local_addr()?);

    // Send 'ping' message to the server
    socket.send_to(b"ping", server_address)?;

    println!("Listening for 'pong' responses...");

    // Buffer to store the responses
    let mut buf = [0; 1024];

    // Continuously listen for 'pong' responses
    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let received = String::from_utf8_lossy(&buf[..amt]);

        println!("Received '{}' from {}", received, src);

        // Wait for a short period before listening again (optional)
        thread::sleep(Duration::from_millis(200)); // Adjust the sleep duration if necessary
    }
}

// Function for the server to receive 'ping' and respond with multiple 'pong' messages
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
            println!("Sending 'pong' messages to {} every second for 30 minutes...", src);

            // Send multiple "pong" messages every second for 30 minutes
            for _ in 0..(30 * 60) {
                let response = "pong";
                socket.send_to(response.as_bytes(), &src)?;
                println!("Sent '{}' to {}", response, src);
                thread::sleep(Duration::from_secs(1)); // Wait for 1 second before sending the next "pong"
            }
            println!("Finished sending 'pong' messages to {}", src);
        }
    }
}

// Function to continuously send 'dong' messages every second
fn send_dong(server_address: &str, local_port: Option<&str>) -> std::io::Result<()> {
    // Create a socket bound to the specified local port or use an ephemeral port
    let local_address = match local_port {
        Some(port) => format!("0.0.0.0:{}", port),
        None => "0.0.0.0:0".to_string(), // Use ephemeral port if not specified
    };
    let socket = UdpSocket::bind(&local_address)?;
    println!("Sending 'dong' messages to {} from local address {}", server_address, socket.local_addr()?);

    // Continuously send 'dong' messages
    loop {
        let message = "dong";
        socket.send_to(message.as_bytes(), server_address)?;
        println!("Sent '{}' to {}", message, server_address);

        // Wait for 1 second before sending the next 'dong'
        thread::sleep(Duration::from_secs(1));
    }
}

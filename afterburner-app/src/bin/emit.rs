use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("Failed to bind");
    let target = "127.0.0.1:8003";

    println!("Sending DYNAMIC Solana packets to {}...", target);

    let mut packet = Vec::new();

    // 1. Signatures
    packet.push(1);
    packet.extend_from_slice(&[0xAA; 64]);

    // 2. Header
    packet.push(1);
    packet.push(0);
    packet.push(1);

    // 3. Accounts
    packet.push(2);
    packet.extend_from_slice(&[0xBB; 32]);
    packet.extend_from_slice(&[0xCC; 32]);

    // 4. Blockhash
    packet.extend_from_slice(&[0xDD; 32]);

    // 5. Instructions
    packet.push(1);
    packet.push(0);
    packet.push(0);

    // Data Length: 4 bytes
    packet.push(4);
    // Base Payload: CA FE BA BE
    packet.extend_from_slice(&[0xCA, 0xFE, 0xBA, 0xBE]);

    let mut counter: u8 = 0;

    loop {
        // Modify the last byte of the packet (The "BE")
        if let Some(last) = packet.last_mut() {
            *last = counter;
        }

        socket.send_to(&packet, target).unwrap();
        println!("Sent Packet ending in {:02X}", counter);

        counter = counter.wrapping_add(1);
        thread::sleep(Duration::from_secs(1));
    }
}

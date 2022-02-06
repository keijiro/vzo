use std::{str, env};
use std::net::{SocketAddr, ToSocketAddrs, UdpSocket};
use rosc::{OscMessage, OscPacket, OscType};

fn get_destination_addr() -> SocketAddr {
    let addr_str = env::args().nth(1).unwrap_or("127.0.0.1:9000".to_string());
    addr_str.to_socket_addrs().unwrap().next().unwrap()
}

fn main() {
    let recv_sock = zmq::Context::new().socket(zmq::SUB).unwrap();
    recv_sock.set_subscribe(b"").unwrap();
    recv_sock.bind("tcp://*:53311").unwrap();

    let send_sock = UdpSocket::bind("0.0.0.0:0").unwrap();
    let send_addr = get_destination_addr();

    println!("Started routing messages to {}", send_addr);
    println!("Waiting for a connection.");

    let term = console::Term::stdout();

    loop {
        let recv_msg = recv_sock.recv_multipart(0).unwrap();
        let addr = str::from_utf8(&recv_msg[0]).unwrap().to_string();
        let value = str::from_utf8(&recv_msg[1]).unwrap().parse::<f32>().unwrap();

        term.clear_last_lines(1).unwrap();
        term.write_line(&format!("{} {}", addr, value)).unwrap();

        let osc_msg = OscMessage { addr: addr, args: vec![OscType::Float(value)] };
        let osc_pak = OscPacket::Message(osc_msg);
        let send_msg = rosc::encoder::encode(&osc_pak).unwrap();

        send_sock.send_to(&send_msg, send_addr).unwrap();
    }
}

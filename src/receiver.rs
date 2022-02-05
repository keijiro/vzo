use rosc::{OscMessage, OscPacket, OscType};
use std::net::UdpSocket;
use std::str;

fn main() {
    let recv_sock = zmq::Context::new().socket(zmq::SUB).unwrap();
    recv_sock.set_subscribe(b"").unwrap();
    recv_sock.bind("tcp://*:9001").unwrap();

    let send_sock = UdpSocket::bind("0.0.0.0:0").unwrap();

    loop {
        let recv_msg = recv_sock.recv_multipart(0).unwrap();
        let addr = str::from_utf8(&recv_msg[0]).unwrap().to_string();
        let value = str::from_utf8(&recv_msg[1]).unwrap().parse::<f32>().unwrap();

        println!("{} {}", addr, value);

        let osc_msg = OscMessage { addr: addr, args: vec![OscType::Float(value)] };
        let osc_pak = OscPacket::Message(osc_msg);
        let send_msg = rosc::encoder::encode(&osc_pak).unwrap();

        send_sock.send_to(&send_msg, "127.0.0.1:9000").unwrap();
    }
}

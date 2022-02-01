fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.set_subscribe(b"").unwrap();
    socket.bind("tcp://*:5556").unwrap();
    loop {
        let data = socket.recv_bytes(0).unwrap();
        println!("{:x} {:x} {:x} {:x}", data[0], data[1], data[2], data[3]);
    }
}

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUB).unwrap();
    socket.connect("tcp://localhost:5556").unwrap();
    let data: [u8; 4] = [0xde, 0xad, 0xbe, 0xef];
    loop {
        socket.send(&data as &[u8], 0).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

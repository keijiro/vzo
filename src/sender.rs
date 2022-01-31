fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUB).unwrap();
    socket.connect("tcp://localhost:5556").unwrap();
    let text = format!("Source: [{}]", std::process::id()).to_string();
    loop {
        socket.send(&text, 0).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

mod custom;

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::PUB).unwrap();
    socket.connect("tcp://localhost:5556").unwrap();
    let data = custom::CustomData { channel: 1, data1: 2, data2: 3 };
    loop {
        socket.send(&data, 0).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.set_subscribe(b"trigger").unwrap();
    socket.bind("tcp://*:5556").unwrap();
    loop {
        let data = socket.recv_string(0).unwrap().unwrap();
        println!("{}", data);
    }
}

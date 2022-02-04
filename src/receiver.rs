mod custom;

fn main() {
    let ctx = zmq::Context::new();
    let socket = ctx.socket(zmq::SUB).unwrap();
    socket.set_subscribe(b"").unwrap();
    socket.bind("tcp://*:5556").unwrap();
    loop {
        let msg = socket.recv_msg(0).unwrap();
        let data = custom::CustomData::from(&msg);
        println!("{:x} {:x} {:x} {:x}", data.channel, data.event, data.data1, data.data2);
    }
}

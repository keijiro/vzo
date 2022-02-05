use rand::Rng;

fn main() {
    let sock = zmq::Context::new().socket(zmq::PUB).unwrap();
    sock.connect("tcp://localhost:9001").unwrap();

    let mut rng = rand::thread_rng();

    loop {
        let addr = format!("/note/{}/{}", rng.gen::<u8>(), rng.gen::<u8>());
        let value = format!("{}", rng.gen::<f32>());

        sock.send_multipart(vec![&addr, &value], 0).unwrap();

        std::thread::sleep(std::time::Duration::from_millis(200));
    }
}

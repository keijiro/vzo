#[derive(Clone)]
pub struct CustomData {
    pub channel: u8,
    pub data1: u8,
    pub data2: u8
}

impl From<&zmq::Message> for CustomData {
    fn from(msg: &zmq::Message) -> Self {
        CustomData { channel:msg[0], data1:msg[1], data2:msg[2] }
    }
}

impl From<&CustomData> for zmq::Message {
    fn from(data: &CustomData) -> Self {
        let array = [data.channel, data.data1, data.data2];
        zmq::Message::from(&array as &[u8])
    }
}
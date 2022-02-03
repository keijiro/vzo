mod custom;

#[macro_use]
extern crate vst;

use vst::api;
use vst::event;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin};

struct OSCBridge {
    socket: zmq::Socket
}

plugin_main!(OSCBridge);

impl Plugin for OSCBridge {
    fn new(_host: HostCallback) -> Self {
        OSCBridge {
            socket: {
                let socket = zmq::Context::new().socket(zmq::PUB).unwrap();
                socket.connect("tcp://localhost:5556").unwrap();
                socket
            }
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "OSCBridge".to_string(),
            vendor: "Keijiro".to_string(),
            unique_id: 362785,
            category: Category::Synth,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: &api::Events) {
        for event in events.events() {
            match event {
                event::Event::Midi(ev) => {
                    let data = custom::CustomData
                      { channel: ev.data[0], data1: ev.data[1], data2: ev.data[2] };
                    self.socket.send(&data, 0).unwrap();
                },
                _ => ()
            }
        }
    }

    fn can_do(&self, can_do: CanDo) -> api::Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => api::Supported::Yes,
            _ => api::Supported::Maybe,
        }
    }
}

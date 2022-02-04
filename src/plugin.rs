mod custom;

#[macro_use]
extern crate vst;

use vst::api;
use vst::event;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct OSCBridge {
    params: Arc<OSCBridgeParameters>,
    socket: zmq::Socket,
}

struct OSCBridgeParameters {
    channel: AtomicFloat,
}

impl Default for OSCBridgeParameters {
    fn default() -> OSCBridgeParameters {
        OSCBridgeParameters {
            channel: AtomicFloat::new(0.0),
        }
    }
}

plugin_main!(OSCBridge);

impl Plugin for OSCBridge {
    fn new(_host: HostCallback) -> Self {
        OSCBridge {
            params: Arc::new(OSCBridgeParameters::default()),
            socket: {
                let socket = zmq::Context::new().socket(zmq::PUB).unwrap();
                socket.connect("tcp://localhost:5556").unwrap();
                socket
            },
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "OSCBridge".to_string(),
            vendor: "Keijiro".to_string(),
            unique_id: 362785,
            category: Category::Synth,
            parameters: 1,
            ..Info::default()
        }
    }

    fn process_events(&mut self, events: &api::Events) {
        for event in events.events() {
            match event {
                event::Event::Midi(ev) => {
                    let data = custom::CustomData
                      { channel:(self.params.channel.get() * 127.0) as u8, event: ev.data[0], data1: ev.data[1], data2: ev.data[2] };
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

    fn get_parameter_object(&mut self) -> Arc<dyn PluginParameters> {
        Arc::clone(&self.params) as Arc<dyn PluginParameters>
    }
}

impl PluginParameters for OSCBridgeParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.channel.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, val: f32) {
        #[allow(clippy::single_match)]
        match index {
            0 => self.channel.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", (self.channel.get() * 127.0) as i32),
            _ => "".to_string(),
        }
    }

    fn get_parameter_name(&self, index: i32) -> String {
        match index {
            0 => "Channel",
            _ => "",
        }
        .to_string()
    }
}

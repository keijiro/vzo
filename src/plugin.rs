#[macro_use]
extern crate vst;

use vst::{api, event};
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin, PluginParameters};
use vst::util::AtomicFloat;

use std::sync::Arc;

struct VzoPlugin {
    params: Arc<VzoParameters>,
    socket: zmq::Socket,
}

impl VzoPlugin {
    fn process_midi_event(&mut self, data: [u8; 3]) {
        let channel = self.params.get_channel_number();

        let (kind, level) = match data[0] & 0xf0 {
            0x80 => ("note", 0),
            0x90 => ("note", data[2]),
            0xb0 => ("cc", data[2]),
            _ => { return },
        };

        let addr = format!("/{}/{}/{}", kind, channel, data[1]);
        let value = format!("{}", (level as f32) / 127.0);

        self.socket.send_multipart(vec![&addr, &value], 0).unwrap();
    }
}

struct VzoParameters {
    channel: AtomicFloat,
}

impl VzoParameters {
    fn get_channel_number(&self) -> i32 {
        (self.channel.get() * 255.0) as i32
    }
}

impl Default for VzoParameters {
    fn default() -> VzoParameters {
        VzoParameters {
            channel: AtomicFloat::new(0.0),
        }
    }
}

impl Plugin for VzoPlugin {
    fn new(_host: HostCallback) -> Self {
        VzoPlugin {
            params: Arc::new(VzoParameters::default()),
            socket: {
                let sock = zmq::Context::new().socket(zmq::PUB).unwrap();
                sock.connect("tcp://127.0.0.1:53311").unwrap();
                sock
            },
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "vzo".to_string(),
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
                event::Event::Midi(ev) => self.process_midi_event(ev.data),
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

impl PluginParameters for VzoParameters {
    fn get_parameter(&self, index: i32) -> f32 {
        match index {
            0 => self.channel.get(),
            _ => 0.0,
        }
    }

    fn set_parameter(&self, index: i32, val: f32) {
        match index {
            0 => self.channel.set(val),
            _ => (),
        }
    }

    fn get_parameter_text(&self, index: i32) -> String {
        match index {
            0 => format!("{}", self.get_channel_number()),
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

plugin_main!(VzoPlugin);

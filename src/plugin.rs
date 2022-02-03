#[macro_use]
extern crate vst;

use vst::api::{Events, Supported};
use vst::buffer::AudioBuffer;
use vst::event::Event;
use vst::plugin::{CanDo, Category, HostCallback, Info, Plugin};

mod custom;

struct OSCBridge {
    data: i32
}

impl Plugin for OSCBridge {
    fn new(_host: HostCallback) -> Self {
        OSCBridge {
            data: 0
        }
    }

    fn get_info(&self) -> Info {
        Info {
            name: "OSCBridge".to_string(),
            vendor: "Keijiro".to_string(),
            unique_id: 2783,
            category: Category::Synth,
            inputs: 2,
            outputs: 2,
            parameters: 0,
            initial_delay: 0,
            ..Info::default()
        }
    }

    #[allow(unused_variables)]
    #[allow(clippy::single_match)]
    fn process_events(&mut self, _events: &Events) {
    }

    fn process(&mut self, buffer: &mut AudioBuffer<f32>) {
    }

    fn can_do(&self, can_do: CanDo) -> Supported {
        match can_do {
            CanDo::ReceiveMidiEvent => Supported::Yes,
            _ => Supported::Maybe,
        }
    }
}

plugin_main!(OSCBridge);

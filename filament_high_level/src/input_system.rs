use crate::input_handler::InputHandler;
use legion::prelude::*;
use shrev::EventChannel;
use winit::{Event, Window};

pub fn build(world: &mut World) -> Box<dyn Schedulable> {
    world.resources.insert(InputHandler::new());

    let mut window_event_reader = world
        .resources
        .get_mut::<EventChannel<Event>>()
        .unwrap()
        .register_reader();

    SystemBuilder::<()>::new("InputSystem")
        .write_resource::<InputHandler>()
        .read_resource::<Window>()
        .read_resource::<EventChannel<Event>>()
        .build(
            move |_commands, _world, (input_handler, window, event_channel), _query| {
                let hidpi = window.get_hidpi_factor();

                input_handler.send_frame_begin();

                for event in event_channel.read(&mut window_event_reader) {
                    input_handler.send_event(event, hidpi as f32);
                }
            },
        )
}

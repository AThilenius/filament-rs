use crate::ThreadLocalSystem;
use legion::prelude::*;
use shrev::EventChannel;
use winit::{Event, EventsLoop, WindowBuilder};

pub fn build(world: &mut World) -> ThreadLocalSystem {
    let mut events_loop = EventsLoop::new();

    // Add the `Window` and `EventChannel<Event>` as world resources.
    world.resources.insert(
        WindowBuilder::new()
            .with_title("Filament")
            .build(&events_loop)
            .unwrap(),
    );
    world.resources.insert(EventChannel::<Event>::new());

    // Static cache for event aggregation.
    let mut events = Vec::with_capacity(100);

    Box::new(move |world| {
        events.clear();

        // Collect Winit events
        events_loop.poll_events(|event| {
            events.push(event);
        });

        // Drain them into the EventChannel
        let mut event_channel = world.resources.get_mut::<EventChannel<Event>>().unwrap();
        event_channel.drain_vec_write(&mut events);
    })
}

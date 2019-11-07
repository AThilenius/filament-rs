use crate::{components::*, input_system, rendering_system, window_system, ThreadLocalSystem};
use legion::prelude::*;
use legion_transform::transform_system_bundle;
use shrev::{EventChannel, ReaderId};
use std::thread::sleep;
use std::time::{Duration, Instant};
use winit::{Event, Window, WindowEvent};

/// Nothing more than a helper to setup some boilerplate and run systems for you at 144hz framerate.
pub struct Engine {
    pub should_exit: bool,

    pub universe: Universe,
    pub world: World,
    pub camera_entity: Entity,

    pub window_system: ThreadLocalSystem,
    pub input_system: Box<dyn Schedulable>,
    pub rendering_system: ThreadLocalSystem,
    pub transform_bundle: Vec<Box<dyn Schedulable>>,

    frame_start: Instant,
    target_frame_time: Duration,
    window_event_reader: ReaderId<Event>,
}

impl Engine {
    pub fn new() -> Self {
        let universe = Universe::new();
        let mut world = universe.create_world();

        // Create a Window, Input and Rendering system,
        let window_system = window_system::build(&mut world);
        let input_system = input_system::build(&mut world);
        let rendering_system = rendering_system::build(&mut world);

        // Pull in the system bundle for legion_transform.
        let transform_bundle = transform_system_bundle::build(&mut world);

        // Setup default world with a perspective main camera.
        let camera_entity = *world
            .insert(
                (),
                vec![(
                    Camera::new_perspective(60.0_f64.to_radians(), 0.1, 10_000.0),
                    Translation::new(0.0, 0.0, 10.0),
                    Rotation::identity(),
                    LocalToWorld::identity(),
                )],
            )
            .first()
            .unwrap();

        world.resources.insert(MainCamera::new(camera_entity));

        let target_frame_time = Duration::from_secs(1) / 144;
        let window_event_reader = world
            .resources
            .get_mut::<EventChannel<Event>>()
            .unwrap()
            .register_reader();

        Self {
            should_exit: false,
            universe,
            world,
            camera_entity,
            window_system,
            input_system,
            rendering_system,
            transform_bundle,
            frame_start: Instant::now(),
            target_frame_time,
            window_event_reader,
        }
    }

    pub fn begin_frame(&mut self) {
        self.frame_start = Instant::now();
        (*self.window_system)(&mut self.world);
        self.input_system.run(&mut self.world);
        self.input_system
            .command_buffer_mut()
            .write(&mut self.world);

        for event in self
            .world
            .resources
            .get::<EventChannel<Event>>()
            .unwrap()
            .read(&mut self.window_event_reader)
        {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    self.should_exit = true;
                }
                _ => {}
            }
        }

        for system in self.transform_bundle.iter() {
            system.run(&mut self.world);
            system.command_buffer_mut().write(&mut self.world);
        }
    }

    pub fn end_frame(&mut self) {
        (*self.rendering_system)(&mut self.world);

        let title = format!(
            "Last frame: {:.2}ms",
            (self.frame_start.elapsed().as_micros() as f64) / 1000_f64
        );
        self.world
            .resources
            .get_mut::<Window>()
            .unwrap()
            .set_title(&title);

        // Try to sleep long enough to hit the target frame time.
        if self.frame_start.elapsed() < self.target_frame_time {
            sleep(self.target_frame_time - self.frame_start.elapsed());
        }
    }
}

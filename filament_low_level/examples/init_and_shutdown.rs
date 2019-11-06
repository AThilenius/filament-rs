/// Demos creating a window (with `winit`), getting a native window handle then passing that off to
/// a Filament `Engine` `SwapChain` as well as creating a renderer. The Engine will be shut down
/// when all references to the internal unmanaged `::filament::Engine` are dropped (not then the
/// `Engine` wrapper is dropped).
extern crate filament_low_level;
extern crate winit;

use filament_low_level::prelude::*;
use winit::{Event, EventsLoop, Window, WindowBuilder, WindowEvent};

#[cfg(target_os = "macos")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
    use winit::os::macos::WindowExt;
    window.get_nsview()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
    use winit::os::windows::WindowExt;
    window.get_hwnd()
}

fn main() {
    // Create a winit event loop and surface
    let mut event_loop = EventsLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("Window Example with Winit");

    // The native surface handle is OS dependant.
    let window_handle = get_active_surface(&window);

    let mut engine = Engine::new(Backend::Default);
    let _swap_chain = engine.create_swap_chain(window_handle);
    let _renderer = engine.create_renderer();

    let mut exit = false;
    while !exit {
        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                exit = true;
            }
            _ => {}
        });
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

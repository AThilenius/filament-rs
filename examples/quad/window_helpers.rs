extern crate winit;

use winit::{EventsLoop, Window, WindowBuilder};

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

pub fn init_window(title: &str) -> (Window, EventsLoop, *mut std::ffi::c_void) {
  let event_loop = EventsLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  window.set_title(title);

  // The native surface handle is OS dependant.
  let window_handle = get_active_surface(&window);

  (window, event_loop, window_handle)
}

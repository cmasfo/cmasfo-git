
// gui window

#![windows_subsystem = "windows"]

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoop},
  window::WindowBuilder,
};

fn main() {

  let event_loop = EventLoop::new();
  let window = WindowBuilder::new().build(&event_loop).unwrap();
  
  // run event loop  
  event_loop.run(move |event, _, control_flow| {

    *control_flow = ControlFlow::Wait;
    
    match event {
      // exit control
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        window_id,
      } if window_id == window.id() => {
        *control_flow = ControlFlow::Exit;
      },
      // else
      _ => (),
    } // match event

  }); // event_loop.run()

} // fn main

use settings::Settings;

use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::PhysicalSize,
};
use winit::event_loop::ControlFlow;

fn main() {

    let event_loop = EventLoop::new();

    let window_size: PhysicalSize<u32> = (640, 480).into();

    let _window = WindowBuilder::new()
        .with_fullscreen(None)
        .with_inner_size(window_size)
        .with_title("MarsEngine")
        .build(&event_loop)
        .unwrap();



    event_loop.run(move |event, _, control_flow| {

        *control_flow = ControlFlow::Poll;

        match event {
            Event::MainEventsCleared => {

            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }
            _ => {}
        }

    })
}

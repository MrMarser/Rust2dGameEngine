use std::time::{Duration, Instant};
use settings::{ApplySettings};
use std::sync::{Arc, Mutex};
use winit::{
    event::*,
    event_loop::EventLoop,
    window::WindowBuilder,
    dpi::PhysicalSize,
};
use winit::event_loop::ControlFlow;

pub fn window_init(){
    let event_loop = EventLoop::new();

    let window_size: PhysicalSize<u32> = (640, 480).into();

    let window = WindowBuilder::new()
        .with_fullscreen(None)
        .with_inner_size(window_size)
        .with_title("Rust2dGameEngine")
        .build(&event_loop)
        .unwrap();

    let framerate = Arc::new(Mutex::new(60));
    ApplySettings::apply_window_settings(&window, Arc::clone(&framerate));

    event_loop.run(move |event, _, control_flow| {
        let fps = *framerate.lock().unwrap();
        if fps == 0 {
            *control_flow = ControlFlow::Poll;
        } else {
            let frame_duration = Duration::from_secs_f64(1.0 / fps as f64);
            let next_frame_time = Instant::now() + frame_duration;
            *control_flow = ControlFlow::WaitUntil(next_frame_time);
        }
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


use winit::event::*;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

pub mod constants;
mod state;
pub use state::State;
mod vertex;
pub use vertex::Vertex;

pub async fn run() {
    let event_loop = EventLoop::new();  // Create event loop to create other stuff based on it
    let window = WindowBuilder::new().build(&event_loop).unwrap(); // Create window

    let mut state = State::new(window).await; // Create state

    // Run the application
    event_loop.run(move |event, _, control_flow| match event { // Handle all events
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == state.window().id() => { // Was a window event on the window that is being managed
            if !state.input(event) { // Send the event to state.input() and only continue if the event is not handled by that function TODO: comment state.input()
                match event { // Shadows event from before, is now specifically a window event
                    WindowEvent::CloseRequested // Should close
                    | WindowEvent::KeyboardInput { // Got keyboard input, ...
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed, // ..., A key got PRESSED, ...
                                virtual_keycode: Some(VirtualKeyCode::Escape), // ..., it was the Escape key
                                ..
                            },
                        ..
                    } => *control_flow = ControlFlow::Exit, // Close the application
                    WindowEvent::Resized(physical_size) => { // The window was resized
                        state.resize(*physical_size); // Set the new size
                    }
                    WindowEvent::ScaleFactorChanged { new_inner_size, .. } => { // The scaling of the operating system changed
                        state.resize(**new_inner_size); // Resize to the new size
                    }
                    _ => {}
                }
            }
        }
        Event::RedrawRequested(window_id) if window_id == state.window().id() => { // Was a request to redraw on the managed window
            state.update(); // Update the state TODO: comment state.update() (is not yet used)
            match state.render() { // Render
                Ok(_) => {} // No error
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size), // Lost the window, update it by resizing it to the current size
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit, // Out of memory, application should probably exit
                Err(e) => eprintln!("{:?}", e), // Log every other error
            }
        }
        Event::MainEventsCleared => { // All events got cleared, good to go
            state.window().request_redraw(); // Redraw, which calls the event above
        }
        _ => {}
    });
}

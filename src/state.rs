use winit::{EventsLoop, Event, WindowEvent, ElementState, VirtualKeyCode};

pub struct State {
    pub camera: Camera,
    pub render_mode: RenderMode,
    pub run: bool,
}

impl State {
    pub fn new() -> State {
        State {
            camera: Camera::LockedOnPlanet (0),
            render_mode: RenderMode::Standard,
            run: true,
        }
    }

    pub fn update(&mut self, events_loop: &mut EventsLoop) {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = event {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => { self.run = false }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let ElementState::Pressed = input.state {
                            if let Some(key_code) = input.virtual_keycode {
                                self.keypress(key_code);
                            }
                        }
                    }
                    _ => {}
                }
            }
        });
    }

    fn keypress(&mut self, key_code: VirtualKeyCode) {
        match key_code {
            VirtualKeyCode::A => {
                self.render_mode = match self.render_mode {
                    RenderMode::Standard => { RenderMode::Wireframe }
                    RenderMode::Wireframe => { RenderMode::Standard }
                };
            }
            _ => { }
        }
    }
}

#[allow(unused)]
pub enum Camera {
    Free { location: [f32; 3], look_at: [f32; 3] },
    LockedOnPlanet (i32)
}

#[allow(unused)]
pub enum RenderMode {
    Standard,
    Wireframe,
}

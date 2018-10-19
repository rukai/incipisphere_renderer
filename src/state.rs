use winit::{EventsLoop, Event, WindowEvent, ElementState, VirtualKeyCode, MouseScrollDelta};
use glm;
use glm::TVec3;

pub struct State {
    pub camera: Camera,
    pub render_mode: RenderMode,
    pub run: bool,
    pub entities: Vec<Entity>,
}

impl State {
    pub fn new() -> State {
        let mut entities = vec!();
        entities.push(Entity {
            location: glm::vec3(0.0, 0.0, 0.0),
            ty: EntityType::BasicPlanet { color: [1.0, 1.0, 1.0, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(5.0, 0.0, 0.0),
            ty: EntityType::BasicPlanet { color: [0.2, 0.5, 1.0, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(-5.0, 0.0, 0.0),
            ty: EntityType::BasicPlanet { color: [0.1, 0.7, 0.1, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(0.0, 0.0, 5.0),
            ty: EntityType::BasicPlanet { color: [0.0, 0.0, 0.4, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(0.0, 0.0, -5.0),
            ty: EntityType::BasicPlanet { color: [1.0, 0.0, 0.0, 1.0] },
        });

        State {
            camera: Camera::Free { eye: glm::vec3(0.0, 0.0, 8.0), look_dir: glm::vec3(0.0, 0.0, -1.0) },
            render_mode: RenderMode::Standard,
            run: true,
            entities,
        }
    }

    pub fn update(&mut self, events_loop: &mut EventsLoop) {
        events_loop.poll_events(|event| {
            if let Event::WindowEvent { event, .. } = &event {
                match event {
                    WindowEvent::CloseRequested | WindowEvent::Destroyed => { self.run = false }
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let ElementState::Pressed = input.state {
                            if let Some(key_code) = input.virtual_keycode {
                                match key_code {
                                    VirtualKeyCode::Z => {
                                        self.render_mode = match self.render_mode {
                                            RenderMode::Standard => { RenderMode::Wireframe }
                                            RenderMode::Wireframe => { RenderMode::Standard }
                                        };
                                    }
                                    _ => { }
                                }
                            }
                        }
                    }
                    _ => {}
                }
                self.camera.update(event);
            }
        });
    }
}

#[allow(unused)]
pub enum Camera {
    Free {
        eye: TVec3<f32>,
        look_dir: TVec3<f32> // TODO: control with click and drag
    },
    LookAtEntity {
        planet_index: usize, // TODO: click on entity to select
        distance: f32, // TODO: control with scroll wheel
        x: f32, // TODO: control with click and drag horizontal
        y: f32, // TODO: control with click and drag vertical
    }
}

impl Camera {
    pub fn eye(&self) -> TVec3<f32> {
        match self {
            Camera::Free { eye, .. } => eye.clone(),
            Camera::LookAtEntity { .. } => {
                unimplemented!()
            }
        }
    }

    pub fn look_at(&self) -> TVec3<f32> {
        match self {
            Camera::Free { eye, look_dir } => eye + look_dir,
            Camera::LookAtEntity { .. } => {
                unimplemented!()
            }
        }
    }

    pub fn update(&mut self, event: &WindowEvent) {
        match self {
            &mut Camera::Free { ref mut eye, ref mut look_dir } => {
                match event {
                    WindowEvent::KeyboardInput { input, .. } => {
                        if let ElementState::Pressed = input.state {
                            if let Some(key_code) = input.virtual_keycode {
                                match key_code {
                                    VirtualKeyCode::A => { eye.x -= 0.5 }
                                    VirtualKeyCode::D => { eye.x += 0.5 }
                                    VirtualKeyCode::W => { eye.y -= 0.5 }
                                    VirtualKeyCode::S => { eye.y += 0.5 }
                                    _ => { }
                                }
                            }
                        }
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        match delta {
                            MouseScrollDelta::LineDelta  (_, y) => { eye.z -= y }
                            MouseScrollDelta::PixelDelta (_) => panic!("Ooer, I dont know how to handle PixelDelta...") // TODO
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        look_dir.x += position.x as f32;
                        look_dir.y += position.y as f32;
                    }
                    _ => {}
                }
            }
            Camera::LookAtEntity { .. } => { } // TODO
        }
    }
}

//pub struct Camera {
//    look_at: CameraLookAt,
//    up: TVec3<f32>,
//    location: TVec3<f32>,
//}

//pub enum CameraLookAt {
//    Vector (TVec3<f32>),
//    Planet (usize)
//}

pub enum RenderMode {
    Standard,
    Wireframe,
}

pub struct Entity {
    pub location: TVec3<f32>,
    pub ty: EntityType,
}

pub enum EntityType {
    BasicPlanet {
        color: [f32; 4]
    }
}

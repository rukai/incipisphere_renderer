use winit::{EventsLoop, Event, WindowEvent, ElementState, VirtualKeyCode};
use glm;
use glm::TVec3;

pub struct State {
    pub hack: TVec3<f32>,
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
            hack: glm::vec3(0.3, 0.3, 8.0),
            camera: Camera::Free { eye: [0.0; 3], look_at: [0.0; 3] },
            render_mode: RenderMode::Standard,
            run: true,
            entities,
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
            VirtualKeyCode::Z => {
                self.render_mode = match self.render_mode {
                    RenderMode::Standard => { RenderMode::Wireframe }
                    RenderMode::Wireframe => { RenderMode::Standard }
                };
            }
            VirtualKeyCode::A => { self.hack.x -= 0.5 }
            VirtualKeyCode::D => { self.hack.x += 0.5 }
            VirtualKeyCode::W => { self.hack.y -= 0.5 }
            VirtualKeyCode::S => { self.hack.y += 0.5 }
            _ => { }
        }
    }
}

#[allow(unused)]
pub enum Camera {
    Free {
        eye: [f32; 3], // TODO: control with wasd
        look_at: [f32; 3] // TODO: control with click and drag
    },
    LookAtEntity {
        planet_index: usize, // TODO: click on entity to select
        distance: f32, // TODO: control with scroll wheel
        x: f32, // TODO: control with click and drag horizontal
        y: f32, // TODO: control with click and drag vertical
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

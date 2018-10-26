use winit::VirtualKeyCode;
use glm;
use glm::TVec3;

use winit_input_helper::WinitInputHelper;

pub struct State {
    pub camera:         Camera,
    pub render_mode:    RenderMode,
    pub run:            bool,
    pub window_resized: bool,
    pub entities:       Vec<Entity>,
}

impl State {
    pub fn new() -> State {
        let mut entities = vec!();
        entities.push(Entity {
            location: glm::vec3(0.0, 0.0, 0.0),
            ty: EntityType::BasicPlanet { color: [1.0, 1.0, 1.0, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(-5.0, 0.0, 0.0),
            ty: EntityType::BasicPlanet { color: [0.2, 0.5, 1.0, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(5.0, 0.0, 0.0),
            ty: EntityType::BasicPlanet { color: [0.1, 0.7, 0.1, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(0.0, 0.0, -5.0),
            ty: EntityType::BasicPlanet { color: [0.0, 0.0, 0.4, 1.0] },
        });
        entities.push(Entity {
            location: glm::vec3(0.0, 0.0, 5.0),
            ty: EntityType::BasicPlanet { color: [1.0, 0.0, 0.0, 1.0] },
        });

        let camera = Camera {
            eye: glm::vec3(0.0, 0.0, -8.0),
            up: glm::vec3(0.0, -1.0, 0.0),
            look_at: CameraLookAt::Dir(glm::vec3(0.0, 0.0, 1.0))
        };

        State {
            render_mode: RenderMode::Standard,
            run: true,
            window_resized: false,
            camera,
            entities,
        }
    }

    pub fn update(&mut self, input: &WinitInputHelper) {
        self.window_resized = input.window_resized().is_some();

        if input.key_pressed(VirtualKeyCode::Z) {
            self.render_mode = match self.render_mode {
                RenderMode::Standard => { RenderMode::Wireframe }
                RenderMode::Wireframe => { RenderMode::Standard }
            };
        }

        if input.mouse_pressed(1) {
            // if clicked on entity
            self.camera.look_at = CameraLookAt::Entity(0); // TODO: Get the clicked on entity

            // if clicked on empty space
            self.camera.look_at = CameraLookAt::Dir(glm::vec3(0.0, 0.0, -1.0)); // TODO: Set the direction such that the camera doesnt move.
        }

        match &mut self.camera.look_at {
            &mut CameraLookAt::Dir ( ref mut dir ) => {
                if input.mouse_held(0) {
                    let mouse_diff = input.mouse_diff();
                    dir.x += mouse_diff.0 / 40.0;
                    dir.y -= mouse_diff.1 / 40.0;
                }

                let mut trans_eye = glm::vec3(0.0, 0.0, 0.0);
                if input.key_held(VirtualKeyCode::A) {
                    trans_eye.x -= 0.1;
                }
                if input.key_held(VirtualKeyCode::D) {
                    trans_eye.x += 0.1;
                }
                if input.key_held(VirtualKeyCode::W) {
                    trans_eye.y += 0.1;
                }
                if input.key_held(VirtualKeyCode::S) {
                    trans_eye.y -= 0.1;
                }
                trans_eye.z += input.scroll_diff();
                // TODO: How do I move the camera in the axis it is looking
                //let vec4 = glm::orientation(dir, &self.camera.up) * glm::vec3_to_vec4(&trans_eye);
                //self.camera.eye += glm::vec4_to_vec3(&vec4);
                self.camera.eye += trans_eye;
            }
            CameraLookAt::Entity { .. } => {
                if input.mouse_held(0) {
                    // self.camera.eye // TODO: control with click and drag
                }
                // self.camera.eye // TODO: scroll wheel needs to move away/towards the entity
            }
        }
    }
}

pub struct Camera {
    pub look_at: CameraLookAt,
    pub up: TVec3<f32>,
    pub eye: TVec3<f32>,
}

pub enum CameraLookAt {
    Dir (TVec3<f32>),
    Entity (usize)
}

impl Camera {
    pub fn look_at(&self, entities: &[Entity]) -> TVec3<f32> {
        match &self.look_at {
            CameraLookAt::Dir (dir) => self.eye + dir,
            CameraLookAt::Entity (index) => entities[*index].location,
        }
    }
}

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

#[macro_use]
extern crate vulkano;
extern crate vulkano_shaders;
extern crate winit;
extern crate vulkano_win;
extern crate genmesh;
extern crate nalgebra_glm as glm;

mod render;
mod state;

use render::Render;
use state::State;

use winit::EventsLoop;

fn main() {
    let mut events_loop = EventsLoop::new();
    let mut render = Render::new(&events_loop);
    let mut state = State::new();

    while state.run {
        render.draw(&state);
        state.update(&mut events_loop);
    }
}

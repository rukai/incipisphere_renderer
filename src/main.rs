mod render;
mod state;

use render::Render;
use state::State;

use winit::EventsLoop;
use winit_input_helper::WinitInputHelper;

fn main() {
    let mut events_loop = EventsLoop::new();
    let mut render = Render::new(&events_loop);
    let mut state = State::new();
    let mut input = WinitInputHelper::new();

    while !input.quit() {
        input.update(&mut events_loop);
        state.update(&input);
        render.draw(&state);
    }
}

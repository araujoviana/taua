mod backend;
mod state;

use crate::backend::winit as winit_backend;
use crate::state::State;
use calloop::EventLoop;
use smithay::reexports::wayland_server::Display;

fn main() {
    // Creating the display
    let mut display: Display<State> = Display::new().unwrap();
    let mut display_handle = display.handle();

    let mut state = State { display_handle };

    let mut event_loop = EventLoop::<State>::try_new().unwrap();

    winit_backend::init_winit(&mut event_loop);

    event_loop.run(None, &mut state, |_| {}).unwrap();
}

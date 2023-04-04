extern crate mouse_keyboard_input;

use mouse_keyboard_input::VirtualDevice;
use mouse_keyboard_input::key_codes::*;

use notan::draw::*;
use notan::prelude::*;

#[notan_main]
fn main() -> Result<(), String> {
    notan::init_with(setup)
        .add_config(DrawConfig)
        .event(event)
        .update(update)
        .build()
}

#[derive(AppState)]
struct State {
    delta: f32,
    start: bool,
    device: VirtualDevice,
}

fn setup(gfx: &mut Graphics) -> State {
    State {
        delta: 0.0,
        start: false,
        device: VirtualDevice::new(),
    }
}

fn event (state: &mut State, event: Event) {
    match event {
        Event::ReceivedCharacter(w) if w != '\u{7f}' => {
            if state.start == false {state.start = true;} else {state.start = false;}
            print!("hit");
        }
        _ => {}
    }
}

fn update(app: &mut App, state: &mut State) {
    if state.start == true {
        state.delta += app.timer.delta_f32();

        if state.delta >= 0.2 {
            //click the left mouse button
            state.device.click(BTN_LEFT).unwrap();
            state.delta = 0.0;
        }
    }
}
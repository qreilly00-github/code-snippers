use notan::draw::*;
use notan::prelude::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

#[notan_main]
fn main() -> Result<(), String> {
    let win_config = WindowConfig::new().size(800, 600).vsync(true);

    notan::init_with(State::new)
        .add_config(win_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {

}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();

    draw.clear(Color::WHITE);
    gfx.render(&draw);
}

#[derive(AppState)]
struct State {

}

impl State {
    fn new() -> Self {
        Self {

        }
    }
}
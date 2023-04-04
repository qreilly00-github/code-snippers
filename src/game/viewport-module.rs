struct Viewport {
    x: f32,
    y: f32,

    width: f32,
    height: f32,
}

impl Viewport {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,

            width: 0.0,
            height: 0.0,
        }
    }

    pub fn with_values(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            x,
            y,

            width,
            height,
        }
    }

    pub fn set(&self, draw: &mut Draw) {
        draw.set_size(WINDOW_WIDTH * self.width, WINDOW_HEIGHT * self.height)
    }

    pub fn draw(&self, draw: &mut Draw, objects: &Vec<Box <dyn Drawable>>) {
        let player = *&objects[0].downcast::<Player>();
        let offset = (self.width * (WINDOW_WIDTH / 2.0), self.height * (WINDOW_HEIGHT / 2.0));

        objects[0].draw(draw, offset.0, offset.1);

        if objects.len() > 1 {


            for object in objects.iter().skip(1) {
                object.draw(draw, offset.0 - player.x(), offset.1 - player.y());
            }
        }


    }

    pub fn x(&mut self) -> f32 { self.x }
    pub fn y(&mut self) -> f32 { self.y }

    pub fn width(&mut self) -> f32 { self.width }
    pub fn height(&mut self) -> f32 { self.height }
}
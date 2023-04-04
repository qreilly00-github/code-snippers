struct Wall {
    x: f32,
    y: f32,

    width: f32,
    height: f32,

    color: Color,
}

impl Wall {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,

            width: 0.0,
            height: 0.0,

            color: Color::BLUE,
        }
    }

    pub fn with_values(x: f32, y: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            x,
            y,

            width,
            height,

            color,
        }
    }

    pub fn x(&mut self) -> f32 { self.x }
    pub fn y(&mut self) -> f32 { self.y }

    pub fn width(&mut self) -> f32 { self.width }
    pub fn height(&mut self) -> f32 { self.height }

    pub fn color(&mut self) -> Color { self.color }
}
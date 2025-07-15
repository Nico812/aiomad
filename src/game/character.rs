pub struct Character {
    pub position: [f32; 3],
    pub speed: f32,
}

impl Character {
    pub fn new(position: [f32; 3]) -> Self {
        Self {
            position,
            speed: 0.1,
        }
    }
}

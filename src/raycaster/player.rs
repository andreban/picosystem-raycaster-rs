pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle_deg: i16,
    pub fov: i16,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 1.5,
            y: 1.5,
            angle_deg: 0,
            fov: 60,
        }
    }
}

use micromath::F32Ext;

pub enum HitType {
    Horizontal,
    Vertical,
}

pub struct Hit {
    pub x: f32,
    pub y: f32,
    pub hit_type: HitType,
}

impl Hit {
    pub fn new(x: f32, y: f32, hit_type: HitType) -> Self {
        Self { x, y, hit_type }
    }

    pub fn distance(&self, x: f32, y: f32) -> f32 {
        f32::sqrt(f32::powi(self.x - x, 2) + f32::powi(self.y - y, 2))
    }
}

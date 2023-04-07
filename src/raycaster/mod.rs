mod hit;
mod map;
mod player;

use hit::{Hit, HitType};
pub use map::Map;
use micromath::F32Ext;
pub use player::Player;

static PI: f32 = 3.14159265358979;

pub struct Raycaster {
    pub player: Player,
    pub map: Map,
    pub screen_width: f32,
    pub screen_height: f32,
    pub tans: [f32; 360],
    pub sins: [f32; 360],
    pub cosins: [f32; 360],
}

const COLOR_GRAY: u16 = u16::from_be(0xc638);
const COLOR_DARKGRAY: u16 = u16::from_be(0x5acb);
const COLOR_BLUE: u16 = u16::from_be(0x001f);
const COLOR_LIGHTBLUE: u16 = u16::from_be(0x39df);

/// Converts degrees to radians.
pub fn degrees_to_radians(deg: i16) -> f32 {
    deg as f32 * PI / 180.0
}

impl Raycaster {
    pub fn new() -> Self {
        let mut tans = [0.0_f32; 360];
        for i in 0..360 {
            tans[i] = f32::tan(degrees_to_radians(i as i16));
        }

        let mut sins = [0.0_f32; 360];
        for i in 0..360 {
            sins[i] = f32::sin(degrees_to_radians(i as i16)) / 16.0;
        }

        let mut cosins = [0.0_f32; 360];
        for i in 0..360 {
            cosins[i] = f32::cos(degrees_to_radians(i as i16)) / 16.0;
        }

        Self {
            player: Player::new(),
            map: Map::new(),
            screen_width: 240.0,
            screen_height: 240.0,
            tans,
            sins,
            cosins,
        }
    }

    fn check_vertical_walls(&self, ray_angle: i16) -> Option<Hit> {
        // If there's no horizontal direction to the ray, it will never hit a vertical wall.
        if ray_angle == 90 || ray_angle == 270 {
            return None;
        }

        // Parameters change depending if the ray direction is left or right. We want to:
        //  - start with the wall right *after* the player position, using `ceil()`,
        //  - iterate walls that are *further head*, using a positive 1.0 increment,
        //  - check the cell closer to the ray when it hits a wall,
        // Those parameters change, depending on the ray pointing left or right.
        let (mut ray_x, ray_x_step, map_offset, round_func) = if ray_angle < 90 || ray_angle > 270 {
            (self.player.x.ceil(), 1.0, 0.0, f32::floor as fn(f32) -> f32)
        } else {
            (
                self.player.x.floor(),
                -1.0,
                -1.0,
                f32::ceil as fn(f32) -> f32,
            )
        };

        let mut ray_y;
        let ray_tan = self.tans[ray_angle as usize]; //degrees_to_radians(ray_angle).tan(); // trig::tans()[ray_angle as usize];//

        loop {
            ray_y = self.player.y + ray_tan * (ray_x - self.player.x); // calculate the Y position.
            let wall_x = (round_func(ray_x) + map_offset) as usize;
            let wall_y = (round_func(ray_y) + map_offset) as usize;
            match self.map.tile_at(wall_x, wall_y) {
                None => break,
                Some(w) if w != 0 => break,
                _ => ray_x += ray_x_step,
            }
        }
        Some(Hit::new(ray_x, ray_y, hit::HitType::Vertical))
    }

    fn check_horizontal_walls(&self, ray_angle: i16) -> Option<Hit> {
        // If there's no vertical direction to the ray, it will never hit an horizontal wall.
        if ray_angle == 0 || ray_angle == 180 {
            return None;
        }

        // Parameters change depending if the ray direction is up or down. We want to:
        //  - start with the wall right *after* the player position, using `ceil()`,
        //  - iterate walls that are *further head*, using a positive 1.0 increment,
        //  - check the cell closer to the ray when it hits a wall,
        // Those parameters change, depending on the ray pointing up or down.
        let (mut ray_y, ray_y_step, map_offset, round_func) = if ray_angle > 0 && ray_angle < 180 {
            (self.player.y.ceil(), 1.0, 0.0, f32::floor as fn(f32) -> f32)
        } else {
            (
                self.player.y.floor(),
                -1.0,
                -1.0,
                f32::ceil as fn(f32) -> f32,
            )
        };

        let ray_tan = self.tans[ray_angle as usize]; //degrees_to_radians(ray_angle).tan();
        let mut ray_x;
        loop {
            ray_x = self.player.x + (ray_y - self.player.y) / ray_tan; // calculate the X position.
            let wall_x = (round_func(ray_x) + map_offset) as usize;
            let wall_y = (round_func(ray_y) + map_offset) as usize;
            match self.map.tile_at(wall_x, wall_y) {
                None => break,
                Some(w) if w != 0 => break,
                _ => ray_y += ray_y_step,
            }
        }
        Some(Hit::new(ray_x, ray_y, hit::HitType::Horizontal))
    }

    fn check_hits_alt(&self, ray_angle: i16) -> Option<Hit> {
        let vertical_hit = self.check_vertical_walls(ray_angle);
        let horizontal_hit = self.check_horizontal_walls(ray_angle);

        if horizontal_hit.is_none() {
            return vertical_hit;
        }

        if vertical_hit.is_none() {
            return horizontal_hit;
        }

        let hh = horizontal_hit.as_ref().unwrap();
        let dh = hh.squared_distance(self.player.x, self.player.y);

        let hv = vertical_hit.as_ref().unwrap();
        let dv = hv.squared_distance(self.player.x, self.player.y);

        if dh < dv {
            return horizontal_hit;
        }

        vertical_hit
    }

    fn check_hits(&self, ray_angle: i16) -> Option<Hit> {
        let (mut ray_x, mut ray_y) = (self.player.x, self.player.y);

        let ray_cos = self.cosins[ray_angle as usize];
        let ray_sin = self.sins[ray_angle as usize];

        let mut wall = 0;
        while wall == 0 {
            ray_x += ray_cos;
            ray_y += ray_sin;
            wall = self.map.tile_at(ray_x as usize, ray_y as usize).unwrap();
        }

        Some(Hit::new(ray_x, ray_y, HitType::Horizontal))
    }

    pub fn ray_casting<F>(&self, draw_line: &mut F)
    where
        F: FnMut(u16, u16, u16, u16, u16),
    {
        let mut ray_angle = self.player.angle_deg - self.player.fov / 2;
        if ray_angle >= 360 {
            ray_angle -= 360;
        }
        if ray_angle < 0 {
            ray_angle += 360;
        }

        let num_rays = 60;
        let increment_angle = self.player.fov / num_rays;
        let line_thickness = (self.screen_width / num_rays as f32).ceil() as u16;
        let half_screen_height = self.screen_height / 2.0;

        for ray_count in 0..num_rays as u16 {
            let hit = self.check_hits(ray_angle).unwrap();

            let mut distance = hit.squared_distance(self.player.x, self.player.y).sqrt();

            // Fish eye fix
            distance = distance * f32::cos(degrees_to_radians(ray_angle - self.player.angle_deg));
            if distance == 0.0 {
                distance = 1.0;
            }

            // let wall_height = (game_data.screen.half_height / distance).floor();
            // Macroquad uses floats, so no need to floor the wall height.
            let wall_height = self.screen_height / distance;

            let x = ray_count * line_thickness;
            let x2 = x + line_thickness;

            draw_line(
                x,
                0,
                x2,
                (half_screen_height - wall_height) as u16,
                COLOR_GRAY,
            );

            let color = match hit.hit_type {
                HitType::Horizontal => COLOR_BLUE,
                HitType::Vertical => COLOR_LIGHTBLUE,
            };
            draw_line(
                x,
                (half_screen_height - wall_height) as u16,
                x2,
                (half_screen_height + wall_height) as u16,
                color,
            );

            draw_line(
                x,
                (half_screen_height + wall_height) as u16,
                x2,
                self.screen_height as u16,
                COLOR_DARKGRAY,
            );

            ray_angle += increment_angle;
            if ray_angle >= 360 {
                ray_angle -= 360;
            }
            if ray_angle < 0 {
                ray_angle += 360;
            }
        }
    }
}

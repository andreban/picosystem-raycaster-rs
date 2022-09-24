#![no_std]
#![no_main]

mod picosystem;
mod raycaster;
mod st7789;

use picosystem::PicoSystem;
use rp_pico::entry;
use micromath::F32Ext;
use raycaster::degrees_to_radians;

// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

#[entry]
fn main() -> ! {
    let mut device = PicoSystem::take().unwrap();
    let mut frame_buffer = [u16::from_be(0xf800_u16); 240 * 240];
    device.display.set_pixels(&frame_buffer).unwrap();

    let mut raycaster = raycaster::Raycaster::new();
    loop {
        if device.button_right.is_pressed() {
            raycaster.player.angle_deg += 4;
        }

        if device.button_left.is_pressed() {
            raycaster.player.angle_deg -= 4;
        }

        if device.button_up.is_pressed() {
            let player_cos = f32::cos(degrees_to_radians(raycaster.player.angle_deg)) * 0.2;
            let player_sin = f32::sin(degrees_to_radians(raycaster.player.angle_deg)) * 0.2;
            let new_x = raycaster.player.x + player_cos;
            let new_y = raycaster.player.y + player_sin;
            if let Some(0) = raycaster.map.tile_at(new_x as usize, new_y as usize) {
                raycaster.player.x = new_x;
                raycaster.player.y = new_y;
            }
        }

        if device.button_down.is_pressed() {
            let player_cos = f32::cos(degrees_to_radians(raycaster.player.angle_deg)) * 0.2;
            let player_sin = f32::sin(degrees_to_radians(raycaster.player.angle_deg)) * 0.2;
            let new_x = raycaster.player.x - player_cos;
            let new_y = raycaster.player.y - player_sin;
            if let Some(0) = raycaster.map.tile_at(new_x as usize, new_y as usize) {
                raycaster.player.x = new_x;
                raycaster.player.y = new_y;
            }
        }

        raycaster.ray_casting(&mut |x1, y1, x2, y2, color| {
            for row in y1..y2 {
                if row >= 240 {
                    continue;
                }
                let row_start = (row * 240) as usize;
                for col in x1..x2 {
                    if col >= 240 {
                        continue;
                    }
                    frame_buffer[row_start + col as usize] = color;
                }
            }
        });
        device.display.set_pixels(&frame_buffer).unwrap();
    }
}

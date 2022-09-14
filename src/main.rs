#![no_std]
#![no_main]

mod picosystem;
mod raycaster;

use embedded_graphics_framebuf::FrameBuf;
use picosystem::PicoSystem;

use embedded_graphics::{
    draw_target::DrawTarget,
    pixelcolor::{Rgb565, RgbColor},
    prelude::*,
    primitives::{Line, PrimitiveStyleBuilder, StyledDrawable}, mono_font::{MonoTextStyleBuilder, ascii::FONT_6X10}, text::Text,
};

use rp_pico::entry;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use heapless::String;

#[entry]
fn main() -> ! {
    let mut data = [Rgb565::BLACK; 240 * 240];
    let mut fbuf = FrameBuf::new(&mut data, 240, 240);
    let mut device = PicoSystem::take().unwrap();
    device.display.clear(Rgb565::BLUE).unwrap();

    let mut raycaster = raycaster::Raycaster {
        player: raycaster::Player::new(),
        map: raycaster::Map::new(),
        screen_width: 240.0,
        screen_height: 240.0,
    };

    loop {
        if device.button_right.is_pressed() {
            raycaster.player.angle_deg += 1;
        }

        if device.button_left.is_pressed() {
            raycaster.player.angle_deg -= 1;
        }

        let start = device.timer.get_counter();
        raycaster.ray_casting(&mut |x1, y1, x2, y2, thickness, color| {
            let line = Line::new(
                Point::new(x1 as i32, y1 as i32),
                Point::new(x2 as i32, y2 as i32),
            );
            let style = PrimitiveStyleBuilder::new()
                .stroke_width(thickness as u32)
                .stroke_color(color)
                .build();
            line.draw_styled(&style, &mut fbuf).unwrap();
        });
        let end = device.timer.get_counter();

        let str = String::<255>::from(end - start);
        let text_style = MonoTextStyleBuilder::new()
            .font(&FONT_6X10)
            .text_color(Rgb565::WHITE)
            .build();
        let text =
            Text::new(&str, Point::new(80, 120), text_style);
        text.draw(&mut fbuf).unwrap();
        device.display.draw_iter(fbuf.into_iter()).unwrap();
    }
}

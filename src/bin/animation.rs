use perlin::*;

use std::time::{Duration, Instant};
use std::thread::sleep;

use image::{Luma, ImageBuffer};
use macroquad::prelude::*;

const WINDOW_WIDTH: i32 = 1280;
const WINDOW_HEIGHT: i32 = 960;
const TARGET_FPS: f32 = 60.0;

const TIME_STEP: f64 = 0.01;
const RES_X: u32 = 100;
const RES_Y: u32 = 100;

const R: f64 = 0.5;

fn window_conf() -> Conf {
    return Conf {
        window_title: "Perlin Noise Animation".to_owned(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        ..Default::default()
    };
}

macro_rules! rgba {
    ($r:expr, $g:expr, $b:expr, $a:expr) => { Color::new(($r as f32) / 255.0, ($g as f32) / 255.0, ($b as f32) / 255.0, ($a as f32) / 255.0) };
}

// const DARK_PURPLE: Color = rgba!(33, 25, 81, 255);
// const PURPLE: Color = rgba!(118, 74, 241, 170);
// const LIGHT_PURPLE: Color = rgba!(131, 111, 255, 255);
// const GREEN: Color = rgba!(21, 245, 186, 255);
// const WHITE: Color = rgba!(240, 243, 255, 255);
// const RED: Color = rgba!(245, 0, 179, 255);
const PINK: Color = rgba!(245, 0, 179, 255);
// const YELLOW: Color = rgba!(255, 245, 186, 255);

const TEXT_COLOR: Color = PINK;

#[macroquad::main(window_conf)]
async fn main() {
    let frame_duration = Duration::from_secs_f32(1.0 / TARGET_FPS);
    let mut time = 0.0;

    let mut noise_map = NoiseMap::new(default_layers(2, 0.5));

    loop {
        let frame_start = Instant::now();

        let mut pixels = Vec::with_capacity((RES_X * RES_Y) as usize);
        for y in 0..RES_Y {
            for x in 0..RES_X {
                let x = x as f64 / 10.0;
                let y = y as f64 / 10.0;
                let l = ((noise_map.get(&vec![x, y, time]) + R) / R / 2.0 * 256.0) as u8;
                pixels.extend_from_slice(&[l, l, l, 255]);
            }
        }

        let texture = Texture2D::from_rgba8(RES_X as u16, RES_Y as u16, &pixels);

        draw_texture_ex(
            &texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32)),
                ..Default::default()
            },
        );

        time += TIME_STEP;

        draw_text(&format!("{:.2}", get_fps()), 5.0, 25.0, 35.0, TEXT_COLOR);

        next_frame().await;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_duration {
            sleep(frame_duration - elapsed);
        }}
}


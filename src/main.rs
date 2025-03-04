use std::f32::consts::PI;

use macroquad::prelude::*;

/// Size of circle
const SIZE: f32 = 600.0;
const WIDTH: f32 = SIZE;
const HEIGHT: f32 = SIZE + 64.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "Cardioid Simulation".to_owned(),
        window_width: WIDTH as i32,
        window_height: HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let radius: f32 = SIZE / 2.0;
    let mut divisions: i32 = 350;
    let mut n: i32 = 2;
    // Color mode
    let mut mode: u8 = 0;
    /// Number of color modes
    const NUM_MODES: u8 = 12;

    loop {
        if is_key_pressed(KeyCode::Left) && divisions > 10 {
            divisions -= 1;
        } else if is_key_pressed(KeyCode::Right) {
            divisions += 1;
        }

        if is_key_pressed(KeyCode::Down) && n > 2 {
            n -= 1;
        } else if is_key_pressed(KeyCode::Up) {
            n += 1;
        }

        if is_key_pressed(KeyCode::C) {
            mode += 1;
            mode %= NUM_MODES;
        }

        if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::Left) && divisions > 10 {
            divisions -= 1;
        } else if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::Right) {
            divisions += 1;
        }

        if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::Down) && n > 2 {
            n -= 1;
        } else if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::Up) {
            n += 1;
        }

        clear_background(BLACK);

        // draw_circle_lines(radius, radius, radius, 1.0, WHITE);

        let angle: f32 = (2.0 * PI) / (divisions as f32);
        for i in 0..divisions {
            let start = get_coord(i, angle, radius);
            let end = get_coord(i * n, angle, radius);
            draw_line(
                start.0,
                start.1,
                end.0,
                end.1,
                1.0,
                get_color(start, end, SIZE, mode),
            );
        }

        draw_text(
            &format!("Divisions: {}", divisions),
            30.0,
            HEIGHT - 24.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("Factor: {}", n),
            WIDTH / 2.0 - 64.0,
            HEIGHT - 24.0,
            24.0,
            WHITE,
        );
        draw_text(
            &format!("Color Mode: {}", mode),
            WIDTH * (2.0 / 3.0),
            HEIGHT - 24.0,
            24.0,
            WHITE,
        );

        next_frame().await;
    }
}

/// Get the coordinate of the i-th point.
fn get_coord(i: i32, angle: f32, radius: f32) -> (f32, f32) {
    (
        radius * f32::cos(angle * (i as f32)) + radius,
        radius * f32::sin(angle * (i as f32)) + radius,
    )
}

/// Generate the line color using the line coordinates and set color mode.
fn get_color(coord1: (f32, f32), coord2: (f32, f32), size: f32, mode: u8) -> Color {
    let c1: f32 = (coord1.0 + coord2.0 + coord1.1 + coord2.1) / (4.0 * size);
    let c2: f32 = (coord1.0 + coord2.0) / (4.0 * size);
    let c3: f32 = (coord1.1 + coord2.1) / (4.0 * size);

    match mode {
        1 => Color { r: c1, g: c2, b: c3, a: 1.0, },
        2 => Color { r: c1, g: c3, b: c2, a: 1.0, },
        3 => Color { r: c2, g: c1, b: c3, a: 1.0, },
        4 => Color { r: c2, g: c3, b: c1, a: 1.0, },
        5 => Color { r: c3, g: c1, b: c2, a: 1.0, },
        6 => Color { r: c3, g: c2, b: c1, a: 1.0, },
        7 => Color { r: c1, g: c2, b: c1, a: 1.0, },
        8 => Color { r: c2, g: c1, b: c2, a: 1.0, },
        9 => Color { r: c3, g: c1, b: c3, a: 1.0, },
        10 => Color { r: c1, g: c3, b: c1, a: 1.0, },
        11 => Color { r: c2, g: c3, b: c2, a: 1.0, },
        _ => Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0, },
    }
}

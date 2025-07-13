mod framebuffer;
mod line;

use framebuffer::Framebuffer;
use line::{dibujar_poligono, rellenar_poligono_scanline};
use raylib::prelude::*;

fn vec2(x: i32, y: i32) -> Vector2 {
    Vector2::new(x as f32, y as f32)
}

fn main() {
    //Definimos el framebuffer
    let mut fb = Framebuffer::new(800, 600, Color::BLACK);

    let poligonos = vec![
        (vec![
            vec2(165, 380), vec2(185, 360), vec2(180, 330), vec2(207, 345), vec2(233, 330),
            vec2(230, 360), vec2(250, 380), vec2(220, 385), vec2(205, 410), vec2(193, 383)
        ], Color::RED),

        (vec![vec2(321, 335), vec2(288, 286), vec2(339, 251), vec2(374, 302)], Color::GREEN),

        (vec![vec2(377, 249), vec2(411, 197), vec2(436, 249)], Color::BLUE),

        (vec![
            vec2(413, 177), vec2(448, 159), vec2(502, 88), vec2(553, 53), vec2(535, 36),
            vec2(676, 37), vec2(660, 52), vec2(750, 145), vec2(761, 179), vec2(672, 192),
            vec2(659, 214), vec2(615, 214), vec2(632, 230), vec2(580, 230), vec2(597, 215),
            vec2(552, 214), vec2(517, 144), vec2(466, 180)
        ], Color::YELLOW)
    ];

    for (puntos, color) in &poligonos {
        fb.set_current_color(*color);
        rellenar_poligono_scanline(&mut fb, puntos);

        fb.set_current_color(Color::WHITE);
        dibujar_poligono(&mut fb, puntos);
    }

    // Agujero dentro del polígono 4
    let agujero = vec![vec2(682, 175), vec2(708, 120), vec2(735, 148), vec2(739, 170)];

    fb.set_current_color(Color::BLACK);
    rellenar_poligono_scanline(&mut fb, &agujero);
    fb.set_current_color(Color::WHITE);
    dibujar_poligono(&mut fb, &agujero);

    fb.render_to_file("out.png");
    println!("Imagen exportada como out.png");
}

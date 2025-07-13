use raylib::prelude::*;
use crate::framebuffer::Framebuffer;

pub fn dibujar_linea_bresenham(fb: &mut Framebuffer, mut x0: i32, mut y0: i32, x1: i32, y1: i32) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        fb.set_pixel(x0 as u32, y0 as u32);

        if x0 == x1 && y0 == y1 {
            break;
        }

        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

pub fn dibujar_poligono(fb: &mut Framebuffer, puntos: &[Vector2]) {
    for i in 0..puntos.len() {
        let a = puntos[i];
        let b = puntos[(i + 1) % puntos.len()];
        dibujar_linea_bresenham(fb, a.x as i32, a.y as i32, b.x as i32, b.y as i32);
    }
}

pub fn rellenar_poligono_scanline(fb: &mut Framebuffer, puntos: &[Vector2]) {
    let mut edges = Vec::new();

    for i in 0..puntos.len() {
        let p1 = puntos[i];
        let p2 = puntos[(i + 1) % puntos.len()];

        if (p1.y - p2.y).abs() < f32::EPSILON {
            continue;
        }

        let (y_min, y_max, x_start, inv_slope) = if p1.y < p2.y {
            (p1.y, p2.y, p1.x, (p2.x - p1.x) / (p2.y - p1.y))
        } else {
            (p2.y, p1.y, p2.x, (p1.x - p2.x) / (p1.y - p2.y))
        };

        edges.push((y_min, y_max, x_start, inv_slope));
    }

    let min_y = puntos.iter().map(|p| p.y).fold(f32::INFINITY, f32::min).floor() as i32;
    let max_y = puntos.iter().map(|p| p.y).fold(f32::NEG_INFINITY, f32::max).ceil() as i32;

    for y in min_y..=max_y {
        let y_f = y as f32;
        let mut intersecciones = Vec::new();

        for &(y_min, y_max, x, inv_slope) in &edges {
            if y_f >= y_min && y_f < y_max {
                intersecciones.push(x + (y_f - y_min) * inv_slope);
            }
        }

        intersecciones.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for pair in intersecciones.chunks(2) {
            if pair.len() == 2 {
                let x_start = pair[0].ceil() as i32;
                let x_end = pair[1].floor() as i32;

                for x in x_start..=x_end {
                    fb.set_pixel(x as u32, y as u32);
                }
            }
        }
    }
}

use raylib::prelude::*;

pub struct GasGiantShader;

impl GasGiantShader {
    pub fn render_at(d: &mut RaylibMode3D<RaylibDrawHandle>, time: f32, pos: Vector3) {
        let rotation = time * 0.6;
        let layers = 80;

        for i in 0..layers {
            let v = i as f32 / layers as f32;
            let y = (v - 0.5) * 0.8;
            let wave = ((v * 15.0) + rotation).sin() * 0.5 + 0.5;

            let r = 0.9 - 0.3 * wave;
            let g = 0.7 - 0.25 * wave;
            let b = 0.5 - 0.15 * wave;
            let alpha = 0.9 - (v - 0.5).abs() * 0.4;

            let color = Color::color_from_normalized(Vector4::new(r, g, b, alpha));
            d.draw_sphere_ex(Vector3::new(pos.x, pos.y + y * 0.15, pos.z), 2.4, 64, 64, color);
        }

        let atmos = Color::color_from_normalized(Vector4::new(0.95, 0.85, 0.75, 0.15));
        d.draw_sphere(pos, 2.6, atmos);

        // Anillos horizontales
        for i in 0..4 {
            let radius = 3.2 + i as f32 * 0.3;
            let alpha = 0.25 - (i as f32 * 0.04);
            let ring_color = Color::LIGHTGRAY.alpha(alpha);
            d.draw_circle_3D(pos, radius, Vector3::up(), 0.0, ring_color);
        }
    }
}

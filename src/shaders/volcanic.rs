use raylib::prelude::*;

pub struct VolcanicShader;

impl VolcanicShader {
    pub fn render_at(d: &mut RaylibMode3D<RaylibDrawHandle>, time: f32, pos: Vector3) {
        for i in 0..40 {
            let f = i as f32 / 40.0;
            let noise = (time * 1.5 + f * 10.0).sin() * 0.5 + 0.5;
            let r = 0.5 + noise * 0.5;
            let g = 0.2 + noise * 0.2;
            let b = 0.05;
            let color = Color::color_from_normalized(Vector4::new(r, g, b, 1.0));
            d.draw_sphere_ex(Vector3::new(pos.x, pos.y + (f - 0.5) * 0.1, pos.z), 1.3, 48, 48, color);
        }

        // Brillo incandescente
        let glow = Color::color_from_normalized(Vector4::new(1.0, 0.4, 0.1, 0.2));
        d.draw_sphere(pos, 1.4, glow);
    }
}

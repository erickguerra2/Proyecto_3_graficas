use raylib::prelude::*;

pub struct IcyShader;

impl IcyShader {
    pub fn render_at(d: &mut RaylibMode3D<RaylibDrawHandle>, time: f32, pos: Vector3) {
        for i in 0..40 {
            let f = i as f32 / 40.0;
            let noise = ((time * 0.7) + f * 5.0).cos() * 0.5 + 0.5;
            let r = 0.7 - noise * 0.3;
            let g = 0.8 - noise * 0.2;
            let b = 1.0 - noise * 0.1;
            let color = Color::color_from_normalized(Vector4::new(r, g, b, 1.0));
            d.draw_sphere_ex(Vector3::new(pos.x, pos.y + (f - 0.5) * 0.08, pos.z), 1.4, 48, 48, color);
        }

        // Halo azulado brillante
        let halo = Color::color_from_normalized(Vector4::new(0.6, 0.9, 1.0, 0.25));
        d.draw_sphere(pos, 1.6, halo);
    }
}

use raylib::prelude::*;

pub struct SunShader;

impl SunShader {
    pub fn render(d: &mut RaylibMode3D<RaylibDrawHandle>, time: f32) {
        let pulsation = (time * 3.0).sin() * 0.5 + 0.5;

        let core = Vector3::new(1.0, 0.95, 0.8);
        let mid = Vector3::new(1.0, 0.8, 0.2);
        let outer = Vector3::new(1.0, 0.5, 0.0);

        for i in 0..35 {
            let f = i as f32 / 35.0;
            let radius = 2.5 + f * 0.18;
            let flicker = 1.0 + (time * 6.0 + f * 10.0).sin() * 0.1;
            let color_vec = core * (1.0 - f) + mid * (f * 0.8) + outer * f * 0.7;
            let color = Color::color_from_normalized(Vector4::new(
                (color_vec.x * flicker).clamp(0.0, 1.0),
                (color_vec.y * flicker).clamp(0.0, 1.0),
                (color_vec.z * flicker).clamp(0.0, 1.0),
                1.0,
            ));
            d.draw_sphere(Vector3::zero(), radius, color);
        }

        let halo = Color::color_from_normalized(Vector4::new(1.0, 0.9, 0.4, 0.25));
        d.draw_sphere(Vector3::zero(), 3.3, halo);
    }
}

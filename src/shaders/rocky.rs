use raylib::prelude::*;

pub struct RockyShader;

impl RockyShader {
    pub fn render_at(d: &mut RaylibMode3D<RaylibDrawHandle>, time: f32, position: Vector3) {
        let base_radius = 1.0;

        for i in 0..8 {
            let noise = ((time * 0.6 + i as f32 * 1.3).sin()
                + (time * 1.1 + i as f32 * 2.1).cos())
                * 0.5
                + 0.5;
            let height = base_radius + noise * 0.03;
            let rocky_color = Vector3::new(
                0.35 + noise * 0.15,
                0.25 + noise * 0.12,
                0.1 + noise * 0.08,
            );
            let c = Color::color_from_normalized(Vector4::new(
                rocky_color.x,
                rocky_color.y,
                rocky_color.z,
                1.0,
            ));
            d.draw_sphere(position, height, c);
        }

        // Atmósfera más visible
        let halo_color = Color::color_from_normalized(Vector4::new(0.4, 0.6, 1.0, 0.25));
        d.draw_sphere(position, base_radius * 1.07, halo_color);
    }
}

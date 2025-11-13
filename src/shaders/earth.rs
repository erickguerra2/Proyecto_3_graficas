use raylib::prelude::*;

pub struct EarthShader;

impl EarthShader {
    pub fn render_at(d: &mut RaylibMode3D<RaylibDrawHandle>, time: f32, pos: Vector3) {
        let radius = 1.1;
        let rotation = time * 0.8;

        // La Tierra se simula con múltiples capas delgadas que mezclan color según ruido esférico
        let layers = 48;
        for i in 0..layers {
            let fi = i as f32 / layers as f32;
            let theta = fi * std::f32::consts::PI; // latitud
            let y = (theta.cos() - 0.5) * 0.4;

            // Pequeñas variaciones en longitudes para ruido natural
            for j in 0..36 {
                let fj = j as f32 / 36.0;
                let phi = fj * std::f32::consts::TAU + rotation;

                // Coordenadas esféricas → ruido pseudoaleatorio estable
                let nx = phi.cos() * theta.sin();
                let ny = theta.cos();
                let nz = phi.sin() * theta.sin();
                let noise = (nx * 3.1 + ny * 4.7 + nz * 5.3).sin() * 0.5 + 0.5;

                // Oceanos y continentes
                let (r, g, b) = if noise > 0.55 {
                    // tierra
                    (
                        0.2 + noise * 0.3,
                        0.4 + noise * 0.3,
                        0.1 + noise * 0.05,
                    )
                } else {
                    // océano
                    (
                        0.0 + noise * 0.1,
                        0.25 + noise * 0.2,
                        0.5 + noise * 0.5,
                    )
                };

                let color = Color::color_from_normalized(Vector4::new(r, g, b, 1.0));

                // Coordenadas 3D para ubicar fragmentos visuales
                let px = pos.x + radius * nx;
                let py = pos.y + radius * ny;
                let pz = pos.z + radius * nz;

                d.draw_sphere(Vector3::new(px, py, pz), 0.05, color);
            }
        }

        // 🌫️ Atmósfera azulada
        let atmosphere = Color::color_from_normalized(Vector4::new(0.4, 0.7, 1.0, 0.18));
        d.draw_sphere(pos, radius * 1.1, atmosphere);
    }
}

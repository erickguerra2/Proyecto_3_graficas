mod shaders;

use raylib::prelude::*;
use shaders::{SunShader, EarthShader, VolcanicShader, IcyShader, GasGiantShader};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(1400, 900)
        .title("Sistema Solar Avanzado - Rust + Raylib 5.5.1")
        .build();

    let camera = Camera3D::perspective(
        Vector3::new(0.0, 16.0, 35.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        45.0,
    );

    rl.set_target_fps(60);
    let mut time = 0.0_f32;

    while !rl.window_should_close() {
        time += rl.get_frame_time();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::new(5, 5, 15, 255));
        let mut mode3d = d.begin_mode3D(camera);

        // ☀️ Sol
        SunShader::render(&mut mode3d, time);

        // 🌎 Tierra
        let earth_orbit_radius = 8.0;
        let earth_angle = time * 0.8;
        let earth_pos = Vector3::new(
            earth_orbit_radius * earth_angle.cos(),
            0.0,
            earth_orbit_radius * earth_angle.sin(),
        );
        EarthShader::render_at(&mut mode3d, time, earth_pos);

        // 🌋 Planeta volcánico
        let volc_orbit_radius = 12.0;
        let volc_angle = time * 0.5;
        let volc_pos = Vector3::new(
            volc_orbit_radius * volc_angle.cos(),
            0.0,
            volc_orbit_radius * volc_angle.sin(),
        );
        VolcanicShader::render_at(&mut mode3d, time, volc_pos);

        // ❄️ Planeta helado
        let icy_orbit_radius = 16.0;
        let icy_angle = time * 0.35;
        let icy_pos = Vector3::new(
            icy_orbit_radius * icy_angle.cos(),
            0.0,
            icy_orbit_radius * icy_angle.sin(),
        );
        IcyShader::render_at(&mut mode3d, time, icy_pos);

        // 🪐 Gigante gaseoso
        let gas_orbit_radius = 21.0;
        let gas_angle = time * 0.25;
        let gas_pos = Vector3::new(
            gas_orbit_radius * gas_angle.cos(),
            0.0,
            gas_orbit_radius * gas_angle.sin(),
        );
        GasGiantShader::render_at(&mut mode3d, time, gas_pos);

        // 🌙 Luna orbitando la Tierra
        let moon_angle = time * 2.0;
        let moon_orbit_radius = 1.5;
        let moon_pos = earth_pos
            + Vector3::new(
                moon_orbit_radius * moon_angle.cos(),
                0.0,
                moon_orbit_radius * moon_angle.sin(),
            );
        mode3d.draw_sphere(moon_pos, 0.4, Color::LIGHTGRAY);

        drop(mode3d);

        d.draw_text("Sistema Solar Procedural - Rust + Raylib", 10, 10, 20, Color::RAYWHITE);
        d.draw_text("Tierra, volcánico, helado y gaseoso con anillos", 10, 40, 20, Color::LIGHTGRAY);
    }
}

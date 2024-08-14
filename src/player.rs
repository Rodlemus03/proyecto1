use nalgebra_glm::Vec2;
use minifb::{Window, Key};
use std::f32::consts::PI;

pub struct Player {
    pub pos: Vec2,
    pub a: f32,
    pub fov: f32,
    pub last_mouse_x: f32, // Agrega esto para rastrear la última posición del mouse
}

pub fn eventos_jugador(window: &Window, player: &mut Player) {
    const WALK_SPEED: f32 = 5.0;
    const RUN_SPEED: f32 = 15.0;  // Velocidad al trotar
    const ROTATION_SPEED: f32 = PI / 35.0;
    const MOUSE_SENSITIVITY: f32 = 0.001; // Sensibilidad del mouse

    let move_speed = if window.is_key_down(Key::Space) {
        RUN_SPEED
    } else {
        WALK_SPEED
    };

    // Obtener la posición del mouse
    if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Discard) {
        let (mouse_x, _) = mouse_pos;

        // Calcular el cambio en la posición del mouse
        let mouse_dx = mouse_x as f32 - player.last_mouse_x;
        player.last_mouse_x = mouse_x as f32;

        // Ajustar la rotación del jugador según el movimiento horizontal del mouse
        player.a -= mouse_dx * MOUSE_SENSITIVITY;

        if window.is_key_down(Key::Left) || window.is_key_down(Key::A) {
            player.a -= ROTATION_SPEED;
        }
        if window.is_key_down(Key::Right) || window.is_key_down(Key::D) {
            player.a += ROTATION_SPEED;
        }
        if window.is_key_down(Key::Up) || window.is_key_down(Key::W) {
            let direction = Vec2::new(player.a.cos(), player.a.sin());
            player.pos += direction * move_speed;
        }
        if window.is_key_down(Key::Down) || window.is_key_down(Key::S) {
            let direction = Vec2::new(player.a.cos(), player.a.sin());
            player.pos -= direction * move_speed;
        }
    }
}

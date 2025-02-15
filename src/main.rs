
use raylib::prelude::*;

fn main() {

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Hello, World")
        .vsync()
        .build();

       let player = Vector2::new(500.0,500.0);
       
       let tex = rl.load_texture(&thread,"assests/img/hammerForge.png").unwrap();

    while !rl.window_should_close() {
        let mut r = rl.begin_drawing(&thread);
        r.clear_background(Color::BLACK);
        r.draw_fps(100, 100);
        r.draw_texture_v(&tex, player, Color::WHITE);
        r.clear_background(Color::WHITE);
        r.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }

}

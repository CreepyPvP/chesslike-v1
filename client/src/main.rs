use raylib::prelude::*;

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(640, 480)
        .title("Hello, World")
        .build();
     
    let pos = Vector2{x: rl.get_screen_width() as f32 / 2.0, y: rl.get_screen_height() as f32 / 2.0 };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.draw_circle_v(pos, 50.0, Color::MAROON);

        if d.is_mouse_button_pressed(MouseButton::MOUSE_LEFT_BUTTON) {
            println!("left click pressed");
        }
         
        d.clear_background(Color::WHITE);
        d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);
    }
}

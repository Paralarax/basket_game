
use macroquad::prelude::*;
use macroquad::audio::*;

#[macroquad::main("Basket")]
async fn main() {
    set_pc_assets_folder("assets");

    let texture: Texture2D = load_texture("basket.png").await.expect("Could not find texture!");

    let game_loop_sound = load_sound("loop_2.wav").await.expect("Could not load sound!");

    let mut x = screen_width() / 2.0 - texture.width() / 2.0;
    let mut y = screen_height() /2.0 - texture.height() / 2.0;

    play_sound(game_loop_sound, PlaySoundParams { looped: true, volume: 1.0 });

    loop {
        clear_background(BLACK);

        if is_key_down(KeyCode::W) {
            y -= 5.0;
        }

        if is_key_down(KeyCode::S) {
            y += 5.0;
        }

        if is_key_down(KeyCode::A) {
            x -= 5.0;
        }

        if is_key_down(KeyCode::D) {
            x += 5.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        
        draw_text("Hello, world!", 5.0, 15.0, 24.0, WHITE);
        draw_texture(texture, x, y, WHITE);

        next_frame().await;
    }
}


use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::audio::*;

fn basket() -> Conf {
    Conf { window_title: "Basket".to_string(), window_width: 800, window_height: 600, fullscreen: false, window_resizable: false, ..Default::default() }
}

#[macroquad::main(basket)]
async fn main() {
    set_pc_assets_folder("assets");

    let basket_texture: Texture2D = load_texture("basket.png").await.expect("Could not find texture!");
    let apple_texture: Texture2D = load_texture("apple.png").await.expect("Could not find texture!");
    
    // let image = load_image("apple.png").await.unwrap();
    // image.sub_image(rect)

    let game_loop_sound = load_sound("loop_2.wav").await.expect("Could not load sound!");

    let mut score: u32 = 0;

    let mut x = screen_width() / 2.0 - basket_texture.width() / 2.0;
    //let mut y = screen_height() /2.0 - texture.height() / 2.0;
    let mut y = 450.0;

    let mut apple_y = 0.0 + apple_texture.height();
    let mut apple_x = screen_width() / 2.0 - apple_texture.width() / 2.0;

    play_sound(game_loop_sound, PlaySoundParams { looped: true, volume: 1.0 });

    loop {
        clear_background(BLACK);
        warn!("{}", y);

        if is_key_down(KeyCode::A) && x != 0.0 {
            x -= 5.0;
        }

        if is_key_down(KeyCode::D) && x != 560.0 {
            x += 5.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        
        draw_text(&format!("Score: {}", score), 5.0, 15.0, 32.0, WHITE);
        draw_texture(basket_texture, x, y, WHITE);
        draw_texture(apple_texture, apple_x, apple_y, WHITE);

        next_frame().await;
    }
}

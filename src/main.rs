
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::audio::*;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;

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

    // let sound_file = File::open("./assets/audio.ogg").unwrap();
    // let mut sound_bytes_reader = BufReader::new(sound_file);
    // let mut sound_bytes: Vec<u8> = vec![];

    // sound_bytes_reader.read_to_end(&mut sound_bytes).unwrap();


    // let sound = load_sound_from_bytes(&sound_bytes).await.unwrap();
    let game_loop_sound = load_sound("audio.ogg").await.expect("Could not load sound!");

    let mut score: u32 = 0;

    warn!("apple height: {}", apple_texture.height());
    warn!("heihgt: {}", screen_height());
    
    let mut x = screen_width() / 2.0 - basket_texture.width() / 2.0;
    //let mut y = screen_height() /2.0 - texture.height() / 2.0;
    let mut y = 450.0;


    let mut apple_y = 0.0;
    let mut apple_x = screen_width() / 2.0 - apple_texture.width() / 2.0;

    let mut basket_rect = Rect::new(x, y, basket_texture.width(), basket_texture.height());
    let mut apple_rect = Rect::new(apple_x, apple_y, apple_texture.width(), apple_texture.height());

    let mut draw_apple = true;

    play_sound(game_loop_sound, PlaySoundParams { looped: true, volume: 1.0 });

    loop {
        clear_background(BLACK);
        //warn!("{}", y);

        warn!("basket react: x: {}, y: {}", basket_rect.x, basket_rect.y);
        warn!("apple react: x: {}, y: {}", apple_rect.x, apple_rect.y);

        if is_key_down(KeyCode::A) && x != 0.0 {
            x -= 5.0;
        }

        if is_key_down(KeyCode::D) && x != 560.0 {
            x += 5.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        basket_rect.x = x;

        apple_y += 1.0;
        apple_rect.y = apple_y;

        if basket_rect.overlaps(&apple_rect) && draw_apple {
            draw_apple = false;
            score += 1;
        }

        if draw_apple {
            draw_texture(apple_texture, apple_x, apple_y, WHITE);
        }
        
        draw_text(&format!("Score: {}", score), 5.0, 15.0, 32.0, WHITE);
        draw_texture(basket_texture, x, y, WHITE);


        next_frame().await;
    }
}

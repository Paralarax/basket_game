
use macroquad::miniquad::conf::Icon;
use macroquad::prelude::*;
use macroquad::audio::*;

fn basket() -> Conf {
    Conf { window_title: "Basket".to_string(), window_width: 800, window_height: 600, fullscreen: false, window_resizable: false, ..Default::default() }
}


#[macroquad::main(basket)]
async fn main() {
    set_pc_assets_folder("assets");

    let font = load_ttf_font("retro.ttf").await.expect("Could not load the font!");
    let text_dimensions = measure_text("LOADING", Some(font), 52, 1.0);
    warn!("{:?}", text_dimensions);

    // clear_background(BLACK);
    // next_frame().await;

    let background_texture: Texture2D = load_texture("tree.png").await.expect("Could not load the background texture!");
    

    let loading_text_settings = TextParams { font: font, font_size: 52, color: WHITE, ..Default::default() };
    let score_text_settings = TextParams { font: font, font_size: 22, color: WHITE, ..Default::default() };

    clear_background(BLACK);
    draw_texture_ex(background_texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(800.0, 600.0)), ..Default::default()});
    draw_text_ex("LOADING", screen_width() / 2.0 - text_dimensions.width / 2.0, screen_height() / 2.0 - text_dimensions.height / 2.0, loading_text_settings);
    next_frame().await;

    let basket_texture: Texture2D = load_texture("basket.png").await.expect("Could not find texture!");
    let apple_texture: Texture2D = load_texture("apple.png").await.expect("Could not find texture!");
    
    
    //let texture = Texture2D::from_image(&image);
    //background_texture.update(&image);

    //background_texture.update(&image);

    // let sound_file = File::open("./assets/audio.ogg").unwrap();
    // let mut sound_bytes_reader = BufReader::new(sound_file);
    // let mut sound_bytes: Vec<u8> = vec![];

    let mut apples_collection: Vec<Rect> = vec![];
    apples_collection.push(Rect { x: screen_width() / 2.0 - apple_texture.width() / 2.0, y: 0.0, w: apple_texture.width(), h: apple_texture.height() });
    apples_collection.push(Rect { x: 10.0, y: 0.0, w: apple_texture.width(), h: apple_texture.height() });
    // sound_bytes_reader.read_to_end(&mut sound_bytes).unwrap();


    // let sound = load_sound_from_bytes(&sound_bytes).await.unwrap();
    let game_loop_sound = load_sound("loop.ogg").await.expect("Could not load sound!");
    let apple_collected_sound = load_sound("apple_collected.ogg").await.expect("Could not loud sound!");

    let mut score: u32 = 0;

    warn!("apple height: {}", apple_texture.height());
    warn!("heihgt: {}", screen_height());
    
    let mut x = screen_width() / 2.0 - basket_texture.width() / 2.0;
    //let mut y = screen_height() /2.0 - texture.height() / 2.0;
    let mut y = 490.0;


    let mut apple_y = 0.0;
    let mut apple_x = screen_width() / 2.0 - apple_texture.width() / 2.0;

    let mut basket_rect = Rect::new(x + 40.0, y + 5.0, basket_texture.width() - 80.0, 14.0);
    let mut apple_rect = Rect::new(apple_x, apple_y, apple_texture.width(), apple_texture.height());

    let mut draw_apple = true;

    play_sound(game_loop_sound, PlaySoundParams { looped: true, volume: 1.0 });

    loop {
        clear_background(BLACK);
        //warn!("{}", y);

        // warn!("basket react: x: {}, y: {}", basket_rect.x, basket_rect.y);
        // warn!("apple react: x: {}, y: {}", apple_rect.x, apple_rect.y);

        if is_key_down(KeyCode::A) && x != 0.0 {
            x -= 5.0;
        }

        if is_key_down(KeyCode::D) && x != 560.0 {
            x += 5.0;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        basket_rect.x = x + 40.0;

        // apple_y += 1.0;
        // apple_rect.y = apple_y;

        // if basket_rect.overlaps(&apple_rect) && draw_apple {
        //     draw_apple = false;
        //     score += 1;
        // }

        for apple in apples_collection.iter_mut() {
            apple.y += 1.0;
        }


        apples_collection.retain(|&apple| {
            let collided = apple.overlaps(&basket_rect);

            if collided {
                score += 1;
                play_sound_once(apple_collected_sound);
            }

            !collided
        });

        //draw_texture(texture, 0.0, 0.0, WHITE);
        draw_texture_ex(background_texture, 0.0, 0.0, WHITE, DrawTextureParams { dest_size: Some(Vec2::new(800.0, 600.0)), ..Default::default()});

        // if draw_apple {
        //     draw_texture(apple_texture, apple_x, apple_y, WHITE);
        // }

        for apple in apples_collection.iter() {
            draw_texture(apple_texture, apple.x, apple.y, WHITE);
        }
        
        draw_text_ex(&format!("SCORE: {}", score), 5.0, 25.0, score_text_settings);
        draw_texture(basket_texture, x, y, WHITE);
        draw_rectangle(basket_rect.x, basket_rect.y, basket_texture.width() - 80.0, 14.0, ORANGE);
        //draw_rectangle(apple_rect.x, apple_rect.y, apple_texture.width(), apple_rect.h, GREEN);

        next_frame().await;
    }
}

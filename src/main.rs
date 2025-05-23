/*
By: <Your Name Here>
Date: 2025-05-16
Program Details: <Program Description Here>
*/

mod modules;

use crate::modules::collision::check_collision;
 use crate::modules::preload_image::TextureManager;
use crate::modules::preload_image::LoadingScreenOptions;
use crate::modules::still_image::StillImage;
use macroquad::prelude::*;
use modules::collision;
/// Set up window settings before the app runs
fn window_conf() -> Conf {
    Conf {
        window_title: "maze".to_string(),
        window_width: 1024,
        window_height: 768,
        fullscreen: false,
        high_dpi: true,
        window_resizable: true,
        sample_count: 4, // MSAA: makes shapes look smoother
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
     let loading_options = LoadingScreenOptions {
        title: Some("MAZE".to_string()),
        background_color: BLACK,
        bar_fill_color: Color::new(1.0, 0.0, 0.0, 1.0), // Brighter green
        // Use default values for other options
        ..Default::default()
    };
    let maze = StillImage::new("assets/maze.png", 500.0, 500.0, 200.0, 60.0, true, 1.0).await;
    let mut character = StillImage::new("assets/amongus.png", 35.0, 35.0, 200.0, 20.0, true, 1.0).await;

let tm = TextureManager::new();
let all_assets = ["assets/maze.png", "assets/amongus.png"];

  tm.preload_with_loading_screen(&all_assets, Some(loading_options)).await;
    let speed = 100.0;



    loop {
    
         let old_spotx=character.pos().x;
         let old_spoty=character.pos().y;
         
        clear_background(BLACK);
        if is_key_down(KeyCode::Right) {
            character.set_position(vec2(character.pos().x + speed*get_frame_time(), character.pos().y));
            if check_collision(&character, &maze, 1) {
                character.set_position(vec2(old_spotx, character.pos().y));
            }
        }
         if is_key_down(KeyCode::Left) {
            character.set_position(vec2(character.pos().x - speed*get_frame_time(), character.pos().y));
            if check_collision(&character, &maze, 1) {
                character.set_position(vec2(old_spotx, character.pos().y));
            }
        }
         if is_key_down(KeyCode::Down) {
            character.set_position(vec2(character.pos().x, character.pos().y+speed*get_frame_time()));
            if check_collision(&character, &maze, 1) {
                character.set_position(vec2(character.pos().x, old_spoty));
            }
        }
         if is_key_down(KeyCode::Up) {
            character.set_position(vec2(character.pos().x, character.pos().y-speed*get_frame_time()));
            if check_collision(&character, &maze, 1) {
                character.set_position(vec2(character.pos().x, old_spoty));
            }
        }

       
        

draw_circle(character.pos().x+11.0, character.pos().y+15.0,  60.0, WHITE);
character.draw();        
maze.draw();
        next_frame().await;
    }
}

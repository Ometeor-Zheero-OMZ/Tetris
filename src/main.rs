extern crate raylib;

use game::game::Game;
use raylib::prelude::*;
use raylib::core::audio::RaylibAudio;

mod game;
mod stage;
mod tetris;
mod utils;
mod rand;

const WIDTH: i32 = 500;
const HEIGHT: i32 = 620;
const TITLE: &'static str = "Tetris"; 

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title(TITLE)
        .vsync()
        .build();

    rl.set_target_fps(60);

    let font = rl.load_font(&thread, "fonts/monogram.ttf").expect("Failed to load font");

    // オーディオデバイスの初期化
    let audio = RaylibAudio::init_audio_device()
        .expect("Failed to initialize audio device");
    let mut game = Game::new(&audio);

    let mut last_update_time = 0.0;

    while !rl.window_should_close() {
        game.music.update_stream();
        game.handle_input(&mut rl);

        let current_time = rl.get_time();
        if current_time - last_update_time >= 0.2 {
            game.move_block_down();
            last_update_time = current_time;
        }

        let score_text = format!("{}", game.score);
        let text_size  = rl.measure_text(&score_text, 38);
    
        let mut rd = rl.begin_drawing(&thread);
        rd.clear_background(Color::DARKBLUE);

        rd.draw_text_ex(&font, "Score", Vector2::new(365.0, 15.0), 38.0, 2.0, Color::WHITE);
        rd.draw_text_ex(&font, "Next", Vector2::new(370.0, 175.0), 38.0, 2.0, Color::WHITE);

        if game.game_over {
            rd.draw_text_ex(&font, "GAME OVER", Vector2::new(320.0, 450.0), 38.0, 2.0, Color::WHITE);
        }

        rd.draw_rectangle_rounded(Rectangle { x: 320.0, y: 55.0, width: 170.0, height: 60.0 },0.3, 6, Color::LIGHTBLUE);
        rd.draw_text_ex(&font, &score_text, Vector2::new(320.0 + (170.0 - text_size as f32) / 2.0, 65.0), 38.0, 2.0, Color::WHITE);
        rd.draw_rectangle_rounded(Rectangle { x: 320.0, y: 215.0, width: 170.0, height: 180.0 },0.3, 6, Color::LIGHTBLUE);

        game.draw(&mut rd);
    }
}
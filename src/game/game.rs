use std::time::SystemTime;

use rand::seq::SliceRandom;
use raylib::prelude::*;

use crate::{rand::SplitMixXoshiro256Rng, stage::grid::Grid, tetris::{blocks::*, tetris_block::TetrisBlock}};

pub struct Game<'a> {
    pub grid: Grid,
    pub blocks: Vec<TetrisBlock>,
    pub current_block: TetrisBlock,
    pub next_block: TetrisBlock,
    pub game_over: bool,
    pub score: u32,
    pub music: Music<'a>,
    pub rotate_sound: Sound<'a>,
    pub clear_sound: Sound<'a>,
}

impl<'a> Game<'a> {
    pub fn new(audio: &'a RaylibAudio) -> Self {
        let mut music = audio.new_music("sounds/A-Type.mp3").expect("Failed to load A-Type.mp3");
        let rotate_sound = audio.new_sound("sounds/rotate.mp3").expect("Failed to load rotate.mp3");
        let clear_sound = audio.new_sound("sounds/clear.mp3").expect("Failed to load clear.mp3");

        music.play_stream();

        Game {
            grid: Grid::new(),
            blocks: Game::get_all_blocks(),
            current_block: Game::get_random_block(),
            next_block: Game::get_random_block(),
            game_over: false,
            score: 0,
            music,
            rotate_sound,
            clear_sound,
        }
    }

    pub fn handle_input(&mut self, rl: &mut RaylibHandle) {
        // キー入力の取得 (そのままOption<KeyboardKey>として扱う)
        let key_pressed = rl.get_key_pressed();
    
        // ゲームオーバー処理
        if self.game_over {
            if let Some(key) = key_pressed { // Optionを直接利用
                if key != KeyboardKey::KEY_NULL {
                    self.music.stop_stream();
                    self.game_over = false;
                    self.reset();
                }
            }
            return; // ゲームオーバー後は処理終了
        }

        let key_down = rl.is_key_down(KeyboardKey::KEY_DOWN);

        // move down continually while pressing ↓ button
        if key_down {
            self.move_block_down();
            self.update_score(0, 1);
        }
    
        // 入力処理
        if let Some(key) = key_pressed { // Optionを直接利用
            match key {
                KeyboardKey::KEY_LEFT => self.move_block_left(),
                KeyboardKey::KEY_RIGHT => self.move_block_right(),
                // KeyboardKey::KEY_DOWN => {
                //     self.move_block_down();
                //     self.update_score(0, 1);
                // },
                KeyboardKey::KEY_UP => self.move_block_up(),
                KeyboardKey::KEY_R => self.rotate_block(),
                _ => {}
            }
        }
    }

    pub fn draw(&self, rd: &mut RaylibDrawHandle) {
        self.grid.draw(rd);
        self.current_block.draw(11, 11, rd);

        match self.next_block.id {
            3 => self.next_block.draw( 255, 290, rd),
            4 => self.next_block.draw( 255, 280, rd),
            _ => self.next_block.draw( 270, 270, rd)
        }
    }

    pub fn move_block_left(&mut self) {
        if !self.game_over {
            self.current_block.move_block(0, -1);
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.move_block(0, 1);
            }
        }
    }

    pub fn move_block_right(&mut self) {
        if !self.game_over {
            self.current_block.move_block(0, 1);
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.move_block(0, -1);
            }
        }
    }

    pub fn move_block_down(&mut self) {
        if !self.game_over {
            self.current_block.move_block(1, 0);

            if self.is_block_outside() || !self.block_fits() {
                self.current_block.move_block(-1, 0);

                if self.current_block.get_cell_positions().iter().any(|tile| tile.row < 0) {
                    return;
                }

                self.lock_block();
            }
        }
    }

    pub fn move_block_up(&mut self) {
        if !self.game_over {
            loop {
                self.current_block.move_block(1, 0);
                if self.is_block_outside() || !self.block_fits() {
                    self.current_block.move_block(-1, 0);
                    break;
                }
            }
            self.lock_block();
        }
    }

    pub fn rotate_block(&mut self) {
        if !self.game_over {
            self.current_block.rotate();
            if self.is_block_outside() || !self.block_fits() {
                self.current_block.undo_rotation();
            } else {
                self.rotate_sound.play();
            }
        }
    }

    pub fn lock_block(&mut self) {
        let tiles = self.current_block.get_cell_positions();

        for tile in tiles {
            let row = tile.row;
            let column = tile.column;

            // 負の座標
            if row < 0 || column < 0 {
                eprintln!("Invalid tile position during lock_block: {:?}", tile);
                continue;
            }

            let row = row as usize;
            let column = column as usize;

            self.grid.grid[row][column] = self.current_block.id as i32;
        }

        self.current_block = self.next_block.clone();
        if !self.block_fits() {
            self.game_over = true;
        }

        self.next_block = Game::get_random_block();
        let rows_cleared = self.grid.clear_full_rows();
        if rows_cleared > 0 {
            self.clear_sound.play();
            self.update_score(rows_cleared.try_into().unwrap(), 0);
        }
    }

    pub fn is_block_outside(&self) -> bool {
        let tiles = self.current_block.get_cell_positions();

        tiles.iter().any(|tile| {
            let row = tile.row;
            let column = tile.column;

            if row < 0 || column < 0 {
                eprintln!("Block outside grid: {:?}", tile);
                return true;
            }

            let row = row as usize;
            let column = column as usize;

            self.grid.is_cell_outside(row, column)
        })
    }

    pub fn block_fits(&self) -> bool {
        let tiles = self.current_block.get_cell_positions();
        let rows = self.grid.rows();
        let columns = self.grid.columns();

        tiles.iter().all(|tile| {
            let row = tile.row;
            let column = tile.column;
    
            // 負の座標は「一時的な許容」として扱う
            if row < 0 {
                return true; // 上部から降ってくる途中は許容する
            }
    
            if row >= rows as i32 || column < 0 || column >= columns as i32 {
                return false; // グリッド外に完全に出た場合は不許容
            }
    
            let row = row as usize;
            let column = column as usize;
    
            self.grid.is_cell_empty(row, column)
        })
    }

    pub fn update_score(&mut self, lines_cleared: u32, move_down_points: u32) {
        self.score += match lines_cleared {
            1 => 100,
            2 => 300,
            3 => 500,
            _ => 0,
        };
        self.score += move_down_points;
    }

    fn get_random_block() -> TetrisBlock {
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .expect("Generate seed with UNIX_EPOCH")
            .as_nanos() as u64;
        let mut rng = SplitMixXoshiro256Rng::new(seed);
    
        // 各ブロックのインスタンスを作成
        let block_types = vec![
            BlockType::IBlock(IBlock::new()),
            BlockType::JBlock(JBlock::new()),
            BlockType::LBlock(LBlock::new()),
            BlockType::OBlock(OBlock::new()),
            BlockType::SBlock(SBlock::new()),
            BlockType::TBlock(TBlock::new()),
            BlockType::ZBlock(ZBlock::new()),
        ];
    
        // ランダムに1つのブロックを選択
        let block_type = block_types.choose(&mut rng).unwrap(); // クローンを使ってインスタンスを取得
        block_type.to_tetris_block() // 新規作成したブロックを返す
    }

    fn get_all_blocks() -> Vec<TetrisBlock> {
        let block_types = vec![
            BlockType::IBlock(IBlock::new()),
            BlockType::JBlock(JBlock::new()),
            BlockType::LBlock(LBlock::new()),
            BlockType::OBlock(OBlock::new()),
            BlockType::SBlock(SBlock::new()),
            BlockType::TBlock(TBlock::new()),
            BlockType::ZBlock(ZBlock::new()),
        ];
        block_types.into_iter().map(|block| block.to_tetris_block()).collect()
    }

    pub fn reset(&mut self) {
        self.grid.init();
        self.music.play_stream();
        self.blocks = Game::get_all_blocks();
        self.current_block = Game::get_random_block();
        self.next_block = Game::get_random_block();
        self.score = 0;
    }

    // pub fn spawn_block(&mut self) {
    //     self.current_block.position = Position { row: 0, column: 3 }; // 初期位置を確保
    // }
}
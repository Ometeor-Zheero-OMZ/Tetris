use raylib::prelude::*;

pub const DARK_GREY: Color = Color::new(26, 31, 40, 255);
pub const GREEN: Color = Color::new(47, 230, 23, 255);
pub const RED: Color = Color::new(232, 18, 18, 255);
pub const ORANGE: Color = Color::new(226, 116, 17, 255);
pub const YELLOW: Color = Color::new(237, 234, 4, 255);
pub const PURPLE: Color = Color::new(166, 0, 247, 255);
pub const CYAN: Color = Color::new(21, 204, 209, 255);
pub const BLUE: Color = Color::new(13, 64, 216, 255);
pub const LIGHT_BLUE: Color = Color::new(59, 85, 162, 255);
pub const DARK_BLUE: Color = Color::new(44, 44, 127, 255);

pub fn get_cell_colors() -> Vec<Color> {
    vec![DARK_GREY, GREEN, RED, ORANGE, YELLOW, PURPLE, CYAN, BLUE, LIGHT_BLUE, DARK_BLUE]
}
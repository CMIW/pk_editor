use include_dir::{include_dir, Dir};

pub static PROJECT_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/assets");
const SCALE: f32 = 0.6;
pub const WINDOW_WIDTH: f32 = 1920.0 * SCALE;
pub const WINDOW_HEIGHT: f32 = 1080.0 * SCALE;

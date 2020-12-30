#[derive(Debug, Clone)]
pub struct Config {
    pub clear_color: (f32, f32, f32, f32),
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clear_color: (0.7, 0.7, 0.7, 1.0),
        }
    }
}

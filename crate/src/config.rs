#[derive(Debug, Clone)]
pub struct Config {
    pub clear_color: (f32, f32, f32, f32),
    pub color_picker: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clear_color: (0.7, 0.7, 0.7, 1.0),
            color_picker: true,
        }
    }
}

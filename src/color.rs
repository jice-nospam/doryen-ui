#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ColorCode {
    Background,
    Foreground,
    ButtonBackground,
    ButtonBackgroundHover,
    ButtonBackgroundFocus,
    Text,
}

impl Into<usize> for ColorCode {
    fn into(self) -> usize {
        match self {
            ColorCode::Background => COLOR_BACKGROUND,
            ColorCode::Foreground => COLOR_FOREGROUND,
            ColorCode::ButtonBackground => COLOR_BUTTON_BACKGROUND,
            ColorCode::ButtonBackgroundHover => COLOR_BUTTON_BACKGROUND_HOVER,
            ColorCode::ButtonBackgroundFocus => COLOR_BUTTON_BACKGROUND_FOCUS,
            ColorCode::Text => COLOR_TEXT,
        }
    }
}

pub type Color = (u8, u8, u8, u8);
const COLOR_BACKGROUND: usize = 0;
const COLOR_FOREGROUND: usize = 1;
const COLOR_BUTTON_BACKGROUND: usize = 2;
const COLOR_BUTTON_BACKGROUND_HOVER: usize = 3;
const COLOR_BUTTON_BACKGROUND_FOCUS: usize = 4;
const COLOR_TEXT: usize = 5;
const COLOR_COUNT: usize = 6;

pub struct ColorManager {
    colors: [Vec<Color>; COLOR_COUNT],
}

impl Default for ColorManager {
    fn default() -> Self {
        Self {
            colors: [
                vec![(245, 245, 245, 255)],
                vec![(200, 200, 255, 255)],
                vec![(201, 201, 201, 255)],
                vec![(201, 239, 254, 255)],
                vec![(151, 232, 235, 255)],
                vec![(104, 104, 104, 255)],
            ],
        }
    }
}

impl ColorManager {
    pub fn push(&mut self, code: ColorCode, c: Color) {
        let idx: usize = code.into();
        self.colors[idx].push(c);
    }
    pub fn pop(&mut self, code: ColorCode) {
        let idx: usize = code.into();
        self.colors[idx].pop();
    }
    pub fn get(&self, code: ColorCode) -> Color {
        let idx: usize = code.into();
        *self.colors[idx].last().unwrap()
    }
}

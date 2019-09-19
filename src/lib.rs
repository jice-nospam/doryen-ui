use std::collections::HashMap;

#[cfg(feature = "doryen")]
mod doryen;

#[cfg(feature = "doryen")]
pub use doryen::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ColorCode {
    Background,
    Foreground,
    ButtonBackground,
    ButtonBackgroundHover,
    ButtonBackgroundFocus,
    Text,
}

pub enum TextAlign {
    Left,
    Center,
    Right,
}

pub type Coord = i32;

#[derive(Clone, Copy, Default, Debug)]
pub struct Pos {
    pub x: Coord,
    pub y: Coord,
}

impl From<Rect> for Pos {
    fn from(r: Rect) -> Self {
        Self { x: r.x, y: r.y }
    }
}
impl From<&Rect> for Pos {
    fn from(r: &Rect) -> Self {
        Self { x: r.x, y: r.y }
    }
}

#[derive(Default, Clone, Copy, Debug)]
pub struct Rect {
    pub x: Coord,
    pub y: Coord,
    pub w: Coord,
    pub h: Coord,
}

impl Rect {
    pub fn new(x: Coord, y: Coord, w: Coord, h: Coord) -> Self {
        Self { x, y, w, h }
    }
    pub fn contains(&self, p: Pos) -> bool {
        p.x >= self.x && p.y >= self.y && p.x < self.x + self.w && p.y < self.y + self.h
    }
}

pub enum Command {
    Rect(Rect, ColorCode),
    Text(String, Pos, ColorCode),
    Frame(String, Rect, ColorCode),
    CheckBox(Pos, bool, ColorCode),
}

pub trait Renderer {
    fn rectangle(&mut self, rect: &Rect, col: ColorCode);
    fn text(&mut self, pos: Pos, txt: &str, col: ColorCode);
    fn frame(&mut self, txt: &str, rect: &Rect, col: ColorCode);
    fn checkbox(&mut self, pos: Pos, checked: bool, col: ColorCode);
}

pub const MOUSE_BUTTON_LEFT: usize = 1;
pub const MOUSE_BUTTON_RIGHT: usize = 2;
pub const MOUSE_BUTTON_MIDDLE: usize = 4;

pub enum Layout {
    Fixed(Pos, Pos),
    Vbox(Pos, Coord),
    Hbox(Pos, Coord),
}

#[derive(Default)]
pub struct Context {
    id: usize,
    focus: usize,
    hover: usize,
    updated_focus: bool,
    mouse_pos: Pos,
    last_mouse_pos: Pos,
    mouse_delta: Pos,
    mouse_pressed: usize,
    mouse_down: usize,
    commands: Vec<Command>,
    layouts: Vec<Layout>,
    last_area: Rect,
    cursor: Pos,
    button_state: HashMap<usize, bool>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            mouse_pos: Pos { x: -1, y: -1 },
            ..Default::default()
        }
    }
    pub fn input_mouse_pos(&mut self, pos: Pos) {
        self.mouse_pos = pos;
    }
    pub fn input_mouse_down(&mut self, button: usize) {
        self.mouse_down |= button;
        self.mouse_pressed |= button;
    }
    pub fn input_mouse_up(&mut self, button: usize) {
        self.mouse_down &= !button;
    }

    pub fn begin(&mut self) {
        self.mouse_delta.x = self.mouse_pos.x - self.last_mouse_pos.x;
        self.mouse_delta.y = self.mouse_pos.y - self.last_mouse_pos.y;
        self.cursor.x = 0;
        self.cursor.y = 0;
        self.id = 0;
    }
    pub fn end(&mut self) {
        self.mouse_pressed = 0;
        self.last_mouse_pos = self.mouse_pos;
    }
    pub fn render(&mut self, renderer: &mut impl Renderer) {
        for c in self.commands.iter() {
            match c {
                Command::Rect(r, col) => renderer.rectangle(r, *col),
                Command::Text(txt, pos, col) => renderer.text(*pos, txt, *col),
                Command::Frame(txt, r, col) => renderer.frame(txt, r, *col),
                Command::CheckBox(pos, checked, col) => renderer.checkbox(*pos, *checked, *col),
            }
        }
        self.commands.clear();
    }
    pub fn get_render_commands(&mut self) -> Vec<Command> {
        let mut v = Vec::new();
        v.append(&mut self.commands);
        self.commands.clear();
        v
    }

    fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }
    pub fn vbox_start(&mut self, width: Coord) {
        self.layouts.push(Layout::Vbox(self.cursor, width));
    }
    pub fn vbox_end(&mut self) {
        self.layouts.pop();
    }
    pub fn hbox_start(&mut self, height: Coord) {
        self.layouts.push(Layout::Hbox(self.cursor, height));
    }
    pub fn hbox_end(&mut self) {
        self.layouts.pop();
    }
    pub fn frame_start(&mut self, title: &str, width: Coord, height: Coord) {
        let id = self.next_id();
        let r = self.layout(Rect::new(0, 0, width, height.max(3)));
        self.update_control(id, &r);
        self.draw_frame(r, title, ColorCode::Background);
        self.cursor.x = r.x + 1;
        self.cursor.y = r.y + 1;
        self.layouts.push(Layout::Vbox(self.cursor, width - 2));
    }
    pub fn frame_end(&mut self) {
        self.layouts.pop();
        self.cursor.y += 1;
        self.cursor.x -= 1;
    }
    pub fn popup_start(&mut self, title: &str, x: Coord, y: Coord, width: Coord, height: Coord) {
        self.layouts.push(Layout::Fixed(self.cursor, Pos { x, y }));
        self.frame_start(title, width, height);
    }
    pub fn popup_end(&mut self) -> bool {
        let ret = self.button("Ok", TextAlign::Center);
        self.frame_end();
        if let Some(Layout::Fixed(oldpos, _)) = self.layouts.pop() {
            self.cursor = oldpos;
        }
        ret
    }
    pub fn label(&mut self, label: &str, align: TextAlign) {
        let id = self.next_id();
        let r = self.layout(Rect::new(0, 0, label.len() as Coord, 1));
        self.update_control(id, &r);
        self.draw_rect(r, ColorCode::Background);
        self.draw_text(r, label, align, ColorCode::Text);
    }
    pub fn checkbox(&mut self, label: &str, checked: bool) -> bool {
        let padded_label = "  ".to_owned() + label;
        let pressed = self.button(&padded_label, TextAlign::Left);
        let checked = {
            let checked = self.button_state.entry(self.id).or_insert(checked);
            if pressed {
                *checked = !*checked;
            }
            *checked
        };
        let pos: Pos = self.last_area.into();
        self.draw_checkbox(pos, checked, ColorCode::Text);
        checked
    }
    pub fn toggle(&mut self, label: &str, align: TextAlign, on: bool) -> bool {
        let id = self.next_id();
        let r = self.layout(Rect::new(0, 0, label.len() as Coord, 1));
        self.update_control(id, &r);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let on = {
            let on = self.button_state.entry(self.id).or_insert(on);
            if pressed {
                *on = !*on;
            }
            *on
        };
        self.draw_rect(
            r,
            if on || hover {
                ColorCode::ButtonBackgroundHover
            } else if focus {
                ColorCode::ButtonBackgroundFocus
            } else {
                ColorCode::ButtonBackground
            },
        );
        self.draw_text(r, label, align, ColorCode::Text);
        on
    }
    pub fn button(&mut self, label: &str, align: TextAlign) -> bool {
        let id = self.next_id();
        let r = self.layout(Rect::new(0, 0, label.len() as Coord, 1));
        self.update_control(id, &r);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        self.draw_rect(
            r,
            if hover {
                ColorCode::ButtonBackgroundHover
            } else if focus {
                ColorCode::ButtonBackgroundFocus
            } else {
                ColorCode::ButtonBackground
            },
        );
        self.draw_text(r, label, align, ColorCode::Text);
        pressed
    }
    fn draw_checkbox(&mut self, p: Pos, checked: bool, col: ColorCode) {
        self.commands.push(Command::CheckBox(p, checked, col));
    }
    fn draw_frame(&mut self, r: Rect, title: &str, col: ColorCode) {
        self.commands.push(Command::Frame(title.to_owned(), r, col));
    }

    fn draw_rect(&mut self, r: Rect, col: ColorCode) {
        self.commands.push(Command::Rect(r, col));
    }

    fn draw_text(&mut self, r: Rect, txt: &str, align: TextAlign, col: ColorCode) {
        let mut p: Pos = r.into();
        let truncated_txt: String;
        let len = txt.len() as Coord;
        match align {
            TextAlign::Left => truncated_txt = txt[0..r.w.min(len) as usize].to_owned(),
            TextAlign::Right => {
                p.x = p.x + r.w - len;
                if p.x < 0 {
                    truncated_txt = txt[(-p.x) as usize..].to_owned();
                    p.x = 0;
                } else {
                    truncated_txt = txt.to_owned();
                }
            }
            TextAlign::Center => {
                if len > r.w {
                    let to_remove = (len - r.w) as usize;
                    let start = (to_remove / 2) as usize;
                    let end = (len as usize - (to_remove - start)) as usize;
                    truncated_txt = txt[start..end].to_owned();
                } else {
                    truncated_txt = txt.to_owned();
                }
                p.x = p.x + r.w / 2 - len / 2;
            }
        };
        self.commands.push(Command::Text(truncated_txt, p, col));
    }

    fn update_control(&mut self, id: usize, r: &Rect) {
        let mouse_over = r.contains(self.mouse_pos);
        let pressed = self.mouse_pressed != 0;
        if mouse_over {
            self.hover = id;
            if pressed {
                self.set_focus(id);
            }
        } else {
            self.hover = 0;
            if self.focus == id && pressed {
                self.set_focus(0);
            }
        }
    }

    fn set_focus(&mut self, id: usize) {
        self.focus = id;
        self.updated_focus = true;
    }

    fn layout(&mut self, local: Rect) -> Rect {
        let mut r = Rect::new(
            local.x + self.cursor.x,
            local.y + self.cursor.y,
            local.w,
            local.h,
        );
        match self.layouts.last() {
            Some(Layout::Fixed(_, Pos { x, y })) => {
                r.x = local.x + x;
                r.y = local.y + y;
            }
            Some(Layout::Vbox(pos, w)) => {
                r.w = *w;
                self.cursor.x = pos.x;
                self.cursor.y += 1;
            }
            Some(Layout::Hbox(pos, h)) => {
                r.h = *h;
                self.cursor.x += r.w;
                self.cursor.y = pos.y;
            }
            None => (),
        }
        self.last_area = r;
        r
    }
}

#[cfg(test)]
mod tests {
    use crate as ui;

    const SCREEN_WIDTH: usize = 80;
    const SCREEN_HEIGHT: usize = 25;
    struct AsciiRenderer {
        character: [[char; SCREEN_HEIGHT]; SCREEN_WIDTH],
        background_color: [[ui::ColorCode; SCREEN_HEIGHT]; SCREEN_WIDTH],
        foreground_color: [[ui::ColorCode; SCREEN_HEIGHT]; SCREEN_WIDTH],
    }
    impl AsciiRenderer {
        pub fn new() -> Self {
            Self {
                character: [[' '; SCREEN_HEIGHT]; SCREEN_WIDTH],
                background_color: [[ui::ColorCode::Background; SCREEN_HEIGHT]; SCREEN_WIDTH],
                foreground_color: [[ui::ColorCode::Foreground; SCREEN_HEIGHT]; SCREEN_WIDTH],
            }
        }
        pub fn assert(&self, t: &str, x: usize, y: usize) -> bool {
            let mut cx = x;
            for c in t.chars() {
                if self.character[cx][y] != c {
                    return false;
                }
                cx += 1;
            }
            true
        }
    }
    impl ui::Renderer for AsciiRenderer {
        fn rectangle(&mut self, rect: &ui::Rect, col: ui::ColorCode) {
            for cx in rect.x as usize..(rect.x + rect.w) as usize {
                for cy in rect.y as usize..(rect.y + rect.h) as usize {
                    self.character[cx][cy] = ' ';
                    self.background_color[cx][cy] = col;
                }
            }
        }
        fn text(&mut self, pos: ui::Pos, txt: &str, col: ui::ColorCode) {
            let mut x = pos.x as usize;
            let y = pos.y as usize;
            for c in txt.chars() {
                self.character[x][y] = c;
                self.foreground_color[x][y] = col;
                x += 1;
            }
        }
        fn frame(&mut self, txt: &str, rect: &ui::Rect, col: ui::ColorCode) {
            let rx = rect.x as usize;
            let ry = rect.y as usize;
            let rx2 = rx + rect.w as usize - 1;
            let ry2 = ry + rect.h as usize - 1;
            self.character[rx][ry] = '1';
            self.character[rx2][ry] = '2';
            self.character[rx][ry2] = '3';
            self.character[rx2][ry2] = '4';
            self.text(rect.into(), txt, col);
        }
        fn checkbox(&mut self, pos: ui::Pos, _checked: bool, _col: ui::ColorCode) {
            self.character[pos.x as usize][pos.y as usize] = '5';
        }
    }

    #[test]
    fn test_button() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.button("test", ui::TextAlign::Left);
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("test", 0, 0));
    }
}

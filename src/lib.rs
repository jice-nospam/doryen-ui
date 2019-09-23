use std::collections::{HashMap, HashSet};

#[cfg(feature = "doryen")]
mod doryen;

mod layout;

#[cfg(feature = "doryen")]
pub use doryen::*;

use layout::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum ColorCode {
    Background,
    Foreground,
    ButtonBackground,
    ButtonBackgroundHover,
    ButtonBackgroundFocus,
    Text,
}

#[derive(Copy, Clone, Debug)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}
impl Default for TextAlign {
    fn default() -> Self {
        TextAlign::Left
    }
}

pub type Coord = i32;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
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

#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
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

#[derive(Debug)]
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

#[derive(Copy, Clone, Debug, Default)]
pub struct LayoutOptions {
    pub margin: Coord,
    pub padding: Coord,
    pub pos: Option<(Coord, Coord)>,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct ToggleOptions {
    pub align: TextAlign,
    pub group: Option<usize>,
    pub active: bool,
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
    button_state: HashMap<usize, bool>,
    toggle_group: HashMap<usize, HashSet<usize>>,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
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
        self.layouts.clear();
        self.commands.clear();
        self.layouts.push(Default::default());
    }
    pub fn end(&mut self) {
        self.mouse_delta.x = self.mouse_pos.x - self.last_mouse_pos.x;
        self.mouse_delta.y = self.mouse_pos.y - self.last_mouse_pos.y;
        self.last_mouse_pos = self.mouse_pos;
        self.mouse_pressed = 0;
        self.id = 0;
        //println!("=================");
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
    }
    pub fn get_render_commands(&mut self) -> &Vec<Command> {
        &self.commands
    }
    pub fn last_id(&self) -> usize {
        self.id
    }
    pub fn set_toggle_status(&mut self, toggle_id: usize, status: bool) {
        self.button_state.insert(toggle_id, status);
    }

    fn next_id(&mut self) -> usize {
        self.id += 1;
        self.id
    }
    fn next_rectangle(&mut self, width: Coord, height: Coord) -> Rect {
        self.layouts.last_mut().unwrap().next_area(width, height)
        //println!("w {:?}", layout);
    }
    fn last_cursor(&self) -> Pos {
        self.layouts.last().unwrap().last_cursor()
    }
    fn next_layout_area(&mut self, w: Coord, h: Coord, opt: LayoutOptions) -> Rect {
        if let Some((x, y)) = opt.pos {
            Rect { x, y, w, h }
        } else {
            self.layouts.last_mut().unwrap().next_area(w, h)
        }
    }
    pub fn grid_begin(
        &mut self,
        cols: usize,
        rows: usize,
        width: Coord,
        height: Coord,
        opt: LayoutOptions,
    ) {
        let mut r = self.next_layout_area(width * cols as Coord, height * rows as Coord, opt);
        r.w = width;
        r.h = height;
        let layout = Layout::new_grid(r, cols, rows, opt.margin, opt.padding);
        //println!("g {:?}", layout);
        self.layouts.push(layout);
    }
    pub fn grid_end(&mut self) {
        self.layouts.pop();
    }
    pub fn vbox_begin(&mut self, width: Coord, height: Coord, opt: LayoutOptions) {
        let r = self.next_layout_area(width, height, opt);
        let layout = Layout::new(LayoutMode::Vertical, r, opt.margin, opt.padding);
        //println!("v {:?}", layout);
        self.layouts.push(layout);
    }
    pub fn vbox_end(&mut self) {
        self.layouts.pop();
    }
    pub fn hbox_begin(&mut self, width: Coord, height: Coord, opt: LayoutOptions) {
        let r = self.next_layout_area(width, height, opt);
        let layout = Layout::new(LayoutMode::Horizontal, r, opt.margin, opt.padding);
        //println!("h {:?}", layout);
        self.layouts.push(layout);
    }
    pub fn hbox_end(&mut self) {
        self.layouts.pop();
    }
    pub fn frame_begin(&mut self, title: &str, width: Coord, height: Coord, opt: LayoutOptions) {
        self.vbox_begin(
            width.max((title.len() + 2) as Coord),
            height,
            LayoutOptions {
                margin: opt.margin + 1,
                ..opt
            },
        );
        self.draw_frame(
            self.layouts.last().unwrap().area(),
            title,
            ColorCode::Background,
        );
    }
    pub fn frame_end(&mut self) {
        self.vbox_end();
    }
    pub fn popup_begin(&mut self, title: &str, width: Coord, height: Coord, opt: LayoutOptions) {
        self.frame_begin(title, width, height, opt);
    }
    pub fn popup_end(&mut self) -> bool {
        let ret = self.button("Ok", TextAlign::Center);
        self.frame_end();
        ret
    }
    pub fn label(&mut self, label: &str, align: TextAlign) {
        let r = self.next_rectangle(label.len() as Coord, 1);
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
        self.draw_checkbox(self.last_cursor(), checked, ColorCode::Text);
        checked
    }
    fn add_group_id(&mut self, group: usize, id: usize) {
        let ids = self.toggle_group.entry(group).or_insert_with(HashSet::new);
        ids.insert(id);
    }
    fn disable_toggle_group(&mut self, group: usize) {
        for id in self.toggle_group.get(&group).unwrap() {
            self.button_state.insert(*id, false);
        }
    }
    /// returns (toggle_status, has_changed_this_frame)
    pub fn toggle(&mut self, label: &str, opt: ToggleOptions) -> (bool, bool) {
        let id = self.next_id();
        if let Some(group) = opt.group {
            self.add_group_id(group, id);
        }
        let r = self.next_rectangle(label.len() as Coord, 1);
        self.update_control(id, &r);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let mut on = *self.button_state.get(&self.id).unwrap_or(&opt.active);
        let mut changed = false;
        if pressed {
            if !on {
                if let Some(group) = opt.group {
                    self.disable_toggle_group(group);
                }
            }
            changed = true;
            on = !on;
        }
        self.button_state.insert(id, on);
        self.draw_rect(
            r,
            if on {
                ColorCode::ButtonBackgroundHover
            } else if focus || hover {
                ColorCode::ButtonBackgroundFocus
            } else {
                ColorCode::ButtonBackground
            },
        );
        self.draw_text(r, label, opt.align, ColorCode::Text);
        (on, changed)
    }
    pub fn button(&mut self, label: &str, align: TextAlign) -> bool {
        let id = self.next_id();
        let r = self.next_rectangle(label.len() as Coord, 1);
        self.update_control(id, &r);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        //println!("{}: {} {} {}",id, focus,hover,pressed);
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
    #[test]
    fn test_vbox() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.vbox_begin(1, 0, Default::default());
        ctx.label("1", ui::TextAlign::Left);
        ctx.label("2", ui::TextAlign::Left);
        ctx.vbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("1", 0, 0));
        assert!(rend.assert("2", 0, 1));
    }
    #[test]
    fn test_hbox() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.hbox_begin(0, 1, Default::default());
        ctx.label("1", ui::TextAlign::Left);
        ctx.label("2", ui::TextAlign::Left);
        ctx.hbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("1", 0, 0));
        assert!(rend.assert("2", 1, 0));
    }
    #[test]
    fn test_margin() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.vbox_begin(
            3,
            0,
            ui::LayoutOptions {
                margin: 1,
                ..Default::default()
            },
        );
        ctx.label("1", ui::TextAlign::Left);
        ctx.label("2", ui::TextAlign::Left);
        ctx.vbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("1", 1, 1));
        assert!(rend.assert("2", 1, 2));
    }
    #[test]
    fn test_padding() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.vbox_begin(
            3,
            0,
            ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            },
        );
        ctx.label("1", ui::TextAlign::Left);
        ctx.label("2", ui::TextAlign::Left);
        ctx.vbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("1", 0, 0));
        assert!(rend.assert("2", 0, 2));
    }
}

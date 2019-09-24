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
    TextColor(String, Pos, TextAlign),
    Frame(String, Rect, ColorCode),
    Line(Pos, Pos, ColorCode),
    CheckBox(Pos, bool, ColorCode),
}

pub trait Renderer {
    fn line(&mut self, p1: Pos, p2: Pos, col: ColorCode);
    fn rectangle(&mut self, rect: &Rect, col: ColorCode);
    fn text(&mut self, pos: Pos, txt: &str, col: ColorCode);
    fn text_color(&mut self, pos: Pos, txt: &str, align: TextAlign);
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
    button_state: HashMap<usize, i32>,
    toggle_group: HashMap<usize, HashSet<usize>>,
    list_button_index: i32,
    list_button_width: Coord,
    list_button_label: String,
    list_button_align: TextAlign,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }
    // =======================================================
    //
    // Input
    //
    // =======================================================
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
    // =======================================================
    //
    // Core
    //
    // =======================================================
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
                Command::TextColor(txt, pos, align) => renderer.text_color(*pos, txt, *align),
                Command::Frame(txt, r, col) => renderer.frame(txt, r, *col),
                Command::Line(p1, p2, col) => renderer.line(*p1, *p2, *col),
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
    // =======================================================
    //
    // Containers
    //
    // =======================================================
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
            width.max((title.chars().count() + 2) as Coord),
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
    // =======================================================
    //
    // Basic widgets
    //
    // =======================================================
    pub fn separator(&mut self) {
        let r = self.next_rectangle(0, 0);
        self.draw_rect(r, ColorCode::Background);
        self.commands.push(Command::Line(
            r.into(),
            Pos {
                x: r.x + r.w - 1,
                y: r.y + r.h - 1,
            },
            ColorCode::Foreground,
        ));
    }

    pub fn label(&mut self, label: &str, align: TextAlign) {
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.draw_rect(r, ColorCode::Background);
        self.draw_text(r, label, align, ColorCode::Text);
    }
    pub fn label_color(&mut self, label: &str, align: TextAlign) {
        let len = text_color_len(label) as Coord;
        let r = self.next_rectangle(len, 1);
        self.draw_rect(r, ColorCode::Background);
        self.draw_text_color(r, label, align);
    }
    // =======================================================
    //
    // Buttons
    //
    // =======================================================
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
    pub fn button(&mut self, label: &str, align: TextAlign) -> bool {
        let id = self.next_id();
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.update_control(id, &r);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
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
    pub fn checkbox(&mut self, label: &str, checked: bool) -> bool {
        let padded_label = "  ".to_owned() + label;
        let pressed = self.button(&padded_label, TextAlign::Left);
        let checked = {
            let checked = self
                .button_state
                .entry(self.id)
                .or_insert(if checked { 1 } else { 0 });
            if pressed {
                *checked = 1 - *checked;
            }
            *checked
        };
        self.draw_checkbox(self.last_cursor(), checked == 1, ColorCode::Text);
        checked == 1
    }

    /// returns (toggle_status, has_changed_this_frame)
    fn add_group_id(&mut self, group: usize, id: usize) {
        let ids = self.toggle_group.entry(group).or_insert_with(HashSet::new);
        ids.insert(id);
    }
    fn disable_toggle_group(&mut self, group: usize) {
        for id in self.toggle_group.get(&group).unwrap() {
            self.button_state.insert(*id, 0);
        }
    }
    pub fn toggle(&mut self, label: &str, opt: ToggleOptions) -> (bool, bool) {
        let id = self.next_id();
        if let Some(group) = opt.group {
            self.add_group_id(group, id);
        }
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.update_control(id, &r);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let mut on = *self
            .button_state
            .get(&self.id)
            .unwrap_or(if opt.active { &1 } else { &0 })
            == 1;
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
        self.button_state.insert(id, if on { 1 } else { 0 });
        self.draw_rect(
            r,
            if on && !hover {
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
    pub fn set_toggle_status(&mut self, toggle_id: usize, status: bool) {
        self.button_state
            .insert(toggle_id, if status { 1 } else { 0 });
    }

    pub fn list_button_begin(&mut self) {
        let id = self.next_id();
        self.list_button_index = 0;
        self.list_button_width = 0;
        self.button_state.entry(id).or_insert(0);
    }

    /// add a new item in the list of values
    /// returns true if this is the current value
    pub fn list_button_item(&mut self, label: &str, align: TextAlign) -> bool {
        self.list_button_width = self.list_button_width.max(label.chars().count() as Coord);
        let list_button_id = self.last_id();
        self.list_button_index += 1;
        assert!(
            self.button_state.get(&list_button_id).is_some(),
            "list_button_item must be called inside list_button_begin/list_button_end"
        );
        if *self.button_state.get(&list_button_id).unwrap() != self.list_button_index - 1 {
            return false;
        }
        self.list_button_label = label.to_owned();
        self.list_button_align = align;
        true
    }

    /// close the list button value list.
    /// returns true if the current value has changed this frame
    /// if display_count is true, shows the item index/count when the mouse is hovering the button
    pub fn list_button_end(&mut self, display_count: bool) -> bool {
        let list_button_id = self.last_id();
        assert!(
            self.button_state.get(&list_button_id).is_some(),
            "list_button_end must be called after list_button_begin"
        );
        self.list_button_width += 2;
        let r = self.next_rectangle(self.list_button_width, 1);
        self.update_control(list_button_id, &r);
        let focus = self.focus == list_button_id;
        let hover = self.hover == list_button_id;
        let pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        //println!("{}: {} {} {}",list_button_id, focus,hover,pressed);
        let cur_index = *self.button_state.get(&list_button_id).unwrap();
        if pressed {
            let next_index = (cur_index + 1) % self.list_button_index;
            self.button_state.insert(list_button_id, next_index);
        }
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
        let label = if hover && display_count {
            let mut label = self.list_button_label.clone();
            let label_len = label.chars().count();
            let suffix = format!("|{}/{}", cur_index + 1, self.list_button_index);
            let suffix_len = suffix.len();
            if suffix_len + label_len > r.w as usize {
                label = label
                    .chars()
                    .take(r.w as usize - suffix_len)
                    .collect::<String>();
            }
            label + &suffix
        } else {
            self.list_button_label.clone()
        };
        self.draw_text(r, &label, self.list_button_align, ColorCode::Text);
        pressed
    }

    // =======================================================
    //
    // Basic drawing functions
    //
    // =======================================================
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
        let (pos, truncated_text) = format_text(r, txt, align);
        self.commands.push(Command::Text(truncated_text, pos, col));
    }

    fn draw_text_color(&mut self, r: Rect, txt: &str, align: TextAlign) {
        self.commands
            .push(Command::TextColor(txt.to_owned(), r.into(), align));
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

fn format_text(r: Rect, txt: &str, align: TextAlign) -> (Pos, String) {
    let mut p: Pos = r.into();
    let truncated_txt: String;
    let len = txt.chars().count() as Coord;
    match align {
        TextAlign::Left => {
            truncated_txt = txt.chars().take(r.w.min(len) as usize).collect::<String>()
        }
        TextAlign::Right => {
            let newx = p.x + r.w - len;
            if newx < p.x {
                truncated_txt = txt.chars().skip((p.x - newx) as usize).collect::<String>();
            } else {
                p.x = newx;
                truncated_txt = txt.to_owned();
            }
        }
        TextAlign::Center => {
            if len > r.w {
                let to_remove = (len - r.w) as usize;
                let start = (to_remove / 2) as usize;
                let end = (len as usize - (to_remove - start)) as usize;
                truncated_txt = txt
                    .chars()
                    .skip(start)
                    .take(end - start)
                    .collect::<String>();
            } else {
                truncated_txt = txt.to_owned();
                p.x = p.x + r.w / 2 - len / 2;
            }
        }
    };
    (p, truncated_txt)
}

#[cfg(test)]
mod tests {
    use crate as ui;

    const SCREEN_WIDTH: usize = 80;
    const SCREEN_HEIGHT: usize = 25;
    struct AsciiRenderer {
        character: [[char; SCREEN_HEIGHT]; SCREEN_WIDTH],
    }
    impl AsciiRenderer {
        pub fn new() -> Self {
            Self {
                character: [[' '; SCREEN_HEIGHT]; SCREEN_WIDTH],
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
        fn line(&mut self, _p1: ui::Pos, _p2: ui::Pos, _col: ui::ColorCode) {}
        fn rectangle(&mut self, rect: &ui::Rect, _col: ui::ColorCode) {
            for cx in rect.x as usize..(rect.x + rect.w) as usize {
                for cy in rect.y as usize..(rect.y + rect.h) as usize {
                    self.character[cx][cy] = ' ';
                }
            }
        }
        fn text(&mut self, pos: ui::Pos, txt: &str, _col: ui::ColorCode) {
            let mut x = pos.x as usize;
            let y = pos.y as usize;
            for c in txt.chars() {
                self.character[x][y] = c;
                x += 1;
            }
        }
        fn text_color(&mut self, pos: ui::Pos, txt: &str, _align: ui::TextAlign) {
            let mut x = pos.x as usize;
            let y = pos.y as usize;
            for c in txt.chars() {
                self.character[x][y] = c;
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

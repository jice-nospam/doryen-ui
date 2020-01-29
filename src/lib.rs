use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

#[cfg(feature = "doryen")]
mod doryen;

mod button;
mod color;
mod container;
mod layout;
mod slider;

#[cfg(feature = "doryen")]
pub use doryen::*;

pub use color::{Color, ColorCode};

use color::*;
use layout::*;

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
pub type Id = u64;
const NULL_ID: Id = 0;

#[derive(Debug, PartialEq, Eq)]
pub enum DeferedCommand {
    Frame(String, Color, Color),
    Button(String, Color, Color),
    CheckBox(bool, Color),
    DropDown(bool, Color),
    Label(Rect, String, Color, Color),
    LabelColor(Rect, String, Color),
}

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
impl From<(f32, f32)> for Pos {
    fn from(p: (f32, f32)) -> Self {
        Self {
            x: p.0 as Coord,
            y: p.1 as Coord,
        }
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
    Rect(Rect, Color),
    Text(String, Pos, Color),
    TextColor(String, Pos, TextAlign),
    Frame(String, Rect, Color, Color),
    Line(Pos, Pos, Color),
    CheckBox(Pos, bool, Color),
    DropDown(Pos, bool, Color),
    Progress(Rect, f32, Color, Color),
}

pub trait Renderer {
    fn line(&mut self, p1: Pos, p2: Pos, col: Color);
    fn rectangle(&mut self, rect: &Rect, col: Color);
    fn text(&mut self, pos: Pos, txt: &str, col: Color);
    fn text_color(&mut self, pos: Pos, txt: &str, align: TextAlign);
    fn frame(&mut self, txt: &str, rect: &Rect, col: Color, coltxt: Color);
    fn checkbox(&mut self, pos: Pos, checked: bool, col: Color);
    fn dropdown(&mut self, pos: Pos, checked: bool, col: Color);
    fn progress(&mut self, rect: &Rect, val: f32, back: Color, fore: Color);
}

pub const MOUSE_BUTTON_LEFT: usize = 1;
pub const MOUSE_BUTTON_RIGHT: usize = 2;
pub const MOUSE_BUTTON_MIDDLE: usize = 4;

#[derive(Default)]
pub struct Context {
    color_manager: ColorManager,
    // id generation
    last_id: Id,
    id_prefix: Vec<String>,
    // user input data
    mouse_pos: (f32, f32),
    mouse_pressed: usize,
    mouse_down: usize,
    // rendering
    commands: Vec<Command>,
    layouts: Vec<Layout>,
    // defered widget creation
    next_layout: Option<Layout>,
    next_align: Option<TextAlign>,
    // state management
    focus: Id,
    hover: Id,
    button_state: HashMap<Id, i32>,
    slider_state: HashMap<Id, f32>,
    toggle_group: HashMap<usize, HashSet<Id>>,
    cur_toggle_group: usize,
    pressed: bool,
    active: bool,
    // list-buttons
    list_button_index: i32,
    list_button_width: Coord,
    list_button_label: String,
    list_button_align: TextAlign,
    // drag'n drop
    dnd_on: bool,
    dnd_start: (f32, f32),
    dnd_value: f32,
}

impl Context {
    pub fn new() -> Self {
        Default::default()
    }
    // =======================================================
    //
    // Color stack
    //
    // =======================================================
    pub fn push_color(&mut self, code: ColorCode, c: Color) {
        self.color_manager.push(code, c);
    }
    pub fn pop_color(&mut self, code: ColorCode) {
        self.color_manager.pop(code);
    }
    pub fn get_color(&self, code: ColorCode) -> Color {
        self.color_manager.get(code)
    }
    // =======================================================
    //
    // Input
    //
    // =======================================================
    pub fn input_mouse_pos(&mut self, x: f32, y: f32) {
        self.mouse_pos = (x, y);
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
        self.try_commit();
        self.mouse_pressed = 0;
        self.last_id = NULL_ID.to_owned();
        self.id_prefix.clear();
        //println!("================");
    }
    pub fn render(&mut self, renderer: &mut impl Renderer) {
        for c in self.commands.iter() {
            match c {
                Command::Rect(r, col) => renderer.rectangle(r, *col),
                Command::Text(txt, pos, col) => renderer.text(*pos, txt, *col),
                Command::TextColor(txt, pos, align) => renderer.text_color(*pos, txt, *align),
                Command::Frame(txt, r, col, coltxt) => renderer.frame(txt, r, *col, *coltxt),
                Command::Line(p1, p2, col) => renderer.line(*p1, *p2, *col),
                Command::CheckBox(pos, checked, col) => renderer.checkbox(*pos, *checked, *col),
                Command::DropDown(pos, checked, col) => renderer.dropdown(*pos, *checked, *col),
                Command::Progress(r, val, back, fore) => renderer.progress(r, *val, *back, *fore),
            }
        }
    }
    pub fn get_render_commands(&mut self) -> &Vec<Command> {
        &self.commands
    }
    pub fn pressed(&mut self) -> bool {
        let r = self.pressed;
        self.pressed = false;
        r
    }
    pub fn active(&mut self) -> bool {
        let r = self.active;
        self.active = false;
        r
    }
    pub fn padding(&mut self, padding: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.padding(padding);
        }
        self
    }
    pub fn vpadding(&mut self, padding: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.vpadding(padding);
        }
        self
    }
    pub fn hpadding(&mut self, padding: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.hpadding(padding);
        }
        self
    }
    pub fn align(&mut self, align: TextAlign) -> &mut Self {
        self.next_align = Some(align);
        self
    }
    pub fn move_cursor(&mut self, deltax: Coord, deltay: Coord) -> &mut Self {
        if let Some(layout) = self.layouts.last_mut() {
            layout.move_cursor(deltax, deltay);
        }
        self
    }
    pub fn spacing(&mut self) -> &mut Self {
        self.move_cursor(0, 1)
    }
    pub fn margin(&mut self, margin: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.margin(margin);
        }
        self
    }
    pub fn min_width(&mut self, value: Coord) -> &mut Self {
        assert!(
            self.next_layout.is_some(),
            "min_width should only be called after a container begin"
        );
        if let Some(layout) = self.next_layout.as_mut() {
            layout.min_width(value);
        }
        self
    }
    pub fn max_width(&mut self, value: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.max_width(value);
        }
        self
    }
    pub fn min_height(&mut self, value: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.min_height(value);
        }
        self
    }
    pub fn max_height(&mut self, value: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.max_height(value);
        }
        self
    }
    pub fn defered(&mut self, cmd: DeferedCommand) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.defered(cmd);
        }
        self
    }
    fn grid(&mut self, cols: usize, rows: usize, width: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.grid(cols, rows, width);
        }
        self
    }
    fn size(&mut self, width: Coord, height: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.size(width, height);
        }
        self
    }
    fn flexgrid(&mut self, widths: &[Coord]) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.flexgrid(&widths);
        }
        self
    }
    fn fixed_size(&mut self, width: Coord, height: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.fixed_size(width, height);
        }
        self
    }
    fn fixed_pos(&mut self, x: Coord, y: Coord, width: Coord, height: Coord) -> &mut Self {
        if let Some(layout) = self.next_layout.as_mut() {
            layout.pos(x, y).size(width, height);
        }
        self
    }
    fn new_layout(&mut self, mode: LayoutMode) -> &mut Self {
        self.next_layout = Some(Layout::new(mode));
        self
    }
    fn try_commit(&mut self) {
        if let Some(mut layout) = self.next_layout.take() {
            if !layout.commited() {
                self.layouts.last_mut().unwrap().commit(&mut layout);
            }
            let r = layout.area();
            for c in layout.defered_iter() {
                self.render_defered(r, c);
            }
            if !layout.is_single() {
                self.layouts.push(layout);
            }
        }
    }
    fn end_container(&mut self) {
        self.try_commit();
        self.layouts.pop();
        self.id_prefix.pop();
    }
    fn next_rectangle(&mut self, width: Coord, height: Coord) -> Rect {
        self.new_layout(LayoutMode::Single).size(width, height);
        if let Some(ref mut layout) = self.next_layout {
            self.layouts.last_mut().unwrap().commit(layout)
        } else {
            unreachable!();
        }
        //println!("w {:?}", layout);
    }
    fn last_cursor(&self) -> Pos {
        self.layouts.last().unwrap().last_cursor()
    }

    // =======================================================
    //
    // Id management
    //
    // =======================================================
    fn prefix_id(&mut self, id: &str) {
        //println!("{}", id);
        self.id_prefix.push(id.to_owned());
    }
    fn generate_id(&mut self, name: &str) -> Id {
        //println!("{}", name);
        let mut hasher = DefaultHasher::new();
        (self.id_prefix.join("/") + "/" + name).hash(&mut hasher);
        self.last_id = hasher.finish();
        self.last_id
    }
    pub fn last_id(&self) -> Id {
        self.last_id
    }

    // =======================================================
    //
    // Basic widgets
    //
    // =======================================================
    pub fn separator(&mut self) {
        self.try_commit();
        let r = self.next_rectangle(0, 0);
        let back = self.get_color(ColorCode::Background);
        let fore = self.get_color(ColorCode::Foreground);
        self.draw_rect(r, back);
        self.draw_line(r.x, r.y, r.x + r.w, r.y + r.h, fore);
    }

    pub fn label(&mut self, label: &str) -> &mut Self {
        self.try_commit();
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        let back = self.get_color(ColorCode::Background);
        let fore = self.get_color(ColorCode::Text);
        self.defered(DeferedCommand::Label(r, label.to_owned(), back, fore));
        self
    }
    pub fn label_color(&mut self, label: &str) -> &mut Self {
        self.try_commit();
        let len = text_color_len(label) as Coord;
        let r = self.next_rectangle(len, 1);
        let back = self.get_color(ColorCode::Background);
        self.defered(DeferedCommand::LabelColor(r, label.to_owned(), back));
        self
    }

    // =======================================================
    //
    // Defered rendering functions
    //
    // =======================================================
    fn render_defered(&mut self, r: Rect, c: &DeferedCommand) {
        match c {
            DeferedCommand::Button(label, col, coltxt) => {
                self.render_button(r, label, *col, *coltxt)
            }
            DeferedCommand::CheckBox(checked, col) => {
                self.draw_checkbox(self.last_cursor(), *checked, *col)
            }
            DeferedCommand::DropDown(checked, col) => {
                self.draw_dropdown(self.last_cursor(), *checked, *col)
            }
            DeferedCommand::Label(r, label, col, coltxt) => {
                self.render_label(*r, label, *col, *coltxt)
            }
            DeferedCommand::LabelColor(r, label, col) => self.render_label_color(*r, label, *col),
            _ => (),
        }
    }
    fn render_label(&mut self, r: Rect, label: &str, col: Color, coltxt: Color) {
        let align = self.next_align.take().unwrap_or(TextAlign::Left);
        self.draw_rect(r, col);
        self.draw_text(r, label, align, coltxt);
    }
    fn render_label_color(&mut self, r: Rect, label: &str, col: Color) {
        let align = self.next_align.take().unwrap_or(TextAlign::Left);
        self.draw_rect(r, col);
        self.draw_text_color(r, label, align);
    }

    fn render_button(&mut self, r: Rect, label: &str, col: Color, coltxt: Color) {
        let align = self.next_align.take().unwrap_or(TextAlign::Center);
        self.draw_rect(r, col);
        self.draw_text(r, label, align, coltxt);
    }
    fn render_frame(&mut self, title: &str, col: Color, coltxt: Color, r: Rect) {
        let title = if title.chars().count() as i32 > r.w - 2 {
            title.chars().take(r.w as usize - 2).collect::<String>()
        } else {
            title.to_owned()
        };
        self.draw_frame(r, &title, col, coltxt);
    }

    // =======================================================
    //
    // Basic drawing functions
    //
    // =======================================================
    fn draw_progress(&mut self, r: Rect, coef: f32, back: Color, fore: Color) {
        self.commands.push(Command::Progress(r, coef, back, fore));
    }
    fn draw_checkbox(&mut self, p: Pos, checked: bool, col: Color) {
        self.commands.push(Command::CheckBox(p, checked, col));
    }
    fn draw_dropdown(&mut self, p: Pos, checked: bool, col: Color) {
        self.commands
            .push(Command::DropDown(Pos { x: p.x + 1, y: p.y }, checked, col));
    }
    fn draw_frame(&mut self, r: Rect, title: &str, col: Color, coltxt: Color) {
        self.commands
            .push(Command::Frame(title.to_owned(), r, col, coltxt));
    }

    fn draw_line(&mut self, x1: Coord, y1: Coord, x2: Coord, y2: Coord, col: Color) {
        self.commands.push(Command::Line(
            Pos { x: x1, y: y1 },
            Pos { x: x2, y: y2 },
            col,
        ));
    }

    fn draw_rect(&mut self, r: Rect, col: Color) {
        self.commands.push(Command::Rect(r, col));
    }

    fn draw_text(&mut self, r: Rect, txt: &str, align: TextAlign, col: Color) {
        let (pos, truncated_text) = format_text(r, txt, align);
        self.commands.push(Command::Text(truncated_text, pos, col));
    }

    fn draw_text_color(&mut self, r: Rect, txt: &str, align: TextAlign) {
        self.commands
            .push(Command::TextColor(txt.to_owned(), r.into(), align));
    }

    fn update_control(&mut self, id: Id, r: &Rect, hold_focus: bool) {
        let mouse_over = r.contains(self.mouse_pos.into());
        let pressed = self.mouse_pressed != 0;
        if mouse_over {
            self.hover = id;
            if pressed {
                self.set_focus(id);
            }
        } else {
            self.hover = NULL_ID.to_owned();
            if self.focus == id
                && ((!hold_focus && pressed) || (hold_focus && self.mouse_down == 0))
            {
                self.set_focus(NULL_ID.to_owned());
            }
        }
    }

    fn start_dnd(&mut self, value: f32) {
        self.dnd_on = true;
        self.dnd_value = value;
        self.dnd_start = self.mouse_pos;
    }

    fn set_focus(&mut self, id: Id) {
        self.focus = id;
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
        fn line(&mut self, _p1: ui::Pos, _p2: ui::Pos, _col: ui::Color) {}
        fn rectangle(&mut self, rect: &ui::Rect, _col: ui::Color) {
            for cx in rect.x as usize..(rect.x + rect.w) as usize {
                for cy in rect.y as usize..(rect.y + rect.h) as usize {
                    self.character[cx][cy] = ' ';
                }
            }
        }
        fn text(&mut self, pos: ui::Pos, txt: &str, _col: ui::Color) {
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
        fn progress(&mut self, _r: &ui::Rect, _value: f32, _back: ui::Color, _fore: ui::Color) {}
        fn frame(&mut self, txt: &str, rect: &ui::Rect, _col: ui::Color, coltxt: ui::Color) {
            let rx = rect.x as usize;
            let ry = rect.y as usize;
            let rx2 = rx + rect.w as usize - 1;
            let ry2 = ry + rect.h as usize - 1;
            self.character[rx][ry] = '1';
            self.character[rx2][ry] = '2';
            self.character[rx][ry2] = '3';
            self.character[rx2][ry2] = '4';
            self.text(rect.into(), txt, coltxt);
        }
        fn checkbox(&mut self, pos: ui::Pos, _checked: bool, _col: ui::Color) {
            self.character[pos.x as usize][pos.y as usize] = '5';
        }
        fn dropdown(&mut self, pos: ui::Pos, _checked: bool, _col: ui::Color) {
            self.character[pos.x as usize][pos.y as usize] = '>';
        }
    }

    #[test]
    fn test_button() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.button("0", "test").align(ui::TextAlign::Left);
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("test", 0, 0));
    }
    #[test]
    fn test_vbox() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.vbox_begin("0", 2);
        ctx.label("1");
        ctx.label("2");
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
        ctx.hbox_begin("0");
        ctx.label("1");
        ctx.label("2");
        ctx.hbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("12", 0, 0));
    }
    #[test]
    fn test_margin() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.vbox_begin("0", 2).margin(1);
        ctx.label("1");
        ctx.label("2");
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
        ctx.vbox_begin("0", 2).padding(1);
        ctx.label("1");
        ctx.label("2");
        ctx.vbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("1", 0, 0));
        assert!(rend.assert("2", 0, 2));
    }
}

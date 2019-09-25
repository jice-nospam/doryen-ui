use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};

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
pub type Id = u64;
const NULL_ID: Id = 0;

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
pub struct ToggleOptions {
    pub align: TextAlign,
    pub group: Option<usize>,
    pub active: bool,
}

#[derive(Default)]
pub struct Context {
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
    next_fixed_pos: Option<Rect>,
    // state management
    focus: Id,
    hover: Id,
    button_state: HashMap<Id, i32>,
    slider_state: HashMap<Id, f32>,
    toggle_group: HashMap<usize, HashSet<Id>>,
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
        self.mouse_pressed = 0;
        self.last_id = NULL_ID.to_owned();
        self.id_prefix.clear();
    }
    fn last_layout(&mut self) -> &mut Layout {
        self.layouts.last_mut().unwrap()
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
    pub fn padding(&mut self, padding: Coord) -> &mut Self {
        self.last_layout().padding(padding);
        self
    }
    pub fn margin(&mut self, margin: Coord) -> &mut Self {
        self.last_layout().margin(margin);
        self
    }
    fn pos(&mut self, x: Coord, y: Coord, width: Coord, height: Coord) -> &mut Self {
        self.next_fixed_pos = Some(Rect::new(x, y, width, height));
        self
    }
    // =======================================================
    //
    // Id management
    //
    // =======================================================
    fn generate_id(&mut self, name: &str) -> Id {
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
    // Containers
    //
    // =======================================================
    /// starts a new grid container.
    /// cols,rows : number of cells in the grid
    /// width,height : size of a cell
    /// Example : 2,2,2,1
    /// 1122
    /// 3344
    /// Margin is around the container :
    /// MMMMMM
    /// M1122M
    /// M3344M
    /// MMMMMM
    /// Padding is between the cells :
    /// 11P22
    /// PPPPP
    /// 33P44
    pub fn grid_begin(
        &mut self,
        id: &str,
        cols: usize,
        rows: usize,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.id_prefix.push(id.to_owned());
        let mut r = self.next_layout_area(width * cols as Coord, height * rows as Coord);
        r.w = width;
        r.h = height;
        let layout = Layout::new_grid(r, cols, rows, 0, 0);
        //println!("g {:?}", layout);
        self.layouts.push(layout);
        self
    }
    pub fn grid_end(&mut self) {
        self.layouts.pop();
        self.id_prefix.pop();
    }
    /// The window behaves like a vbox, but it resets the cursor position
    pub fn window_begin(
        &mut self,
        id: &str,
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.pos(x, y, width, height).vbox_begin(id, width, height)
    }
    pub fn window_end(&mut self) {
        self.vbox_end();
    }
    /// the frame_window behaves like a frame, but it resets the cursor position
    pub fn frame_window_begin(
        &mut self,
        id: &str,
        title: &str,
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.pos(x, y, width, height)
            .frame_begin(id, title, width, height)
    }
    pub fn frame_window_end(&mut self) {
        self.frame_end();
    }
    /// starts a new vertical container
    /// ///
    /// margin is around the container :
    /// MMMM
    /// M11M
    /// M22M
    /// M33M
    /// MMMM
    ///
    /// padding is between the rows :
    /// 11
    /// PP
    /// 22
    /// PP
    /// 33
    pub fn vbox_begin(&mut self, id: &str, width: Coord, height: Coord) -> &mut Self {
        self.id_prefix.push(id.to_owned());
        let r = self.next_layout_area(width, height);
        let layout = Layout::new(LayoutMode::Vertical, r, 0, 0);
        //println!("v {:?}", layout);
        self.layouts.push(layout);
        self
    }
    pub fn vbox_end(&mut self) {
        self.layouts.pop();
        self.id_prefix.pop();
    }
    /// starts a new horizontal container
    ///
    /// margin is around the container :
    /// MMMMMMMM
    /// M112233M
    /// M112233M
    /// MMMMMMMM
    ///
    /// padding is between the columns :
    /// 11P22P33
    /// 11P22P33
    /// 11P22P33
    pub fn hbox_begin(&mut self, id: &str, width: Coord, height: Coord) -> &mut Self {
        self.id_prefix.push(id.to_owned());
        let r = self.next_layout_area(width, height);
        let layout = Layout::new(LayoutMode::Horizontal, r, 0, 0);
        //println!("h {:?}", layout);
        self.layouts.push(layout);
        self
    }
    pub fn hbox_end(&mut self) {
        self.layouts.pop();
        self.id_prefix.pop();
    }
    /// a frame behaves like a vbox with a drawn border and a title
    pub fn frame_begin(&mut self, id: &str, title: &str, width: Coord, height: Coord) -> &mut Self {
        self.vbox_begin(id, width.max((title.chars().count() + 2) as Coord), height)
            .margin(1);
        self.draw_frame(
            self.layouts.last().unwrap().area(),
            title,
            ColorCode::Background,
        );
        self
    }
    pub fn frame_end(&mut self) {
        self.vbox_end();
    }
    /// a popup is a frame_window with an automatic "Ok" button at the bottom
    pub fn popup_begin(
        &mut self,
        id: &str,
        title: &str,
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.frame_window_begin(id, title, x, y, width, height)
    }
    pub fn popup_end(&mut self) -> bool {
        let ret = self.button("popup_ok", "Ok", TextAlign::Center);
        self.frame_window_end();
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
        self.draw_line(r.x, r.y, r.x + r.w, r.y + r.h, ColorCode::Foreground);
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
    fn next_rectangle(&mut self, width: Coord, height: Coord) -> Rect {
        self.layouts.last_mut().unwrap().next_area(width, height)
        //println!("w {:?}", layout);
    }
    fn last_cursor(&self) -> Pos {
        self.layouts.last().unwrap().last_cursor()
    }
    fn next_layout_area(&mut self, w: Coord, h: Coord) -> Rect {
        if let Some(r) = self.next_fixed_pos.take() {
            r
        } else {
            self.layouts.last_mut().unwrap().next_area(w, h)
        }
    }
    pub fn button(&mut self, id: &str, label: &str, align: TextAlign) -> bool {
        let id = self.generate_id(id);
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.update_control(id, &r, false);
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
    /// same as toggle button, but displays a checkbox left to the label
    /// returns (checkbox_status, status_has_changed_this_frame)
    pub fn checkbox(&mut self, id: &str, label: &str, initial_state: bool) -> (bool, bool) {
        let padded_label = "  ".to_owned() + label;
        let pressed = self.button(id, &padded_label, TextAlign::Left);
        let checked = {
            let checked = self
                .button_state
                .entry(self.last_id)
                .or_insert(if initial_state { 1 } else { 0 });
            if pressed {
                *checked = 1 - *checked;
            }
            *checked
        };
        self.draw_checkbox(self.last_cursor(), checked == 1, ColorCode::Text);
        (checked == 1, pressed)
    }

    // =======================================================
    //
    // Toggle button
    //
    // =======================================================

    fn add_group_id(&mut self, group: usize, id: Id) {
        let ids = self.toggle_group.entry(group).or_insert_with(HashSet::new);
        ids.insert(id);
    }
    fn disable_toggle_group(&mut self, group: usize) {
        for id in self.toggle_group.get(&group).unwrap() {
            self.button_state.insert(*id, 0);
        }
    }
    /// a button that switches between active/inactive when clicked.
    /// returns (button_status, status_has_changed_this_frame)
    pub fn toggle(&mut self, id: &str, label: &str, opt: ToggleOptions) -> (bool, bool) {
        let id = self.generate_id(id);
        if let Some(group) = opt.group {
            self.add_group_id(group, id);
        }
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.update_control(id, &r, false);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let mut on = *self
            .button_state
            .get(&self.last_id)
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
    pub fn set_toggle_status(&mut self, toggle_id: Id, status: bool) {
        self.button_state
            .insert(toggle_id, if status { 1 } else { 0 });
    }

    // =======================================================
    //
    // List button
    //
    // =======================================================

    /// a button that cycles over a list of values when clicked
    pub fn list_button_begin(&mut self, id: &str) {
        let id = self.generate_id(id);
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

    /// end the value list.
    /// returns true if the current value has changed this frame
    /// if display_count is true, shows the selected item index / items count when the mouse is hovering the button
    pub fn list_button_end(&mut self, display_count: bool) -> bool {
        let list_button_id = self.last_id();
        assert!(
            self.button_state.get(&list_button_id).is_some(),
            "list_button_end must be called after list_button_begin"
        );
        self.list_button_width += 2;
        let r = self.next_rectangle(self.list_button_width, 1);
        self.update_control(list_button_id, &r, false);
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
    // Sliders
    //
    // =======================================================
    pub fn fslider(
        &mut self,
        id1: &str,
        width: Coord,
        min_val: f32,
        max_val: f32,
        start_val: f32,
    ) -> f32 {
        assert!(min_val < max_val);
        assert!(start_val >= min_val && start_val <= max_val);
        let id = self.generate_id(id1);
        let value = *self.slider_state.entry(id).or_insert(start_val);
        let r = self.next_rectangle(width, 1);
        let was_focus = self.focus == id;
        self.update_control(id, &r, true);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && self.mouse_down == MOUSE_BUTTON_LEFT;
        if pressed {
            if !self.dnd_on {
                self.start_dnd(value);
            } else {
                let delta = self.mouse_pos.0 - self.dnd_start.0;
                let value_delta = delta as f32 * (max_val - min_val) / width as f32;
                let new_value = (self.dnd_value + value_delta).max(min_val).min(max_val);
                self.slider_state.insert(id, new_value);
            }
        } else if was_focus {
            self.dnd_on = false;
        }
        let coef = (value - min_val) / (max_val - min_val);
        let handle_pos = r.x + ((r.w as f32 * coef + 0.5) as Coord).min(r.w - 1);
        self.draw_slider(r, handle_pos, focus || hover);
        value
    }

    pub fn islider(
        &mut self,
        id: &str,
        width: Coord,
        min_val: i32,
        max_val: i32,
        start_val: i32,
    ) -> i32 {
        assert!(min_val < max_val);
        assert!(start_val >= min_val && start_val <= max_val);
        let id = self.generate_id(id);
        let value = *self.button_state.entry(id).or_insert(start_val);
        let r = self.next_rectangle(width, 1);
        let was_focus = self.focus == id;
        self.update_control(id, &r, true);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && self.mouse_down == MOUSE_BUTTON_LEFT;
        if pressed {
            if !self.dnd_on {
                self.start_dnd(value as f32);
            } else {
                let delta = self.mouse_pos.0 - self.dnd_start.0;
                let value_delta = delta as f32 * (max_val - min_val) as f32 / width as f32;
                let new_value = ((self.dnd_value + value_delta) as i32)
                    .max(min_val)
                    .min(max_val);
                self.button_state.insert(id, new_value);
            }
        } else if was_focus {
            self.dnd_on = false;
        }
        let coef = (value - min_val) as f32 / (max_val - min_val) as f32;
        let handle_pos = r.x + ((r.w as f32 * coef + 0.5) as Coord).min(r.w - 1);
        self.draw_slider(r, handle_pos, focus || hover);
        value
    }

    fn draw_slider(&mut self, r: Rect, handle_pos: Coord, active: bool) {
        let col = if active {
            ColorCode::ButtonBackgroundHover
        } else {
            ColorCode::ButtonBackground
        };
        self.draw_rect(r, col);
        self.draw_line(r.x, r.y, r.x + r.w, r.y + r.h, ColorCode::Text);
        let handle_area = Rect {
            x: handle_pos,
            y: r.y,
            w: 1,
            h: 1,
        };
        self.draw_text(handle_area, "|", TextAlign::Left, ColorCode::Text);
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

    fn draw_line(&mut self, x1: Coord, y1: Coord, x2: Coord, y2: Coord, col: ColorCode) {
        self.commands.push(Command::Line(
            Pos { x: x1, y: y1 },
            Pos { x: x2, y: y2 },
            col,
        ));
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
        ctx.button("0", "test", ui::TextAlign::Left);
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("test", 0, 0));
    }
    #[test]
    fn test_vbox() {
        let mut rend = AsciiRenderer::new();
        let mut ctx = ui::Context::new();
        ctx.begin();
        ctx.vbox_begin("0", 1, 0);
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
        ctx.hbox_begin("0", 0, 1);
        ctx.label("1", ui::TextAlign::Left);
        ctx.label("2", ui::TextAlign::Left);
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
        ctx.vbox_begin("0", 3, 0).margin(1);
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
        ctx.vbox_begin("0", 3, 0).padding(1);
        ctx.label("1", ui::TextAlign::Left);
        ctx.label("2", ui::TextAlign::Left);
        ctx.vbox_end();
        ctx.end();
        ctx.render(&mut rend);
        assert!(rend.assert("1", 0, 0));
        assert!(rend.assert("2", 0, 2));
    }
}

use crate::{Command, Context, Pos, Rect, MOUSE_BUTTON_LEFT};
use doryen_rs::{Color, Console, DoryenApi, TextAlign, CHAR_LINE_H};

pub fn text_color_len(txt: &str) -> usize {
    Console::text_color_len(txt)
}

pub fn update_doryen_input_data(api: &mut dyn DoryenApi, ctx: &mut Context) {
    let input = api.input();
    let (mx, my) = input.mouse_pos();
    ctx.input_mouse_pos(mx, my);
    if input.mouse_button_pressed(0) {
        ctx.input_mouse_down(MOUSE_BUTTON_LEFT);
    } else if input.mouse_button_released(0) {
        ctx.input_mouse_up(MOUSE_BUTTON_LEFT);
    }
}

impl From<crate::TextAlign> for TextAlign {
    fn from(t: crate::TextAlign) -> Self {
        match t {
            crate::TextAlign::Left => TextAlign::Left,
            crate::TextAlign::Right => TextAlign::Right,
            crate::TextAlign::Center => TextAlign::Center,
        }
    }
}

pub fn render_doryen(con: &mut Console, ctx: &mut Context) {
    for c in ctx.get_render_commands().iter() {
        match c {
            Command::Rect(r, col) => render_rect(con, &r, *col),
            Command::Line(p1, p2, col) => render_line(con, *p1, *p2, *col),
            Command::Text(txt, pos, col) => render_text(con, *pos, &txt, *col),
            Command::TextColor(txt, pos, align) => {
                render_text_color(con, *pos, &txt, (*align).into())
            }
            Command::Frame(txt, r, col, coltext) => render_frame(con, &txt, &r, *col, *coltext),
            Command::CheckBox(pos, checked, col) => {
                render_checkbox(con, *pos, *checked, *col);
            }
        }
    }
}

fn render_rect(con: &mut Console, r: &Rect, col: Color) {
    con.area(r.x, r.y, r.w as u32, r.h as u32, None, Some(col), None);
}
fn render_line(con: &mut Console, p1: Pos, p2: Pos, col: Color) {
    con.area(
        p1.x,
        p1.y,
        (p2.x - p1.x) as u32,
        (p2.y - p1.y) as u32,
        Some(col),
        None,
        Some(CHAR_LINE_H),
    );
}
fn render_text(con: &mut Console, pos: Pos, txt: &str, col: Color) {
    con.print(pos.x, pos.y, txt, TextAlign::Left, Some(col), None);
}
fn render_text_color(con: &mut Console, pos: Pos, txt: &str, align: TextAlign) {
    con.print_color(pos.x, pos.y, txt, align, None);
}
fn render_checkbox(con: &mut Console, pos: Pos, checked: bool, col: Color) {
    con.ascii(pos.x, pos.y, if checked { 225 } else { 224 });
    con.fore(pos.x, pos.y, col);
}
fn render_frame(con: &mut Console, txt: &str, r: &Rect, col: Color, txtcol: Color) {
    let con = con;
    con.rectangle(
        r.x,
        r.y,
        r.w as u32,
        r.h as u32,
        Some(txtcol),
        Some(col),
        None,
    );
    con.print(
        r.x + r.w / 2,
        r.y,
        txt,
        TextAlign::Center,
        Some(txtcol),
        None,
    );
}

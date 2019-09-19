use std::collections::HashMap;
use std::hash::BuildHasher;

use crate::{ColorCode, Command, Context, Pos, Rect};
use doryen_rs::{Color, Console, DoryenApi, TextAlign};

pub fn update_doryen_input_data(api: &mut dyn DoryenApi, ctx: &mut Context) {
    let input = api.input();
    let (mx, my) = input.mouse_pos();
    let mpos = Pos {
        x: mx as i32,
        y: my as i32,
    };
    ctx.input_mouse_pos(mpos);
    if input.mouse_button_pressed(0) {
        ctx.input_mouse_down(1);
    } else if input.mouse_button_released(0) {
        ctx.input_mouse_up(1);
    }
}

pub fn render_doryen<S: BuildHasher>(
    con: &mut Console,
    ctx: &mut Context,
    colormap: &HashMap<ColorCode, Color, S>,
) {
    for c in ctx.get_render_commands() {
        match c {
            Command::Rect(r, col) => render_rect(con, &r, conv_color(col, colormap)),
            Command::Text(txt, pos, col) => render_text(con, pos, &txt, conv_color(col, colormap)),
            Command::Frame(txt, r, col) => render_frame(
                con,
                &txt,
                &r,
                conv_color(col, colormap),
                conv_color(ColorCode::Text, colormap),
            ),
            Command::CheckBox(pos, checked, col) => {
                render_checkbox(con, pos, checked, conv_color(col, colormap));
            }
        }
    }
}

fn conv_color<S: BuildHasher>(col: ColorCode, colormap: &HashMap<ColorCode, Color, S>) -> Color {
    *colormap.get(&col).unwrap()
}

fn render_rect(con: &mut Console, r: &Rect, col: Color) {
    con.area(r.x, r.y, r.w as u32, r.h as u32, None, Some(col), None);
}
fn render_text(con: &mut Console, pos: Pos, txt: &str, col: Color) {
    con.print(pos.x, pos.y, txt, TextAlign::Left, Some(col), None);
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

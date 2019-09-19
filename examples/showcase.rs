extern crate doryen_rs;
extern crate doryen_ui;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine, TextAlign};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct Showcase {
    ctx: ui::Context,
    status: String,
    button_popup: bool,
}

impl Showcase {
    pub fn new() -> Self {
        Default::default()
    }
    fn update_input_data(&mut self, api: &mut dyn DoryenApi) {
        let input = api.input();
        let (mx, my) = input.mouse_pos();
        let mpos = ui::Pos {
            x: mx as i32,
            y: my as i32,
        };
        self.ctx.input_mouse_pos(mpos);
        if input.mouse_button_pressed(0) {
            self.ctx.input_mouse_down(1);
        } else if input.mouse_button_released(0) {
            self.ctx.input_mouse_up(1);
        }
    }
    fn build_ui(&mut self) {
        self.ctx.begin();
        self.ctx.frame_start("buttons", 18, 5);
        if self.ctx.button("button", ui::TextAlign::Center) {
            self.button_popup = true;
        }
        if self.button_popup {
            self.ctx.popup_start("button pressed!", 10, 10, 19, 3);
            if self.ctx.popup_end() {
                self.button_popup = false;
            }
        }
        self.ctx.toggle("toggle", ui::TextAlign::Center, false);
        self.ctx.checkbox("checkbox", false);
        self.ctx.frame_end();
        self.ctx.label(&self.status, ui::TextAlign::Left);
        self.ctx.end();
    }
    fn render_rect(&self, api: &mut dyn DoryenApi, r: &ui::Rect, col: Color) {
        api.con()
            .area(r.x, r.y, r.w as u32, r.h as u32, None, Some(col), None);
    }
    fn render_text(&self, api: &mut dyn DoryenApi, x: i32, y: i32, txt: &str, col: Color) {
        api.con().print(x, y, txt, TextAlign::Left, Some(col), None);
    }
    fn render_checkbox(&self, api: &mut dyn DoryenApi, x: i32, y: i32, checked: bool, col: Color) {
        api.con().ascii(x, y, if checked { 225 } else { 224 });
        api.con().fore(x, y, col);
    }
    fn render_frame(&self, api: &mut dyn DoryenApi, txt: &str, r: &ui::Rect, col: Color) {
        let con = api.con();
        let txtcol = conv_color(ui::ColorCode::Text);
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
}

impl Engine for Showcase {
    fn init(&mut self, _api: &mut dyn DoryenApi) {}
    fn update(&mut self, api: &mut dyn DoryenApi) {
        self.update_input_data(api);
        self.build_ui();
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con()
            .clear(None, Some((0, 0, 0, 255)), Some(' ' as u16));
        for c in self.ctx.get_render_commands() {
            match c {
                ui::Command::Rect(r, col) => self.render_rect(api, &r, conv_color(col)),
                ui::Command::Text(txt, pos, col) => {
                    self.render_text(api, pos.x, pos.y, &txt, conv_color(col))
                }
                ui::Command::Frame(txt, r, col) => {
                    self.render_frame(api, &txt, &r, conv_color(col))
                }
                ui::Command::CheckBox(pos, checked, col) => {
                    self.render_checkbox(api, pos.x, pos.y, checked, conv_color(col));
                }
            }
        }
    }
    fn resize(&mut self, _api: &mut dyn DoryenApi) {}
}

fn conv_color(c: ui::ColorCode) -> Color {
    match c {
        ui::ColorCode::Background => (10, 10, 20, 255),
        ui::ColorCode::Foreground => (200, 200, 255, 255),
        ui::ColorCode::ButtonBackground => (50, 60, 70, 255),
        ui::ColorCode::ButtonBackgroundHover => (100, 130, 170, 255),
        ui::ColorCode::ButtonBackgroundFocus => (60, 80, 100, 255),
        ui::ColorCode::Text => (200, 220, 250, 255),
    }
}

fn main() {
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 8,
        screen_height: CONSOLE_HEIGHT * 8,
        window_title: "doryen-ui showcase".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(Showcase::new()));
    app.run();
}

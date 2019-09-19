extern crate doryen_rs;
extern crate doryen_ui;

use std::collections::HashMap;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct Showcase {
    ctx: ui::Context,
    status: String,
    button_popup: bool,
    colormap: HashMap<ui::ColorCode, Color>,
}

impl Showcase {
    pub fn new() -> Self {
        Default::default()
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
}

impl Engine for Showcase {
    fn init(&mut self, _api: &mut dyn DoryenApi) {
        self.colormap
            .insert(ui::ColorCode::Background, (10, 10, 20, 255));
        self.colormap
            .insert(ui::ColorCode::Foreground, (200, 200, 255, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackground, (50, 60, 70, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackgroundHover, (100, 130, 170, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackgroundFocus, (60, 80, 100, 255));
        self.colormap
            .insert(ui::ColorCode::Text, (200, 220, 250, 255));
    }
    fn update(&mut self, api: &mut dyn DoryenApi) {
        ui::update_doryen_input_data(api, &mut self.ctx);
        self.build_ui();
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con()
            .clear(None, Some((0, 0, 0, 255)), Some(' ' as u16));
        ui::render_doryen(api.con(), &mut self.ctx, &self.colormap);
    }
    fn resize(&mut self, _api: &mut dyn DoryenApi) {}
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

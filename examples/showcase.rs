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
    button_popup: bool,
    colormap: HashMap<ui::ColorCode, Color>,
}

impl Showcase {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        let ctx = &mut self.ctx;
        ctx.begin();
        ctx.frame_begin("buttons", "buttons", 17, 6);
        if ctx
            .button("button", "  button")
            .align(ui::TextAlign::Left)
            .pressed()
        {
            self.button_popup = true;
        }
        if self.button_popup {
            ctx.popup_begin("button_msg", "button pressed!", 20, 10, 19, 3);
            if ctx.popup_end() {
                self.button_popup = false;
            }
        }
        ctx.toggle("toggle", "  toggle", Default::default())
            .align(ui::TextAlign::Left);
        ctx.checkbox("checkbox", "checkbox", false);
        ctx.list_button_begin("list_button");
        ctx.list_button_item("list value 1", ui::TextAlign::Center);
        ctx.list_button_item("list value 2", ui::TextAlign::Center);
        ctx.list_button_end(true);
        ctx.frame_end();
        ctx.frame_begin("margin", "margin", 17, 7).margin(2);
        ctx.toggle("margin", "margin", Default::default())
            .align(ui::TextAlign::Left);
        ctx.frame_end();
        ctx.frame_begin("padding", "padding", 17, 3);
        ctx.hbox_begin("pad_hbox").padding(6);
        ctx.toggle("pad1", "1", Default::default())
            .align(ui::TextAlign::Left);
        ctx.toggle("pad2", "2", Default::default())
            .align(ui::TextAlign::Left);
        ctx.toggle("pad3", "3", Default::default())
            .align(ui::TextAlign::Left);
        ctx.hbox_end();
        ctx.frame_end();
        ctx.frame_begin("grid_frame", "grid", 17, 4);
        ctx.grid_begin("grid", 3, 2, 5, 1);
        ctx.toggle("grid1", "1", Default::default())
            .align(ui::TextAlign::Left);
        ctx.toggle("grid2", "2", Default::default())
            .align(ui::TextAlign::Left);
        ctx.toggle("grid3", "3", Default::default())
            .align(ui::TextAlign::Left);
        ctx.toggle("grid4", "4", Default::default())
            .align(ui::TextAlign::Left);
        ctx.grid_end();
        ctx.frame_end();
        ctx.frame_begin("labels", "labels", 17, 5);
        ctx.label("right").align(ui::TextAlign::Right);
        ctx.label("center").align(ui::TextAlign::Center);
        ctx.label_color("#[yellow]colored #[orange]labels");
        ctx.frame_end();
        ctx.frame_begin("trunc", "truncation", 17, 5);
        ctx.label("truncated right text")
            .align(ui::TextAlign::Right);
        ctx.label("truncated centered text")
            .align(ui::TextAlign::Center);
        ctx.label("truncated left text");
        ctx.frame_end();
        ctx.frame_begin("sliders", "sliders", 17, 6);
        let value = ctx.fslider("fslider", 15, 0.0, 10.0, 5.0);
        ctx.label(&format!("{:.2}", value));
        let value = ctx.islider("islider", 15, 0, 10, 5);
        ctx.label(&format!("{}", value));
        ctx.frame_end();
        ctx.end();
    }
}

impl Engine for Showcase {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.colormap
            .insert(ui::ColorCode::Background, (245, 245, 245, 255));
        self.colormap
            .insert(ui::ColorCode::Foreground, (200, 200, 255, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackground, (201, 201, 201, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackgroundHover, (201, 239, 254, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackgroundFocus, (151, 232, 235, 255));
        self.colormap
            .insert(ui::ColorCode::Text, (104, 104, 104, 255));
        api.con().register_color("yellow", (200, 200, 100, 255));
        api.con().register_color("orange", (150, 150, 50, 255));
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

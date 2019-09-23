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
        ctx.frame_begin("buttons", 17, 5, Default::default());
        if ctx.button("  button", ui::TextAlign::Left) {
            self.button_popup = true;
        }
        if self.button_popup {
            ctx.popup_begin(
                "button pressed!",
                19,
                3,
                ui::LayoutOptions {
                    pos: Some((20, 10)),
                    ..Default::default()
                },
            );
            if ctx.popup_end() {
                self.button_popup = false;
            }
        }
        let toggle_opt = ui::ToggleOptions {
            align: ui::TextAlign::Left,
            ..Default::default()
        };
        ctx.toggle("  toggle", toggle_opt);
        ctx.checkbox("checkbox", false);
        ctx.frame_end();
        ctx.frame_begin(
            "margin",
            17,
            7,
            ui::LayoutOptions {
                margin: 2,
                ..Default::default()
            },
        );
        ctx.toggle("margin", toggle_opt);
        ctx.frame_end();
        ctx.frame_begin("padding", 17, 3, Default::default());
        ctx.hbox_begin(
            0,
            1,
            ui::LayoutOptions {
                padding: 6,
                ..Default::default()
            },
        );
        ctx.toggle("1", toggle_opt);
        ctx.toggle("2", toggle_opt);
        ctx.toggle("3", toggle_opt);
        ctx.hbox_end();
        ctx.frame_end();
        ctx.frame_begin("grid", 17, 4, Default::default());
        ctx.grid_begin(3, 2, 5, 1, Default::default());
        ctx.toggle("1", toggle_opt);
        ctx.toggle("2", toggle_opt);
        ctx.toggle("3", toggle_opt);
        ctx.toggle("4", toggle_opt);
        ctx.grid_end();
        ctx.frame_end();
        ctx.frame_begin("labels", 17, 5, Default::default());
        ctx.label("right", ui::TextAlign::Right);
        ctx.label("center", ui::TextAlign::Center);
        ctx.label_color("#[yellow]colored #[orange]labels", ui::TextAlign::Left);
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

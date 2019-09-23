extern crate doryen_rs;
extern crate doryen_ui;

use std::collections::HashMap;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct RfxGen {
    ctx: ui::Context,
    colormap: HashMap<ui::ColorCode, Color>,
}

impl RfxGen {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        self.ctx.begin();
        self.ctx.hbox_begin(
            20,
            0,
            ui::LayoutOptions {
                margin:2,
                ..Default::default()
            },
        );
        self.ctx.vbox_begin(20, 1, ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            });
        {
            self.ctx.label("rFXGen v2.1", ui::TextAlign::Left);
            self.ctx.button(&format!(" {} Pickup/Coin",184 as char), ui::TextAlign::Left);
            self.ctx.button(&format!(" {}{} Laser/Shoot", 196 as char,15 as char), ui::TextAlign::Left);
            self.ctx.button(&format!(" {} Explosion",15 as char), ui::TextAlign::Left);
            self.ctx.button(&format!(" {} PowerUp",251 as char), ui::TextAlign::Left);
            self.ctx.button(&format!(" {} Hit/Hurt", 2 as char), ui::TextAlign::Left);
            self.ctx.button(" ^ Jump", ui::TextAlign::Left);
            self.ctx.button(&format!(" {} Bip/Select", 26 as char), ui::TextAlign::Left);
            self.ctx.separator();
            self.ctx.toggle(
                " Square",
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: true,
                },
            );
            self.ctx.toggle(
                " Sawtooth",
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: false,
                },
            );
            self.ctx.toggle(
                " Sinwave",
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: false,
                },
            );
            self.ctx.toggle(
                "Noise",
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: false,
                },
            );
            self.ctx.separator();
            self.ctx.button("Mutate", ui::TextAlign::Center);
            self.ctx.button("Randomize", ui::TextAlign::Center);
        }
        self.ctx.vbox_end();
        self.ctx.hbox_end();
        self.ctx.end();
    }
}

impl Engine for RfxGen {
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
        api.con().register_color("grey", (180, 180, 180, 255));
        api.con().register_color("text", (200, 200, 80, 255));
    }
    fn update(&mut self, api: &mut dyn DoryenApi) {
        ui::update_doryen_input_data(api, &mut self.ctx);
        self.build_ui();
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con()
            .clear(None, Some((245, 245, 245, 255)), Some(' ' as u16));
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
        window_title: "rFXGen v2.1 - A simple and easy-to-use sounds generator".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(RfxGen::new()));
    app.run();
}

extern crate doryen_rs;
extern crate doryen_ui;

use std::collections::HashMap;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 45;

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
        self.ctx.hbox_begin("columns", 20, 0).padding(1).margin(2);
        self.ctx.vbox_begin("left_col", 20, 1).padding(1);
        {
            self.ctx.label("rFXGen v2.1", ui::TextAlign::Left);
            self.ctx.button(
                "coin",
                &format!(" {}  Pickup/Coin", 184 as char),
                ui::TextAlign::Left,
            );
            self.ctx.button(
                "laser",
                &format!(" {}* Laser/Shoot", 196 as char),
                ui::TextAlign::Left,
            );
            self.ctx.button(
                "explo",
                &format!(" {}  Explosion", 15 as char),
                ui::TextAlign::Left,
            );
            self.ctx.button(
                "powerup",
                &format!(" {}  PowerUp", 251 as char),
                ui::TextAlign::Left,
            );
            self.ctx.button(
                "hit",
                &format!(" {}  Hit/Hurt", 2 as char),
                ui::TextAlign::Left,
            );
            self.ctx.button("jump", " ^  Jump", ui::TextAlign::Left);
            self.ctx.button(
                "select",
                &format!(" {}  Bip/Select", 26 as char),
                ui::TextAlign::Left,
            );
            self.ctx.separator();
            self.ctx.toggle(
                "square",
                &format!(" {} Square", 224 as char),
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: true,
                },
            );
            self.ctx.toggle(
                "saw",
                " ^ Sawtooth",
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: false,
                },
            );
            self.ctx.toggle(
                "sin",
                " ~ Sinwave",
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: false,
                },
            );
            self.ctx.toggle(
                "noise",
                &format!(" {} Noise", 176 as char),
                ui::ToggleOptions {
                    group: Some(1),
                    align: ui::TextAlign::Left,
                    active: false,
                },
            );
            self.ctx.separator();
            self.ctx.button("mutate", "Mutate", ui::TextAlign::Center);
            self.ctx.button("rand", "Randomize", ui::TextAlign::Center);
        }
        self.ctx.vbox_end();
        self.ctx.vbox_begin("sliders", 40, 1).padding(1);
        self.slider("volume", 0.0, 100.0, 60.0, true);
        self.ctx.separator();
        self.slider("attack time", 0.0, 1.0, 0.0, false);
        self.slider("sustain time", 0.0, 1.0, 0.0, false);
        self.slider("sustain punch", 0.0, 1.0, 0.0, false);
        self.slider("decay time", 0.0, 1.0, 0.0, false);
        self.ctx.separator();
        self.slider("start frequency", 0.0, 1.0, 0.0, false);
        self.slider("min frequency", 0.0, 1.0, 0.0, false);
        self.slider("slide", 0.0, 1.0, 0.0, false);
        self.slider("delta slide", 0.0, 1.0, 0.0, false);
        self.slider("vibrato depth", 0.0, 1.0, 0.0, false);
        self.slider("vibrato speed", 0.0, 1.0, 0.0, false);
        self.ctx.separator();
        self.slider("change amount", 0.0, 1.0, 0.0, false);
        self.slider("change speed", 0.0, 1.0, 0.0, false);
        self.ctx.separator();
        self.slider("square duty", 0.0, 1.0, 0.0, false);
        self.slider("duty sweep", 0.0, 1.0, 0.0, false);
        self.ctx.separator();
        self.slider("repeat speed", 0.0, 1.0, 0.0, false);
        self.ctx.vbox_end();
        self.ctx.vbox_begin("right_column", 16, 1).padding(1);
        self.ctx.checkbox("play_on_change", "Play on change", true);
        self.ctx.button(
            "play",
            &format!("{} Play Sound", 16 as char),
            ui::TextAlign::Center,
        );
        self.ctx.hbox_begin("slots", 4, 1).padding(1);
        self.ctx.label("Slot", ui::TextAlign::Right);
        self.ctx.toggle(
            "slot1",
            "1",
            ui::ToggleOptions {
                active: true,
                group: Some(2),
                ..Default::default()
            },
        );
        self.ctx.toggle(
            "slot2",
            "2",
            ui::ToggleOptions {
                group: Some(2),
                ..Default::default()
            },
        );
        self.ctx.toggle(
            "slot3",
            "3",
            ui::ToggleOptions {
                group: Some(2),
                ..Default::default()
            },
        );
        self.ctx.toggle(
            "slot4",
            "4",
            ui::ToggleOptions {
                group: Some(2),
                ..Default::default()
            },
        );
        self.ctx.hbox_end();
        self.ctx.separator();
        self.ctx.button(
            "load",
            &format!("{} Load Sound", 30 as char),
            ui::TextAlign::Center,
        );
        self.ctx.button(
            "save",
            &format!("{} Save Sound", 31 as char),
            ui::TextAlign::Center,
        );
        self.ctx.separator();
        self.list_button("freq", &["44100 Hz", "22050 Hz"]);
        self.list_button("bits", &["16 bit", "32 bit", "8 bit"]);
        self.list_button("fmt", &["WAV", "MP3", "OGG"]);
        self.ctx.button(
            "export",
            &format!("{} Export", 18 as char),
            ui::TextAlign::Center,
        );
        self.ctx.separator();
        self.ctx.label("Visual style :", ui::TextAlign::Left);
        self.list_button("vstyle", &["default", "jungle", "candy", "lavanda"]);
        self.ctx.toggle(
            "screen",
            "Screen size x2",
            ui::ToggleOptions {
                align: ui::TextAlign::Center,
                ..Default::default()
            },
        );
        self.ctx.separator();
        self.ctx.button("about", "i ABOUT", ui::TextAlign::Center);
        self.ctx.vbox_end();
        self.ctx.hbox_end();
        self.ctx.end();
    }
    fn list_button(&mut self, label: &str, values: &[&str]) {
        self.ctx.list_button_begin(label);
        for value in values {
            self.ctx.list_button_item(value, ui::TextAlign::Left);
        }
        self.ctx.list_button_end(true);
    }
    fn slider(&mut self, label: &str, min_val: f32, max_val: f32, start_val: f32, use_int: bool) {
        self.ctx.grid_begin(label, 3, 1, 18, 1);
        self.ctx.label(label, ui::TextAlign::Right);
        if use_int {
            let value =
                self.ctx
                    .islider("is", 10, min_val as i32, max_val as i32, start_val as i32);
            self.ctx.label(&format!("{}", value), ui::TextAlign::Left);
        } else {
            let value = self.ctx.fslider("fs", 10, min_val, max_val, start_val);
            self.ctx
                .label(&format!("{:.2}", value), ui::TextAlign::Left);
        }
        self.ctx.grid_end();
    }
}

impl Engine for RfxGen {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.colormap
            .insert(ui::ColorCode::Background, (245, 245, 245, 255));
        self.colormap
            .insert(ui::ColorCode::Foreground, (104, 104, 104, 255));
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

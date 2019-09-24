extern crate doryen_rs;
extern crate doryen_ui;

use std::collections::HashMap;

use doryen_rs::{App, AppOptions, Color, DoryenApi, Engine};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct Astacia {
    ctx: ui::Context,
    colormap: HashMap<ui::ColorCode, Color>,
    option_panel: bool,
}

impl Astacia {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        self.ctx.begin();
        self.ctx.vbox_begin(
            "main_menu",
            15,
            0,
            ui::LayoutOptions {
                padding: 1,
                pos: Some((5, 30)),
                ..Default::default()
            },
        );
        if self
            .ctx
            .button("new_game", "New game", ui::TextAlign::Center)
        {
            self.option_panel = false;
        }
        if self
            .ctx
            .button("continue", "Continue", ui::TextAlign::Center)
        {
            self.option_panel = false;
        }
        if self
            .ctx
            .button("options", "Options ", ui::TextAlign::Center)
        {
            self.option_panel = true;
        }
        if self.option_panel {
            self.options_panel();
        }
        if self.ctx.button("quit", "Quit game", ui::TextAlign::Center) {
            self.option_panel = false;
        }
        self.ctx.vbox_end();
        self.ctx.end();
    }

    fn options_panel(&mut self) {
        self.ctx.frame_begin(
            "options",
            "Options",
            50,
            40,
            ui::LayoutOptions {
                margin: 3,
                padding: 1,
                pos: Some((25, 5)),
            },
        );
        self.ctx.label("Game settings", ui::TextAlign::Left);
        self.ctx.grid_begin(
            "game_settings",
            2,
            3,
            22,
            2,
            ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            },
        );
        {
            self.list_button("Font", &["arial_8x8.png", "consolas_12x12.png"]);
            self.list_button("FPS", &["30", "60"]);
            self.list_button("Resolution", &["128x96", "96x72", "80x50"]);
        }
        self.ctx.grid_end();

        self.ctx.label("Controls", ui::TextAlign::Left);
        self.ctx.grid_begin(
            "controls",
            2,
            9,
            22,
            2,
            ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            },
        );
        {
            self.list_button("Move up", &["Arrow up"]);
            self.list_button("Move down", &["Arrow down"]);
            self.list_button("Move left", &["Arrow left"]);
            self.list_button("Move right", &["Arrow right"]);
            self.list_button("Equipment", &["E"]);
            self.list_button("Inventory", &["I"]);
            self.list_button("Talk to NPC", &["T"]);
            self.list_button("Show message", &["M"]);
            self.list_button("Return / Menu", &["ESC"]);
        }
        self.ctx.grid_end();
        self.ctx
            .hbox_begin("options_actions", 0, 1, Default::default());
        if self.ctx.button("ok", "   Ok   ", ui::TextAlign::Left) {
            self.option_panel = false;
        }
        if self.ctx.button("cancel", " Cancel ", ui::TextAlign::Left) {
            self.option_panel = false;
        }
        self.ctx.hbox_end();
        self.ctx.frame_end();
    }

    fn list_button(&mut self, label: &str, values: &[&str]) {
        self.ctx
            .label(&format!("{} :", label), ui::TextAlign::Right);
        self.ctx.list_button_begin(label);
        for value in values {
            self.ctx.list_button_item(value, ui::TextAlign::Left);
        }
        self.ctx.list_button_end(true);
    }
}

impl Engine for Astacia {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.colormap
            .insert(ui::ColorCode::Background, (0, 0, 0, 255));
        self.colormap
            .insert(ui::ColorCode::Foreground, (220, 220, 180, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackground, (10, 10, 10, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackgroundHover, (50, 50, 50, 255));
        self.colormap
            .insert(ui::ColorCode::ButtonBackgroundFocus, (100, 100, 100, 255));
        self.colormap
            .insert(ui::ColorCode::Text, (200, 200, 80, 255));
        api.con().register_color("grey", (180, 180, 180, 255));
        api.con().register_color("text", (200, 200, 80, 255));
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
        window_title: "Rise of Astacia".to_owned(),
        font_path: "terminal_8x8.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
    });
    app.set_engine(Box::new(Astacia::new()));
    app.run();
}

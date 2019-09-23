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
            15,
            0,
            ui::LayoutOptions {
                padding: 1,
                pos: Some((5, 30)),
                ..Default::default()
            },
        );
        if self.ctx.button("New game", ui::TextAlign::Center) {
            self.option_panel = false;
        }
        if self.ctx.button("Continue", ui::TextAlign::Center) {
            self.option_panel = false;
        }
        if self.ctx.button("Options ", ui::TextAlign::Center) {
            self.option_panel = true;
        }
        if self.option_panel {
            self.options_panel();
        }
        if self.ctx.button("Quit game", ui::TextAlign::Center) {
            self.option_panel = false;
        }
        self.ctx.vbox_end();
        self.ctx.end();
    }

    fn options_panel(&mut self) {
        self.ctx.frame_begin(
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
            2,
            3,
            20,
            1,
            ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            },
        );
        {
            self.drop_down("Font", "arial_8x8.png");
            self.drop_down("FPS", "30");
            self.drop_down("Resolution", "128x96");
        }
        self.ctx.grid_end();

        self.ctx.label("Controls", ui::TextAlign::Left);
        self.ctx.grid_begin(
            2,
            9,
            20,
            1,
            ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            },
        );
        {
            self.drop_down("Move up", "Arrow up");
            self.drop_down("Move down", "Arrow down");
            self.drop_down("Move left", "Arrow left");
            self.drop_down("Move right", "Arrow right");
            self.drop_down("Equipment", "E");
            self.drop_down("Inventory", "I");
            self.drop_down("Talk to NPC", "T");
            self.drop_down("Show message", "M");
            self.drop_down("Return / Menu", "ESC");
        }
        self.ctx.grid_end();
        self.ctx.hbox_begin(0, 1, Default::default());
        if self.ctx.button("   Ok   ", ui::TextAlign::Left) {
            self.option_panel = false;
        }
        if self.ctx.button(" Cancel ", ui::TextAlign::Left) {
            self.option_panel = false;
        }
        self.ctx.hbox_end();
        self.ctx.frame_end();
    }

    fn drop_down(&mut self, label: &str, value: &str) {
        self.ctx
            .label(&format!("{} :", label), ui::TextAlign::Right);
        self.ctx.label_color(
            &format!("#[text][ #[grey]{:13}#[]    ]", value),
            ui::TextAlign::Left,
        );
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

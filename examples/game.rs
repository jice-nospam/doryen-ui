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
}

impl Astacia {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        self.ctx.begin();
        self.ctx.fixed_layout_begin(5, 30);
        self.ctx.vbox_begin(
            15,
            ui::LayoutOptions {
                padding: 1,
                ..Default::default()
            },
        );
        if self
            .ctx
            .toggle("  New game", ui::TextAlign::Left, false, Some(1))
        {}
        if self
            .ctx
            .toggle("  Continue", ui::TextAlign::Left, false, Some(1))
        {}
        if self
            .ctx
            .toggle("  Options", ui::TextAlign::Left, false, Some(1))
        {
            self.ctx.fixed_layout_begin(25, 5);
            self.ctx.frame_begin(
                "Options",
                50,
                40,
                ui::LayoutOptions {
                    margin: 3,
                    padding: 1,
                    ..Default::default()
                },
            );
            self.ctx.label("Game settings", ui::TextAlign::Left);
            self.ctx.label("Font :", ui::TextAlign::Right);
            self.ctx.label("FPS :", ui::TextAlign::Right);
            self.ctx.label("Resolution :", ui::TextAlign::Right);

            self.ctx.label("Controls", ui::TextAlign::Left);
            self.ctx.label("Move up :", ui::TextAlign::Right);
            self.ctx.label("Move down :", ui::TextAlign::Right);
            self.ctx.label("Move left :", ui::TextAlign::Right);
            self.ctx.label("Move right :", ui::TextAlign::Right);
            self.ctx.label("Equipment :", ui::TextAlign::Right);
            self.ctx.label("Inventory :", ui::TextAlign::Right);
            self.ctx.label("Talk to NPC :", ui::TextAlign::Right);
            self.ctx.label("Show message :", ui::TextAlign::Right);
            self.ctx.label("Return / Menu :", ui::TextAlign::Right);
            self.ctx.hbox_begin(1, Default::default());
            if self.ctx.button("   Ok   ", ui::TextAlign::Left) {}
            if self.ctx.button(" Cancel ", ui::TextAlign::Left) {}
            self.ctx.hbox_end();
            self.ctx.frame_end();
            self.ctx.fixed_layout_end();
        }
        if self
            .ctx
            .toggle("  Quit game", ui::TextAlign::Left, false, Some(1))
        {}
        self.ctx.vbox_end();
        self.ctx.fixed_layout_end();
        self.ctx.end();
    }
}

impl Engine for Astacia {
    fn init(&mut self, _api: &mut dyn DoryenApi) {
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

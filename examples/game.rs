extern crate doryen_rs;
extern crate doryen_ui;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, UpdateEvent};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct Astacia {
    ctx: ui::Context,
    option_panel: bool,
}

impl Astacia {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        self.ctx.begin();
        self.ctx
            .window_begin("main_menu", 5, 30, 15, 0)
            .padding(1)
            .min_width(15);
        if self.ctx.button("new_game", "New game").pressed() {
            self.option_panel = false;
        }
        if self.ctx.button("continue", "Continue").pressed() {
            self.option_panel = false;
        }
        if self.ctx.button("options", "Options ").pressed() {
            self.option_panel = true;
        }
        if self.option_panel {
            self.options_panel();
        }
        if self.ctx.button("quit", "Quit game").pressed() {
            self.option_panel = false;
        }
        self.ctx.window_end();
        self.ctx.end();
    }

    fn options_panel(&mut self) {
        self.ctx
            .frame_window_begin("options", "Options", 25, 5, 50, 40)
            .margin(3)
            .padding(1);
        self.ctx.label("Game settings");
        self.ctx.grid_begin("game_settings", 2, 3, 22, 2).padding(1);
        {
            self.list_button("Font", &["arial_8x8.png", "consolas_12x12.png"]);
            self.list_button("FPS", &["30", "60"]);
            self.list_button("Resolution", &["128x96", "96x72", "80x50"]);
        }
        self.ctx.grid_end();

        self.ctx.label("Controls");
        self.ctx.grid_begin("controls", 2, 9, 22, 2).padding(1);
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
        self.ctx.hbox_begin("options_actions");
        if self
            .ctx
            .button("ok", "   Ok   ")
            .align(ui::TextAlign::Left)
            .pressed()
        {
            self.option_panel = false;
        }
        if self
            .ctx
            .button("cancel", " Cancel ")
            .align(ui::TextAlign::Left)
            .pressed()
        {
            self.option_panel = false;
        }
        self.ctx.hbox_end();
        self.ctx.frame_window_end();
    }

    fn list_button(&mut self, label: &str, values: &[&str]) {
        self.ctx
            .label(&format!("{} :", label))
            .align(ui::TextAlign::Right);
        self.ctx.list_button_begin(label);
        for value in values {
            self.ctx.list_button_item(value, ui::TextAlign::Left);
        }
        self.ctx.list_button_end(true);
    }
}

impl Engine for Astacia {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.ctx
            .push_color(ui::ColorCode::Background, (0, 0, 0, 255));
        self.ctx
            .push_color(ui::ColorCode::Foreground, (220, 220, 180, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackground, (10, 10, 10, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackgroundHover, (50, 50, 50, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackgroundFocus, (100, 100, 100, 255));
        self.ctx
            .push_color(ui::ColorCode::Text, (200, 200, 80, 255));
        api.con().register_color("grey", (180, 180, 180, 255));
        api.con().register_color("text", (200, 200, 80, 255));
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        ui::update_doryen_input_data(api, &mut self.ctx);
        self.build_ui();
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con()
            .clear(None, Some((0, 0, 0, 255)), Some(' ' as u16));
        ui::render_doryen(api.con(), &mut self.ctx);
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
        ..Default::default()
    });
    app.set_engine(Box::new(Astacia::new()));
    app.run();
}

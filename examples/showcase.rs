extern crate doryen_rs;
extern crate doryen_ui;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, UpdateEvent};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct Showcase {
    ctx: ui::Context,
    button_popup: bool,
    pgbar_value: f32,
}

impl Showcase {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        let ctx = &mut self.ctx;
        ctx.begin();
        if ctx
            .dropdown_panel_begin("dropdown", "dropdown", true, 17, 4)
            .active()
        {
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
            ctx.list_button_begin("list_button", 1);
            ctx.list_button_item("list value 1", ui::TextAlign::Center);
            ctx.list_button_item("list value 2", ui::TextAlign::Center);
            ctx.list_button_end(true);
        }
        ctx.dropdown_panel_end();
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
        ctx.toggle("grid1", "1", Default::default());
        ctx.toggle("grid2", "2", Default::default());
        ctx.toggle("grid3", "3", Default::default());
        ctx.toggle("grid4", "4", Default::default());
        ctx.grid_end();
        ctx.frame_end();
        ctx.frame_begin("flexgrid_frame", "flexgrid", 17, 4);
        ctx.flexgrid_begin("flexgrid", &[3, 5, 7], 2);
        ctx.toggle("grid1", "1", Default::default());
        ctx.toggle("grid2", "2", Default::default());
        ctx.toggle("grid3", "3", Default::default());
        ctx.toggle("grid4", "4", Default::default());
        ctx.flexgrid_end();
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
        ctx.frame_begin("pgbar", "progressbar", 17, 7)
            .margin(1)
            .padding(1);
        ctx.progress_bar(13, 0.0, 1.0, self.pgbar_value, None);
        ctx.progress_bar(
            13,
            0.0,
            1.0,
            self.pgbar_value,
            Some(&format!("{:.2}", self.pgbar_value)),
        );
        self.pgbar_value = (self.pgbar_value + 0.01) % 1.0;
        ctx.frame_end();
        ctx.frame_begin("txtbox", "text box", 17, 4);
        ctx.textbox("txtbox1", 15, None, Some("type here"));
        let textbox_id = ctx.last_id();
        // how to get the textbox value
        let _ = ctx.text(textbox_id);
        // how to programmatically change the textbox value
        if ctx.button("txtboxbutton", "set value").pressed() {
            ctx.set_textbox_value(textbox_id, "a value");
        }
        ctx.frame_end();
        ctx.end();
    }
}

impl Engine for Showcase {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.ctx
            .push_color(ui::ColorCode::Background, (245, 245, 245, 255));
        self.ctx
            .push_color(ui::ColorCode::Foreground, (200, 200, 255, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackground, (201, 201, 201, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackgroundHover, (201, 239, 254, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackgroundFocus, (151, 232, 235, 255));
        self.ctx
            .push_color(ui::ColorCode::Text, (104, 104, 104, 255));
        api.con().register_color("yellow", (200, 200, 100, 255));
        api.con().register_color("orange", (150, 150, 50, 255));
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
        window_title: "doryen-ui showcase".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(Showcase::new()));
    app.run();
}

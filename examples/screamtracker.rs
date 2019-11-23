extern crate doryen_rs;
extern crate doryen_ui;

use doryen_rs::{App, AppOptions, DoryenApi, Engine, UpdateEvent};
use doryen_ui as ui;

const CONSOLE_WIDTH: u32 = 80;
const CONSOLE_HEIGHT: u32 = 50;

#[derive(Default)]
struct ScreamTracker {
    ctx: ui::Context,
}

impl ScreamTracker {
    pub fn new() -> Self {
        Default::default()
    }
    fn build_ui(&mut self) {
        self.ctx.begin();
        self.header();
        self.middle();
        self.bottom();
        self.ctx.end();
    }
    fn label_value(&mut self, label: &str, value: &str) {
        self.ctx.push_color(ui::ColorCode::Text, (129, 61, 0, 255));
        self.ctx.label(label).align(ui::TextAlign::Right);
        self.ctx.pop_color(ui::ColorCode::Text);
        self.ctx
            .push_color(ui::ColorCode::Background, (0, 0, 0, 255));
        self.ctx.push_color(ui::ColorCode::Text, (236, 93, 89, 255));
        self.ctx.label(value);
        self.ctx.pop_color(ui::ColorCode::Background);
        self.ctx.pop_color(ui::ColorCode::Text);
    }
    fn header(&mut self) {
        self.ctx.spacing();
        self.ctx.hbox_begin("title").min_width(38);
        {
            self.ctx
                .label("Scream Tracker V3.2")
                .align(ui::TextAlign::Center);
            self.ctx
                .label("Copyright (C) 1993,1994 Sami Tammilehto")
                .align(ui::TextAlign::Center);
        }
        self.ctx.hbox_end();
        self.ctx.flexgrid_begin("song_data", &[12, 30, 9, 28], 2);
        {
            self.ctx.spacing();
            self.label_value("Song", "Va Yeven Uziahu");
            self.label_value("File", "UZIAHU.S3M (S3M)");
            self.label_value("Instrument", "02:GM57:Trumpet");
            self.label_value("Chord", "none");
        }
        self.ctx.flexgrid_end();
        self.ctx
            .flexgrid_begin("song_data", &[12, 7, 7, 2, 12, 2, 9, 2, 24, 2], 3);
        {
            self.label_value("Order", "016/015");
            self.label_value("", "");
            self.label_value("", "");
            self.label_value("C.Tempo", "D8");
            self.label_value("", "");
            self.label_value("Pattern", "07");
            self.label_value("Row", "00");
            self.label_value("Channel", "03");
            self.label_value("C.Speed", "04");
            self.label_value("Baseoctave", "3");
        }
        self.ctx.flexgrid_end();
        self.ctx.flexgrid_begin("labels", &[60, 20], 2);
        {
            self.ctx.push_color(ui::ColorCode::Text, (15, 0, 0, 255));
            self.ctx.move_cursor(3, 0);
            self.ctx
                .label("Playing; loop:0 ord:000/015 pat:00 row:57 played:05%");
            self.ctx.pop_color(ui::ColorCode::Text);
            self.ctx.push_color(ui::ColorCode::Text, (129, 61, 0, 255));
            self.ctx.label("FreeMem:   255K");
            self.ctx.label("");
            self.ctx.label("FreeEMS: 15040K");
            self.ctx.pop_color(ui::ColorCode::Text);
        }
        self.ctx.flexgrid_end();
        self.ctx.grid_begin("menu", 2, 3, 25, 1);
        {
            self.ctx.label("ESC ..... Main Menu");
            self.ctx.label("F1..F4 .. Edit Screen");
            self.ctx.label("F10 ..... Quick-Help");
            self.ctx.label("CTRL-L .. Load Module");
            self.ctx.label("CTRL-Q .. Quit to DOS");
            self.ctx.label("F5/F8 ... Play / Stop");
        }
        self.ctx.grid_end();
        self.ctx.move_cursor(-3, 0);
        self.ctx.spacing();
    }
    fn middle(&mut self) {
        self.ctx.push_color(ui::ColorCode::Text, (129, 61, 0, 255));
        self.ctx.label(
            "---------------------------------InfoPage (F5)----------------------------------",
        );
        self.ctx
            .flexgrid_begin("labels", &[3, 16, 25, 25], 8)
            .margin(2)
            .hpadding(2);
        self.ctx.toggle_group(1);
        self.info("A1", 0.85, "02:GM57:Trumpet", " None");
        self.info("A2", 0.95, "03:GM58:Trombone", " Track-5");
        self.info("A3", 0.5, "02:GM57:Trumpet", " Track-8");
        self.info("A4", 0.85, "02:GM57:Trumpet", " Track-18");
        self.info("A5", 0.45, "04:GP38:Acoustic Snare", "[ChannelScope]");
        self.info("A6", 0.9, "05:GP44:padel Hi-Hat", "[SOutputScope]");
        self.info("A7", 0.53, "01:GM33:AcouBass", " NoteSpectrum");
        self.info("A8", 0.0, "", " NoteDots8");

        self.ctx.flexgrid_end();
        self.ctx.pop_color(ui::ColorCode::Text);
    }
    fn info(&mut self, label: &str, value: f32, label2: &str, toggle: &str) {
        self.ctx.label(label);
        self.ctx.progress_bar(16, 0.0, 1.0, value, None);
        self.black_and_white(ui::ColorCode::Background, ui::ColorCode::Text);
        self.ctx.label(label2);
        self.pop_color(ui::ColorCode::Background, ui::ColorCode::Text);
        self.ctx
            .toggle(label, toggle, false)
            .align(ui::TextAlign::Left);
    }
    fn bottom(&mut self) {
        self.ctx.move_cursor(2, 4);
        self.ctx
            .flexgrid_begin("table", &[2, 8, 8, 8, 8, 8, 8, 8, 8], 21)
            .hpadding(1);
        {
            // table header
            self.ctx.label("");
            self.ctx
                .push_color(ui::ColorCode::Background, (129, 61, 0, 255));
            for i in 1..=8 {
                self.ctx.label(&format!("{:02} : A{}", i, i));
            }
            self.ctx.pop_color(ui::ColorCode::Background);
            // table content
            for i in 43..64 {
                self.ctx.push_color(ui::ColorCode::Text, (129, 61, 0, 255));
                self.ctx.label(&format!("{}", i));
                self.ctx.pop_color(ui::ColorCode::Text);
                self.black_and_white(ui::ColorCode::ButtonBackground, ui::ColorCode::Text);
                for j in 1..=8 {
                    self.ctx.button(&format!("b{}-{}", i, j), "...  .00");
                }
                self.pop_color(ui::ColorCode::ButtonBackground, ui::ColorCode::Text);
            }
        }
        self.ctx.flexgrid_end();
        self.ctx.hbox_end();
    }
    fn black_and_white(&mut self, back: ui::ColorCode, fore: ui::ColorCode) {
        self.ctx.push_color(fore, (255, 255, 255, 255));
        self.ctx.push_color(back, (0, 0, 0, 255));
    }
    fn pop_color(&mut self, back: ui::ColorCode, fore: ui::ColorCode) {
        self.ctx.pop_color(fore);
        self.ctx.pop_color(back);
    }
}

impl Engine for ScreamTracker {
    fn init(&mut self, api: &mut dyn DoryenApi) {
        self.ctx
            .push_color(ui::ColorCode::Background, (228, 141, 88, 255));
        self.ctx
            .push_color(ui::ColorCode::Foreground, (104, 104, 104, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackground, (201, 201, 201, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackgroundHover, (201, 239, 254, 255));
        self.ctx
            .push_color(ui::ColorCode::ButtonBackgroundFocus, (151, 232, 235, 255));
        self.ctx
            .push_color(ui::ColorCode::Text, (255, 197, 134, 255));
        api.con().register_color("label", (129, 61, 0, 255));
    }
    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {
        ui::update_doryen_input_data(api, &mut self.ctx);
        self.build_ui();
        None
    }
    fn render(&mut self, api: &mut dyn DoryenApi) {
        api.con()
            .clear(None, Some((228, 141, 88, 255)), Some(' ' as u16));
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
        window_title: "ScreamTracker V3.2".to_owned(),
        ..Default::default()
    });
    app.set_engine(Box::new(ScreamTracker::new()));
    app.run();
}

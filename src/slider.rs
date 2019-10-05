use crate::{ColorCode, Context, Coord, Rect, TextAlign, MOUSE_BUTTON_LEFT};

impl Context {
    // =======================================================
    //
    // Sliders
    //
    // =======================================================
    pub fn fslider(
        &mut self,
        id1: &str,
        width: Coord,
        min_val: f32,
        max_val: f32,
        start_val: f32,
    ) -> f32 {
        assert!(min_val < max_val);
        assert!(start_val >= min_val && start_val <= max_val);
        self.try_commit();
        let id = self.generate_id(id1);
        let value = *self.slider_state.entry(id).or_insert(start_val);
        let r = self.next_rectangle(width, 1);
        let was_focus = self.focus == id;
        self.update_control(id, &r, true);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && self.mouse_down == MOUSE_BUTTON_LEFT;
        if pressed {
            if !self.dnd_on {
                self.start_dnd(value);
            } else {
                let delta = self.mouse_pos.0 - self.dnd_start.0;
                let value_delta = delta as f32 * (max_val - min_val) / width as f32;
                let new_value = (self.dnd_value + value_delta).max(min_val).min(max_val);
                self.slider_state.insert(id, new_value);
            }
        } else if was_focus {
            self.dnd_on = false;
        }
        let coef = (value - min_val) / (max_val - min_val);
        let handle_pos = r.x + ((r.w as f32 * coef + 0.5) as Coord).min(r.w - 1);
        self.draw_slider(r, handle_pos, focus || hover);
        value
    }

    pub fn islider(
        &mut self,
        id: &str,
        width: Coord,
        min_val: i32,
        max_val: i32,
        start_val: i32,
    ) -> i32 {
        assert!(min_val < max_val);
        assert!(start_val >= min_val && start_val <= max_val);
        self.try_commit();
        let id = self.generate_id(id);
        let value = *self.button_state.entry(id).or_insert(start_val);
        let r = self.next_rectangle(width, 1);
        let was_focus = self.focus == id;
        self.update_control(id, &r, true);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = focus && self.mouse_down == MOUSE_BUTTON_LEFT;
        if pressed {
            if !self.dnd_on {
                self.start_dnd(value as f32);
            } else {
                let delta = self.mouse_pos.0 - self.dnd_start.0;
                let value_delta = delta as f32 * (max_val - min_val) as f32 / width as f32;
                let new_value = ((self.dnd_value + value_delta) as i32)
                    .max(min_val)
                    .min(max_val);
                self.button_state.insert(id, new_value);
            }
        } else if was_focus {
            self.dnd_on = false;
        }
        let coef = (value - min_val) as f32 / (max_val - min_val) as f32;
        let handle_pos = r.x + ((r.w as f32 * coef + 0.5) as Coord).min(r.w - 1);
        self.draw_slider(r, handle_pos, focus || hover);
        value
    }

    fn draw_slider(&mut self, r: Rect, handle_pos: Coord, active: bool) {
        let back = self.get_color(if active {
            ColorCode::ButtonBackgroundHover
        } else {
            ColorCode::ButtonBackground
        });
        self.draw_rect(r, back);
        let fore = self.get_color(ColorCode::Text);
        self.draw_line(r.x, r.y, r.x + r.w, r.y + r.h, fore);
        let handle_area = Rect {
            x: handle_pos,
            y: r.y,
            w: 1,
            h: 1,
        };
        self.draw_text(handle_area, "|", TextAlign::Left, fore);
    }
    // =======================================================
    //
    // ProgressBar
    //
    // =======================================================
    pub fn progress_bar(
        &mut self,
        width: Coord,
        min_value: f32,
        max_value: f32,
        value: f32,
        msg: Option<&str>,
    ) {
        assert!(min_value < max_value);
        self.try_commit();
        let r = self.next_rectangle(width, 1);
        let cval = value.min(max_value).max(min_value);
        let coef = (cval - min_value) / (max_value - min_value);
        self.draw_progress(
            r,
            coef,
            self.get_color(ColorCode::ProgressBack),
            self.get_color(ColorCode::ProgressFore),
        );
        if let Some(msg) = msg {
            let align = self.next_align.take().unwrap_or(TextAlign::Center);
            self.draw_text(r, msg, align, self.get_color(ColorCode::ProgressText));
        }
    }
}

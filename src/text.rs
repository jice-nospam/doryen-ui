use crate::{
    ColorCode, Context, Coord, DeferedCommand, SpecialKey, TextBoxState, MOUSE_BUTTON_LEFT,
};

const CURSOR_DELAY: usize = 10;

impl Context {
    // =======================================================
    //
    // Buttons
    //
    // =======================================================
    pub fn textbox(
        &mut self,
        id: &str,
        width: usize,
        default_value: Option<&str>,
        bkgnd_text: Option<&str>,
    ) -> &mut Self {
        self.try_commit();
        let id = self.generate_id(id);
        let r = self.next_rectangle(width as Coord, 1);
        self.update_control(id, &r, false);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let (current_value, bkgnd_text, cursor, offset) = {
            let state = self.textbox_state.entry(id).or_insert(TextBoxState {
                bkgnd_text: bkgnd_text.map_or(String::new(), |t| t.to_owned()),
                value: default_value.map_or(String::new(), |t| t.to_owned()),
                offset: 0,
                cursor_pos: 0,
            });
            if focus {
                for k in self.special_keys.drain(0..) {
                    match k {
                        SpecialKey::Backspace => {
                            if state.cursor_pos > 0 {
                                state.value.remove(state.cursor_pos - 1);
                                state.cursor_pos -= 1;
                            }
                        }
                        SpecialKey::Delete => {
                            if state.cursor_pos < state.value.len() {
                                state.value.remove(state.cursor_pos);
                            }
                        }
                        SpecialKey::Left => {
                            if state.cursor_pos > 0 {
                                state.cursor_pos -= 1;
                            }
                        }
                        SpecialKey::Right => {
                            if state.cursor_pos + 1 < state.value.len() {
                                state.cursor_pos += 1;
                            }
                        }
                        SpecialKey::End => {
                            state.cursor_pos = state.value.len();
                        }
                        SpecialKey::Home => {
                            state.cursor_pos = 0;
                        }
                    }
                }
                if !self.text_input.is_empty() {
                    state.value = insert_text(&state.value, state.cursor_pos, &self.text_input);
                    state.cursor_pos += self.text_input.len();
                }
                state.offset = state.offset.min(state.cursor_pos);
                if state.cursor_pos >= r.w as usize {
                    state.offset = state.offset.max(state.cursor_pos + 1 - r.w as usize);
                }
            }
            (
                state.value.to_owned(),
                state.bkgnd_text.to_owned(),
                state.cursor_pos,
                state.offset,
            )
        };
        self.pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let background_code = if hover || focus {
            ColorCode::ButtonBackgroundFocus
        } else {
            ColorCode::ButtonBackgroundHover
        };
        let foreground_code = if current_value.is_empty() {
            ColorCode::ButtonTextDisabled
        } else {
            ColorCode::ButtonText
        };
        let back = self.get_color(background_code);
        let fore = self.get_color(foreground_code);
        let mut value = if current_value.is_empty() && !focus {
            bkgnd_text
        } else {
            current_value
        };
        if focus && self.timer % CURSOR_DELAY < CURSOR_DELAY / 2 {
            value = add_cursor(&value, cursor);
        }
        self.defered(DeferedCommand::Label(
            r,
            value[offset..].to_owned(),
            back,
            fore,
        ));
        self
    }
}

fn add_cursor(value: &str, cursor: usize) -> String {
    let head = if cursor > 0 && !value.is_empty() {
        &value[0..cursor]
    } else {
        ""
    };
    let tail = if cursor + 1 < value.len() {
        &value[cursor + 1..]
    } else {
        ""
    };
    head.to_owned() + "_" + tail
}

fn insert_text(value: &str, cursor: usize, txt: &str) -> String {
    let head = if cursor > 0 && !value.is_empty() {
        &value[0..cursor]
    } else {
        ""
    };
    let tail = if cursor < value.len() {
        &value[cursor..]
    } else {
        ""
    };
    head.to_owned() + txt + tail
}

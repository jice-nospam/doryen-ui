use unicode_segmentation::UnicodeSegmentation;

use crate::{
    ColorCode, Context, Coord, DeferedCommand, Id, SpecialKey, TextBoxState, MOUSE_BUTTON_LEFT,
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
        let (current_value, bkgnd_text, cursor, offset) =
            self.update_text_state(id, bkgnd_text, default_value, focus, r.w as usize);
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
        if offset > 0 {
            value = value
                .graphemes(true)
                .skip(offset)
                .collect::<Vec<&str>>()
                .join("");
        }
        self.defered(DeferedCommand::Label(r, value, back, fore));
        self
    }
    /// returns (value, bkgnd_text, cursor_pos, offset)
    fn update_text_state(
        &mut self,
        id: Id,
        bkgnd_text: Option<&str>,
        default_value: Option<&str>,
        focus: bool,
        width: usize,
    ) -> (String, String, usize, usize) {
        {
            let state = self.textbox_state.entry(id).or_insert(TextBoxState {
                bkgnd_text: bkgnd_text.map_or(String::new(), |t| t.to_owned()),
                value: default_value.map_or(String::new(), |t| t.to_owned()),
                offset: 0,
                cursor_pos: 0,
            });
            if focus {
                for k in self.special_keys.drain(0..) {
                    let slen = state.value.graphemes(true).count();
                    match k {
                        SpecialKey::Backspace => {
                            if state.cursor_pos > 0 {
                                state.value = remove_grapheme(&state.value, state.cursor_pos - 1);
                                state.cursor_pos -= 1;
                            }
                        }
                        SpecialKey::Delete => {
                            if state.cursor_pos < slen {
                                state.value = remove_grapheme(&state.value, state.cursor_pos);
                            }
                        }
                        SpecialKey::Left => {
                            if state.cursor_pos > 0 {
                                state.cursor_pos -= 1;
                            }
                        }
                        SpecialKey::Right => {
                            if state.cursor_pos < slen {
                                state.cursor_pos += 1;
                            }
                        }
                        SpecialKey::End => {
                            state.cursor_pos = slen;
                        }
                        SpecialKey::Home => {
                            state.cursor_pos = 0;
                        }
                    }
                }
                if !self.text_input.is_empty() {
                    state.value = insert_text(&state.value, state.cursor_pos, &self.text_input);
                    state.cursor_pos += self.text_input.graphemes(true).count();
                }
                state.offset = state.offset.min(state.cursor_pos);
                if state.cursor_pos >= width {
                    state.offset = state.offset.max(state.cursor_pos + 1 - width);
                }
            }
            (
                state.value.to_owned(),
                state.bkgnd_text.to_owned(),
                state.cursor_pos,
                state.offset,
            )
        }
    }
}

fn add_cursor(value: &str, cursor: usize) -> String {
    let head = if cursor > 0 && !value.is_empty() {
        value
            .graphemes(true)
            .take(cursor)
            .collect::<Vec<&str>>()
            .join("")
    } else {
        String::new()
    };
    let tail = if cursor + 1 < value.len() {
        value
            .graphemes(true)
            .skip(cursor + 1)
            .collect::<Vec<&str>>()
            .join("")
    } else {
        String::new()
    };
    head.to_owned() + "_" + &tail
}

fn insert_text(value: &str, cursor: usize, txt: &str) -> String {
    let head = if cursor > 0 && !value.is_empty() {
        value
            .graphemes(true)
            .take(cursor)
            .collect::<Vec<&str>>()
            .join("")
    } else {
        String::new()
    };
    let tail = if cursor < value.len() {
        value
            .graphemes(true)
            .skip(cursor)
            .collect::<Vec<&str>>()
            .join("")
    } else {
        String::new()
    };
    head.to_owned() + txt + &tail
}

fn remove_grapheme(value: &str, pos: usize) -> String {
    let ret: String = value
        .graphemes(true)
        .enumerate()
        .filter(|(i, _)| *i != pos)
        .map(|(_, v)| v)
        .collect();
    ret
}

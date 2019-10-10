use std::collections::HashSet;

use crate::{ColorCode, Context, Coord, DeferedCommand, Id, TextAlign, MOUSE_BUTTON_LEFT};

impl Context {
    // =======================================================
    //
    // Buttons
    //
    // =======================================================
    pub fn button(&mut self, id: &str, label: &str) -> &mut Self {
        self.try_commit();
        let id = self.generate_id(id);
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.update_control(id, &r, false);
        let focus = self.focus == id;
        let hover = self.hover == id;
        self.pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let (background_code, foreground_code) = if hover {
            (ColorCode::ButtonBackgroundHover, ColorCode::ButtonTextHover)
        } else if focus {
            (ColorCode::ButtonBackgroundFocus, ColorCode::ButtonTextFocus)
        } else {
            (ColorCode::ButtonBackground, ColorCode::ButtonText)
        };
        let back = self.get_color(background_code);
        let fore = self.get_color(foreground_code);
        self.defered(DeferedCommand::Button(label.to_owned(), back, fore));
        //println!("{}: {} {} {}",id, focus,hover,pressed);
        self
    }

    /// returns (checkbox_status, status_has_changed_this_frame)
    pub fn checkbox(&mut self, id: &str, label: &str, initial_state: bool) -> &mut Self {
        let padded_label = "  ".to_owned() + label;
        let pressed = self
            .button(id, &padded_label)
            .align(TextAlign::Left)
            .pressed();
        let checked = {
            let checked = self
                .button_state
                .entry(self.last_id)
                .or_insert(if initial_state { 1 } else { 0 });
            if pressed {
                *checked = 1 - *checked;
            }
            *checked == 1
        };
        let fore = self.get_color(ColorCode::Text);
        self.defered(DeferedCommand::CheckBox(checked, fore));
        self.active = checked;
        self
    }

    // =======================================================
    //
    // Toggle button
    //
    // =======================================================

    fn add_group_id(&mut self, group: usize, id: Id) {
        let ids = self.toggle_group.entry(group).or_insert_with(HashSet::new);
        ids.insert(id);
    }
    fn disable_toggle_group(&mut self, group: usize) {
        for id in self.toggle_group.get(&group).unwrap() {
            self.button_state.insert(*id, 0);
        }
    }
    pub fn toggle_group(&mut self, group: usize) {
        self.cur_toggle_group = group;
    }
    /// a button that switches between active/inactive when clicked.
    pub fn toggle(&mut self, id: &str, label: &str, active: bool) -> &mut Self {
        self.try_commit();
        let id = self.generate_id(id);
        self.add_group_id(self.cur_toggle_group, id);
        let r = self.next_rectangle(label.chars().count() as Coord, 1);
        self.update_control(id, &r, false);
        let focus = self.focus == id;
        let hover = self.hover == id;
        let pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        let mut on = *self
            .button_state
            .get(&self.last_id)
            .unwrap_or(if active { &1 } else { &0 })
            == 1;
        if pressed {
            if !on {
                self.disable_toggle_group(self.cur_toggle_group);
            }
            on = !on;
        }
        self.button_state.insert(id, if on { 1 } else { 0 });
        let (background_code, foreground_code) = if on && !hover {
            (ColorCode::ButtonBackgroundHover, ColorCode::ButtonTextHover)
        } else if focus || hover {
            (ColorCode::ButtonBackgroundFocus, ColorCode::ButtonTextFocus)
        } else {
            (ColorCode::ButtonBackground, ColorCode::ButtonText)
        };
        let back = self.get_color(background_code);
        let fore = self.get_color(foreground_code);
        self.defered(DeferedCommand::Button(label.to_owned(), back, fore));
        self.pressed = pressed;
        self.active = on;
        self
    }
    pub fn set_toggle_status(&mut self, toggle_id: Id, status: bool) {
        self.button_state
            .insert(toggle_id, if status { 1 } else { 0 });
    }

    // =======================================================
    //
    // List button
    //
    // =======================================================

    /// a button that cycles over a list of values when clicked
    pub fn list_button_begin(&mut self, id: &str) {
        self.try_commit();
        let id = self.generate_id(id);
        self.list_button_index = 0;
        self.list_button_width = 0;
        self.button_state.entry(id).or_insert(0);
    }

    /// add a new item in the list of values
    /// returns true if this is the current value
    pub fn list_button_item(&mut self, label: &str, align: TextAlign) -> bool {
        self.list_button_width = self.list_button_width.max(label.chars().count() as Coord);
        let list_button_id = self.last_id();
        self.list_button_index += 1;
        assert!(
            self.button_state.get(&list_button_id).is_some(),
            "list_button_item must be called inside list_button_begin/list_button_end"
        );
        if *self.button_state.get(&list_button_id).unwrap() != self.list_button_index - 1 {
            return false;
        }
        self.list_button_label = label.to_owned();
        self.list_button_align = align;
        true
    }

    /// end the value list.
    /// returns true if the current value has changed this frame
    /// if display_count is true, shows the selected item index / items count when the mouse is hovering the button
    pub fn list_button_end(&mut self, display_count: bool) -> bool {
        let list_button_id = self.last_id();
        assert!(
            self.button_state.get(&list_button_id).is_some(),
            "list_button_end must be called after list_button_begin"
        );
        self.list_button_width += 2;
        let r = self.next_rectangle(self.list_button_width, 1);
        self.update_control(list_button_id, &r, false);
        let focus = self.focus == list_button_id;
        let hover = self.hover == list_button_id;
        let pressed = hover && self.mouse_pressed == MOUSE_BUTTON_LEFT;
        //println!("{}: {} {} {}",list_button_id, focus,hover,pressed);
        let cur_index = *self.button_state.get(&list_button_id).unwrap();
        if pressed {
            let next_index = (cur_index + 1) % self.list_button_index;
            self.button_state.insert(list_button_id, next_index);
        }
        let background_code = if hover {
            ColorCode::ButtonBackgroundHover
        } else if focus {
            ColorCode::ButtonBackgroundFocus
        } else {
            ColorCode::ButtonBackground
        };
        let back = self.get_color(background_code);
        let fore = self.get_color(ColorCode::Text);
        self.draw_rect(r, back);
        let label = if hover && display_count {
            let mut label = self.list_button_label.clone();
            let label_len = label.chars().count();
            let suffix = format!("|{}/{}", cur_index + 1, self.list_button_index);
            let suffix_len = suffix.len();
            if suffix_len + label_len > r.w as usize {
                label = label
                    .chars()
                    .take(r.w as usize - suffix_len)
                    .collect::<String>();
            }
            label + &suffix
        } else {
            self.list_button_label.clone()
        };
        self.draw_text(r, &label, self.list_button_align, fore);
        pressed
    }
}

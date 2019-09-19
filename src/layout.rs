use crate::{Coord, Pos, Rect};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LayoutMode {
    Single,
    Horizontal,
    Vertical,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Vertical
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Layout {
    pub r: Rect,
    padded_child: bool, // false for first child
    pub mode: LayoutMode,
    pub margin: Coord,
    pub padding: Coord,
    cursor: Pos,
    pub last_cursor: Pos,
    fixed: bool,
}

impl Layout {
    pub fn new_fixed(
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
        margin: Coord,
        padding: Coord,
    ) -> Layout {
        let mut layout: Layout = Default::default();
        layout.r.x = x;
        layout.r.y = y;
        layout.r.w = width;
        layout.r.h = height;
        layout.margin = margin;
        layout.padding = padding;
        layout.cursor.x = layout.r.x + margin;
        layout.cursor.y = layout.r.y + margin;
        layout.fixed = true;
        layout
    }
    pub fn next_layout(
        &mut self,
        width: Coord,
        height: Coord,
        margin: Coord,
        padding: Coord,
    ) -> Self {
        assert!(self.mode != LayoutMode::Single);
        let r = match self.mode {
            LayoutMode::Single => unreachable!(),
            LayoutMode::Horizontal => self.next_column(width, height),
            LayoutMode::Vertical => self.next_row(width, height),
        };
        Self {
            r,
            mode: LayoutMode::Single,
            cursor: Pos {
                x: r.x + margin,
                y: r.y + margin,
            },
            margin,
            padding,
            ..Default::default()
        }
    }
    fn next_column(&mut self, width: Coord, height: Coord) -> Rect {
        if self.padded_child {
            self.cursor.x += self.padding;
        }
        self.padded_child = true;
        let r = Rect {
            x: self.cursor.x,
            y: self.cursor.y,
            w: width.max(1),
            h: if self.r.h == 0 {
                height
            } else {
                self.r.h - 2 * self.margin
            },
        };
        if !self.fixed {
            self.last_cursor = self.cursor;
        }
        self.cursor.x += width.max(1);
        r
    }
    fn next_row(&mut self, width: Coord, height: Coord) -> Rect {
        if self.padded_child {
            self.cursor.y += self.padding;
        }
        self.padded_child = true;
        let r = Rect {
            x: self.cursor.x,
            y: self.cursor.y,
            w: if self.r.w == 0 {
                width
            } else {
                self.r.w - 2 * self.margin
            },
            h: height.max(1),
        };
        if !self.fixed {
            self.last_cursor = self.cursor;
        }
        self.cursor.y += height.max(1);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_layout(layout: &Layout, x: Coord, y: Coord, w: Coord, h: Coord) {
        assert_eq!(
            *layout,
            Layout {
                cursor: Pos { x, y },
                margin: 0,
                padding: 0,
                padded_child: false,
                mode: LayoutMode::Single,
                last_cursor: layout.last_cursor,
                r: Rect { x, y, w, h },
                fixed: false,
            }
        );
    }

    pub fn new_vertical(margin: Coord, padding: Coord) -> Layout {
        Layout {
            mode: LayoutMode::Vertical,
            cursor: Pos {
                x: margin,
                y: margin,
            },
            margin,
            padding,
            ..Default::default()
        }
    }
    pub fn new_horizontal(width: Coord, margin: Coord, padding: Coord) -> Layout {
        Layout {
            mode: LayoutMode::Horizontal,
            cursor: Pos {
                x: margin,
                y: margin,
            },
            margin,
            padding,
            r: Rect {
                x: 0,
                y: 0,
                w: width,
                h: 0,
            },
            ..Default::default()
        }
    }

    #[test]
    fn test_vbox() {
        let mut root = new_vertical(0, 0);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 0, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 1, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 2, 5, 1);
    }
    #[test]
    fn test_vbox_margin() {
        let mut root = new_vertical(1, 0);
        assert_layout(&root.next_layout(5, 1, 0, 0), 1, 1, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 1, 2, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 1, 3, 5, 1);
    }
    #[test]
    fn test_vbox_padding() {
        let mut root = new_vertical(0, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 0, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 2, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 4, 5, 1);
    }
    #[test]
    fn test_vbox_padding_and_margin() {
        let mut root = new_vertical(2, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 2, 2, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 2, 4, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 2, 6, 5, 1);
    }
    #[test]
    fn test_hbox() {
        let mut root = new_horizontal(5, 0, 0);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 0, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 5, 0, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 10, 0, 5, 1);
    }
    #[test]
    fn test_hbox_margin() {
        let mut root = new_horizontal(5, 1, 0);
        assert_layout(&root.next_layout(5, 1, 0, 0), 1, 1, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 6, 1, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 11, 1, 5, 1);
    }
    #[test]
    fn test_hbox_padding() {
        let mut root = new_horizontal(5, 0, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 0, 0, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 6, 0, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 12, 0, 5, 1);
    }
    #[test]
    fn test_hbox_padding_and_margin() {
        let mut root = new_horizontal(5, 1, 2);
        assert_layout(&root.next_layout(5, 1, 0, 0), 1, 1, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 8, 1, 5, 1);
        assert_layout(&root.next_layout(5, 1, 0, 0), 15, 1, 5, 1);
    }
}

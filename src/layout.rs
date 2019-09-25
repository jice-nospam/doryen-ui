use crate::{Coord, Pos, Rect};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum LayoutMode {
    Single,
    Horizontal,
    Vertical,
    Grid,
}
impl Default for LayoutMode {
    fn default() -> Self {
        Self::Vertical
    }
}

#[derive(Default, PartialEq, Eq, Debug)]
pub struct Layout {
    r: Rect,
    padded_child: bool, // false for first child
    mode: LayoutMode,
    margin: Coord,
    padding: Coord,
    cursor: Pos,
    last_cursor: Pos,
    grid_cols: usize,
    grid_rows: usize,
    grid_col: usize,
    grid_row: usize,
}

impl Layout {
    pub fn area(&self) -> Rect {
        self.r
    }
    pub fn last_cursor(&self) -> Pos {
        self.last_cursor
    }
    pub fn padding(&mut self, value: Coord) {
        self.padding = value;
    }
    pub fn margin(&mut self, value: Coord) {
        self.margin += value;
        self.cursor.x += value;
        self.cursor.y += value;
    }
    pub fn next_area(&mut self, width: Coord, height: Coord) -> Rect {
        assert!(self.mode != LayoutMode::Single);
        match self.mode {
            LayoutMode::Single => unreachable!(),
            LayoutMode::Horizontal => self.next_column(width, height),
            LayoutMode::Vertical => self.next_row(width, height),
            LayoutMode::Grid => self.next_grid_cell(width, height),
        }
    }
    pub fn new(mode: LayoutMode, r: Rect, margin: Coord, padding: Coord) -> Self {
        Self {
            r,
            mode,
            cursor: Pos {
                x: r.x + margin,
                y: r.y + margin,
            },
            margin,
            padding,
            ..Default::default()
        }
    }
    pub fn new_grid(r: Rect, cols: usize, rows: usize, margin: Coord, padding: Coord) -> Self {
        let mut l = Self::new(LayoutMode::Grid, r, margin, padding);
        l.grid_cols = cols;
        l.grid_rows = rows;
        l
    }
    fn next_grid_cell(&mut self, width: Coord, height: Coord) -> Rect {
        if !self.padded_child {
            self.last_cursor = self.cursor;
        }
        self.padded_child = true;
        let r = Rect {
            x: self.cursor.x,
            y: self.cursor.y,
            w: if self.r.w == 0 {
                width
            } else {
                self.r.w - self.padding
            },
            h: if self.r.h == 0 {
                height
            } else {
                self.r.h - self.padding
            },
        };
        self.grid_col += 1;
        if self.grid_col == self.grid_cols {
            self.grid_col = 0;
            self.cursor.x = self.r.x + self.margin;
            self.cursor.y += self.r.h;
            self.grid_row += 1;
        } else {
            self.cursor.x += self.r.w;
        }
        r
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
        self.last_cursor = self.cursor;
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
        self.last_cursor = self.cursor;
        self.cursor.y += height.max(1);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_layout(layout: &Rect, x: Coord, y: Coord, w: Coord, h: Coord) {
        assert_eq!(*layout, Rect { x, y, w, h });
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
        assert!(width > 2 * margin);
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
    pub fn new_grid(
        cols: usize,
        rows: usize,
        width: Coord,
        height: Coord,
        margin: Coord,
        padding: Coord,
    ) -> Layout {
        assert!(width > padding);
        assert!(height > padding);
        Layout {
            mode: LayoutMode::Grid,
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
                h: height,
            },
            grid_cols: cols,
            grid_rows: rows,
            ..Default::default()
        }
    }
    #[test]
    fn test_vbox() {
        let mut root = new_vertical(0, 0);
        assert_layout(&root.next_area(5, 1), 0, 0, 5, 1);
        assert_layout(&root.next_area(5, 1), 0, 1, 5, 1);
        assert_layout(&root.next_area(5, 1), 0, 2, 5, 1);
    }
    #[test]
    fn test_vbox_margin() {
        let mut root = new_vertical(1, 0);
        assert_layout(&root.next_area(5, 1), 1, 1, 5, 1);
        assert_layout(&root.next_area(5, 1), 1, 2, 5, 1);
        assert_layout(&root.next_area(5, 1), 1, 3, 5, 1);
    }
    #[test]
    fn test_vbox_padding() {
        let mut root = new_vertical(0, 1);
        assert_layout(&root.next_area(5, 1), 0, 0, 5, 1);
        assert_layout(&root.next_area(5, 1), 0, 2, 5, 1);
        assert_layout(&root.next_area(5, 1), 0, 4, 5, 1);
    }
    #[test]
    fn test_vbox_padding_and_margin() {
        let mut root = new_vertical(2, 1);
        assert_layout(&root.next_area(5, 1), 2, 2, 5, 1);
        assert_layout(&root.next_area(5, 1), 2, 4, 5, 1);
        assert_layout(&root.next_area(5, 1), 2, 6, 5, 1);
    }
    #[test]
    fn test_hbox() {
        let mut root = new_horizontal(5, 0, 0);
        assert_layout(&root.next_area(5, 1), 0, 0, 5, 1);
        assert_layout(&root.next_area(5, 1), 5, 0, 5, 1);
        assert_layout(&root.next_area(5, 1), 10, 0, 5, 1);
    }
    #[test]
    fn test_hbox_margin() {
        let mut root = new_horizontal(5, 1, 0);
        assert_layout(&root.next_area(5, 1), 1, 1, 5, 1);
        assert_layout(&root.next_area(5, 1), 6, 1, 5, 1);
        assert_layout(&root.next_area(5, 1), 11, 1, 5, 1);
    }
    #[test]
    fn test_hbox_padding() {
        let mut root = new_horizontal(5, 0, 1);
        assert_layout(&root.next_area(5, 1), 0, 0, 5, 1);
        assert_layout(&root.next_area(5, 1), 6, 0, 5, 1);
        assert_layout(&root.next_area(5, 1), 12, 0, 5, 1);
    }
    #[test]
    fn test_hbox_padding_and_margin() {
        let mut root = new_horizontal(5, 1, 2);
        assert_layout(&root.next_area(5, 1), 1, 1, 5, 1);
        assert_layout(&root.next_area(5, 1), 8, 1, 5, 1);
        assert_layout(&root.next_area(5, 1), 15, 1, 5, 1);
    }
    #[test]
    fn test_grid() {
        let mut root = new_grid(2, 2, 1, 1, 0, 0);
        assert_layout(&root.next_area(1, 1), 0, 0, 1, 1);
        assert_layout(&root.next_area(1, 1), 1, 0, 1, 1);
        assert_layout(&root.next_area(1, 1), 0, 1, 1, 1);
        assert_layout(&root.next_area(1, 1), 1, 1, 1, 1);
    }
    #[test]
    fn test_grid_margin() {
        let mut root = new_grid(2, 2, 1, 1, 1, 0);
        assert_layout(&root.next_area(1, 1), 1, 1, 1, 1);
        assert_layout(&root.next_area(1, 1), 2, 1, 1, 1);
        assert_layout(&root.next_area(1, 1), 1, 2, 1, 1);
        assert_layout(&root.next_area(1, 1), 2, 2, 1, 1);
    }
    #[test]
    fn test_grid_padding() {
        let mut root = new_grid(2, 2, 2, 2, 0, 1);
        assert_layout(&root.next_area(1, 1), 0, 0, 1, 1);
        assert_layout(&root.next_area(1, 1), 2, 0, 1, 1);
        assert_layout(&root.next_area(1, 1), 0, 2, 1, 1);
        assert_layout(&root.next_area(1, 1), 2, 2, 1, 1);
    }
    #[test]
    fn test_grid_padding_margin() {
        let mut root = new_grid(2, 2, 2, 2, 1, 1);
        assert_layout(&root.next_area(1, 1), 1, 1, 1, 1);
        assert_layout(&root.next_area(1, 1), 3, 1, 1, 1);
        assert_layout(&root.next_area(1, 1), 1, 3, 1, 1);
        assert_layout(&root.next_area(1, 1), 3, 3, 1, 1);
    }
}

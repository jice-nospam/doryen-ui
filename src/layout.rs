use crate::{Coord, DeferedCommand, Pos, Rect};

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
    vpadding: Coord,
    hpadding: Coord,
    min_width: Coord,
    max_width: Coord,
    min_height: Coord,
    max_height: Coord,
    cursor: Pos,
    commited: bool,
    last_cursor: Pos,
    grid_widths: Vec<Coord>,
    grid_cols: usize,
    grid_col: usize,
    grid_row: usize,
    defered: Vec<DeferedCommand>,
}

impl Layout {
    pub fn new(mode: LayoutMode) -> Self {
        Self {
            mode,
            ..Default::default()
        }
    }
    pub fn is_single(&self) -> bool {
        self.mode == LayoutMode::Single
    }
    pub fn area(&self) -> Rect {
        self.r
    }
    pub fn commited(&self) -> bool {
        self.commited
    }
    pub fn last_cursor(&self) -> Pos {
        self.last_cursor
    }
    pub fn padding(&mut self, value: Coord) -> &mut Self {
        self.vpadding = value;
        self.hpadding = value;
        self
    }
    pub fn vpadding(&mut self, value: Coord) -> &mut Self {
        self.vpadding = value;
        self
    }
    pub fn hpadding(&mut self, value: Coord) -> &mut Self {
        self.hpadding = value;
        self
    }
    pub fn defered(&mut self, defered: DeferedCommand) -> &mut Self {
        self.defered.push(defered);
        self
    }
    pub fn defered_iter(&mut self) -> std::slice::Iter<DeferedCommand> {
        self.defered.iter()
    }
    pub fn margin(&mut self, value: Coord) -> &mut Self {
        self.margin += value;
        self.cursor.x += value;
        self.cursor.y += value;
        self
    }
    pub fn fixed_size(&mut self, w: Coord, h: Coord) -> &mut Self {
        self.min_width = w;
        self.max_width = w;
        self.min_height = h;
        self.max_height = h;
        self.r.w = w;
        self.r.h = h;
        self
    }
    pub fn move_cursor(&mut self, deltax: Coord, deltay: Coord) -> &mut Self {
        self.cursor.x += deltax;
        self.cursor.y += deltay;
        self
    }
    pub fn min_width(&mut self, value: Coord) -> &mut Self {
        self.min_width = value;
        self.r.w = self.r.w.max(value);
        self
    }
    pub fn max_width(&mut self, value: Coord) -> &mut Self {
        self.max_width = value;
        self
    }
    pub fn pos(&mut self, x: Coord, y: Coord) -> &mut Self {
        self.r.x = x;
        self.r.y = y;
        self.cursor.x = x + self.margin;
        self.cursor.y = y + self.margin;
        self.commited = true;
        self
    }
    pub fn size(&mut self, w: Coord, h: Coord) -> &mut Self {
        self.r.w = w;
        self.r.h = h;
        self
    }
    pub fn min_height(&mut self, value: Coord) -> &mut Self {
        self.min_height = value;
        self.r.h = self.r.h.max(value);
        self
    }
    pub fn max_height(&mut self, value: Coord) -> &mut Self {
        self.max_height = value;
        self
    }
    pub fn grid(&mut self, cols: usize, rows: usize, width: Coord) -> &mut Self {
        self.mode = LayoutMode::Grid;
        self.grid_cols = cols;
        self.grid_widths = vec![width; cols];
        self.r.w = self.max_width * cols as Coord + self.margin * 2;
        self.r.h = self.max_height * rows as Coord + self.margin * 2;
        self
    }
    pub fn flexgrid(&mut self, widths: &[Coord]) -> &mut Self {
        self.mode = LayoutMode::Grid;
        self.grid_cols = widths.len();
        let width = widths.iter().sum::<Coord>();
        self.max_width = widths[0];
        self.min_width = widths[0];
        self.min_height = 1;
        self.max_height = 1;
        self.grid_widths = widths.to_vec();
        self.r.w = width + self.margin * 2;

        self.r.h = self.max_height + self.margin * 2;
        self
    }
    pub fn commit(&mut self, child: &mut Layout) -> Rect {
        assert!(self.mode != LayoutMode::Single);
        child.commited = true;
        match self.mode {
            LayoutMode::Single => unreachable!(),
            LayoutMode::Horizontal => self.next_column(child),
            LayoutMode::Vertical => self.next_row(child),
            LayoutMode::Grid => self.next_grid_cell(child),
        }
    }
    fn next_grid_cell(&mut self, child: &mut Layout) -> Rect {
        if !self.padded_child {
            self.last_cursor = self.cursor;
        }
        self.padded_child = true;
        child.cursor.x += self.cursor.x - child.r.x;
        child.cursor.y += self.cursor.y - child.r.y;
        child.r.x = self.cursor.x;
        child.r.y = self.cursor.y;
        child.r.w = self.max_width;
        child.r.h = self.max_height;
        self.grid_col += 1;
        if self.grid_col == self.grid_cols {
            self.grid_col = 0;
            self.cursor.x = self.r.x + self.margin;
            self.cursor.y += self.max_height + self.vpadding;
            self.grid_row += 1;
        } else {
            self.cursor.x += self.max_width + self.hpadding;
        }
        self.max_width = self.grid_widths[self.grid_col];
        self.min_width = self.max_width;
        child.r
    }
    fn next_column(&mut self, child: &mut Layout) -> Rect {
        if self.padded_child {
            self.cursor.x += self.hpadding;
        }
        self.padded_child = true;
        child.cursor.x += self.cursor.x - child.r.x;
        child.cursor.y += self.cursor.y - child.r.y;
        child.r.x = self.cursor.x;
        child.r.y = self.cursor.y;
        child.r.w = child.r.w.max(self.min_width);
        if self.max_width > 0 {
            child.r.w = child.r.w.min(self.max_width);
        }
        child.r.h = child.r.h.max(self.min_height);
        if self.max_height > 0 {
            child.r.h = child.r.h.min(self.max_height - 2 * self.margin);
        }
        self.last_cursor = self.cursor;
        self.cursor.x += child.r.w;
        child.r
    }
    fn next_row(&mut self, child: &mut Layout) -> Rect {
        if self.padded_child {
            self.cursor.y += self.vpadding;
        }
        self.padded_child = true;
        child.cursor.x += self.cursor.x - child.r.x;
        child.cursor.y += self.cursor.y - child.r.y;
        child.r.x = self.cursor.x;
        child.r.y = self.cursor.y;
        child.r.w = child.r.w.max(self.min_width - 2 * self.margin);
        if self.max_width > 0 {
            child.r.w = child.r.w.min(self.max_width - 2 * self.margin);
        }
        self.last_cursor = self.cursor;
        self.cursor.y += child.r.h;
        child.r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_layout(layout: &Rect, x: Coord, y: Coord, w: Coord, h: Coord) {
        assert_eq!(*layout, Rect { x, y, w, h });
    }

    pub fn new_vertical(margin: Coord, padding: Coord) -> Layout {
        let mut layout = Layout::new(LayoutMode::Vertical);
        layout.margin(margin).padding(padding);
        layout
    }
    pub fn new_horizontal(width: Coord, margin: Coord, padding: Coord) -> Layout {
        assert!(width > 2 * margin);
        let mut layout = Layout::new(LayoutMode::Horizontal);
        layout.margin(margin).padding(padding).min_width(width);
        layout
    }
    pub fn new_grid(
        cols: usize,
        rows: usize,
        width: Coord,
        height: Coord,
        margin: Coord,
        padding: Coord,
    ) -> Layout {
        let mut layout = Layout::new(LayoutMode::Grid);
        layout
            .grid(cols, rows, width)
            .margin(margin)
            .padding(padding)
            .fixed_size(width, height);
        layout
    }
    fn inject_widget(root: &mut Layout, w: Coord, h: Coord) -> Rect {
        root.commit(&mut Layout::new(LayoutMode::Single).size(w, h))
    }
    #[test]
    fn test_vbox() {
        let mut root = new_vertical(0, 0);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 0, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 1, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 2, 5, 1);
    }
    #[test]
    fn test_vbox_margin() {
        let mut root = new_vertical(1, 0);
        assert_layout(&inject_widget(&mut root, 5, 1), 1, 1, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 1, 2, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 1, 3, 5, 1);
    }
    #[test]
    fn test_vbox_padding() {
        let mut root = new_vertical(0, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 0, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 2, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 4, 5, 1);
    }
    #[test]
    fn test_vbox_padding_and_margin() {
        let mut root = new_vertical(2, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 2, 2, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 2, 4, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 2, 6, 5, 1);
    }
    #[test]
    fn test_hbox() {
        let mut root = new_horizontal(5, 0, 0);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 0, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 5, 0, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 10, 0, 5, 1);
    }
    #[test]
    fn test_hbox_margin() {
        let mut root = new_horizontal(5, 1, 0);
        assert_layout(&inject_widget(&mut root, 5, 1), 1, 1, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 6, 1, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 11, 1, 5, 1);
    }
    #[test]
    fn test_hbox_padding() {
        let mut root = new_horizontal(5, 0, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 0, 0, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 6, 0, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 12, 0, 5, 1);
    }
    #[test]
    fn test_hbox_padding_and_margin() {
        let mut root = new_horizontal(5, 1, 2);
        assert_layout(&inject_widget(&mut root, 5, 1), 1, 1, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 8, 1, 5, 1);
        assert_layout(&inject_widget(&mut root, 5, 1), 15, 1, 5, 1);
    }
    #[test]
    fn test_grid() {
        let mut root = new_grid(2, 2, 1, 1, 0, 0);
        assert_layout(&inject_widget(&mut root, 1, 1), 0, 0, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 1, 0, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 0, 1, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 1, 1, 1, 1);
    }
    #[test]
    fn test_grid_margin() {
        let mut root = new_grid(2, 2, 1, 1, 1, 0);
        assert_layout(&inject_widget(&mut root, 1, 1), 1, 1, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 2, 1, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 1, 2, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 2, 2, 1, 1);
    }
    #[test]
    fn test_grid_padding() {
        let mut root = new_grid(2, 2, 1, 1, 0, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 0, 0, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 2, 0, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 0, 2, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 2, 2, 1, 1);
    }
    #[test]
    fn test_grid_padding_margin() {
        let mut root = new_grid(2, 2, 1, 1, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 1, 1, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 3, 1, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 1, 3, 1, 1);
        assert_layout(&inject_widget(&mut root, 1, 1), 3, 3, 1, 1);
    }
}

use crate::{ColorCode, Context, Coord, DeferedCommand, LayoutMode};

impl Context {
    // =======================================================
    //
    // Containers
    //
    // =======================================================
    /// starts a new grid container.
    /// cols,rows : number of cells in the grid
    /// cell_width,cell_height : size of a cell
    /// Example : 2,2,2,1
    /// 1122
    /// 3344
    /// Margin is around the container :
    /// MMMMMM
    /// M1122M
    /// M3344M
    /// MMMMMM
    /// Padding is between the cells :
    /// 11P22
    /// PPPPP
    /// 33P44
    pub fn grid_begin(
        &mut self,
        id: &str,
        cols: usize,
        rows: usize,
        cell_width: Coord,
        cell_height: Coord,
    ) -> &mut Self {
        self.try_commit();
        self.prefix_id(id);
        self.new_layout(LayoutMode::Grid)
            .fixed_size(cell_width, cell_height)
            .grid(cols, rows, cell_width)
    }
    pub fn grid_end(&mut self) {
        self.end_container();
    }
    pub fn flexgrid_begin(&mut self, id: &str, widths: &[Coord], height: Coord) -> &mut Self {
        self.try_commit();
        self.prefix_id(id);
        self.new_layout(LayoutMode::Grid)
            .flexgrid(widths)
            .min_height(height)
    }
    pub fn flexgrid_end(&mut self) {
        self.end_container();
    }
    /// The window behaves like a vbox, but it resets the cursor position
    pub fn window_begin(
        &mut self,
        id: &str,
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.vbox_begin(id).fixed_pos(x, y, width, height)
    }
    pub fn window_end(&mut self) {
        self.vbox_end();
    }
    /// the frame_window behaves like a frame, but it resets the cursor position
    pub fn frame_window_begin(
        &mut self,
        id: &str,
        title: &str,
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.frame_begin(id, title, width, height)
            .fixed_pos(x, y, width, height)
    }
    pub fn frame_window_end(&mut self) {
        self.frame_end();
    }
    /// starts a new vertical container
    ///
    /// margin is around the container :
    /// MMMM
    /// M11M
    /// M22M
    /// M33M
    /// MMMM
    ///
    /// padding is between the rows :
    /// 11
    /// PP
    /// 22
    /// PP
    /// 33
    pub fn vbox_begin(&mut self, id: &str) -> &mut Self {
        self.try_commit();
        self.prefix_id(id);
        self.new_layout(LayoutMode::Vertical)
    }
    pub fn vbox_end(&mut self) {
        self.end_container();
    }
    /// starts a new horizontal container
    ///
    /// margin is around the container :
    /// MMMMMMMM
    /// M112233M
    /// M112233M
    /// MMMMMMMM
    ///
    /// padding is between the columns :
    /// 11P22P33
    /// 11P22P33
    /// 11P22P33
    pub fn hbox_begin(&mut self, id: &str) -> &mut Self {
        self.try_commit();
        self.prefix_id(id);
        self.new_layout(LayoutMode::Horizontal).min_height(1)
    }
    pub fn hbox_end(&mut self) {
        self.end_container();
    }
    /// a frame behaves like a vbox with a drawn border and a title
    pub fn frame_begin(&mut self, id: &str, title: &str, width: Coord, height: Coord) -> &mut Self {
        let back = self.get_color(ColorCode::Background);
        let fore = self.get_color(ColorCode::Text);
        self.vbox_begin(id)
            .fixed_size(width, height)
            .margin(1)
            .defered(DeferedCommand::Frame(title.to_owned(), back, fore))
    }
    pub fn frame_end(&mut self) {
        self.try_commit();
        let mut layout = self.layouts.pop().unwrap();
        let r = layout.area();
        match layout.defered_iter().next() {
            Some(DeferedCommand::Frame(title, col, coltxt)) => {
                self.render_frame(&title, *col, *coltxt, r)
            }
            Some(c) => panic!(
                "unmatched begin/end calls. Expected Frame instead of {:?}",
                c
            ),
            None => panic!("unmatched begin/end calls"),
        }
        self.id_prefix.pop();
    }
    /// a popup is a frame_window with an automatic "Ok" button at the bottom
    pub fn popup_begin(
        &mut self,
        id: &str,
        title: &str,
        x: Coord,
        y: Coord,
        width: Coord,
        height: Coord,
    ) -> &mut Self {
        self.frame_window_begin(id, title, x, y, width, height)
    }
    pub fn popup_end(&mut self) -> bool {
        let ret = self.button("popup_ok", "Ok").pressed();
        self.frame_window_end();
        ret
    }
}

/// 整个应用的布局状态
pub(crate) struct LayoutState {
    pub show_left_side: bool,
    pub show_right_side: bool,
}

impl Default for LayoutState {
    fn default() -> Self {
        Self {
            show_left_side: true,
            show_right_side: false,
        }
    }
}

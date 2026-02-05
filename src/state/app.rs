pub(crate) struct AppState {
    pub show_about: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self { show_about: false }
    }
}

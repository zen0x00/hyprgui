#[derive(Debug, Clone)]
pub struct GeneralState {
    pub border_size: i32,
}

impl Default for GeneralState {
    fn default() -> Self {
        Self {
            border_size: 1,
        }
    }
}

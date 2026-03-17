#[derive(Debug, Clone)]
pub struct GeneralState {
    pub border_size: i32,
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub active_border: String,
    pub inactive_border: String,
}

impl Default for GeneralState {
    fn default() -> Self {
        Self {
            border_size: 1,
            gaps_in: 2,
            gaps_out: 10,
            active_border: "0xffffffff".to_string(),
            inactive_border: "0xff444444".to_string(),
        }
    }
}

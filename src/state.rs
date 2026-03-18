#[derive(Debug, Clone)]
pub struct GeneralState {
    pub border_size: i32,
    pub gaps_in: i32,
    pub gaps_out: i32,
    pub active_border: String,
    pub inactive_border: String,
    pub rounding: i32,
    pub rounding_power: f64,
    pub active_opacity: f64,
    pub inactive_opacity: f64,
    pub dim_modal: bool,
    pub dim_inactive: bool,
    pub dim_strength: f64,
    pub dim_special: f64,
    pub dim_around: f64,
    pub border_part_of_window: bool,
}

impl Default for GeneralState {
    fn default() -> Self {
        Self {
            border_size: 1,
            gaps_in: 2,
            gaps_out: 10,
            active_border: "0xffffffff".to_string(),
            inactive_border: "0xff444444".to_string(),
            rounding: 0,
            rounding_power: 2.0,
            active_opacity: 1.0,
            inactive_opacity: 1.0,
            dim_modal: true,
            dim_inactive: false,
            dim_strength: 0.5,
            dim_special: 0.2,
            dim_around: 0.4,
            border_part_of_window: true,
        }
    }
}

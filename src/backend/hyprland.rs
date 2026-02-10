use std::process::Command;
use crate::state::GeneralState;

pub fn apply_general(state: &GeneralState) -> Result<(), String> {
    let status = Command::new("hyprctl")
        .args(["keyword", "general:border_size", &state.border_size.to_string()])
        .status()
        .map_err(|e| e.to_string())?;

    if !status.success() {
        return Err("hyprctl failed".into());
    }

    Ok(())
}

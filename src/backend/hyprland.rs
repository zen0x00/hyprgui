use std::process::Command;
use crate::state::GeneralState;

pub fn apply_general(state: &GeneralState) -> Result<(), String> {
    let cmds = [
        ("general:border_size", state.border_size.to_string()),
        ("general:gaps_in", state.gaps_in.to_string()),
        ("general:gaps_out", state.gaps_out.to_string()),
    ];

    for (key, value) in cmds {
        let status = std::process::Command
            ::new("hyprctl")
            .args(["keyword", key, &value])
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(format!("Failed to apply {}", key));
        }
    }

    Ok(())
}

use std::process::Command;
use serde_json::Value;

use crate::state::GeneralState;

pub fn read_general() -> Result<GeneralState, String> {
    Ok(GeneralState {
        border_size: read_int("general:border_size")?,
        gaps_in: read_gap("general:gaps_in", 5)?,
        gaps_out: read_gap("general:gaps_out", 20)?,
    })
}

fn read_int(key: &str) -> Result<i32, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    json["int"]
        .as_i64()
        .map(|v| v as i32)
        .ok_or_else(|| format!("Invalid int for {}", key))
}

fn read_gap(key: &str, fallback: i32) -> Result<i32, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    // Hyprland returns: "custom": "10 10 10 10"
    if let Some(s) = json["custom"].as_str() {
        return s
            .split_whitespace()
            .next()
            .and_then(|v| v.parse::<i32>().ok())
            .ok_or_else(|| format!("Invalid gap for {}", key));
    }

    Ok(fallback)
}

pub fn apply_general(state: &GeneralState) -> Result<(), String> {
    let cmds = [
        ("general:border_size", state.border_size.to_string()),
        ("general:gaps_in", state.gaps_in.to_string()),
        ("general:gaps_out", state.gaps_out.to_string()),
    ];

    for (key, value) in cmds {
        let status = Command::new("hyprctl")
            .args(["keyword", key, &value])
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(format!("Failed to apply {}", key));
        }
    }

    Ok(())
}

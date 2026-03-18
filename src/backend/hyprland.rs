use std::process::Command;
use serde_json::Value;

use crate::state::GeneralState;

pub fn read_general() -> Result<GeneralState, String> {
    Ok(GeneralState {
        border_size: read_int("general:border_size")?,
        gaps_in: read_gap("general:gaps_in", 5)?,
        gaps_out: read_gap("general:gaps_out", 20)?,
        active_border: read_gradient("general:col.active_border", "0xffffffff")?,
        inactive_border: read_gradient("general:col.inactive_border", "0xff444444")?,
        rounding: read_int("decoration:rounding")?,
        rounding_power: read_float("decoration:rounding_power", 2.0)?,
        active_opacity: read_float("decoration:active_opacity", 1.0)?,
        inactive_opacity: read_float("decoration:inactive_opacity", 1.0)?,
        dim_modal: read_bool("decoration:dim_modal", true)?,
        dim_inactive: read_bool("decoration:dim_inactive", false)?,
        dim_strength: read_float("decoration:dim_strength", 0.5)?,
        dim_special: read_float("decoration:dim_special", 0.2)?,
        dim_around: read_float("decoration:dim_around", 0.4)?,
        border_part_of_window: read_bool("decoration:border_part_of_window", true)?,
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

fn read_gradient(key: &str, fallback: &str) -> Result<String, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    Ok(json["str"]
        .as_str()
        .unwrap_or(fallback)
        .to_string())
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

fn read_bool(key: &str, fallback: bool) -> Result<bool, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    Ok(json["int"].as_i64().map(|v| v != 0).unwrap_or(fallback))
}

fn read_float(key: &str, fallback: f64) -> Result<f64, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    Ok(json["float"].as_f64().unwrap_or(fallback))
}

pub fn apply_keyword(key: &str, value: &str) {
    let _ = Command::new("hyprctl")
        .args(["keyword", key, value])
        .status();
}

pub fn apply_general(state: &GeneralState) -> Result<(), String> {
    let cmds = [
        ("general:border_size", state.border_size.to_string()),
        ("general:gaps_in", state.gaps_in.to_string()),
        ("general:gaps_out", state.gaps_out.to_string()),
        ("general:col.active_border", state.active_border.clone()),
        ("general:col.inactive_border", state.inactive_border.clone()),
        ("decoration:rounding", state.rounding.to_string()),
        ("decoration:rounding_power", state.rounding_power.to_string()),
        ("decoration:active_opacity", state.active_opacity.to_string()),
        ("decoration:inactive_opacity", state.inactive_opacity.to_string()),
        ("decoration:dim_modal", if state.dim_modal { "1" } else { "0" }.to_string()),
        ("decoration:dim_inactive", if state.dim_inactive { "1" } else { "0" }.to_string()),
        ("decoration:dim_strength", state.dim_strength.to_string()),
        ("decoration:dim_special", state.dim_special.to_string()),
        ("decoration:dim_around", state.dim_around.to_string()),
        ("decoration:border_part_of_window", if state.border_part_of_window { "1" } else { "0" }.to_string()),
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

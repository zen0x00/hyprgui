use std::process::Command;
use crate::state::GeneralState;
use serde_json::Value;

/*
    ─────────────────────────────────────────────
    READ
    ─────────────────────────────────────────────
    IMPORTANT RULE:
    - NEVER fail the entire read because one option failed
    - Hyprland options are inconsistent (int vs custom string)
    - We must be PARTIAL-SAFE
*/

pub fn read_general() -> Result<GeneralState, String> {
    let mut state = GeneralState::default();

    if let Ok(v) = read_int("general:border_size") {
        state.border_size = v;
    }

    if let Ok(v) = read_gap("general:gaps_in", state.gaps_in) {
        state.gaps_in = v;
    }

    if let Ok(v) = read_gap("general:gaps_out", state.gaps_out) {
        state.gaps_out = v;
    }

    Ok(state)
}

/*
    Read a gap option.
    Hyprland returns either:
    - { "custom": "10 10 10 10" }
    - { "int": 10 }
*/
fn read_gap(key: &str, fallback: i32) -> Result<i32, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    // Preferred: custom quad string
    if let Some(s) = json.get("custom").and_then(|v| v.as_str()) {
        if let Some(first) = s.split_whitespace().next() {
            if let Ok(v) = first.parse::<i32>() {
                return Ok(v);
            }
        }
    }

    // Fallback: plain int
    if let Some(v) = json.get("int").and_then(|v| v.as_i64()) {
        return Ok(v as i32);
    }

    // Final fallback: keep previous value
    Ok(fallback)
}

/*
    Read a simple integer option (border_size)
*/
fn read_int(key: &str) -> Result<i32, String> {
    let output = Command::new("hyprctl")
        .args(["getoption", key, "-j"])
        .output()
        .map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;

    json.get("int")
        .and_then(|v| v.as_i64())
        .map(|v| v as i32)
        .ok_or_else(|| format!("Invalid int value for {}", key))
}

/*
    ─────────────────────────────────────────────
    APPLY
    ─────────────────────────────────────────────
    NOTE:
    - hyprctl keyword only affects runtime
    - persistence is handled elsewhere (or later)
*/
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

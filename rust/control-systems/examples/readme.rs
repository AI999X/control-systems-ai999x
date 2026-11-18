//! Control Systems: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p control-systems-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&control_systems_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "ctrl_lag_design", "inputs": {"beta": "10", "w_c": "1"}}));
    println!();
    show(json!({"key": "ctrl_specs_to_poles", "inputs": {"OS": "0.1630335348", "Ts": "2"}}));
    println!();
    show(json!({"key": "ctrl_backward_euler", "inputs": {"num": [1], "den": [1, 1], "T": "0.1"}}));
    println!();
    show(json!({"key": "ctrl_kalman_scalar_step", "inputs": {"x": "0", "P": "1", "a": "1", "q": "1", "r": "2", "z": "4"}}));
    println!();
    show(json!({"key": "ctrl_bandwidth", "inputs": {"num": [1], "den": [1, 1]}}));
    println!();
    show(json!({"key": "ctrl_log_decrement", "inputs": {"x1": "1", "xn": "0.5", "n": "1"}}));
    println!();
    show(json!({"key": "ctrl_dc_motor", "inputs": {"J": "0.01", "b": "0.1", "Kt": "0.01", "R": "1", "L": "0.5"}}));
    println!();
    show(json!({"key": "ctrl_lqr", "inputs": {"A": [[0, 1], [0, 0]], "B": [0, 1], "Q": [[1, 0], [0, 1]], "R": "1"}}));
    println!();
    show(json!({"key": "ctrl_pid_closed_loop", "inputs": {"num": [1], "den": [1, 1], "Kp": "3", "Ki": "2"}}));
    println!();
    show(json!({"key": "ctrl_arrival_angle", "inputs": {"num": [1, 2, 2], "den": [1, 0]}}));
    println!();
    show(json!({"key": "ctrl_lyapunov", "inputs": {"A": [[-1, 0], [0, -2]], "Q": [[2, 0], [0, 4]]}}));
    println!();
    show(json!({"key": "ctrl_state_feedback_poles", "inputs": {"A": [[0, 1], [0, 0]], "B": [0, 1], "K": [4, 4]}}));
    println!();
    show(json!({"key": "ctrl_error_constants", "inputs": {"num": [10], "den": [1, 3, 2], "input": "step"}}));
    println!();
    show(json!({"key": "ctrl_pade", "inputs": {"theta": "2", "n": "1"}}));
    println!();
    show(json!({"key": "ctrl_damping_from_overshoot", "inputs": {"OS": "0.2"}}));
}

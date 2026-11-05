//! control_systems: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/control_systems/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "ctrl_ackermann",
    "ctrl_arrival_angle",
    "ctrl_backward_euler",
    "ctrl_bandwidth",
    "ctrl_bandwidth_second",
    "ctrl_bode_point",
    "ctrl_breakaway_points",
    "ctrl_c2d_foh",
    "ctrl_c2d_zoh",
    "ctrl_closed_loop_first_order_p",
    "ctrl_cohen_coon",
    "ctrl_controllability",
    "ctrl_ctrb_gramian",
    "ctrl_damping_from_overshoot",
    "ctrl_dc_gain",
    "ctrl_dc_motor",
    "ctrl_deadbeat_gain",
    "ctrl_departure_angle",
    "ctrl_discrete_dc_gain",
    "ctrl_discrete_kalman_gain",
    "ctrl_discrete_lyapunov",
    "ctrl_discrete_pid",
    "ctrl_discrete_step",
    "ctrl_dlqr",
    "ctrl_error_constants",
    "ctrl_feedback_connection",
    "ctrl_final_value_theorem",
    "ctrl_first_order_freq",
    "ctrl_first_order_specs",
    "ctrl_first_order_step",
    "ctrl_fopdt_smith",
    "ctrl_fopdt_step",
    "ctrl_forward_euler",
    "ctrl_h2_norm",
    "ctrl_hinf_norm",
    "ctrl_identify_second_order",
    "ctrl_initial_value_theorem",
    "ctrl_jury_test",
    "ctrl_kalman_bucy",
    "ctrl_kalman_scalar_steady",
    "ctrl_kalman_scalar_step",
    "ctrl_lag_design",
    "ctrl_lead_design",
    "ctrl_lead_max_phase",
    "ctrl_locus_asymptotes",
    "ctrl_locus_gain_at_point",
    "ctrl_log_decrement",
    "ctrl_lqr",
    "ctrl_lyapunov",
    "ctrl_mass_spring_damper",
    "ctrl_nyquist",
    "ctrl_observability",
    "ctrl_observer_gain",
    "ctrl_obsv_gramian",
    "ctrl_pade",
    "ctrl_parallel_connection",
    "ctrl_peak_time",
    "ctrl_percent_overshoot",
    "ctrl_pi_pole_placement",
    "ctrl_pid_closed_loop",
    "ctrl_pid_parallel_to_standard",
    "ctrl_pid_standard_to_parallel",
    "ctrl_pole_stability",
    "ctrl_poles_zeros",
    "ctrl_resonant_peak",
    "ctrl_rise_time_second",
    "ctrl_routh_hurwitz",
    "ctrl_s_to_z_pole",
    "ctrl_scalar_lqr",
    "ctrl_second_order_freq",
    "ctrl_second_order_from_poles",
    "ctrl_second_order_params",
    "ctrl_second_order_step",
    "ctrl_sensitivity_peak",
    "ctrl_series_connection",
    "ctrl_settling_time",
    "ctrl_simc_pi",
    "ctrl_specs_to_poles",
    "ctrl_ss_dc_gain",
    "ctrl_ss_eigenvalues",
    "ctrl_ss_response",
    "ctrl_ss_to_tf",
    "ctrl_stability_margins",
    "ctrl_stable_gain_range",
    "ctrl_state_feedback_poles",
    "ctrl_state_transition",
    "ctrl_step_response",
    "ctrl_tf_evaluate",
    "ctrl_tf_to_ss",
    "ctrl_true_settling_time",
    "ctrl_tustin",
    "ctrl_tustin_prewarp",
    "ctrl_z_to_s_pole",
    "ctrl_zn_from_plant",
    "ctrl_zn_reaction",
    "ctrl_zn_ultimate",
    "ctrl_zoh_first_order",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "ctrl_ackermann" => {
            let r = f_ackermann(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_112(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_arrival_angle" => {
            let r = f_arrival_angle(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_274(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_backward_euler" => {
            let r = f_backward_euler(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_367(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_bandwidth" => {
            let r = f_bandwidth_general(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_561(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_bandwidth_second" => {
            let r = f_bandwidth_second(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_607(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_bode_point" => {
            let r = f_bode_point(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_680(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_breakaway_points" => {
            let r = f_breakaway_points(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_835(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_c2d_foh" => {
            let r = f_c2d_foh(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1046(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_c2d_zoh" => {
            let r = f_c2d_zoh(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1254(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_closed_loop_first_order_p" => {
            let r = f_closed_loop_first_order_p(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1302(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_cohen_coon" => {
            let r = f_cohen_coon(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("controller") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1356(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_controllability" => {
            let r = f_controllability(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1463(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_ctrb_gramian" => {
            let r = f_ctrb_gramian(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1692(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_damping_from_overshoot" => {
            let r = f_damping_from_overshoot(match v.get("OS") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1738(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_dc_gain" => {
            let r = f_dc_gain(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1890(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_dc_motor" => {
            let r = f_dc_motor(match v.get("J") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kt") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2017(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_deadbeat_gain" => {
            let r = f_deadbeat_gain(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2160(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_departure_angle" => {
            let r = f_departure_angle(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2322(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_discrete_dc_gain" => {
            let r = f_discrete_dc_gain(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2474(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_discrete_kalman_gain" => {
            let r = f_discrete_kalman_gain(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2698(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_discrete_lyapunov" => {
            let r = f_discrete_lyapunov(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2932(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_discrete_pid" => {
            let r = f_discrete_pid(match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2980(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_discrete_step" => {
            let r = f_discrete_step(match v.get("numz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("denz") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3058(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_dlqr" => {
            let r = f_dlqr(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3371(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_error_constants" => {
            let r = f_error_constants(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("input") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3470(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_feedback_connection" => {
            let r = f_feedback_connection(match v.get("num_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den_g") { Some(x) => py::from_val(x), None => { return None } }, match v.get("num_h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("den_h") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("sign") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::str("negative")) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3563(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_final_value_theorem" => {
            let r = f_final_value_theorem(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3726(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_first_order_freq" => {
            let r = f_first_order_freq(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3774(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_first_order_specs" => {
            let r = f_first_order_specs(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3822(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_first_order_step" => {
            let r = f_first_order_step(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3875(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_fopdt_smith" => {
            let r = f_fopdt_smith(match v.get("t28") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t63") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dy") { Some(x) => py::from_val(x), None => { return None } }, match v.get("du") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3930(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_fopdt_step" => {
            let r = f_fopdt_step(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_forward_euler" => {
            let r = f_forward_euler(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4078(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_h2_norm" => {
            let r = f_h2_norm(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4198(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_hinf_norm" => {
            let r = f_hinf_norm(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4403(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_identify_second_order" => {
            let r = f_identify_second_order(match v.get("y_peak") { Some(x) => py::from_val(x), None => { return None } }, match v.get("y_final") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_peak") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4457(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_initial_value_theorem" => {
            let r = f_initial_value_theorem(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4529(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_jury_test" => {
            let r = f_jury_test(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4589(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_kalman_bucy" => {
            let r = f_kalman_bucy(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4902(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_kalman_scalar_steady" => {
            let r = f_kalman_scalar_steady(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4952(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_kalman_scalar_step" => {
            let r = f_kalman_scalar_step(match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5002(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_lag_design" => {
            let r = f_lag_design(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ratio") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(10.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5052(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_lead_design" => {
            let r = f_lead_design(match v.get("phi_max_deg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w_m") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5101(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_lead_max_phase" => {
            let r = f_lead_max_phase(match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5149(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_locus_asymptotes" => {
            let r = f_locus_asymptotes(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5221(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_locus_gain_at_point" => {
            let r = f_locus_gain_at_point(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_im") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5293(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_log_decrement" => {
            let r = f_log_decrement(match v.get("x1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("xn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5343(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_lqr" => {
            let r = f_lqr(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5655(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_lyapunov" => {
            let r = f_lyapunov(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5888(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_mass_spring_damper" => {
            let r = f_mass_spring_damper(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5946(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_nyquist" => {
            let r = f_nyquist_encirclements(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6103(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_observability" => {
            let r = f_observability(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6211(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_observer_gain" => {
            let r = f_observer_gain(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6354(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_obsv_gramian" => {
            let r = f_obsv_gramian(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6581(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_pade" => {
            let r = f_pade(match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6634(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_parallel_connection" => {
            let r = f_parallel_connection(match v.get("num1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("num2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6712(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_peak_time" => {
            let r = f_peak_time(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6762(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_percent_overshoot" => {
            let r = f_percent_overshoot(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6809(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_pi_pole_placement" => {
            let r = f_pi_pole_placement(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6857(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_pid_closed_loop" => {
            let r = f_pid_closed_loop(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7012(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_pid_parallel_to_standard" => {
            let r = f_pid_parallel_to_standard(match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ki") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Kd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7060(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_pid_standard_to_parallel" => {
            let r = f_pid_standard_to_parallel(match v.get("Kp") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ti") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Td") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7108(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_pole_stability" => {
            let r = f_pole_stability(match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7250(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_poles_zeros" => {
            let r = f_poles_zeros(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7388(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_resonant_peak" => {
            let r = f_resonant_peak(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_rise_time_second" => {
            let r = f_rise_time_second(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7505(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_routh_hurwitz" => {
            let r = f_routh_hurwitz(match v.get("coefficients") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7572(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_s_to_z_pole" => {
            let r = f_s_to_z_pole(match v.get("s_re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_im") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7622(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_scalar_lqr" => {
            let r = f_scalar_lqr(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7670(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_second_order_freq" => {
            let r = f_second_order_freq(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7724(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_second_order_from_poles" => {
            let r = f_second_order_from_poles(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7770(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_second_order_params" => {
            let r = f_second_order_params(match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_second_order_step" => {
            let r = f_second_order_step(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7895(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_sensitivity_peak" => {
            let r = f_sensitivity_peak(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8098(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_series_connection" => {
            let r = f_series_connection(match v.get("num1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("num2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8171(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_settling_time" => {
            let r = f_settling_time(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("band") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.02)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8224(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_simc_pi" => {
            let r = f_simc_pi(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8272(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_specs_to_poles" => {
            let r = f_specs_to_poles(match v.get("OS") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ts") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8321(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_ss_dc_gain" => {
            let r = f_ss_dc_gain(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8413(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_ss_eigenvalues" => {
            let r = f_ss_eigenvalues(match v.get("A") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8586(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_ss_response" => {
            let r = f_ss_response(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8694(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_ss_to_tf" => {
            let r = f_ss_to_tf(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8784(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_stability_margins" => {
            let r = f_stability_margins(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_8980(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_stable_gain_range" => {
            let r = f_stable_gain_range(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9207(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_state_feedback_poles" => {
            let r = f_state_feedback_poles(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9383(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_state_transition" => {
            let r = f_state_transition(match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9484(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_step_response" => {
            let r = f_step_response(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9669(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_tf_evaluate" => {
            let r = f_tf_evaluate(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s_im") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9740(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_tf_to_ss" => {
            let r = f_tf_to_ss(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9813(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_true_settling_time" => {
            let r = f_true_settling_time(match v.get("zeta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("wn") { Some(x) => py::from_val(x), None => { return None } }, match v.get("band") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.02)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9892(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_tustin" => {
            let r = f_tustin(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_9985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_tustin_prewarp" => {
            let r = f_tustin_prewarp(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("w") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_10080(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_z_to_s_pole" => {
            let r = f_z_to_s_pole(match v.get("z_re") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z_im") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_10133(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_zn_from_plant" => {
            let r = f_zn_from_plant(match v.get("num") { Some(x) => py::from_val(x), None => { return None } }, match v.get("den") { Some(x) => py::from_val(x), None => { return None } }, match v.get("controller") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_10335(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_zn_reaction" => {
            let r = f_zn_reaction(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("controller") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_10389(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_zn_ultimate" => {
            let r = f_zn_ultimate(match v.get("Ku") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("controller") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_10443(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "ctrl_zoh_first_order" => {
            let r = f_zoh_first_order(match v.get("K") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_10489(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_INF() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(f64::INFINITY)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_PI() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(3.141592653589793)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__G(mut v_num: Py, mut v_den: Py, mut v_s: Py) -> R {
    let mut v_d: Py = Py::None;
    v_d = f__pval(v_den.clone(), v_s.clone())?;
    let _ = f__need({ let t10311 = py::abs(&v_d.clone())?; Py::Bool({ let t10312 = Py::Float(1e-300); if py::gt(&t10311, &t10312)? { true } else { false } }) }, Py::str("the transfer function has a pole at this point"))?;
    return Ok(py::truediv(&f__pval(v_num.clone(), v_s.clone())?, &v_d.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__acker(mut v_A: Py, mut v_B: Py, mut v_alpha: Py) -> R {
    let mut v_Af: Py = Py::None;
    let mut v_Bf: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_al: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    let t6274 = (Py::tuple(vec![f__frac(v_A.clone())?, f__frac(v_B.clone())?])).items()?;
    if t6274.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Af = t6274[0].clone();
    v_Bf = t6274[1].clone();
    v_al = { let mut out = vec![]; for t6275 in py::iter(&(v_alpha.clone()))? { let mut v_x: Py; 
v_x = t6275;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    let _ = f__need({ let t6276 = py::len(&v_al.clone())?; Py::Bool({ let t6277 = py::add(&v_n.clone(), &Py::Int(1i64))?; if py::eq(&t6276, &t6277) { true } else { false } }) }, Py::str("the desired characteristic polynomial must have degree n"))?;
    let _ = f__need({ let t6278 = py::getitem(&v_al.clone(), &Py::Int(0i64))?; Py::Bool({ let t6279 = Py::Int(0i64); if py::ne(&t6278, &t6279) { true } else { false } }) }, Py::str("the desired polynomial's leading coefficient must not be zero"))?;
    v_al = { let mut out = vec![]; for t6280 in py::iter(&(v_al.clone()))? { let mut v_x: Py; 
v_x = t6280;
out.push(py::truediv(&v_x.clone(), &py::getitem(&v_al.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    v_W = f__ctrb(v_Af.clone(), v_Bf.clone())?;
    let _ = f__need({ let t6287 = f__rank(v_W.clone())?; Py::Bool({ let t6288 = v_n.clone(); if py::eq(&t6287, &t6288) { true } else { false } }) }, Py::str("(A, B) is not controllable: the poles cannot all be placed"))?;
    v_M = f__eye(v_n.clone(), py::fraction(&[Py::Int(1i64)])?)?;
    v_I = f__eye(v_n.clone(), py::fraction(&[Py::Int(1i64)])?)?;
    for t6312 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t6312;
        v_M = f__madd(f__mmul(v_Af.clone(), v_M.clone())?, f__mscale(v_I.clone(), py::getitem(&v_al.clone(), &v_k.clone())?)?)?;
    }
    v_e = py::add(&py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?, &Py::list(vec![py::fraction(&[Py::Int(1i64)])?]))?;
    v_v = f__solve(f__mT(v_W.clone())?, v_e.clone(), Py::Bool(true))?;
    return Ok({ let mut out = vec![]; for t6351 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6351;
out.push(py::sum(&{ let mut out = vec![]; for t6352 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6352;
out.push(py::mul(&py::getitem(&v_v.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__angle_rule(mut v_target: Py, mut v_others_plus: Py, mut v_others_minus: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_z: Py = Py::None;
    v_a = Py::Float(180.0);
    for t2316 in py::iter(&(v_others_plus.clone()))? {
        v_z = t2316;
        v_a = py::iadd(&v_a, &f__deg(py::sub(&py::complex(&[v_target.clone()])?, &py::complex(&[v_z.clone()])?)?)?)?;
    }
    for t2317 in py::iter(&(v_others_minus.clone()))? {
        v_p = t2317;
        v_a = py::sub(&v_a, &f__deg(py::sub(&py::complex(&[v_target.clone()])?, &py::complex(&[v_p.clone()])?)?)?)?;
    }
    return Ok(f__wrap180(v_a.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_flo: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_mid: Py = Py::None;
    v_flo = py::call(&v_f.clone(), &[v_lo.clone()])?;
    for t9863 in py::range_iter(&[(Py::Int(300i64)).i()?]) {
        v__ = t9863;
        v_mid = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
        if ({ let t9868 = { let t9864 = v_mid.clone(); Py::Bool({ let t9865 = v_lo.clone(); if py::eq(&t9864, &t9865) { true } else { false } }) }; if t9868.truthy() { t9868 } else { { let t9866 = v_mid.clone(); Py::Bool({ let t9867 = v_hi.clone(); if py::eq(&t9866, &t9867) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_mid.clone()])?;
        if ({ let t9869 = { let t9871 = v_fm.clone(); Py::Bool({ let t9872 = Py::Int(0i64); if py::gt(&t9871, &t9872)? { true } else { false } }) }; Py::Bool({ let t9870 = { let t9873 = v_flo.clone(); Py::Bool({ let t9874 = Py::Int(0i64); if py::gt(&t9873, &t9874)? { true } else { false } }) }; if py::eq(&t9869, &t9870) { true } else { false } }) }).truthy() {
            let t9875 = (Py::tuple(vec![v_mid.clone(), v_fm.clone()])).items()?;
            if t9875.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_lo = t9875[0].clone();
            v_flo = t9875[1].clone();
        } else {
            v_hi = v_mid.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__canon(mut v_roots: Py, mut v_tol: Py) -> R {
    let mut v_best: Py = Py::None;
    let mut v_big: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dn: Py = Py::None;
    let mut v_downs: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let mut v_real: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_ups: Py = Py::None;
    let t9546 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![]), Py::list(vec![])])).items()?;
    if t9546.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_real = t9546[0].clone();
    v_ups = t9546[1].clone();
    v_downs = t9546[2].clone();
    v_big = { let t = { let mut out = vec![]; for t9547 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t9547;
out.push(py::abs(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }; if t.items()?.is_empty() { Py::Float(0.0) } else { py::minmax(&[t], true)? } };
    for t9548 in py::iter(&(v_roots.clone()))? {
        v_r = t9548;
        v_r = py::complex(&[v_r.clone()])?;
        if ({ let t9549 = py::abs(&v_r.clone())?; Py::Bool({ let t9550 = py::mul(&Py::Float(1e-12), &v_big.clone())?; if py::le(&t9549, &t9550)? { true } else { false } }) }).truthy() {
            v_r = Py::Complex(0.0, 0.0);
        }
        if ({ let t9551 = py::abs(&py::real(&v_r.clone())?)?; Py::Bool({ let t9552 = py::mul(&Py::Float(1e-12), &py::abs(&v_r.clone())?)?; if py::le(&t9551, &t9552)? { true } else { false } }) }).truthy() {
            v_r = py::complex(&[Py::Float(0.0), py::imag(&v_r.clone())?])?;
        }
        if ({ let t9553 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t9554 = py::mul(&v_tol.clone(), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::le(&t9553, &t9554)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_real.clone(), "append", &[py::real(&v_r.clone())?])?;
        } else {
            if ({ let t9555 = py::imag(&v_r.clone())?; Py::Bool({ let t9556 = Py::Int(0i64); if py::gt(&t9555, &t9556)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_ups.clone(), "append", &[v_r.clone()])?;
            } else {
                let _ = py::method(&v_downs.clone(), "append", &[v_r.clone()])?;
            }
        }
    }
    v_out = { let mut out = vec![]; for t9557 in py::iter(&(v_real.clone()))? { let mut v_x: Py; 
v_x = t9557;
out.push(f__r12(v_x.clone())?); } Py::list(out) };
    v_ups = py::sorted_key(&v_ups.clone(), &{ let v_big = v_big.clone(); let v_downs = v_downs.clone(); let v_out = v_out.clone(); let v_r = v_r.clone(); let v_real = v_real.clone(); let v_roots = v_roots.clone(); let v_tol = v_tol.clone(); let v_ups = v_ups.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&v_x.clone())?, py::imag(&v_x.clone())?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, (Py::Bool(false)).truthy())?;
    for t9558 in py::iter(&(v_ups.clone()))? {
        v_u = t9558;
        if (py::not(&v_downs.clone())).truthy() {
            break;
        }
        v_best = Py::Int(0i64);
        for t9559 in py::range_iter(&[(py::len(&v_downs.clone())?).i()?]) {
            v_k = t9559;
            if ({ let t9560 = py::abs(&py::sub(&py::getitem(&v_downs.clone(), &v_k.clone())?, &py::method(&v_u.clone(), "conjugate", &[])?)?)?; Py::Bool({ let t9561 = py::abs(&py::sub(&py::getitem(&v_downs.clone(), &v_best.clone())?, &py::method(&v_u.clone(), "conjugate", &[])?)?)?; if py::lt(&t9560, &t9561)? { true } else { false } }) }).truthy() {
                v_best = v_k.clone();
            }
        }
        v_dn = py::method(&v_downs.clone(), "pop", &[v_best.clone()])?;
        v_re_ = f__r12(py::truediv(&py::add(&py::real(&v_u.clone())?, &py::real(&v_dn.clone())?)?, &Py::Int(2i64))?)?;
        v_im = f__r12(py::truediv(&py::sub(&py::imag(&v_u.clone())?, &py::imag(&v_dn.clone())?)?, &Py::Int(2i64))?)?;
        let _ = py::method(&v_out.clone(), "append", &[py::complex(&[v_re_.clone(), py::neg(&v_im.clone())?])?])?;
        let _ = py::method(&v_out.clone(), "append", &[py::complex(&[v_re_.clone(), v_im.clone()])?])?;
    }
    for t9562 in py::iter(&(v_downs.clone()))? {
        v_d = t9562;
        let _ = py::method(&v_out.clone(), "append", &[py::complex(&[f__r12(py::real(&v_d.clone())?)?, f__r12(py::imag(&v_d.clone())?)?])?])?;
    }
    let _ = py::method_sort(&v_out.clone(), Some(&{ let v_best = v_best.clone(); let v_big = v_big.clone(); let v_d = v_d.clone(); let v_dn = v_dn.clone(); let v_downs = v_downs.clone(); let v_im = v_im.clone(); let v_k = v_k.clone(); let v_out = v_out.clone(); let v_r = v_r.clone(); let v_re_ = v_re_.clone(); let v_real = v_real.clone(); let v_roots = v_roots.clone(); let v_tol = v_tol.clone(); let v_u = v_u.clone(); let v_ups = v_ups.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_x: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(Py::tuple(vec![py::real(&py::complex(&[v_x.clone()])?)?, py::imag(&py::complex(&[v_x.clone()])?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }), false)?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__care(mut v_A: Py, mut v_B: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_Acl: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_Kn: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_Qk: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rho: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    let _ = f__need({ let t5434 = f__rank(f__ctrb(f__frac(v_A.clone())?, f__frac(v_B.clone())?)?)?; Py::Bool({ let t5435 = v_n.clone(); if py::eq(&t5434, &t5435) { true } else { false } }) }, Py::str("(A, B) must be controllable"))?;
    v_rho = py::add(&Py::Float(1.0), &py::minmax(&[{ let mut out = vec![]; for t5461 in py::iter(&(v_A.clone()))? { let mut v_r: Py; 
v_r = t5461;
out.push(py::sum(&{ let mut out = vec![]; for t5462 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t5462;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }, None)?); } Py::list(out) }], true)?)?;
    v_alpha = Py::list(vec![Py::Float(1.0)]);
    for t5463 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5463;
        v_alpha = f__pmul(v_alpha.clone(), Py::list(vec![Py::Float(1.0), py::add(&py::float(&py::math1("ceil", &v_rho.clone())?)?, &v_i.clone())?]))?;
    }
    v_K = { let mut out = vec![]; for t5524 in py::iter(&(f__acker(v_A.clone(), v_B.clone(), v_alpha.clone())?))? { let mut v_x: Py; 
v_x = t5524;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    v_P = Py::None;
    for t5525 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t5525;
        v_Acl = { let mut out = vec![]; for t5526 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5526;
out.push({ let mut out = vec![]; for t5527 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5527;
out.push(py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
        v_Qk = { let mut out = vec![]; for t5528 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5528;
out.push({ let mut out = vec![]; for t5529 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5529;
out.push(py::add(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::mul(&v_R.clone(), &py::getitem(&v_K.clone(), &v_i.clone())?)?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
        v_P = f__lyap(v_Acl.clone(), v_Qk.clone(), Py::Bool(false))?;
        v_Kn = { let mut out = vec![]; for t5538 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5538;
out.push(py::truediv(&py::sum(&{ let mut out = vec![]; for t5539 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5539;
out.push(py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?, &v_R.clone())?); } Py::list(out) };
        v_d = py::minmax(&[{ let mut out = vec![]; for t5540 in py::iter(&(py::zip(&[v_Kn.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5541 = (t5540).items()?;
if t5541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5541[0].clone();
v_b = t5541[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) }], true)?;
        v_K = v_Kn.clone();
        if ({ let t5542 = v_d.clone(); Py::Bool({ let t5543 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t5544 in py::iter(&(v_K.clone()))? { let mut v_x: Py; 
v_x = t5544;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?], true)?)?; if py::le(&t5542, &t5543)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_Acl = { let mut out = vec![]; for t5545 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5545;
out.push({ let mut out = vec![]; for t5546 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5546;
out.push(py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_Qk = { let mut out = vec![]; for t5547 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5547;
out.push({ let mut out = vec![]; for t5548 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5548;
out.push(py::add(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::mul(&v_R.clone(), &py::getitem(&v_K.clone(), &v_i.clone())?)?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_P = f__lyap(v_Acl.clone(), v_Qk.clone(), Py::Bool(false))?;
    v_K = { let mut out = vec![]; for t5549 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5549;
out.push(py::truediv(&py::sum(&{ let mut out = vec![]; for t5550 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5550;
out.push(py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?, &v_R.clone())?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_P.clone(), v_K.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ccf(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_den = f__ptrim(v_den.clone())?;
    v_num = f__ptrim(v_num.clone())?;
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    v_a0 = py::getitem(&v_den.clone(), &Py::Int(0i64))?;
    v_den = { let mut out = vec![]; for t9797 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t9797;
out.push(py::truediv(&v_x.clone(), &v_a0.clone())?); } Py::list(out) };
    v_num = f__pad({ let mut out = vec![]; for t9798 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t9798;
out.push(py::truediv(&v_x.clone(), &v_a0.clone())?); } Py::list(out) }, py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    v_D = py::getitem(&v_num.clone(), &Py::Int(0i64))?;
    v_b = { let mut out = vec![]; for t9799 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t9799;
out.push(py::sub(&py::getitem(&v_num.clone(), &v_i.clone())?, &py::mul(&v_D.clone(), &py::getitem(&v_den.clone(), &v_i.clone())?)?)?); } Py::list(out) };
    v_A = { let mut out = vec![]; for t9800 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t9800;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?); } Py::list(out) };
    py::setitem(&v_A.clone(), &Py::Int(0i64), { let mut out = vec![]; for t9801 in py::iter(&(py::slice(&v_den.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? { let mut v_x: Py; 
v_x = t9801;
out.push(py::neg(&v_x.clone())?); } Py::list(out) })?;
    for t9802 in py::range_iter(&[(Py::Int(1i64)).i()?, (v_n.clone()).i()?]) {
        v_i = t9802;
        py::setitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &py::sub(&v_i.clone(), &Py::Int(1i64))?, Py::Float(1.0))?;
    }
    v_B = py::add(&Py::list(vec![Py::Float(1.0)]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
    v_C = py::slice(&v_b.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    return Ok(Py::tuple(vec![v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cl_poles(mut v_A: Py, mut v_B: Py, mut v_K: Py) -> R {
    let mut v_Acl: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_Acl = { let mut out = vec![]; for t5561 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t5561;
out.push({ let mut out = vec![]; for t5562 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t5562;
out.push(py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    let t5566 = (f__faddeev(v_Acl.clone())?).items()?;
    if t5566.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t5566[0].clone();
    v__ = t5566[1].clone();
    return Ok(f__roots({ let mut out = vec![]; for t5654 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t5654;
out.push(py::float(&v_x.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__class(mut v_zeta: Py) -> R {
    if ({ let t7821 = v_zeta.clone(); Py::Bool({ let t7822 = Py::Int(0i64); if py::eq(&t7821, &t7822) { true } else { false } }) }).truthy() {
        return Ok(Py::str("undamped"));
    }
    if ({ let t7823 = v_zeta.clone(); Py::Bool({ let t7824 = Py::Int(1i64); if py::lt(&t7823, &t7824)? { true } else { false } }) }).truthy() {
        return Ok(Py::str("underdamped"));
    }
    if ({ let t7825 = v_zeta.clone(); Py::Bool({ let t7826 = Py::Int(1i64); if py::eq(&t7825, &t7826) { true } else { false } }) }).truthy() {
        return Ok(Py::str("critically damped"));
    }
    return Ok(Py::str("overdamped"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__classify(mut v_poles: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_on_axis: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rhp: Py = Py::None;
    v_on_axis = { let mut out = vec![]; for t7236 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t7236;
if ({ let t7237 = py::abs(&py::real(&py::complex(&[v_p.clone()])?)?)?; Py::Bool({ let t7238 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_p.clone()])?)?], true)?)?; if py::le(&t7237, &t7238)? { true } else { false } }) }).truthy() { out.push(v_p.clone()); }} Py::list(out) };
    v_rhp = { let mut out = vec![]; for t7239 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t7239;
if ({ let t7244 = { let t7240 = py::real(&py::complex(&[v_p.clone()])?)?; Py::Bool({ let t7241 = Py::Int(0i64); if py::gt(&t7240, &t7241)? { true } else { false } }) }; if !t7244.truthy() { t7244 } else { { let t7242 = v_p.clone(); Py::Bool({ let t7243 = v_on_axis.clone(); if !py::contains(&t7243, &t7242)? { true } else { false } }) } } }).truthy() { out.push(v_p.clone()); }} Py::list(out) };
    if (v_rhp.clone()).truthy() {
        return Ok(Py::str("unstable"));
    }
    if (py::not(&v_on_axis.clone())).truthy() {
        return Ok(Py::str("asymptotically stable"));
    }
    for t7245 in py::iter(&(py::enumerate(&v_on_axis.clone(), 0)?))? {
        let t7249 = (t7245).items()?;
        if t7249.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t7249[0].clone();
        v_p = t7249[1].clone();
        for t7246 in py::iter(&(py::slice(&v_on_axis.clone(), Some((py::add(&v_i.clone(), &Py::Int(1i64))?).i()?), None, None)?))? {
            v_q = t7246;
            if ({ let t7247 = py::abs(&py::sub(&py::complex(&[v_p.clone()])?, &py::complex(&[v_q.clone()])?)?)?; Py::Bool({ let t7248 = py::mul(&Py::Float(1e-07), &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_p.clone()])?)?], true)?)?; if py::le(&t7247, &t7248)? { true } else { false } }) }).truthy() {
                return Ok(Py::str("unstable"));
            }
        }
    }
    return Ok(Py::str("marginally stable"));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ctrb(mut v_A: Py, mut v_B: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_cols = Py::list(vec![Py::list((v_B.clone()).items()?)]);
    for t6281 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t6281;
        let _ = py::method(&v_cols.clone(), "append", &[f__mv(v_A.clone(), py::getitem(&v_cols.clone(), &py::neg(&Py::Int(1i64))?)?)?])?;
    }
    return Ok({ let mut out = vec![]; for t6285 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6285;
out.push({ let mut out = vec![]; for t6286 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6286;
out.push(py::getitem(&py::getitem(&v_cols.clone(), &v_j.clone())?, &v_i.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dare(mut v_A: Py, mut v_B: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_Acl: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_Kn: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_PB: Py = Py::None;
    let mut v_Qk: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_it: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    let _ = f__need({ let t3149 = f__rank(f__ctrb(f__frac(v_A.clone())?, f__frac(v_B.clone())?)?)?; Py::Bool({ let t3150 = v_n.clone(); if py::eq(&t3149, &t3150) { true } else { false } }) }, Py::str("(A, B) must be controllable"))?;
    v_K = { let mut out = vec![]; for t3230 in py::iter(&(f__acker(v_A.clone(), v_B.clone(), py::add(&Py::list(vec![Py::Float(1.0)]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?)?)?))? { let mut v_x: Py; 
v_x = t3230;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    for t3231 in py::range_iter(&[(Py::Int(201i64)).i()?]) {
        v_it = t3231;
        v_Acl = { let mut out = vec![]; for t3232 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3232;
out.push({ let mut out = vec![]; for t3233 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3233;
out.push(py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
        v_Qk = { let mut out = vec![]; for t3234 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3234;
out.push({ let mut out = vec![]; for t3235 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3235;
out.push(py::add(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::mul(&v_R.clone(), &py::getitem(&v_K.clone(), &v_i.clone())?)?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
        v_P = f__dlyap(v_Acl.clone(), v_Qk.clone(), Py::Bool(false))?;
        v_PB = { let mut out = vec![]; for t3245 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3245;
out.push(py::sum(&{ let mut out = vec![]; for t3246 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3246;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_B.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
        v_s = py::add(&v_R.clone(), &py::sum(&{ let mut out = vec![]; for t3247 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3247;
out.push(py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_PB.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?;
        v_Kn = { let mut out = vec![]; for t3248 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3248;
out.push(py::truediv(&py::sum(&{ let mut out = vec![]; for t3249 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3249;
out.push(py::mul(&py::getitem(&v_PB.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?, &v_s.clone())?); } Py::list(out) };
        v_d = py::minmax(&[{ let mut out = vec![]; for t3250 in py::iter(&(py::zip(&[v_Kn.clone(), v_K.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3251 = (t3250).items()?;
if t3251.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3251[0].clone();
v_b = t3251[1].clone();
out.push(py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?); } Py::list(out) }], true)?;
        v_K = v_Kn.clone();
        if ({ let t3257 = { let t3252 = v_d.clone(); Py::Bool({ let t3253 = py::mul(&Py::Float(1e-15), &py::minmax(&[Py::Float(1.0), py::minmax(&[{ let mut out = vec![]; for t3254 in py::iter(&(v_K.clone()))? { let mut v_x: Py; 
v_x = t3254;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?], true)?)?; if py::le(&t3252, &t3253)? { true } else { false } }) }; if !t3257.truthy() { t3257 } else { { let t3255 = v_it.clone(); Py::Bool({ let t3256 = Py::Int(0i64); if py::gt(&t3255, &t3256)? { true } else { false } }) } } }).truthy() {
            break;
        }
    }
    v_Acl = { let mut out = vec![]; for t3258 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3258;
out.push({ let mut out = vec![]; for t3259 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3259;
out.push(py::sub(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_Qk = { let mut out = vec![]; for t3260 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3260;
out.push({ let mut out = vec![]; for t3261 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3261;
out.push(py::add(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::mul(&v_R.clone(), &py::getitem(&v_K.clone(), &v_i.clone())?)?, &py::getitem(&v_K.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    v_P = f__dlyap(v_Acl.clone(), v_Qk.clone(), Py::Bool(false))?;
    v_PB = { let mut out = vec![]; for t3262 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3262;
out.push(py::sum(&{ let mut out = vec![]; for t3263 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3263;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_B.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_s = py::add(&v_R.clone(), &py::sum(&{ let mut out = vec![]; for t3264 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3264;
out.push(py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &py::getitem(&v_PB.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?;
    v_K = { let mut out = vec![]; for t3265 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3265;
out.push(py::truediv(&py::sum(&{ let mut out = vec![]; for t3266 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t3266;
out.push(py::mul(&py::getitem(&v_PB.clone(), &v_k.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_k.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?, &v_s.clone())?); } Py::list(out) };
    return Ok(Py::tuple(vec![v_P.clone(), v_K.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__deg(mut v_z: Py) -> R {
    return Ok(py::math1("degrees", &py::cmath("phase", &[py::complex(&[py::add(&py::real(&v_z.clone())?, &Py::Float(0.0))?, py::add(&py::imag(&v_z.clone())?, &Py::Float(0.0))?])?])?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__det(mut v_M: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_A = { let mut out = vec![]; for t6562 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t6562;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
    v_det = py::add(&Py::Int(1i64), &py::mul(&Py::Int(0i64), &py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?)?;
    for t6563 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t6563;
        v_p = Py::None;
        for t6564 in py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?]) {
            v_r = t6564;
            if ({ let t6565 = py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t6566 = Py::Int(0i64); if py::ne(&t6565, &t6566) { true } else { false } }) }).truthy() {
                v_p = v_r.clone();
                break;
            }
        }
        if ({ let t6567 = v_p.clone(); Py::Bool({ let t6568 = Py::None; if py::is(&t6567, &t6568) { true } else { false } }) }).truthy() {
            return Ok(py::mul(&Py::Int(0i64), &v_det.clone())?);
        }
        if ({ let t6569 = v_p.clone(); Py::Bool({ let t6570 = v_c.clone(); if py::ne(&t6569, &t6570) { true } else { false } }) }).truthy() {
            let t6571 = (Py::tuple(vec![py::getitem(&v_A.clone(), &v_p.clone())?, py::getitem(&v_A.clone(), &v_c.clone())?])).items()?;
            if t6571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            py::setitem(&v_A.clone(), &v_c.clone(), t6571[0].clone())?;
            py::setitem(&v_A.clone(), &v_p.clone(), t6571[1].clone())?;
            v_det = py::neg(&v_det.clone())?;
        }
        v_det = py::mul(&v_det.clone(), &py::getitem(&py::getitem(&v_A.clone(), &v_c.clone())?, &v_c.clone())?)?;
        for t6572 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t6572;
            if ({ let t6573 = py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t6574 = Py::Int(0i64); if py::ne(&t6573, &t6574) { true } else { false } }) }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_c.clone())?, &v_c.clone())?)?;
                py::setitem(&v_A.clone(), &v_r.clone(), { let mut out = vec![]; for t6575 in py::iter(&(py::zip(&[py::getitem(&v_A.clone(), &v_r.clone())?, py::getitem(&v_A.clone(), &v_c.clone())?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t6576 = (t6575).items()?;
if t6576.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t6576[0].clone();
v_y = t6576[1].clone();
out.push(py::sub(&v_x.clone(), &py::mul(&v_f.clone(), &v_y.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    return Ok(v_det.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__diff(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    if ({ let t10497 = Py::Bool(matches!(v_a.clone(), Py::Str(_))); if t10497.truthy() { t10497 } else { { let t10496 = Py::Bool(matches!(v_b.clone(), Py::Str(_))); if t10496.truthy() { t10496 } else { { let t10495 = Py::Bool(matches!(v_a.clone(), Py::Bool(_))); if t10495.truthy() { t10495 } else { Py::Bool(matches!(v_b.clone(), Py::Bool(_))) } } } } } }).truthy() {
        return Ok((if ({ let t10498 = v_a.clone(); Py::Bool({ let t10499 = v_b.clone(); if py::eq(&t10498, &t10499) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    if ({ let t10500 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t10500.truthy() { t10500 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t10504 = py::not(&{ let t10501 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t10501.truthy() { t10501 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t10504.truthy() { t10504 } else { { let t10502 = py::len(&v_a.clone())?; Py::Bool({ let t10503 = py::len(&v_b.clone())?; if py::ne(&t10502, &t10503) { true } else { false } }) } } }).truthy() {
            return Ok(c_INF()?);
        }
        v_w = Py::Float(0.0);
        for t10505 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? {
            let t10506 = (t10505).items()?;
            if t10506.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_x = t10506[0].clone();
            v_y = t10506[1].clone();
            v_w = py::minmax(&[v_w.clone(), f__diff(v_x.clone(), v_y.clone())?], true)?;
        }
        return Ok(v_w.clone());
    }
    if ({ let t10511 = { let t10507 = v_a.clone(); Py::Bool({ let t10508 = Py::None; if py::is(&t10507, &t10508) { true } else { false } }) }; if t10511.truthy() { t10511 } else { { let t10509 = v_b.clone(); Py::Bool({ let t10510 = Py::None; if py::is(&t10509, &t10510) { true } else { false } }) } } }).truthy() {
        return Ok((if ({ let t10512 = v_a.clone(); Py::Bool({ let t10513 = v_b.clone(); if py::is(&t10512, &t10513) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { c_INF()? }));
    }
    let t10515: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t10514 = (Py::tuple(vec![py::complex(&[v_a.clone()])?, py::complex(&[v_b.clone()])?])).items()?;
            if t10514.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t10514[0].clone();
            v_b = t10514[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10515 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    if ({ let t10516 = v_a.clone(); Py::Bool({ let t10517 = v_b.clone(); if py::eq(&t10516, &t10517) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t10518 = v_d.clone(); Py::Bool({ let t10519 = v_d.clone(); if py::ne(&t10518, &t10519) { true } else { false } }) }).truthy() {
        return Ok(c_INF()?);
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__disc_out(mut v_numz: Py, mut v_denz: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_big_n: Py = Py::None;
    let mut v_lead: Py = Py::None;
    v_denz = f__ptrim(v_denz.clone())?;
    let _ = f__need({ let t10047 = py::getitem(&v_denz.clone(), &Py::Int(0i64))?; Py::Bool({ let t10048 = Py::Int(0i64); if py::ne(&t10047, &t10048) { true } else { false } }) }, Py::str("the discrete denominator vanishes"))?;
    v_lead = py::getitem(&v_denz.clone(), &Py::Int(0i64))?;
    v_numz = f__pad(f__ptrim(v_numz.clone())?, py::len(&v_denz.clone())?)?;
    v_big_n = { let t10050 = py::minmax(&[{ let mut out = vec![]; for t10049 in py::iter(&(v_numz.clone()))? { let mut v_x: Py; 
v_x = t10049;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?; if t10050.truthy() { t10050 } else { Py::Float(1.0) } };
    v_numz = { let mut out = vec![]; for t10051 in py::iter(&(v_numz.clone()))? { let mut v_x: Py; 
v_x = t10051;
out.push(py::truediv(&v_x.clone(), &v_lead.clone())?); } Py::list(out) };
    v_denz = { let mut out = vec![]; for t10052 in py::iter(&(v_denz.clone()))? { let mut v_x: Py; 
v_x = t10052;
out.push(py::truediv(&v_x.clone(), &v_lead.clone())?); } Py::list(out) };
    v_big = py::minmax(&[py::minmax(&[{ let mut out = vec![]; for t10053 in py::iter(&(v_numz.clone()))? { let mut v_x: Py; 
v_x = t10053;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?, Py::Float(1e-300)], true)?;
    v_numz = { let mut out = vec![]; for t10054 in py::iter(&(v_numz.clone()))? { let mut v_x: Py; 
v_x = t10054;
out.push((if ({ let t10055 = py::abs(&v_x.clone())?; Py::Bool({ let t10056 = py::mul(&Py::Float(1e-13), &v_big.clone())?; if py::le(&t10055, &t10056)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() })); } Py::list(out) };
    return Ok(f__rnd(Py::dict(vec![(Py::str("numz"), v_numz.clone()), (Py::str("denz"), v_denz.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__disc_tf_in(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let t10041 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t10041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t10041[0].clone();
    v_den = t10041[1].clone();
    return Ok(Py::tuple(vec![v_num.clone(), v_den.clone(), f__pos(v_T.clone(), Py::str("T"))?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__dlyap(mut v_Acl: Py, mut v_Q: Py, mut v_exact: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_zero: Py = Py::None;
    v_n = py::len(&v_Acl.clone())?;
    v_zero = (if (v_exact.clone()).truthy() { py::fraction(&[Py::Int(0i64)])? } else { Py::Float(0.0) });
    let t3236 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t3236.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_M = t3236[0].clone();
    v_rhs = t3236[1].clone();
    for t3237 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t3237;
        for t3238 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t3238;
            v_row = py::mul(&Py::list(vec![v_zero.clone()]), &py::mul(&v_n.clone(), &v_n.clone())?)?;
            py::setitem(&v_row.clone(), &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_j.clone())?, py::add(&py::getitem(&v_row.clone(), &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_j.clone())?)?, &Py::Int(1i64))?)?;
            for t3239 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_k = t3239;
                for t3240 in py::range_iter(&[(v_n.clone()).i()?]) {
                    v_l = t3240;
                    py::setitem(&v_row.clone(), &py::add(&py::mul(&v_k.clone(), &v_n.clone())?, &v_l.clone())?, py::sub(&py::getitem(&v_row.clone(), &py::add(&py::mul(&v_k.clone(), &v_n.clone())?, &v_l.clone())?)?, &py::mul(&py::getitem(&py::getitem(&v_Acl.clone(), &v_k.clone())?, &v_i.clone())?, &py::getitem(&py::getitem(&v_Acl.clone(), &v_l.clone())?, &v_j.clone())?)?)?)?;
                }
            }
            let _ = py::method(&v_M.clone(), "append", &[v_row.clone()])?;
            let _ = py::method(&v_rhs.clone(), "append", &[py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?])?;
        }
    }
    v_x = f__solve(v_M.clone(), v_rhs.clone(), v_exact.clone())?;
    v_P = { let mut out = vec![]; for t3241 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3241;
out.push({ let mut out = vec![]; for t3242 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3242;
out.push(py::getitem(&v_x.clone(), &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    return Ok({ let mut out = vec![]; for t3243 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t3243;
out.push({ let mut out = vec![]; for t3244 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t3244;
out.push(py::truediv(&py::add(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_i.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__expm(mut v_M: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nrm: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_nrm = f__mnorm1(v_M.clone())?;
    v_s = Py::Int(0i64);
    while ({ let t9639 = v_nrm.clone(); Py::Bool({ let t9640 = Py::Float(0.5); if py::gt(&t9639, &t9640)? { true } else { false } }) }).truthy() {
        v_nrm = py::truediv(&v_nrm, &Py::Float(2.0))?;
        v_s = py::iadd(&v_s, &Py::Int(1i64))?;
    }
    v_X = f__mscale(v_M.clone(), py::truediv(&Py::Float(1.0), &py::pow(&Py::Float(2.0), &v_s.clone())?)?)?;
    v_E = f__eye(v_n.clone(), Py::Float(1.0))?;
    v_T = f__eye(v_n.clone(), Py::Float(1.0))?;
    for t9647 in py::range_iter(&[(Py::Int(1i64)).i()?, (Py::Int(30i64)).i()?]) {
        v_k = t9647;
        v_T = f__mscale(f__mmul(v_T.clone(), v_X.clone())?, py::truediv(&Py::Float(1.0), &v_k.clone())?)?;
        v_E = f__madd(v_E.clone(), v_T.clone())?;
        if ({ let t9656 = f__mnorm1(v_T.clone())?; Py::Bool({ let t9657 = py::mul(&Py::Float(1e-18), &f__mnorm1(v_E.clone())?)?; if py::le(&t9656, &t9657)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    for t9658 in py::range_iter(&[(v_s.clone()).i()?]) {
        v__ = t9658;
        v_E = f__mmul(v_E.clone(), v_E.clone())?;
    }
    return Ok(v_E.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__eye(mut v_n: Py, mut v_one: Py) -> R {
    return Ok({ let mut out = vec![]; for t9643 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t9643;
out.push({ let mut out = vec![]; for t9644 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t9644;
out.push((if ({ let t9645 = v_i.clone(); Py::Bool({ let t9646 = v_j.clone(); if py::eq(&t9645, &t9646) { true } else { false } }) }).truthy() { v_one.clone() } else { py::mul(&Py::Int(0i64), &v_one.clone())? })); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__faddeev(mut v_A: Py) -> R {
    let mut v_I: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_Ms: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_one: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_one = py::add(&Py::Int(1i64), &py::mul(&Py::Int(0i64), &py::getitem(&py::getitem(&v_A.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?)?;
    v_I = f__eye(v_n.clone(), v_one.clone())?;
    v_M = { let mut out = vec![]; for t9277 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v__: Py; 
v__ = t9277;
out.push(py::mul(&Py::list(vec![py::mul(&Py::Int(0i64), &v_one.clone())?]), &v_n.clone())?); } Py::list(out) };
    v_c = Py::list(vec![v_one.clone()]);
    v_Ms = Py::list(vec![]);
    for t9278 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t9278;
        v_M = f__madd(f__mmul(v_A.clone(), v_M.clone())?, f__mscale(v_I.clone(), py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?)?)?;
        let _ = py::method(&v_Ms.clone(), "append", &[v_M.clone()])?;
        let _ = py::method(&v_c.clone(), "append", &[py::truediv(&py::neg(&f__trace(f__mmul(v_A.clone(), v_M.clone())?)?)?, &v_k.clone())?])?;
    }
    return Ok(Py::tuple(vec![v_c.clone(), v_Ms.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fl(mut v_X: Py) -> R {
    if (Py::Bool(matches!(v_X.clone(), Py::List(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t6524 in py::iter(&(v_X.clone()))? { let mut v_x: Py; 
v_x = t6524;
out.push(f__fl(v_x.clone())?); } Py::list(out) });
    }
    return Ok(py::float(&v_X.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_X: Py) -> R {
    if (Py::Bool(matches!(py::getitem(&v_X.clone(), &Py::Int(0i64))?, Py::List(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t9267 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t9267;
out.push({ let mut out = vec![]; for t9268 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t9268;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) }); } Py::list(out) });
    }
    return Ok({ let mut out = vec![]; for t9269 in py::iter(&(v_X.clone()))? { let mut v_x: Py; 
v_x = t9269;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__gain_crossovers(mut v_num: Py, mut v_den: Py) -> R {
    return Ok(f__real_pos_roots(f__ptrim(f__psub(f__mag2(v_num.clone())?, f__mag2(v_den.clone())?)?)?, Py::Float(1e-07))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__is_hurwitz(mut v_A: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t6434 = (f__faddeev(f__frac(v_A.clone())?)?).items()?;
    if t6434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t6434[0].clone();
    v__ = t6434[1].clone();
    return Ok({ let t6435 = py::minmax(&[{ let mut out = vec![]; for t6525 in py::iter(&(f__roots(f__fl(v_c.clone())?)?))? { let mut v_r: Py; 
v_r = t6525;
out.push(py::real(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t6436 = Py::Int(0i64); if py::lt(&t6435, &t6436)? { true } else { false } }) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__jw(mut v_p: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_idx: Py = Py::None;
    let mut v_im: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_re_: Py = Py::None;
    v_n = py::sub(&py::len(&v_p.clone())?, &Py::Int(1i64))?;
    v_re_ = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    v_im = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    for t10192 in py::iter(&(py::enumerate(&v_p.clone(), 0)?))? {
        let t10207 = (t10192).items()?;
        if t10207.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10207[0].clone();
        v_a = t10207[1].clone();
        v_m = py::sub(&v_n.clone(), &v_k.clone())?;
        v_idx = py::sub(&v_n.clone(), &v_m.clone())?;
        v_q = py::modulo(&v_m.clone(), &Py::Int(4i64))?;
        if ({ let t10193 = v_q.clone(); Py::Bool({ let t10194 = Py::Int(0i64); if py::eq(&t10193, &t10194) { true } else { false } }) }).truthy() {
            let t10195 = v_re_.clone(); let t10196 = v_idx.clone();
            py::setitem(&t10195, &t10196, py::iadd(&py::getitem(&t10195, &t10196)?, &v_a.clone())?)?;
        } else {
            if ({ let t10197 = v_q.clone(); Py::Bool({ let t10198 = Py::Int(1i64); if py::eq(&t10197, &t10198) { true } else { false } }) }).truthy() {
                let t10199 = v_im.clone(); let t10200 = v_idx.clone();
                py::setitem(&t10199, &t10200, py::iadd(&py::getitem(&t10199, &t10200)?, &v_a.clone())?)?;
            } else {
                if ({ let t10201 = v_q.clone(); Py::Bool({ let t10202 = Py::Int(2i64); if py::eq(&t10201, &t10202) { true } else { false } }) }).truthy() {
                    let t10203 = v_re_.clone(); let t10204 = v_idx.clone();
                    py::setitem(&t10203, &t10204, py::sub(&py::getitem(&t10203, &t10204)?, &v_a.clone())?)?;
                } else {
                    let t10205 = v_im.clone(); let t10206 = v_idx.clone();
                    py::setitem(&t10205, &t10206, py::sub(&py::getitem(&t10205, &t10206)?, &v_a.clone())?)?;
                }
            }
        }
    }
    return Ok(Py::tuple(vec![f__ptrim(v_re_.clone())?, f__ptrim(v_im.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__lyap(mut v_Acl: Py, mut v_Q: Py, mut v_exact: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_rhs: Py = Py::None;
    let mut v_row: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_zero: Py = Py::None;
    v_n = py::len(&v_Acl.clone())?;
    v_zero = (if (v_exact.clone()).truthy() { py::fraction(&[Py::Int(0i64)])? } else { Py::Float(0.0) });
    let t6526 = (Py::tuple(vec![Py::list(vec![]), Py::list(vec![])])).items()?;
    if t6526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_M = t6526[0].clone();
    v_rhs = t6526[1].clone();
    for t6527 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t6527;
        for t6528 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t6528;
            v_row = py::mul(&Py::list(vec![v_zero.clone()]), &py::mul(&v_n.clone(), &v_n.clone())?)?;
            for t6529 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_k = t6529;
                py::setitem(&v_row.clone(), &py::add(&py::mul(&v_k.clone(), &v_n.clone())?, &v_j.clone())?, py::add(&py::getitem(&v_row.clone(), &py::add(&py::mul(&v_k.clone(), &v_n.clone())?, &v_j.clone())?)?, &py::getitem(&py::getitem(&v_Acl.clone(), &v_k.clone())?, &v_i.clone())?)?)?;
                py::setitem(&v_row.clone(), &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_k.clone())?, py::add(&py::getitem(&v_row.clone(), &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_k.clone())?)?, &py::getitem(&py::getitem(&v_Acl.clone(), &v_k.clone())?, &v_j.clone())?)?)?;
            }
            let _ = py::method(&v_M.clone(), "append", &[v_row.clone()])?;
            let _ = py::method(&v_rhs.clone(), "append", &[py::neg(&py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?)?])?;
        }
    }
    v_x = f__solve(v_M.clone(), v_rhs.clone(), v_exact.clone())?;
    v_P = { let mut out = vec![]; for t6556 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6556;
out.push({ let mut out = vec![]; for t6557 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6557;
out.push(py::getitem(&v_x.clone(), &py::add(&py::mul(&v_i.clone(), &v_n.clone())?, &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) };
    return Ok({ let mut out = vec![]; for t6558 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t6558;
out.push({ let mut out = vec![]; for t6559 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t6559;
out.push(py::truediv(&py::add(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_P.clone(), &v_j.clone())?, &v_i.clone())?)?, &Py::Int(2i64))?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mT(mut v_X: Py) -> R {
    return Ok({ let mut out = vec![]; for t6349 in py::range_iter(&[(py::len(&py::getitem(&v_X.clone(), &Py::Int(0i64))?)?).i()?]) { let mut v_j: Py; 
v_j = t6349;
out.push({ let mut out = vec![]; for t6350 in py::range_iter(&[(py::len(&v_X.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6350;
out.push(py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_j.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__madd(mut v_X: Py, mut v_Y: Py) -> R {
    return Ok({ let mut out = vec![]; for t9652 in py::iter(&(py::zip(&[v_X.clone(), v_Y.clone()])?))? { let mut v_r: Py; let mut v_s: Py; 
let t9653 = (t9652).items()?;
if t9653.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_r = t9653[0].clone();
v_s = t9653[1].clone();
out.push({ let mut out = vec![]; for t9654 in py::iter(&(py::zip(&[v_r.clone(), v_s.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t9655 = (t9654).items()?;
if t9655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t9655[0].clone();
v_b = t9655[1].clone();
out.push(py::add(&v_a.clone(), &v_b.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mag2(mut v_p: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t8943 = (f__jw(v_p.clone())?).items()?;
    if t8943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_r = t8943[0].clone();
    v_i = t8943[1].clone();
    return Ok(f__padd(f__pmul(v_r.clone(), v_r.clone())?, f__pmul(v_i.clone(), v_i.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mat(mut v_v: Py, mut v_name: Py, mut v_hi: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let _ = f__need({ let t9417 = Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_))); if !t9417.truthy() { t9417 } else { { let t9415 = py::len(&v_v.clone())?; Py::Bool({ let t9416 = Py::Int(0i64); if py::gt(&t9415, &t9416)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix".to_string()].concat()))?;
    v_rows = Py::list(vec![]);
    for t9418 in py::iter(&(v_v.clone()))? {
        v_r = t9418;
        let _ = f__need(Py::Bool(matches!(v_r.clone(), Py::List(_) | Py::Tuple(_))), Py::str(&[py::format(&v_name.clone(), "")?, " must be a matrix (a list of rows)".to_string()].concat()))?;
        let _ = py::method(&v_rows.clone(), "append", &[{ let mut out = vec![]; for t9419 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t9419;
out.push(f__num(v_x.clone(), v_name.clone())?); } Py::list(out) }])?;
    }
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t9422 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t9422;
out.push({ let t9423 = py::len(&v_r.clone())?; Py::Bool({ let t9424 = py::len(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?)?; if py::eq(&t9423, &t9424) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be rectangular".to_string()].concat()))?;
    let _ = f__need({ let t9429 = { let t9425 = py::len(&v_rows.clone())?; Py::Bool({ let t9426 = v_hi.clone(); if py::le(&t9425, &t9426)? { true } else { false } }) }; if !t9429.truthy() { t9429 } else { { let t9427 = py::len(&py::getitem(&v_rows.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t9428 = v_hi.clone(); if py::le(&t9427, &t9428)? { true } else { false } }) } } }, Py::str(&[py::format(&v_name.clone(), "")?, " is limited to ".to_string(), py::format(&v_hi.clone(), "")?, " x ".to_string(), py::format(&v_hi.clone(), "")?].concat()))?;
    return Ok(v_rows.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mexp_out(mut v_E: Py) -> R {
    return Ok(f__rnd(f__snapm(v_E.clone(), Py::Float(1e-11))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mmul(mut v_X: Py, mut v_Y: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_row: Py = Py::None;
    let t9648 = (Py::tuple(vec![py::len(&v_X.clone())?, py::len(&v_Y.clone())?, py::len(&py::getitem(&v_Y.clone(), &Py::Int(0i64))?)?])).items()?;
    if t9648.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t9648[0].clone();
    v_m = t9648[1].clone();
    v_p = t9648[2].clone();
    v_out = Py::list(vec![]);
    for t9649 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t9649;
        v_row = Py::list(vec![]);
        for t9650 in py::range_iter(&[(v_p.clone()).i()?]) {
            v_j = t9650;
            v_acc = py::mul(&Py::Int(0i64), &py::getitem(&py::getitem(&v_X.clone(), &Py::Int(0i64))?, &Py::Int(0i64))?)?;
            for t9651 in py::range_iter(&[(v_m.clone()).i()?]) {
                v_k = t9651;
                v_acc = py::add(&v_acc.clone(), &py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_Y.clone(), &v_k.clone())?, &v_j.clone())?)?)?;
            }
            let _ = py::method(&v_row.clone(), "append", &[v_acc.clone()])?;
        }
        let _ = py::method(&v_out.clone(), "append", &[v_row.clone()])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mnorm1(mut v_X: Py) -> R {
    return Ok(py::minmax(&[{ let mut out = vec![]; for t9637 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t9637;
out.push(py::sum(&{ let mut out = vec![]; for t9638 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t9638;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }, None)?); } Py::list(out) }], true)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mscale(mut v_X: Py, mut v_k: Py) -> R {
    return Ok({ let mut out = vec![]; for t9641 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t9641;
out.push({ let mut out = vec![]; for t9642 in py::iter(&(v_r.clone()))? { let mut v_a: Py; 
v_a = t9642;
out.push(py::mul(&v_k.clone(), &v_a.clone())?); } Py::list(out) }); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__multiples(mut v_a: Py, mut v_roots: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_dd: Py = Py::None;
    let mut v_der: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_group: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_ok: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_used: Py = Py::None;
    v_n = py::len(&v_roots.clone())?;
    v_used = py::mul(&Py::list(vec![Py::Bool(false)]), &v_n.clone())?;
    v_out = Py::list(vec![]);
    v_scale = py::minmax(&[{ let mut out = vec![]; for t10269 in py::iter(&(v_a.clone()))? { let mut v_x: Py; 
v_x = t10269;
out.push(py::abs(&v_x.clone())?); } Py::list(out) }], true)?;
    for t10270 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t10270;
        if (py::getitem(&v_used.clone(), &v_i.clone())?).truthy() {
            continue;
        }
        v_group = Py::list(vec![v_i.clone()]);
        for t10271 in py::range_iter(&[(py::add(&v_i.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_j = t10271;
            if ({ let t10274 = py::not(&py::getitem(&v_used.clone(), &v_j.clone())?); if !t10274.truthy() { t10274 } else { { let t10272 = py::abs(&py::sub(&py::getitem(&v_roots.clone(), &v_j.clone())?, &py::getitem(&v_roots.clone(), &v_i.clone())?)?)?; Py::Bool({ let t10273 = py::mul(&Py::Float(0.0001), &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_roots.clone(), &v_i.clone())?)?], true)?)?; if py::le(&t10272, &t10273)? { true } else { false } }) } } }).truthy() {
                let _ = py::method(&v_group.clone(), "append", &[v_j.clone()])?;
            }
        }
        for t10275 in py::iter(&(v_group.clone()))? {
            v_j = t10275;
            py::setitem(&v_used.clone(), &v_j.clone(), Py::Bool(true))?;
        }
        v_m = py::len(&v_group.clone())?;
        if ({ let t10276 = v_m.clone(); Py::Bool({ let t10277 = Py::Int(1i64); if py::eq(&t10276, &t10277) { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[py::getitem(&v_roots.clone(), &v_i.clone())?])?;
            continue;
        }
        v_c = py::truediv(&py::sum(&{ let mut out = vec![]; for t10278 in py::iter(&(v_group.clone()))? { let mut v_j: Py; 
v_j = t10278;
out.push(py::getitem(&v_roots.clone(), &v_j.clone())?); } Py::list(out) }, None)?, &v_m.clone())?;
        v_der = v_a.clone();
        for t10279 in py::range_iter(&[(py::sub(&v_m.clone(), &Py::Int(1i64))?).i()?]) {
            v__ = t10279;
            v_der = f__pder(v_der.clone())?;
        }
        v_d1 = f__pder(v_der.clone())?;
        for t10280 in py::range_iter(&[(Py::Int(60i64)).i()?]) {
            v__ = t10280;
            v_dv = f__pval(v_d1.clone(), v_c.clone())?;
            if ({ let t10281 = v_dv.clone(); Py::Bool({ let t10282 = Py::Int(0i64); if py::eq(&t10281, &t10282) { true } else { false } }) }).truthy() {
                break;
            }
            v_step = py::truediv(&f__pval(v_der.clone(), v_c.clone())?, &v_dv.clone())?;
            v_c = py::sub(&v_c.clone(), &v_step.clone())?;
            if ({ let t10283 = py::abs(&v_step.clone())?; Py::Bool({ let t10284 = py::mul(&Py::Float(1e-16), &py::minmax(&[Py::Float(1.0), py::abs(&v_c.clone())?], true)?)?; if py::le(&t10283, &t10284)? { true } else { false } }) }).truthy() {
                break;
            }
        }
        v_ok = Py::Bool(true);
        v_dd = v_a.clone();
        for t10285 in py::range_iter(&[(py::sub(&v_m.clone(), &Py::Int(1i64))?).i()?]) {
            v_k = t10285;
            v_mag = py::sum(&{ let mut out = vec![]; for t10286 in py::iter(&(py::enumerate(&v_dd.clone(), 0)?))? { let mut v_t: Py; let mut v_x: Py; 
let t10287 = (t10286).items()?;
if t10287.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t10287[0].clone();
v_x = t10287[1].clone();
out.push(py::mul(&py::abs(&v_x.clone())?, &py::pow(&py::minmax(&[Py::Float(1.0), py::abs(&v_c.clone())?], true)?, &py::sub(&py::sub(&py::len(&v_dd.clone())?, &Py::Int(1i64))?, &v_t.clone())?)?)?); } Py::list(out) }, None)?;
            if ({ let t10288 = py::abs(&f__pval(v_dd.clone(), v_c.clone())?)?; Py::Bool({ let t10289 = py::mul(&py::mul(&Py::Float(1e-09), &py::minmax(&[v_mag.clone(), Py::Float(1e-300)], true)?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?; if py::gt(&t10288, &t10289)? { true } else { false } }) }).truthy() {
                v_ok = Py::Bool(false);
            }
            v_dd = f__pder(v_dd.clone())?;
        }
        if (v_ok.clone()).truthy() {
            let _ = py::method(&v_out.clone(), "extend", &[py::mul(&Py::list(vec![v_c.clone()]), &v_m.clone())?])?;
        } else {
            let _ = py::method(&v_out.clone(), "extend", &[{ let mut out = vec![]; for t10290 in py::iter(&(v_group.clone()))? { let mut v_j: Py; 
v_j = t10290;
out.push(py::getitem(&v_roots.clone(), &v_j.clone())?); } Py::list(out) }])?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__mv(mut v_X: Py, mut v_v: Py) -> R {
    return Ok({ let mut out = vec![]; for t8773 in py::iter(&(v_X.clone()))? { let mut v_r: Py; 
v_r = t8773;
out.push(py::sum(&{ let mut out = vec![]; for t8774 in py::iter(&(py::zip(&[v_r.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t8775 = (t8774).items()?;
if t8775.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t8775[0].clone();
v_b = t8775[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t10114 = v_v.clone(); Py::Bool({ let t10115 = Py::Int(0i64); if py::ge(&t10114, &t10115)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__num(mut v_v: Py, mut v_name: Py) -> R {
    if ({ let t10476 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t10476.truthy() { t10476 } else { { let t10475 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_)))); if t10475.truthy() { t10475 } else { py::not(&py::math1("isfinite", &v_v.clone())?) } } } }).truthy() {
        return Err(py::value_error(py::to_str(&Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))));
    }
    return Ok(py::float(&v_v.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__one_pair(mut v_roots: Py, mut v_what: Py) -> R {
    let mut v_cpx: Py = Py::None;
    v_cpx = { let mut out = vec![]; for t2284 in py::iter(&(v_roots.clone()))? { let mut v_r: Py; 
v_r = t2284;
if ({ let t2287 = Py::Bool(matches!(v_r.clone(), Py::Complex(..))); if !t2287.truthy() { t2287 } else { { let t2285 = py::imag(&v_r.clone())?; Py::Bool({ let t2286 = Py::Int(0i64); if py::gt(&t2285, &t2286)? { true } else { false } }) } } }).truthy() { out.push(v_r.clone()); }} Py::list(out) };
    let _ = f__need({ let t2288 = py::len(&v_cpx.clone())?; Py::Bool({ let t2289 = Py::Int(1i64); if py::eq(&t2288, &t2289) { true } else { false } }) }, Py::str(&["this capability needs exactly one complex-conjugate pair of ".to_string(), py::format(&v_what.clone(), "")?].concat()))?;
    return Ok(py::getitem(&v_cpx.clone(), &Py::Int(0i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pad(mut v_a: Py, mut v_n: Py) -> R {
    return Ok(py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_a.clone())?)?)?, &Py::list((v_a.clone()).items()?))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__padd(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::minmax(&[py::len(&v_a.clone())?, py::len(&v_b.clone())?], true)?;
    v_a = py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_a.clone())?)?)?, &Py::list((v_a.clone()).items()?))?;
    v_b = py::add(&py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&v_n.clone(), &py::len(&v_b.clone())?)?)?, &Py::list((v_b.clone()).items()?))?;
    return Ok({ let mut out = vec![]; for t10210 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t10211 = (t10210).items()?;
if t10211.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t10211[0].clone();
v_y = t10211[1].clone();
out.push(py::add(&v_x.clone(), &v_y.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pder(mut v_c: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    if ({ let t10227 = v_n.clone(); Py::Bool({ let t10228 = Py::Int(0i64); if py::le(&t10227, &t10228)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![Py::Float(0.0)]));
    }
    return Ok({ let mut out = vec![]; for t10229 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t10229;
out.push(py::mul(&py::getitem(&v_c.clone(), &v_i.clone())?, &py::sub(&v_n.clone(), &v_i.clone())?)?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__phase_crossovers(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_Di: Py = Py::None;
    let mut v_Dr: Py = Py::None;
    let mut v_Ni: Py = Py::None;
    let mut v_Nr: Py = Py::None;
    let mut v_imp: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t10208 = (f__jw(v_num.clone())?).items()?;
    if t10208.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Nr = t10208[0].clone();
    v_Ni = t10208[1].clone();
    let t10209 = (f__jw(v_den.clone())?).items()?;
    if t10209.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Dr = t10209[0].clone();
    v_Di = t10209[1].clone();
    v_imp = f__ptrim(f__psub(f__pmul(v_Ni.clone(), v_Dr.clone())?, f__pmul(v_Nr.clone(), v_Di.clone())?)?)?;
    v_out = Py::list(vec![]);
    if ({ let t10224 = { let t10219 = py::len(&v_imp.clone())?; Py::Bool({ let t10220 = Py::Int(2i64); if py::lt(&t10219, &t10220)? { true } else { false } }) }; if t10224.truthy() { t10224 } else { Py::Bool(({ let mut out = vec![]; for t10221 in py::iter(&(v_imp.clone()))? { let mut v_x: Py; 
v_x = t10221;
out.push({ let t10222 = v_x.clone(); Py::Bool({ let t10223 = Py::Int(0i64); if py::eq(&t10222, &t10223) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }).truthy() {
        return Ok(v_out.clone());
    }
    for t10308 in py::iter(&(f__real_pos_roots(v_imp.clone(), Py::Float(1e-07))?))? {
        v_w = t10308;
        if ({ let t10309 = py::real(&f__G(v_num.clone(), v_den.clone(), py::complex(&[Py::Float(0.0), v_w.clone()])?)?)?; Py::Bool({ let t10310 = Py::Int(0i64); if py::lt(&t10309, &t10310)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[v_w.clone()])?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pid_out(mut v_Kp: Py, mut v_Ti: Py, mut v_Td: Py) -> R {
    let mut v_Ki: Py = Py::None;
    v_Ki = (if ({ let t10430 = v_Ti.clone(); Py::Bool({ let t10431 = Py::Int(0i64); if py::gt(&t10430, &t10431)? { true } else { false } }) }).truthy() { py::truediv(&v_Kp.clone(), &v_Ti.clone())? } else { Py::Float(0.0) });
    return Ok(f__rnd(Py::dict(vec![(Py::str("Kp"), v_Kp.clone()), (Py::str("Ti"), v_Ti.clone()), (Py::str("Td"), v_Td.clone()), (Py::str("Ki"), v_Ki.clone()), (Py::str("Kd"), py::mul(&v_Kp.clone(), &v_Td.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pmul(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_y: Py = Py::None;
    v_out = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::sub(&py::add(&py::len(&v_a.clone())?, &py::len(&v_b.clone())?)?, &Py::Int(1i64))?)?;
    for t10213 in py::iter(&(py::enumerate(&v_a.clone(), 0)?))? {
        let t10218 = (t10213).items()?;
        if t10218.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t10218[0].clone();
        v_x = t10218[1].clone();
        for t10214 in py::iter(&(py::enumerate(&v_b.clone(), 0)?))? {
            let t10217 = (t10214).items()?;
            if t10217.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_j = t10217[0].clone();
            v_y = t10217[1].clone();
            let t10215 = v_out.clone(); let t10216 = py::add(&v_i.clone(), &v_j.clone())?;
            py::setitem(&t10215, &t10216, py::iadd(&py::getitem(&t10215, &t10216)?, &py::mul(&v_x.clone(), &v_y.clone())?)?)?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_v: Py, mut v_name: Py) -> R {
    v_v = f__num(v_v.clone(), v_name.clone())?;
    let _ = f__need({ let t10477 = v_v.clone(); Py::Bool({ let t10478 = Py::Int(0i64); if py::gt(&t10477, &t10478)? { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be positive".to_string()].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__posdef(mut v_Q: Py) -> R {
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_Q.clone())?;
    return Ok(Py::Bool(({ let mut out = vec![]; for t5869 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t5869;
out.push({ let t5870 = f__det({ let mut out = vec![]; for t5887 in py::iter(&(py::slice(&v_Q.clone(), None, Some((v_k.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t5887;
out.push(py::slice(&v_r.clone(), None, Some((v_k.clone()).i()?), None)?); } Py::list(out) })?; Py::Bool({ let t5871 = Py::Int(0i64); if py::gt(&t5870, &t5871)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ppow(mut v_a: Py, mut v_k: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_out = Py::list(vec![Py::Float(1.0)]);
    for t10076 in py::range_iter(&[(v_k.clone()).i()?]) {
        v__ = t10076;
        v_out = f__pmul(v_out.clone(), v_a.clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pscale(mut v_a: Py, mut v_k: Py) -> R {
    return Ok({ let mut out = vec![]; for t10069 in py::iter(&(v_a.clone()))? { let mut v_x: Py; 
v_x = t10069;
out.push(py::mul(&v_k.clone(), &v_x.clone())?); } Py::list(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__psub(mut v_a: Py, mut v_b: Py) -> R {
    return Ok(f__padd(v_a.clone(), { let mut out = vec![]; for t10212 in py::iter(&(v_b.clone()))? { let mut v_x: Py; 
v_x = t10212;
out.push(py::neg(&v_x.clone())?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ptrim(mut v_c: Py) -> R {
    v_c = { let mut out = vec![]; for t10165 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t10165;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    while ({ let t10170 = { let t10166 = py::len(&v_c.clone())?; Py::Bool({ let t10167 = Py::Int(1i64); if py::gt(&t10166, &t10167)? { true } else { false } }) }; if !t10170.truthy() { t10170 } else { { let t10168 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t10169 = Py::Int(0i64); if py::eq(&t10168, &t10169) { true } else { false } }) } } }).truthy() {
        v_c = py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?;
    }
    return Ok(v_c.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pval(mut v_c: Py, mut v_x: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_acc: Py = Py::None;
    v_acc = py::mul(&Py::Int(0i64), &v_x.clone())?;
    for t10248 in py::iter(&(v_c.clone()))? {
        v_a = t10248;
        v_acc = py::add(&py::mul(&v_acc.clone(), &v_x.clone())?, &v_a.clone())?;
    }
    return Ok(v_acc.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r12(mut v_x: Py) -> R {
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rank(mut v_M: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cols: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rk: Py = Py::None;
    let mut v_rows: Py = Py::None;
    v_A = { let mut out = vec![]; for t6289 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t6289;
out.push(Py::list((v_r.clone()).items()?)); } Py::list(out) };
    let t6290 = (Py::tuple(vec![py::len(&v_A.clone())?, py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?])).items()?;
    if t6290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_rows = t6290[0].clone();
    v_cols = t6290[1].clone();
    v_rk = Py::Int(0i64);
    for t6291 in py::range_iter(&[(v_cols.clone()).i()?]) {
        v_c = t6291;
        v_p = Py::None;
        for t6292 in py::range_iter(&[(v_rk.clone()).i()?, (v_rows.clone()).i()?]) {
            v_r = t6292;
            if ({ let t6293 = py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t6294 = Py::Int(0i64); if py::ne(&t6293, &t6294) { true } else { false } }) }).truthy() {
                v_p = v_r.clone();
                break;
            }
        }
        if ({ let t6295 = v_p.clone(); Py::Bool({ let t6296 = Py::None; if py::is(&t6295, &t6296) { true } else { false } }) }).truthy() {
            continue;
        }
        let t6297 = (Py::tuple(vec![py::getitem(&v_A.clone(), &v_p.clone())?, py::getitem(&v_A.clone(), &v_rk.clone())?])).items()?;
        if t6297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_A.clone(), &v_rk.clone(), t6297[0].clone())?;
        py::setitem(&v_A.clone(), &v_p.clone(), t6297[1].clone())?;
        for t6298 in py::range_iter(&[(v_rows.clone()).i()?]) {
            v_r = t6298;
            if ({ let t6303 = { let t6299 = v_r.clone(); Py::Bool({ let t6300 = v_rk.clone(); if py::ne(&t6299, &t6300) { true } else { false } }) }; if !t6303.truthy() { t6303 } else { { let t6301 = py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t6302 = Py::Int(0i64); if py::ne(&t6301, &t6302) { true } else { false } }) } } }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_rk.clone())?, &v_c.clone())?)?;
                py::setitem(&v_A.clone(), &v_r.clone(), { let mut out = vec![]; for t6304 in py::iter(&(py::zip(&[py::getitem(&v_A.clone(), &v_r.clone())?, py::getitem(&v_A.clone(), &v_rk.clone())?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t6305 = (t6304).items()?;
if t6305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t6305[0].clone();
v_y = t6305[1].clone();
out.push(py::sub(&v_x.clone(), &py::mul(&v_f.clone(), &v_y.clone())?)?); } Py::list(out) })?;
            }
        }
        v_rk = py::iadd(&v_rk, &Py::Int(1i64))?;
        if ({ let t6306 = v_rk.clone(); Py::Bool({ let t6307 = v_rows.clone(); if py::eq(&t6306, &t6307) { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Ok(v_rk.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__real_pos_roots(mut v_c: Py, mut v_tol: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_found: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_c = f__ptrim(v_c.clone())?;
    if ({ let t10225 = py::len(&v_c.clone())?; Py::Bool({ let t10226 = Py::Int(2i64); if py::lt(&t10225, &t10226)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_d = f__pder(v_c.clone())?;
    v_found = Py::list(vec![]);
    for t10291 in py::iter(&(f__roots_raw(v_c.clone())?))? {
        v_r = t10291;
        v_r = py::complex(&[v_r.clone()])?;
        if ({ let t10296 = { let t10292 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t10293 = py::mul(&v_tol.clone(), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t10292, &t10293)? { true } else { false } }) }; if t10296.truthy() { t10296 } else { { let t10294 = py::real(&v_r.clone())?; Py::Bool({ let t10295 = Py::Int(0i64); if py::le(&t10294, &t10295)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        v_x = py::real(&v_r.clone())?;
        for t10297 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
            v__ = t10297;
            v_dv = f__pval(v_d.clone(), v_x.clone())?;
            if ({ let t10298 = v_dv.clone(); Py::Bool({ let t10299 = Py::Int(0i64); if py::eq(&t10298, &t10299) { true } else { false } }) }).truthy() {
                break;
            }
            v_step = py::truediv(&f__pval(v_c.clone(), v_x.clone())?, &v_dv.clone())?;
            if ({ let t10300 = py::abs(&v_step.clone())?; Py::Bool({ let t10301 = py::mul(&Py::Float(1e-06), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?; if py::gt(&t10300, &t10301)? { true } else { false } }) }).truthy() {
                break;
            }
            v_x = py::sub(&v_x, &v_step.clone())?;
        }
        if ({ let t10302 = v_x.clone(); Py::Bool({ let t10303 = Py::Float(1e-12); if py::gt(&t10302, &t10303)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_found.clone(), "append", &[v_x.clone()])?;
        }
    }
    let _ = py::method_sort(&v_found.clone(), None, false)?;
    v_out = Py::list(vec![]);
    for t10304 in py::iter(&(v_found.clone()))? {
        v_x = t10304;
        if ({ let t10307 = v_out.clone(); if !t10307.truthy() { t10307 } else { { let t10305 = py::abs(&py::sub(&v_x.clone(), &py::getitem(&v_out.clone(), &py::neg(&Py::Int(1i64))?)?)?)?; Py::Bool({ let t10306 = py::mul(&Py::Float(1e-07), &py::minmax(&[Py::Float(1.0), v_x.clone()], true)?)?; if py::le(&t10305, &t10306)? { true } else { false } }) } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("a repeated (tangential) crossing: the result is not well defined"))));
        }
        let _ = py::method(&v_out.clone(), "append", &[v_x.clone()])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__riccati_in(mut v_A: Py, mut v_B: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let t5402 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t5402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5402[0].clone();
    v_B = t5402[1].clone();
    v_Q = f__square(v_Q.clone(), Py::str("Q"), Py::Int(6i64))?;
    let _ = f__need({ let t5403 = py::len(&v_Q.clone())?; Py::Bool({ let t5404 = py::len(&v_A.clone())?; if py::eq(&t5403, &t5404) { true } else { false } }) }, Py::str("Q must be n x n"))?;
    let _ = f__sym(v_Q.clone(), Py::str("Q"))?;
    let _ = f__need(f__posdef(f__frac(v_Q.clone())?)?, Py::str("Q must be positive definite"))?;
    v_R = f__pos(v_R.clone(), Py::str("R"))?;
    return Ok(Py::tuple(vec![v_A.clone(), v_B.clone(), v_Q.clone(), v_R.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rnd(mut v_v: Py) -> R {
    let mut v_im: Py = Py::None;
    if ({ let t10483 = Py::Bool(matches!(v_v.clone(), Py::Bool(_))); if t10483.truthy() { t10483 } else { { let t10482 = Py::Bool(matches!(v_v.clone(), Py::Str(_))); if t10482.truthy() { t10482 } else { { let t10480 = v_v.clone(); Py::Bool({ let t10481 = Py::None; if py::is(&t10480, &t10481) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_v.clone());
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Float(_)))).truthy() {
        return Ok(f__r12(v_v.clone())?);
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Complex(..)))).truthy() {
        v_im = f__r12(py::imag(&v_v.clone())?)?;
        if ({ let t10484 = v_im.clone(); Py::Bool({ let t10485 = Py::Int(0i64); if py::eq(&t10484, &t10485) { true } else { false } }) }).truthy() {
            return Ok(f__r12(py::real(&v_v.clone())?)?);
        }
        return Ok(py::complex(&[f__r12(py::real(&v_v.clone())?)?, v_im.clone()])?);
    }
    if (Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t10486 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t10486;
out.push(f__rnd(v_x.clone())?); } Py::list(out) });
    }
    if (Py::Bool(matches!(v_v.clone(), Py::Dict(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t10487 in py::iter(&(py::method(&v_v.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_x: Py; 
let t10488 = (t10487).items()?;
if t10488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t10488[0].clone();
v_x = t10488[1].clone();
out.push((v_k.clone(), f__rnd(v_x.clone())?)); } Py::dict(out) });
    }
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots(mut v_c: Py) -> R {
    return Ok(f__canon(f__roots_raw(v_c.clone())?, Py::Float(1e-09))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__roots_raw(mut v_c: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_biggest: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_dp: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_ratio: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_x: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zeros: Py = Py::None;
    v_c = f__ptrim(v_c.clone())?;
    if ({ let t10230 = py::len(&v_c.clone())?; Py::Bool({ let t10231 = Py::Int(2i64); if py::lt(&t10230, &t10231)? { true } else { false } }) }).truthy() {
        return Ok(Py::list(vec![]));
    }
    v_zeros = Py::Int(0i64);
    while ({ let t10236 = { let t10232 = py::len(&v_c.clone())?; Py::Bool({ let t10233 = Py::Int(1i64); if py::gt(&t10232, &t10233)? { true } else { false } }) }; if !t10236.truthy() { t10236 } else { { let t10234 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t10235 = Py::Int(0i64); if py::eq(&t10234, &t10235) { true } else { false } }) } } }).truthy() {
        v_c = py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
        v_zeros = py::iadd(&v_zeros, &Py::Int(1i64))?;
    }
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    if ({ let t10237 = v_n.clone(); Py::Bool({ let t10238 = Py::Int(0i64); if py::eq(&t10237, &t10238) { true } else { false } }) }).truthy() {
        return Ok(py::mul(&Py::list(vec![Py::Float(0.0)]), &v_zeros.clone())?);
    }
    v_a = { let mut out = vec![]; for t10239 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t10239;
out.push(py::truediv(&v_x.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?); } Py::list(out) };
    if ({ let t10240 = v_n.clone(); Py::Bool({ let t10241 = Py::Int(1i64); if py::eq(&t10240, &t10241) { true } else { false } }) }).truthy() {
        return Ok(py::add(&Py::list(vec![py::complex(&[py::neg(&py::getitem(&v_a.clone(), &Py::Int(1i64))?)?, Py::Float(0.0)])?]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &v_zeros.clone())?)?);
    }
    v_d = f__pder(v_a.clone())?;
    v_r = py::pow(&py::abs(&py::getitem(&v_a.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::truediv(&Py::Float(1.0), &v_n.clone())?)?;
    for t10242 in py::iter(&(py::slice(&v_a.clone(), Some((Py::Int(1i64)).i()?), None, None)?))? {
        v_x = t10242;
        v_r = py::minmax(&[v_r.clone(), py::mul(&py::pow(&py::abs(&v_x.clone())?, &Py::Float(0.5))?, &Py::Float(0.5))?], true)?;
    }
    if ({ let t10243 = v_r.clone(); Py::Bool({ let t10244 = Py::Int(0i64); if py::eq(&t10243, &t10244) { true } else { false } }) }).truthy() {
        v_r = Py::Float(1.0);
    }
    v_z = { let mut out = vec![]; for t10245 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t10245;
out.push(py::mul(&v_r.clone(), &py::cmath("exp", &[py::mul(&Py::Complex(0.0, 1.0), &py::add(&py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_k.clone())?, &v_n.clone())?, &Py::Float(0.4))?)?])?)?); } Py::list(out) };
    for t10246 in py::range_iter(&[(Py::Int(600i64)).i()?]) {
        v__ = t10246;
        v_biggest = Py::Float(0.0);
        for t10247 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_i = t10247;
            v_p = f__pval(v_a.clone(), py::getitem(&v_z.clone(), &v_i.clone())?)?;
            if ({ let t10249 = v_p.clone(); Py::Bool({ let t10250 = Py::Int(0i64); if py::eq(&t10249, &t10250) { true } else { false } }) }).truthy() {
                continue;
            }
            v_dp = f__pval(v_d.clone(), py::getitem(&v_z.clone(), &v_i.clone())?)?;
            v_ratio = (if ({ let t10251 = v_dp.clone(); Py::Bool({ let t10252 = Py::Int(0i64); if py::ne(&t10251, &t10252) { true } else { false } }) }).truthy() { py::truediv(&v_p.clone(), &v_dp.clone())? } else { py::mul(&v_p.clone(), &Py::Float(0.001))? });
            v_s = Py::Complex(0.0, 0.0);
            for t10253 in py::range_iter(&[(v_n.clone()).i()?]) {
                v_j = t10253;
                if ({ let t10258 = { let t10254 = v_j.clone(); Py::Bool({ let t10255 = v_i.clone(); if py::ne(&t10254, &t10255) { true } else { false } }) }; if !t10258.truthy() { t10258 } else { { let t10256 = py::getitem(&v_z.clone(), &v_i.clone())?; Py::Bool({ let t10257 = py::getitem(&v_z.clone(), &v_j.clone())?; if py::ne(&t10256, &t10257) { true } else { false } }) } } }).truthy() {
                    v_s = py::iadd(&v_s, &py::truediv(&Py::Float(1.0), &py::sub(&py::getitem(&v_z.clone(), &v_i.clone())?, &py::getitem(&v_z.clone(), &v_j.clone())?)?)?)?;
                }
            }
            v_w = py::truediv(&v_ratio.clone(), &py::sub(&Py::Int(1i64), &py::mul(&v_ratio.clone(), &v_s.clone())?)?)?;
            let t10259 = v_z.clone(); let t10260 = v_i.clone();
            py::setitem(&t10259, &t10260, py::sub(&py::getitem(&t10259, &t10260)?, &v_w.clone())?)?;
            v_biggest = py::minmax(&[v_biggest.clone(), py::truediv(&py::abs(&v_w.clone())?, &py::minmax(&[Py::Float(1.0), py::abs(&py::getitem(&v_z.clone(), &v_i.clone())?)?], true)?)?], true)?;
        }
        if ({ let t10261 = v_biggest.clone(); Py::Bool({ let t10262 = Py::Float(1e-17); if py::lt(&t10261, &t10262)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    v_out = Py::list(vec![]);
    for t10263 in py::iter(&(v_z.clone()))? {
        v_x = t10263;
        for t10264 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
            v__ = t10264;
            v_dp = f__pval(v_d.clone(), v_x.clone())?;
            if ({ let t10265 = v_dp.clone(); Py::Bool({ let t10266 = Py::Int(0i64); if py::ne(&t10265, &t10266) { true } else { false } }) }).truthy() {
                v_step = py::truediv(&f__pval(v_a.clone(), v_x.clone())?, &v_dp.clone())?;
                if ({ let t10267 = py::abs(&v_step.clone())?; Py::Bool({ let t10268 = py::mul(&Py::Float(0.001), &py::minmax(&[Py::Float(1.0), py::abs(&v_x.clone())?], true)?)?; if py::lt(&t10267, &t10268)? { true } else { false } }) }).truthy() {
                    v_x = py::sub(&v_x.clone(), &v_step.clone())?;
                }
            }
        }
        let _ = py::method(&v_out.clone(), "append", &[v_x.clone()])?;
    }
    v_out = f__multiples(v_a.clone(), v_out.clone())?;
    return Ok(py::add(&v_out.clone(), &py::mul(&Py::list(vec![Py::Float(0.0)]), &v_zeros.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__routh_ok(mut v_c: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_width: Py = Py::None;
    v_c = { let mut out = vec![]; for t9175 in py::iter(&(f__ptrim(v_c.clone())?))? { let mut v_x: Py; 
v_x = t9175;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    v_n = py::sub(&py::len(&v_c.clone())?, &Py::Int(1i64))?;
    if ({ let t9176 = v_n.clone(); Py::Bool({ let t9177 = Py::Int(0i64); if py::eq(&t9176, &t9177) { true } else { false } }) }).truthy() {
        return Ok(Py::Bool(true));
    }
    if ({ let t9178 = py::getitem(&v_c.clone(), &Py::Int(0i64))?; Py::Bool({ let t9179 = Py::Int(0i64); if py::lt(&t9178, &t9179)? { true } else { false } }) }).truthy() {
        v_c = { let mut out = vec![]; for t9180 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t9180;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
    }
    v_width = py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
    v_r0 = py::add(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_c.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_r1 = py::add(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_rows = Py::list(vec![v_r0.clone(), v_r1.clone()]);
    for t9181 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t9181;
        let t9182 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t9182.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t9182[0].clone();
        v_b = t9182[1].clone();
        if ({ let t9183 = py::getitem(&v_b.clone(), &Py::Int(0i64))?; Py::Bool({ let t9184 = Py::Int(0i64); if py::eq(&t9183, &t9184) { true } else { false } }) }).truthy() {
            return Ok(Py::Bool(false));
        }
        let _ = py::method(&v_rows.clone(), "append", &[py::add(&{ let mut out = vec![]; for t9185 in py::range_iter(&[(py::sub(&v_width.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t9185;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![py::fraction(&[Py::Int(0i64)])?]))?])?;
    }
    return Ok(Py::Bool(({ let mut out = vec![]; for t9186 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t9186;
out.push({ let t9187 = py::getitem(&v_r.clone(), &Py::Int(0i64))?; Py::Bool({ let t9188 = Py::Int(0i64); if py::gt(&t9187, &t9188)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__series_inf(mut v_num: Py, mut v_den: Py, mut v_k: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_rem: Py = Py::None;
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    v_num = f__pad(v_num.clone(), v_n.clone())?;
    v_rem = py::add(&Py::list((v_num.clone()).items()?), &py::mul(&Py::list(vec![Py::Float(0.0)]), &v_k.clone())?)?;
    v_out = Py::list(vec![]);
    for t4514 in py::range_iter(&[(v_k.clone()).i()?]) {
        v_i = t4514;
        v_c = py::truediv(&py::getitem(&v_rem.clone(), &v_i.clone())?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?;
        let _ = py::method(&v_out.clone(), "append", &[v_c.clone()])?;
        for t4515 in py::range_iter(&[(py::len(&v_den.clone())?).i()?]) {
            v_j = t4515;
            if ({ let t4516 = py::add(&v_i.clone(), &v_j.clone())?; Py::Bool({ let t4517 = py::len(&v_rem.clone())?; if py::lt(&t4516, &t4517)? { true } else { false } }) }).truthy() {
                let t4518 = v_rem.clone(); let t4519 = py::add(&v_i.clone(), &v_j.clone())?;
                py::setitem(&t4518, &t4519, py::sub(&py::getitem(&t4518, &t4519)?, &py::mul(&v_c.clone(), &py::getitem(&v_den.clone(), &v_j.clone())?)?)?)?;
            }
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__snapm(mut v_m: Py, mut v_rel: Py) -> R {
    let mut v_big: Py = Py::None;
    let mut v_flat: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_r: Py = Py::None;
    v_flat = { let mut out = vec![]; for t9456 in py::iter(&(v_m.clone()))? { let mut v_r: Py; 
v_r = t9456;
for t9457 in py::iter(&((if (Py::Bool(matches!(v_r.clone(), Py::List(_)))).truthy() { v_r.clone() } else { Py::list(vec![v_r.clone()]) })))? { let mut v_x: Py; 
v_x = t9457;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) };
    v_big = (if (v_flat.clone()).truthy() { py::minmax(&[v_flat.clone()], true)? } else { Py::Float(0.0) });
    v_out = Py::list(vec![]);
    for t9458 in py::iter(&(v_m.clone()))? {
        v_r = t9458;
        if (Py::Bool(matches!(v_r.clone(), Py::List(_)))).truthy() {
            let _ = py::method(&v_out.clone(), "append", &[{ let mut out = vec![]; for t9459 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t9459;
out.push((if ({ let t9460 = py::abs(&v_x.clone())?; Py::Bool({ let t9461 = py::mul(&v_rel.clone(), &v_big.clone())?; if py::le(&t9460, &t9461)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() })); } Py::list(out) }])?;
        } else {
            let _ = py::method(&v_out.clone(), "append", &[(if ({ let t9462 = py::abs(&v_r.clone())?; Py::Bool({ let t9463 = py::mul(&v_rel.clone(), &v_big.clone())?; if py::le(&t9462, &t9463)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_r.clone() })])?;
        }
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve(mut v_M: Py, mut v_b: Py, mut v_exact: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_multi: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_w: Py = Py::None;
    v_n = py::len(&v_M.clone())?;
    v_multi = Py::Bool(matches!(py::getitem(&v_b.clone(), &Py::Int(0i64))?, Py::List(_)));
    v_A = { let mut out = vec![]; for t8381 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t8381;
out.push(py::add(&Py::list((py::getitem(&v_M.clone(), &v_i.clone())?).items()?), &(if (v_multi.clone()).truthy() { Py::list((py::getitem(&v_b.clone(), &v_i.clone())?).items()?) } else { Py::list(vec![py::getitem(&v_b.clone(), &v_i.clone())?]) }))?); } Py::list(out) };
    v_w = py::len(&py::getitem(&v_A.clone(), &Py::Int(0i64))?)?;
    v_scale = (if (py::not(&v_exact.clone())).truthy() { py::minmax(&[{ let mut out = vec![]; for t8382 in py::iter(&(v_M.clone()))? { let mut v_r: Py; 
v_r = t8382;
for t8383 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t8383;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }], true)? } else { Py::Int(0i64) });
    for t8384 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t8384;
        v_p = v_c.clone();
        for t8385 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t8385;
            if ({ let t8386 = py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?)?; Py::Bool({ let t8387 = py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_p.clone())?, &v_c.clone())?)?; if py::gt(&t8386, &t8387)? { true } else { false } }) }).truthy() {
                v_p = v_r.clone();
            }
        }
        if ({ let t8394 = { let t8390 = v_exact.clone(); if !t8390.truthy() { t8390 } else { { let t8388 = py::getitem(&py::getitem(&v_A.clone(), &v_p.clone())?, &v_c.clone())?; Py::Bool({ let t8389 = Py::Int(0i64); if py::eq(&t8388, &t8389) { true } else { false } }) } } }; if t8394.truthy() { t8394 } else { { let t8393 = py::not(&v_exact.clone()); if !t8393.truthy() { t8393 } else { { let t8391 = py::abs(&py::getitem(&py::getitem(&v_A.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t8392 = py::mul(&Py::Float(1e-13), &py::minmax(&[v_scale.clone(), Py::Float(1e-300)], true)?)?; if py::le(&t8391, &t8392)? { true } else { false } }) } } } } }).truthy() {
            return Err(py::value_error(py::to_str(&Py::str("the matrix is singular"))));
        }
        let t8395 = (Py::tuple(vec![py::getitem(&v_A.clone(), &v_p.clone())?, py::getitem(&v_A.clone(), &v_c.clone())?])).items()?;
        if t8395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_A.clone(), &v_c.clone(), t8395[0].clone())?;
        py::setitem(&v_A.clone(), &v_p.clone(), t8395[1].clone())?;
        for t8396 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_r = t8396;
            if ({ let t8401 = { let t8397 = v_r.clone(); Py::Bool({ let t8398 = v_c.clone(); if py::ne(&t8397, &t8398) { true } else { false } }) }; if !t8401.truthy() { t8401 } else { { let t8399 = py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?; Py::Bool({ let t8400 = Py::Int(0i64); if py::ne(&t8399, &t8400) { true } else { false } }) } } }).truthy() {
                v_f = py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_c.clone())?, &v_c.clone())?)?;
                py::setitem(&v_A.clone(), &v_r.clone(), { let mut out = vec![]; for t8402 in py::iter(&(py::zip(&[py::getitem(&v_A.clone(), &v_r.clone())?, py::getitem(&v_A.clone(), &v_c.clone())?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t8403 = (t8402).items()?;
if t8403.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t8403[0].clone();
v_y = t8403[1].clone();
out.push(py::sub(&v_x.clone(), &py::mul(&v_f.clone(), &v_y.clone())?)?); } Py::list(out) })?;
            }
        }
    }
    v_out = { let mut out = vec![]; for t8404 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t8404;
out.push({ let mut out = vec![]; for t8405 in py::range_iter(&[(v_n.clone()).i()?, (v_w.clone()).i()?]) { let mut v_j: Py; 
v_j = t8405;
out.push(py::truediv(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_i.clone())?)?); } Py::list(out) }); } Py::list(out) };
    return Ok((if (v_multi.clone()).truthy() { v_out.clone() } else { { let mut out = vec![]; for t8406 in py::iter(&(v_out.clone()))? { let mut v_r: Py; 
v_r = t8406;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) } }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__square(mut v_v: Py, mut v_name: Py, mut v_hi: Py) -> R {
    let mut v_m: Py = Py::None;
    v_m = f__mat(v_v.clone(), v_name.clone(), v_hi.clone())?;
    let _ = f__need({ let t9430 = py::len(&v_m.clone())?; Py::Bool({ let t9431 = py::len(&py::getitem(&v_m.clone(), &Py::Int(0i64))?)?; if py::eq(&t9430, &t9431) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be square".to_string()].concat()))?;
    return Ok(v_m.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ss2tf(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py) -> R {
    let mut v_Ms: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    let t8771 = (f__faddeev(v_A.clone())?).items()?;
    if t8771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t8771[0].clone();
    v_Ms = t8771[1].clone();
    v_num = Py::list(vec![py::mul(&v_D.clone(), &py::getitem(&v_c.clone(), &Py::Int(0i64))?)?]);
    for t8772 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_k = t8772;
        v_v = f__mv(py::getitem(&v_Ms.clone(), &v_k.clone())?, v_B.clone())?;
        let _ = py::method(&v_num.clone(), "append", &[py::add(&py::sum(&{ let mut out = vec![]; for t8776 in py::iter(&(py::zip(&[v_C.clone(), v_v.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t8777 = (t8776).items()?;
if t8777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t8777[0].clone();
v_b = t8777[1].clone();
out.push(py::mul(&v_a.clone(), &v_b.clone())?); } Py::list(out) }, None)?, &py::mul(&v_D.clone(), &py::getitem(&v_c.clone(), &py::add(&v_k.clone(), &Py::Int(1i64))?)?)?)?])?;
    }
    return Ok(Py::tuple(vec![v_num.clone(), v_c.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ss_in(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py) -> R {
    let mut v_n: Py = Py::None;
    let mut v_out: Py = Py::None;
    v_A = f__square(v_A.clone(), Py::str("A"), Py::Int(6i64))?;
    v_n = py::len(&v_A.clone())?;
    v_out = Py::list(vec![v_A.clone()]);
    if ({ let t9432 = v_B.clone(); Py::Bool({ let t9433 = Py::None; if !py::is(&t9432, &t9433) { true } else { false } }) }).truthy() {
        v_B = f__vec(v_B.clone(), Py::str("B"), v_n.clone(), v_n.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[v_B.clone()])?;
    }
    if ({ let t9438 = v_C.clone(); Py::Bool({ let t9439 = Py::None; if !py::is(&t9438, &t9439) { true } else { false } }) }).truthy() {
        v_C = f__vec(v_C.clone(), Py::str("C"), v_n.clone(), v_n.clone())?;
        let _ = py::method(&v_out.clone(), "append", &[v_C.clone()])?;
    }
    if ({ let t9440 = v_D.clone(); Py::Bool({ let t9441 = Py::None; if !py::is(&t9440, &t9441) { true } else { false } }) }).truthy() {
        let _ = py::method(&v_out.clone(), "append", &[f__num(v_D.clone(), Py::str("D"))?])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__strip_origin(mut v_c: Py) -> R {
    let mut v_k: Py = Py::None;
    v_c = f__ptrim(v_c.clone())?;
    v_k = Py::Int(0i64);
    while ({ let t3624 = { let t3620 = py::len(&v_c.clone())?; Py::Bool({ let t3621 = Py::Int(1i64); if py::gt(&t3620, &t3621)? { true } else { false } }) }; if !t3624.truthy() { t3624 } else { { let t3622 = py::getitem(&v_c.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t3623 = Py::Int(0i64); if py::eq(&t3622, &t3623) { true } else { false } }) } } }).truthy() {
        v_c = py::slice(&v_c.clone(), None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
    }
    return Ok(Py::tuple(vec![v_c.clone(), v_k.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__subst(mut v_poly: Py, mut v_n: Py, mut v_c: Py, mut v_P: Py, mut v_Q: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_term: Py = Py::None;
    v_poly = f__pad(v_poly.clone(), py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    v_out = Py::list(vec![Py::Float(0.0)]);
    for t10066 in py::iter(&(py::enumerate(&v_poly.clone(), 0)?))? {
        let t10079 = (t10066).items()?;
        if t10079.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_i = t10079[0].clone();
        v_a = t10079[1].clone();
        v_k = py::sub(&v_n.clone(), &v_i.clone())?;
        if ({ let t10067 = v_a.clone(); Py::Bool({ let t10068 = Py::Int(0i64); if py::eq(&t10067, &t10068) { true } else { false } }) }).truthy() {
            continue;
        }
        v_term = f__pscale(f__pmul(f__ppow(v_P.clone(), v_k.clone())?, f__ppow(v_Q.clone(), py::sub(&v_n.clone(), &v_k.clone())?)?)?, py::mul(&v_a.clone(), &py::pow(&v_c.clone(), &v_k.clone())?)?)?;
        v_out = f__padd(v_out.clone(), v_term.clone())?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sym(mut v_Q: Py, mut v_name: Py) -> R {
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    v_n = py::len(&v_Q.clone())?;
    for t5717 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t5717;
        for t5718 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t5718;
            let _ = f__need({ let t5719 = py::getitem(&py::getitem(&v_Q.clone(), &v_i.clone())?, &v_j.clone())?; Py::Bool({ let t5720 = py::getitem(&py::getitem(&v_Q.clone(), &v_j.clone())?, &v_i.clone())?; if py::eq(&t5719, &t5720) { true } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " must be symmetric".to_string()].concat()))?;
        }
    }
    return Ok(v_Q.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__tf_in(mut v_num: Py, mut v_den: Py, mut v_proper: Py, mut v_strict: Py, mut v_hi: Py) -> R {
    v_num = f__ptrim(f__vec(v_num.clone(), Py::str("num"), Py::Int(1i64), v_hi.clone())?)?;
    v_den = f__ptrim(f__vec(v_den.clone(), Py::str("den"), Py::Int(1i64), v_hi.clone())?)?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t10177 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t10177;
out.push({ let t10178 = v_x.clone(); Py::Bool({ let t10179 = Py::Int(0i64); if py::ne(&t10178, &t10179) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())), Py::str("the denominator must not be zero"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t10180 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t10180;
out.push({ let t10181 = v_x.clone(); Py::Bool({ let t10182 = Py::Int(0i64); if py::ne(&t10181, &t10182) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())), Py::str("the numerator must not be zero"))?;
    let _ = f__need({ let t10183 = py::len(&v_den.clone())?; Py::Bool({ let t10184 = Py::Int(2i64); if py::ge(&t10183, &t10184)? { true } else { false } }) }, Py::str("the denominator needs degree 1 or more"))?;
    if (v_strict.clone()).truthy() {
        let _ = f__need({ let t10185 = py::len(&v_num.clone())?; Py::Bool({ let t10186 = py::len(&v_den.clone())?; if py::lt(&t10185, &t10186)? { true } else { false } }) }, Py::str("the transfer function must be strictly proper (numerator degree below the denominator's)"))?;
    } else {
        if (v_proper.clone()).truthy() {
            let _ = f__need({ let t10187 = py::len(&v_num.clone())?; Py::Bool({ let t10188 = py::len(&v_den.clone())?; if py::le(&t10187, &t10188)? { true } else { false } }) }, Py::str("the transfer function must be proper (numerator degree at most the denominator's)"))?;
        }
    }
    return Ok(Py::tuple(vec![v_num.clone(), v_den.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trace(mut v_X: Py) -> R {
    return Ok(py::sum(&{ let mut out = vec![]; for t9289 in py::range_iter(&[(py::len(&v_X.clone())?).i()?]) { let mut v_i: Py; 
v_i = t9289;
out.push(py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__vec(mut v_v: Py, mut v_name: Py, mut v_lo: Py, mut v_hi: Py) -> R {
    let mut v_out: Py = Py::None;
    let _ = f__need(Py::Bool(matches!(v_v.clone(), Py::List(_) | Py::Tuple(_))), Py::str(&[py::format(&v_name.clone(), "")?, " must be a list of numbers".to_string()].concat()))?;
    v_out = { let mut out = vec![]; for t10171 in py::iter(&(v_v.clone()))? { let mut v_x: Py; 
v_x = t10171;
out.push(f__num(v_x.clone(), v_name.clone())?); } Py::list(out) };
    let _ = f__need({ let t10174 = v_lo.clone(); Py::Bool({ let t10175 = py::len(&v_out.clone())?; if py::le(&t10174, &t10175)? { { let t10176 = v_hi.clone(); if py::le(&t10175, &t10176)? { true } else { false } } } else { false } }) }, Py::str(&[py::format(&v_name.clone(), "")?, " needs between ".to_string(), py::format(&v_lo.clone(), "")?, " and ".to_string(), py::format(&v_hi.clone(), "")?, " entries".to_string()].concat()))?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wrap180(mut v_a: Py) -> R {
    v_a = py::fmod(&v_a.clone(), &Py::Float(360.0))?;
    if ({ let t8976 = v_a.clone(); Py::Bool({ let t8977 = py::neg(&Py::Float(180.0))?; if py::le(&t8976, &t8977)? { true } else { false } }) }).truthy() {
        v_a = py::iadd(&v_a, &Py::Float(360.0))?;
    } else {
        if ({ let t8978 = v_a.clone(); Py::Bool({ let t8979 = Py::Float(180.0); if py::gt(&t8978, &t8979)? { true } else { false } }) }).truthy() {
            v_a = py::sub(&v_a, &Py::Float(360.0))?;
        }
    }
    return Ok(py::add(&v_a.clone(), &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__y2(mut v_zeta: Py, mut v_wn: Py, mut v_t: Py) -> R {
    let mut v_q: Py = Py::None;
    let mut v_s1: Py = Py::None;
    let mut v_s2: Py = Py::None;
    let mut v_wd: Py = Py::None;
    if ({ let t9876 = v_t.clone(); Py::Bool({ let t9877 = Py::Int(0i64); if py::eq(&t9876, &t9877) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t9878 = v_zeta.clone(); Py::Bool({ let t9879 = Py::Int(1i64); if py::lt(&t9878, &t9879)? { true } else { false } }) }).truthy() {
        v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?;
        return Ok(py::sub(&Py::Int(1i64), &py::mul(&py::math1("exp", &py::mul(&py::mul(&py::neg(&v_zeta.clone())?, &v_wn.clone())?, &v_t.clone())?)?, &py::add(&py::math1("cos", &py::mul(&v_wd.clone(), &v_t.clone())?)?, &py::mul(&py::truediv(&v_zeta.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?, &py::math1("sin", &py::mul(&v_wd.clone(), &v_t.clone())?)?)?)?)?)?);
    }
    if ({ let t9880 = v_zeta.clone(); Py::Bool({ let t9881 = Py::Int(1i64); if py::eq(&t9880, &t9881) { true } else { false } }) }).truthy() {
        return Ok(py::sub(&Py::Int(1i64), &py::mul(&py::math1("exp", &py::mul(&py::neg(&v_wn.clone())?, &v_t.clone())?)?, &py::add(&Py::Int(1i64), &py::mul(&v_wn.clone(), &v_t.clone())?)?)?)?);
    }
    v_q = py::math1("sqrt", &py::sub(&py::mul(&v_zeta.clone(), &v_zeta.clone())?, &Py::Int(1i64))?)?;
    let t9882 = (Py::tuple(vec![py::mul(&v_wn.clone(), &py::sub(&v_zeta.clone(), &v_q.clone())?)?, py::mul(&v_wn.clone(), &py::add(&v_zeta.clone(), &v_q.clone())?)?])).items()?;
    if t9882.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s1 = t9882[0].clone();
    v_s2 = t9882[1].clone();
    return Ok(py::sub(&Py::Int(1i64), &py::truediv(&py::sub(&py::mul(&v_s2.clone(), &py::math1("exp", &py::mul(&py::neg(&v_s1.clone())?, &v_t.clone())?)?)?, &py::mul(&v_s1.clone(), &py::math1("exp", &py::mul(&py::neg(&v_s2.clone())?, &v_t.clone())?)?)?)?, &py::sub(&v_s2.clone(), &v_s1.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__zoh_ss(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t1196 = (f__ccf(v_num.clone(), v_den.clone())?).items()?;
    if t1196.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1196[0].clone();
    v_B = t1196[1].clone();
    v_C = t1196[2].clone();
    v_D = t1196[3].clone();
    v_n = py::len(&v_A.clone())?;
    v_M = py::add(&{ let mut out = vec![]; for t1197 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1197;
out.push(py::add(&{ let mut out = vec![]; for t1198 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t1198;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &v_T.clone())?); } Py::list(out) }, &Py::list(vec![py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &v_T.clone())?]))?); } Py::list(out) }, &Py::list(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?]))?;
    v_E = f__expm(v_M.clone())?;
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t1221 in py::iter(&(py::slice(&v_E.clone(), None, Some((v_n.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t1221;
out.push(py::slice(&v_r.clone(), None, Some((v_n.clone()).i()?), None)?); } Py::list(out) }, { let mut out = vec![]; for t1222 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1222;
out.push(py::getitem(&py::getitem(&v_E.clone(), &v_i.clone())?, &v_n.clone())?); } Py::list(out) }, v_C.clone(), v_D.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ackermann(mut v_A: Py, mut v_B: Py, mut v_alpha: Py) -> R {
    let t28 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t28.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t28[0].clone();
    v_B = t28[1].clone();
    v_alpha = f__vec(v_alpha.clone(), Py::str("alpha"), Py::Int(2i64), Py::Int(7i64))?;
    return Ok(Py::dict(vec![(Py::str("K"), { let mut out = vec![]; for t111 in py::iter(&(f__acker(v_A.clone(), v_B.clone(), v_alpha.clone())?))? { let mut v_x: Py; 
v_x = t111;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_arrival_angle(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_others: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let mut v_z: Py = Py::None;
    let mut v_zeros: Py = Py::None;
    let t168 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t168.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t168[0].clone();
    v_den = t168[1].clone();
    let _ = f__need({ let t169 = py::mul(&py::getitem(&v_num.clone(), &Py::Int(0i64))?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t170 = Py::Int(0i64); if py::gt(&t169, &t170)? { true } else { false } }) }, Py::str("the leading coefficients must have the same sign (the K > 0 locus)"))?;
    let t235 = (Py::tuple(vec![f__roots_raw(v_den.clone())?, f__roots_raw(v_num.clone())?])).items()?;
    if t235.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_poles = t235[0].clone();
    v_zeros = t235[1].clone();
    v_z = f__one_pair(f__canon(v_zeros.clone(), Py::Float(1e-09))?, Py::str("zeros"))?;
    v_others = Py::list((v_zeros.clone()).items()?);
    v_k = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_others.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_den = v_den.clone(); let v_num = v_num.clone(); let v_others = v_others.clone(); let v_poles = v_poles.clone(); let v_z = v_z.clone(); let v_zeros = v_zeros.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&py::complex(&[py::getitem(&v_others.clone(), &v_i.clone())?])?, &v_z.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
    let _ = py::method(&v_others.clone(), "pop", &[v_k.clone()])?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("zero"), v_z.clone()), (Py::str("arrival_deg"), f__angle_rule(v_z.clone(), v_poles.clone(), v_others.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_backward_euler(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let mut v_n: Py = Py::None;
    let t333 = (f__disc_tf_in(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t333.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_num = t333[0].clone();
    v_den = t333[1].clone();
    v_T = t333[2].clone();
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    return Ok(f__disc_out(f__subst(v_num.clone(), v_n.clone(), py::truediv(&Py::Float(1.0), &v_T.clone())?, Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0), Py::Float(0.0)]))?, f__subst(v_den.clone(), v_n.clone(), py::truediv(&Py::Float(1.0), &v_T.clone())?, Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0), Py::Float(0.0)]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bandwidth_general(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_g0: Py = Py::None;
    let mut v_roots: Py = Py::None;
    let t423 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(true), Py::Int(9i64))?).items()?;
    if t423.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t423[0].clone();
    v_den = t423[1].clone();
    let _ = f__need(f__routh_ok(v_den.clone())?, Py::str("the system must be stable"))?;
    v_g0 = py::truediv(&py::getitem(&v_num.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_den.clone(), &py::neg(&Py::Int(1i64))?)?)?;
    let _ = f__need({ let t438 = v_g0.clone(); Py::Bool({ let t439 = Py::Int(0i64); if py::ne(&t438, &t439) { true } else { false } }) }, Py::str("the DC gain is zero"))?;
    v_roots = f__real_pos_roots(f__ptrim(f__psub(f__pscale(f__mag2(v_num.clone())?, Py::Float(2.0))?, f__pscale(f__mag2(v_den.clone())?, py::mul(&v_g0.clone(), &v_g0.clone())?)?)?)?, Py::Float(1e-07))?;
    let _ = f__need({ let t550 = py::len(&v_roots.clone())?; Py::Bool({ let t551 = Py::Int(0i64); if py::gt(&t550, &t551)? { true } else { false } }) }, Py::str("the gain never falls 3 dB below its DC value"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("wb"), py::getitem(&v_roots.clone(), &Py::Int(0i64))?), (Py::str("dc_gain"), v_g0.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bandwidth_second(mut v_zeta: Py, mut v_wn: Py) -> R {
    let mut v_x: Py = Py::None;
    let t597 = (Py::tuple(vec![f__pos(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?])).items()?;
    if t597.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t597[0].clone();
    v_wn = t597[1].clone();
    v_x = py::sub(&Py::Int(1i64), &py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &v_zeta.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("wb"), py::mul(&v_wn.clone(), &py::math1("sqrt", &py::add(&v_x.clone(), &py::math1("sqrt", &py::add(&py::mul(&v_x.clone(), &v_x.clone())?, &Py::Int(1i64))?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bode_point(mut v_num: Py, mut v_den: Py, mut v_w: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let t663 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t663.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t663[0].clone();
    v_den = t663[1].clone();
    v_w = f__pos(v_w.clone(), Py::str("w"))?;
    v_g = f__G(v_num.clone(), v_den.clone(), py::complex(&[Py::Float(0.0), v_w.clone()])?)?;
    v_mag = py::hypot(&[py::real(&v_g.clone())?, py::imag(&v_g.clone())?])?;
    let _ = f__need({ let t669 = v_mag.clone(); Py::Bool({ let t670 = Py::Int(0i64); if py::gt(&t669, &t670)? { true } else { false } }) }, Py::str("G(jw) is zero at this frequency"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("magnitude"), v_mag.clone()), (Py::str("magnitude_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_mag.clone())?)?), (Py::str("phase_deg"), f__deg(v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_breakaway_points(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_dP: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t736 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t736.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t736[0].clone();
    v_den = t736[1].clone();
    v_P = f__ptrim(f__psub(f__pmul(v_num.clone(), f__pder(v_den.clone())?)?, f__pmul(f__pder(v_num.clone())?, v_den.clone())?)?)?;
    v_pts = Py::list(vec![]);
    if ({ let t749 = py::len(&v_P.clone())?; Py::Bool({ let t750 = Py::Int(2i64); if py::ge(&t749, &t750)? { true } else { false } }) }).truthy() {
        v_dP = f__pder(v_P.clone())?;
        for t812 in py::iter(&(f__roots_raw(v_P.clone())?))? {
            v_r = t812;
            v_r = py::complex(&[v_r.clone()])?;
            if ({ let t813 = py::abs(&py::imag(&v_r.clone())?)?; Py::Bool({ let t814 = py::mul(&Py::Float(1e-07), &py::minmax(&[Py::Float(1.0), py::abs(&v_r.clone())?], true)?)?; if py::gt(&t813, &t814)? { true } else { false } }) }).truthy() {
                continue;
            }
            v_x = py::real(&v_r.clone())?;
            for t815 in py::range_iter(&[(Py::Int(3i64)).i()?]) {
                v__ = t815;
                v_dv = f__pval(v_dP.clone(), v_x.clone())?;
                if ({ let t816 = v_dv.clone(); Py::Bool({ let t817 = Py::Int(0i64); if py::ne(&t816, &t817) { true } else { false } }) }).truthy() {
                    v_x = py::sub(&v_x, &py::truediv(&f__pval(v_P.clone(), v_x.clone())?, &v_dv.clone())?)?;
                }
            }
            v_nv = f__pval(v_num.clone(), v_x.clone())?;
            if ({ let t818 = v_nv.clone(); Py::Bool({ let t819 = Py::Int(0i64); if py::eq(&t818, &t819) { true } else { false } }) }).truthy() {
                continue;
            }
            v_k = py::truediv(&py::neg(&f__pval(v_den.clone(), v_x.clone())?)?, &v_nv.clone())?;
            if ({ let t820 = v_k.clone(); Py::Bool({ let t821 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&f__pval(v_den.clone(), v_x.clone())?)?], true)?)?; if py::gt(&t820, &t821)? { true } else { false } }) }).truthy() {
                let _ = py::method(&v_pts.clone(), "append", &[Py::tuple(vec![v_x.clone(), v_k.clone()])])?;
            }
        }
    }
    let _ = py::method_sort(&v_pts.clone(), None, false)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("points"), { let mut out = vec![]; for t831 in py::iter(&(v_pts.clone()))? { let mut v__: Py; let mut v_p: Py; 
let t832 = (t831).items()?;
if t832.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_p = t832[0].clone();
v__ = t832[1].clone();
out.push(v_p.clone()); } Py::list(out) }), (Py::str("gains"), { let mut out = vec![]; for t833 in py::iter(&(v_pts.clone()))? { let mut v__: Py; let mut v_k: Py; 
let t834 = (t833).items()?;
if t834.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v__ = t834[0].clone();
v_k = t834[1].clone();
out.push(v_k.clone()); } Py::list(out) }), (Py::str("count"), py::len(&v_pts.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_c2d_foh(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Bd: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_Dd: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_G1: Py = Py::None;
    let mut v_G2: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_PG2: Py = Py::None;
    let mut v_Phi: Py = Py::None;
    let mut v_dz: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_nz: Py = Py::None;
    let t894 = (f__disc_tf_in(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t894.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_num = t894[0].clone();
    v_den = t894[1].clone();
    v_T = t894[2].clone();
    let _ = f__need({ let t895 = py::mul(&py::minmax(&[{ let mut out = vec![]; for t978 in py::iter(&(f__roots(v_den.clone())?))? { let mut v_p: Py; 
v_p = t978;
out.push(py::abs(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?, &v_T.clone())?; Py::Bool({ let t896 = Py::Int(20i64); if py::le(&t895, &t896)? { true } else { false } }) }, Py::str("|pole| T is too large (limit 20)"))?;
    let t985 = (f__ccf(v_num.clone(), v_den.clone())?).items()?;
    if t985.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t985[0].clone();
    v_B = t985[1].clone();
    v_C = t985[2].clone();
    v_D = t985[3].clone();
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t986 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(2i64))?).i()?]) { let mut v__: Py; 
v__ = t986;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(2i64))?)?); } Py::list(out) };
    for t987 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_i = t987;
        for t988 in py::range_iter(&[(v_n.clone()).i()?]) {
            v_j = t988;
            py::setitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_j.clone(), py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &v_T.clone())?)?;
        }
        py::setitem(&py::getitem(&v_M.clone(), &v_i.clone())?, &v_n.clone(), py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &v_T.clone())?)?;
    }
    py::setitem(&py::getitem(&v_M.clone(), &v_n.clone())?, &py::add(&v_n.clone(), &Py::Int(1i64))?, Py::Float(1.0))?;
    v_E = f__expm(v_M.clone())?;
    v_Phi = { let mut out = vec![]; for t1011 in py::iter(&(py::slice(&v_E.clone(), None, Some((v_n.clone()).i()?), None)?))? { let mut v_r: Py; 
v_r = t1011;
out.push(py::slice(&v_r.clone(), None, Some((v_n.clone()).i()?), None)?); } Py::list(out) };
    v_G1 = { let mut out = vec![]; for t1012 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1012;
out.push(py::getitem(&py::getitem(&v_E.clone(), &v_i.clone())?, &v_n.clone())?); } Py::list(out) };
    v_G2 = { let mut out = vec![]; for t1013 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1013;
out.push(py::getitem(&py::getitem(&v_E.clone(), &v_i.clone())?, &py::add(&v_n.clone(), &Py::Int(1i64))?)?); } Py::list(out) };
    v_PG2 = f__mv(v_Phi.clone(), v_G2.clone())?;
    v_Bd = { let mut out = vec![]; for t1017 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1017;
out.push(py::add(&py::sub(&py::getitem(&v_G1.clone(), &v_i.clone())?, &py::getitem(&v_G2.clone(), &v_i.clone())?)?, &py::getitem(&v_PG2.clone(), &v_i.clone())?)?); } Py::list(out) };
    v_Dd = py::add(&v_D.clone(), &py::sum(&{ let mut out = vec![]; for t1018 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t1018;
out.push(py::mul(&py::getitem(&v_C.clone(), &v_i.clone())?, &py::getitem(&v_G2.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?;
    let t1026 = (f__ss2tf(v_Phi.clone(), v_Bd.clone(), v_C.clone(), v_Dd.clone())?).items()?;
    if t1026.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nz = t1026[0].clone();
    v_dz = t1026[1].clone();
    return Ok(f__disc_out(v_nz.clone(), v_dz.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_c2d_zoh(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let mut v_Ad: Py = Py::None;
    let mut v_Bd: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_dz: Py = Py::None;
    let mut v_nz: Py = Py::None;
    let t1105 = (f__disc_tf_in(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t1105.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1105[0].clone();
    v_den = t1105[1].clone();
    v_T = t1105[2].clone();
    let _ = f__need({ let t1106 = py::mul(&py::minmax(&[{ let mut out = vec![]; for t1189 in py::iter(&(f__roots(v_den.clone())?))? { let mut v_p: Py; 
v_p = t1189;
out.push(py::abs(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?, &v_T.clone())?; Py::Bool({ let t1107 = Py::Int(20i64); if py::le(&t1106, &t1107)? { true } else { false } }) }, Py::str("|pole| T is too large (limit 20)"))?;
    let t1223 = (f__zoh_ss(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t1223.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Ad = t1223[0].clone();
    v_Bd = t1223[1].clone();
    v_C = t1223[2].clone();
    v_D = t1223[3].clone();
    let t1234 = (f__ss2tf(v_Ad.clone(), v_Bd.clone(), v_C.clone(), v_D.clone())?).items()?;
    if t1234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nz = t1234[0].clone();
    v_dz = t1234[1].clone();
    return Ok(f__disc_out(v_nz.clone(), v_dz.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_closed_loop_first_order_p(mut v_K: Py, mut v_tau: Py, mut v_Kp: Py) -> R {
    let mut v_L: Py = Py::None;
    let t1290 = (Py::tuple(vec![f__num(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__num(v_Kp.clone(), Py::str("Kp"))?])).items()?;
    if t1290.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t1290[0].clone();
    v_tau = t1290[1].clone();
    v_Kp = t1290[2].clone();
    v_L = py::mul(&v_K.clone(), &v_Kp.clone())?;
    let _ = f__need({ let t1291 = py::add(&Py::Int(1i64), &v_L.clone())?; Py::Bool({ let t1292 = Py::Int(0i64); if py::gt(&t1291, &t1292)? { true } else { false } }) }, Py::str("the closed loop is unstable (1 + K Kp must be positive)"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dc_gain_cl"), py::truediv(&v_L.clone(), &py::add(&Py::Int(1i64), &v_L.clone())?)?), (Py::str("tau_cl"), py::truediv(&v_tau.clone(), &py::add(&Py::Int(1i64), &v_L.clone())?)?), (Py::str("ess"), py::truediv(&Py::Float(1.0), &py::add(&Py::Int(1i64), &v_L.clone())?)?), (Py::str("pole"), py::truediv(&py::neg(&py::add(&Py::Int(1i64), &v_L.clone())?)?, &v_tau.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_cohen_coon(mut v_K: Py, mut v_tau: Py, mut v_theta: Py, mut v_controller: Py) -> R {
    let mut v_r: Py = Py::None;
    let t1338 = (Py::tuple(vec![f__pos(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__pos(v_theta.clone(), Py::str("theta"))?])).items()?;
    if t1338.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t1338[0].clone();
    v_tau = t1338[1].clone();
    v_theta = t1338[2].clone();
    let _ = f__need({ let t1339 = v_controller.clone(); Py::Bool({ let t1340 = Py::tuple(vec![Py::str("P"), Py::str("PI"), Py::str("PID")]); if py::contains(&t1340, &t1339)? { true } else { false } }) }, Py::str("controller must be P, PI or PID"))?;
    v_r = py::truediv(&v_theta.clone(), &v_tau.clone())?;
    if ({ let t1341 = v_controller.clone(); Py::Bool({ let t1342 = Py::str("P"); if py::eq(&t1341, &t1342) { true } else { false } }) }).truthy() {
        return Ok(f__pid_out(py::mul(&py::mul(&py::truediv(&Py::Int(1i64), &v_K.clone())?, &py::truediv(&Py::Int(1i64), &v_r.clone())?)?, &py::add(&Py::Int(1i64), &py::truediv(&v_r.clone(), &Py::Int(3i64))?)?)?, Py::Float(0.0), Py::Float(0.0))?);
    }
    if ({ let t1354 = v_controller.clone(); Py::Bool({ let t1355 = Py::str("PI"); if py::eq(&t1354, &t1355) { true } else { false } }) }).truthy() {
        return Ok(f__pid_out(py::mul(&py::mul(&py::truediv(&Py::Int(1i64), &v_K.clone())?, &py::truediv(&Py::Int(1i64), &v_r.clone())?)?, &py::add(&Py::Float(0.9), &py::truediv(&v_r.clone(), &Py::Int(12i64))?)?)?, py::truediv(&py::mul(&v_theta.clone(), &py::add(&Py::Int(30i64), &py::mul(&Py::Int(3i64), &v_r.clone())?)?)?, &py::add(&Py::Int(9i64), &py::mul(&Py::Int(20i64), &v_r.clone())?)?)?, Py::Float(0.0))?);
    }
    return Ok(f__pid_out(py::mul(&py::mul(&py::truediv(&Py::Int(1i64), &v_K.clone())?, &py::truediv(&Py::Int(1i64), &v_r.clone())?)?, &py::add(&py::truediv(&Py::Float(4.0), &Py::Int(3i64))?, &py::truediv(&v_r.clone(), &Py::Int(4i64))?)?)?, py::truediv(&py::mul(&v_theta.clone(), &py::add(&Py::Int(32i64), &py::mul(&Py::Int(6i64), &v_r.clone())?)?)?, &py::add(&Py::Int(13i64), &py::mul(&Py::Int(8i64), &v_r.clone())?)?)?, py::truediv(&py::mul(&Py::Int(4i64), &v_theta.clone())?, &py::add(&Py::Int(11i64), &py::mul(&Py::Int(2i64), &v_r.clone())?)?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_controllability(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_W: Py = Py::None;
    let mut v_r: Py = Py::None;
    let t1415 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t1415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1415[0].clone();
    v_B = t1415[1].clone();
    v_W = f__ctrb(f__frac(v_A.clone())?, f__frac(v_B.clone())?)?;
    v_r = f__rank(v_W.clone())?;
    return Ok(Py::dict(vec![(Py::str("Wc"), { let mut out = vec![]; for t1444 in py::iter(&(v_W.clone()))? { let mut v_row: Py; 
v_row = t1444;
out.push({ let mut out = vec![]; for t1445 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t1445;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("rank"), v_r.clone()), (Py::str("controllable"), { let t1446 = v_r.clone(); Py::Bool({ let t1447 = py::len(&v_A.clone())?; if py::eq(&t1446, &t1447) { true } else { false } }) }), (Py::str("det_Wc"), f__r12(py::float(&f__det(v_W.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ctrb_gramian(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_Bf: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t1522 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t1522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t1522[0].clone();
    v_B = t1522[1].clone();
    let _ = f__need(f__is_hurwitz(v_A.clone())?, Py::str("A must be stable for the infinite-horizon Gramian"))?;
    v_Bf = f__frac(v_B.clone())?;
    v_W = f__lyap(f__mT(f__frac(v_A.clone())?)?, { let mut out = vec![]; for t1671 in py::range_iter(&[(py::len(&v_B.clone())?).i()?]) { let mut v_i: Py; 
v_i = t1671;
out.push({ let mut out = vec![]; for t1672 in py::range_iter(&[(py::len(&v_B.clone())?).i()?]) { let mut v_j: Py; 
v_j = t1672;
out.push(py::mul(&py::getitem(&v_Bf.clone(), &v_i.clone())?, &py::getitem(&v_Bf.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) }, Py::Bool(true))?;
    v_d = f__det(v_W.clone())?;
    return Ok(Py::dict(vec![(Py::str("Wc"), { let mut out = vec![]; for t1688 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t1688;
out.push({ let mut out = vec![]; for t1689 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t1689;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("controllable"), { let t1690 = v_d.clone(); Py::Bool({ let t1691 = Py::Int(0i64); if py::ne(&t1690, &t1691) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_damping_from_overshoot(mut v_OS: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    v_mp = f__num(v_OS.clone(), Py::str("OS"))?;
    let _ = f__need({ let t1726 = Py::Int(0i64); Py::Bool({ let t1727 = v_mp.clone(); if py::lt(&t1726, &t1727)? { { let t1728 = Py::Int(1i64); if py::lt(&t1727, &t1728)? { true } else { false } } } else { false } }) }, Py::str("the overshoot must lie strictly between 0 and 100 %"))?;
    v_L = py::math1("log", &v_mp.clone())?;
    v_zeta = py::truediv(&py::neg(&v_L.clone())?, &py::math1("sqrt", &py::add(&py::mul(&c_PI()?, &c_PI()?)?, &py::mul(&v_L.clone(), &v_L.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("zeta"), v_zeta.clone()), (Py::str("pole_angle_deg"), py::math1("degrees", &py::math1("acos", &v_zeta.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_gain(mut v_num: Py, mut v_den: Py, mut v_A: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let t1794 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t1794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1794[0].clone();
    v_den = t1794[1].clone();
    v_A = f__num(v_A.clone(), Py::str("A"))?;
    let _ = f__need({ let t1795 = py::getitem(&v_den.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t1796 = Py::Int(0i64); if py::ne(&t1795, &t1796) { true } else { false } }) }, Py::str("G(s) has a pole at s = 0: the DC gain is infinite"))?;
    v_poles = f__roots(v_den.clone())?;
    let _ = f__need({ let t1878 = py::minmax(&[{ let mut out = vec![]; for t1880 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t1880;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t1879 = Py::Int(0i64); if py::lt(&t1878, &t1879)? { true } else { false } }) }, Py::str("the system is not stable, so its step response has no final value"))?;
    v_g = py::truediv(&py::getitem(&v_num.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_den.clone(), &py::neg(&Py::Int(1i64))?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dc_gain"), v_g.clone()), (Py::str("final_value"), py::mul(&v_g.clone(), &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dc_motor(mut v_J: Py, mut v_b: Py, mut v_Kt: Py, mut v_R: Py, mut v_L: Py) -> R {
    let mut v_den: Py = Py::None;
    let t1928 = (Py::tuple(vec![f__pos(v_J.clone(), Py::str("J"))?, f__nonneg(v_b.clone(), Py::str("b"))?, f__pos(v_Kt.clone(), Py::str("Kt"))?, f__pos(v_R.clone(), Py::str("R"))?, f__pos(v_L.clone(), Py::str("L"))?])).items()?;
    if t1928.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_J = t1928[0].clone();
    v_b = t1928[1].clone();
    v_Kt = t1928[2].clone();
    v_R = t1928[3].clone();
    v_L = t1928[4].clone();
    v_den = Py::list(vec![py::mul(&v_J.clone(), &v_L.clone())?, py::add(&py::mul(&v_b.clone(), &v_L.clone())?, &py::mul(&v_J.clone(), &v_R.clone())?)?, py::add(&py::mul(&v_b.clone(), &v_R.clone())?, &py::mul(&v_Kt.clone(), &v_Kt.clone())?)?]);
    return Ok(Py::dict(vec![(Py::str("num"), Py::list(vec![f__r12(v_Kt.clone())?])), (Py::str("den"), { let mut out = vec![]; for t1929 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t1929;
out.push(f__r12(v_x.clone())?); } Py::list(out) }), (Py::str("dc_gain"), f__r12(py::truediv(&v_Kt.clone(), &py::getitem(&v_den.clone(), &Py::Int(2i64))?)?)?), (Py::str("poles"), f__roots(v_den.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_deadbeat_gain(mut v_A: Py, mut v_B: Py) -> R {
    let mut v_n: Py = Py::None;
    let t2076 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t2076.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2076[0].clone();
    v_B = t2076[1].clone();
    v_n = py::len(&v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("K"), { let mut out = vec![]; for t2159 in py::iter(&(f__acker(v_A.clone(), v_B.clone(), py::add(&Py::list(vec![Py::Float(1.0)]), &py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?)?)?))? { let mut v_x: Py; 
v_x = t2159;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_departure_angle(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_others: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let mut v_zeros: Py = Py::None;
    let t2216 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t2216.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2216[0].clone();
    v_den = t2216[1].clone();
    let _ = f__need({ let t2217 = py::mul(&py::getitem(&v_num.clone(), &Py::Int(0i64))?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?; Py::Bool({ let t2218 = Py::Int(0i64); if py::gt(&t2217, &t2218)? { true } else { false } }) }, Py::str("the leading coefficients must have the same sign (the K > 0 locus)"))?;
    let t2283 = (Py::tuple(vec![f__roots_raw(v_den.clone())?, f__roots_raw(v_num.clone())?])).items()?;
    if t2283.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_poles = t2283[0].clone();
    v_zeros = t2283[1].clone();
    v_p = f__one_pair(f__canon(v_poles.clone(), Py::Float(1e-09))?, Py::str("poles"))?;
    v_others = Py::list((v_poles.clone()).items()?);
    v_k = py::minmax_key(&[Py::list((py::range_iter(&[(py::len(&v_others.clone())?).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_den = v_den.clone(); let v_num = v_num.clone(); let v_others = v_others.clone(); let v_p = v_p.clone(); let v_poles = v_poles.clone(); let v_zeros = v_zeros.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_i: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::sub(&py::complex(&[py::getitem(&v_others.clone(), &v_i.clone())?])?, &v_p.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, false)?;
    let _ = py::method(&v_others.clone(), "pop", &[v_k.clone()])?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("pole"), v_p.clone()), (Py::str("departure_deg"), f__angle_rule(v_p.clone(), v_zeros.clone(), v_others.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discrete_dc_gain(mut v_num: Py, mut v_den: Py, mut v_A: Py) -> R {
    let mut v_g: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let t2378 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t2378.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2378[0].clone();
    v_den = t2378[1].clone();
    v_A = f__num(v_A.clone(), Py::str("A"))?;
    let _ = f__need({ let t2379 = py::sum(&v_den.clone(), None)?; Py::Bool({ let t2380 = Py::Int(0i64); if py::ne(&t2379, &t2380) { true } else { false } }) }, Py::str("G(z) has a pole at z = 1: the DC gain is infinite"))?;
    v_poles = f__roots(v_den.clone())?;
    let _ = f__need({ let t2462 = py::minmax(&[{ let mut out = vec![]; for t2464 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t2464;
out.push(py::abs(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t2463 = Py::Int(1i64); if py::lt(&t2462, &t2463)? { true } else { false } }) }, Py::str("the system is not stable (a pole on or outside the unit circle)"))?;
    v_g = py::truediv(&py::sum(&v_num.clone(), None)?, &py::sum(&v_den.clone(), None)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("dc_gain"), v_g.clone()), (Py::str("final_value"), py::mul(&v_g.clone(), &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discrete_kalman_gain(mut v_A: Py, mut v_C: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_PC: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t2533 = (f__ss_in(v_A.clone(), Py::None, v_C.clone(), Py::None)?).items()?;
    if t2533.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2533[0].clone();
    v_C = t2533[1].clone();
    v_Q = f__square(v_Q.clone(), Py::str("Q"), Py::Int(6i64))?;
    let _ = f__need({ let t2534 = py::len(&v_Q.clone())?; Py::Bool({ let t2535 = py::len(&v_A.clone())?; if py::eq(&t2534, &t2535) { true } else { false } }) }, Py::str("Q must be n x n"))?;
    let _ = f__sym(v_Q.clone(), Py::str("Q"))?;
    let _ = f__need(f__posdef(f__frac(v_Q.clone())?)?, Py::str("Q must be positive definite"))?;
    v_R = f__pos(v_R.clone(), Py::str("R"))?;
    let _ = f__need({ let t2564 = f__rank(f__ctrb(f__frac(f__mT(v_A.clone())?)?, f__frac(v_C.clone())?)?)?; Py::Bool({ let t2565 = py::len(&v_A.clone())?; if py::eq(&t2564, &t2565) { true } else { false } }) }, Py::str("(A, C) must be observable"))?;
    let t2684 = (f__dare(f__mT(v_A.clone())?, v_C.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t2684.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t2684[0].clone();
    v__ = t2684[1].clone();
    v_n = py::len(&v_A.clone())?;
    v_PC = { let mut out = vec![]; for t2685 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2685;
out.push(py::sum(&{ let mut out = vec![]; for t2686 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t2686;
out.push(py::mul(&py::getitem(&py::getitem(&v_P.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_C.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) };
    v_s = py::add(&v_R.clone(), &py::sum(&{ let mut out = vec![]; for t2687 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t2687;
out.push(py::mul(&py::getitem(&v_C.clone(), &v_i.clone())?, &py::getitem(&v_PC.clone(), &v_i.clone())?)?); } Py::list(out) }, None)?)?;
    return Ok(Py::dict(vec![(Py::str("K"), f__rnd({ let mut out = vec![]; for t2697 in py::iter(&(v_PC.clone()))? { let mut v_x: Py; 
v_x = t2697;
out.push(py::truediv(&v_x.clone(), &v_s.clone())?); } Py::list(out) })?), (Py::str("P_prior"), f__rnd(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discrete_lyapunov(mut v_A: Py, mut v_Q: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let t2757 = (f__ss_in(v_A.clone(), Py::None, Py::None, Py::None)?).items()?;
    if t2757.len() != 1 { return Err(PyErr::Value("unpack".into())); }
    v_A = t2757[0].clone();
    v_Q = f__square(v_Q.clone(), Py::str("Q"), Py::Int(6i64))?;
    let _ = f__need({ let t2758 = py::len(&v_Q.clone())?; Py::Bool({ let t2759 = py::len(&v_A.clone())?; if py::eq(&t2758, &t2759) { true } else { false } }) }, Py::str("Q must be n x n"))?;
    let _ = f__sym(v_Q.clone(), Py::str("Q"))?;
    let t2784 = (f__faddeev(f__frac(v_A.clone())?)?).items()?;
    if t2784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t2784[0].clone();
    v__ = t2784[1].clone();
    let _ = f__need({ let t2785 = py::minmax(&[{ let mut out = vec![]; for t2875 in py::iter(&(f__roots({ let mut out = vec![]; for t2874 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t2874;
out.push(py::float(&v_x.clone())?); } Py::list(out) })?))? { let mut v_r: Py; 
v_r = t2875;
out.push(py::abs(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t2786 = Py::Int(1i64); if py::lt(&t2785, &t2786)? { true } else { false } }) }, Py::str("A must be stable (every eigenvalue inside the unit circle)"))?;
    v_P = f__dlyap(f__frac(v_A.clone())?, f__frac(v_Q.clone())?, Py::Bool(true))?;
    return Ok(Py::dict(vec![(Py::str("P"), { let mut out = vec![]; for t2911 in py::iter(&(v_P.clone()))? { let mut v_r: Py; 
v_r = t2911;
out.push({ let mut out = vec![]; for t2912 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t2912;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("positive_definite"), f__posdef(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discrete_pid(mut v_Kp: Py, mut v_Ki: Py, mut v_Kd: Py, mut v_T: Py) -> R {
    let t2970 = (Py::tuple(vec![f__num(v_Kp.clone(), Py::str("Kp"))?, f__nonneg(v_Ki.clone(), Py::str("Ki"))?, f__nonneg(v_Kd.clone(), Py::str("Kd"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t2970.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Kp = t2970[0].clone();
    v_Ki = t2970[1].clone();
    v_Kd = t2970[2].clone();
    v_T = t2970[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("q0"), py::add(&py::add(&v_Kp.clone(), &py::mul(&v_Ki.clone(), &v_T.clone())?)?, &py::truediv(&v_Kd.clone(), &v_T.clone())?)?), (Py::str("q1"), py::sub(&py::neg(&v_Kp.clone())?, &py::truediv(&py::mul(&Py::Int(2i64), &v_Kd.clone())?, &v_T.clone())?)?), (Py::str("q2"), py::truediv(&v_Kd.clone(), &v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_discrete_step(mut v_numz: Py, mut v_denz: Py, mut v_k: Py, mut v_A: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_acc: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_num: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t3036 = (f__tf_in(v_numz.clone(), v_denz.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t3036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t3036[0].clone();
    v_den = t3036[1].clone();
    v_k = py::int(&v_k.clone())?;
    let _ = f__need({ let t3037 = Py::Int(0i64); Py::Bool({ let t3038 = v_k.clone(); if py::le(&t3037, &t3038)? { { let t3039 = Py::Int(500i64); if py::le(&t3038, &t3039)? { true } else { false } } } else { false } }) }, Py::str("k must be between 0 and 500"))?;
    v_A = f__num(v_A.clone(), Py::str("A"))?;
    v_a0 = py::getitem(&v_den.clone(), &Py::Int(0i64))?;
    v_den = { let mut out = vec![]; for t3040 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t3040;
out.push(py::truediv(&v_x.clone(), &v_a0.clone())?); } Py::list(out) };
    v_num = f__pad({ let mut out = vec![]; for t3041 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t3041;
out.push(py::truediv(&v_x.clone(), &v_a0.clone())?); } Py::list(out) }, py::len(&v_den.clone())?)?;
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    v_y = Py::list(vec![]);
    for t3042 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) {
        v_j = t3042;
        v_acc = Py::Float(0.0);
        for t3043 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3043;
            if ({ let t3044 = py::sub(&v_j.clone(), &v_i.clone())?; Py::Bool({ let t3045 = Py::Int(0i64); if py::ge(&t3044, &t3045)? { true } else { false } }) }).truthy() {
                v_acc = py::iadd(&v_acc, &py::mul(&py::getitem(&v_num.clone(), &v_i.clone())?, &v_A.clone())?)?;
            }
        }
        for t3046 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
            v_i = t3046;
            if ({ let t3047 = py::sub(&v_j.clone(), &v_i.clone())?; Py::Bool({ let t3048 = Py::Int(0i64); if py::ge(&t3047, &t3048)? { true } else { false } }) }).truthy() {
                v_acc = py::sub(&v_acc, &py::mul(&py::getitem(&v_den.clone(), &v_i.clone())?, &py::getitem(&v_y.clone(), &py::sub(&v_j.clone(), &v_i.clone())?)?)?)?;
            }
        }
        let _ = py::method(&v_y.clone(), "append", &[v_acc.clone()])?;
    }
    return Ok(f__rnd(Py::dict(vec![(Py::str("y"), py::getitem(&v_y.clone(), &v_k.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dlqr(mut v_A: Py, mut v_B: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_P: Py = Py::None;
    let t3148 = (f__riccati_in(v_A.clone(), v_B.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t3148.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t3148[0].clone();
    v_B = t3148[1].clone();
    v_Q = t3148[2].clone();
    v_R = t3148[3].clone();
    let t3267 = (f__dare(v_A.clone(), v_B.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t3267.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t3267[0].clone();
    v_K = t3267[1].clone();
    return Ok(Py::dict(vec![(Py::str("K"), f__rnd(v_K.clone())?), (Py::str("P"), f__rnd(v_P.clone())?), (Py::str("poles"), f__cl_poles(v_A.clone(), v_B.clone(), v_K.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_error_constants(mut v_num: Py, mut v_den: Py, mut v_input: Py, mut v_A: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_N: Py = Py::None;
    let mut v_d0: Py = Py::None;
    let mut v_ess: Py = Py::None;
    let mut v_kd: Py = Py::None;
    let mut v_kn: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let mut v_order: Py = Py::None;
    let t3427 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t3427.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t3427[0].clone();
    v_den = t3427[1].clone();
    v_A = f__num(v_A.clone(), Py::str("A"))?;
    let _ = f__need({ let t3428 = v_input.clone(); Py::Bool({ let t3429 = Py::tuple(vec![Py::str("step"), Py::str("ramp"), Py::str("parabola")]); if py::contains(&t3429, &t3428)? { true } else { false } }) }, Py::str("input must be step, ramp or parabola"))?;
    let _ = f__need(f__routh_ok(f__padd(v_den.clone(), v_num.clone())?)?, Py::str("the closed loop is not stable: there is no steady-state error"))?;
    let t3451 = (f__strip_origin(v_num.clone())?).items()?;
    if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n0 = t3451[0].clone();
    v_kn = t3451[1].clone();
    let t3452 = (f__strip_origin(v_den.clone())?).items()?;
    if t3452.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d0 = t3452[0].clone();
    v_kd = t3452[1].clone();
    let _ = f__need({ let t3453 = v_kn.clone(); Py::Bool({ let t3454 = Py::Int(0i64); if py::eq(&t3453, &t3454) { true } else { false } }) }, Py::str("G(s) has a zero at the origin"))?;
    v_N = v_kd.clone();
    v_K = py::truediv(&py::getitem(&v_n0.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_d0.clone(), &py::neg(&Py::Int(1i64))?)?)?;
    v_order = py::getitem(&Py::dict(vec![(Py::str("step"), Py::Int(0i64)), (Py::str("ramp"), Py::Int(1i64)), (Py::str("parabola"), Py::Int(2i64))]), &v_input.clone())?;
    let _ = f__need({ let t3455 = v_N.clone(); Py::Bool({ let t3456 = v_order.clone(); if py::ge(&t3455, &t3456)? { true } else { false } }) }, Py::str("the steady-state error grows without bound (the system type is below the input's order)"))?;
    if ({ let t3457 = v_N.clone(); Py::Bool({ let t3458 = v_order.clone(); if py::gt(&t3457, &t3458)? { true } else { false } }) }).truthy() {
        v_ess = Py::Float(0.0);
    } else {
        if ({ let t3459 = v_N.clone(); Py::Bool({ let t3460 = Py::Int(0i64); if py::eq(&t3459, &t3460) { true } else { false } }) }).truthy() {
            v_ess = py::truediv(&v_A.clone(), &py::add(&Py::Int(1i64), &v_K.clone())?)?;
        } else {
            v_ess = py::truediv(&v_A.clone(), &v_K.clone())?;
        }
    }
    return Ok(f__rnd(Py::dict(vec![(Py::str("system_type"), v_N.clone()), (Py::str("K_static"), v_K.clone()), (Py::str("ess"), v_ess.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_feedback_connection(mut v_num_g: Py, mut v_den_g: Py, mut v_num_h: Py, mut v_den_h: Py, mut v_sign: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_dg: Py = Py::None;
    let mut v_dh: Py = Py::None;
    let mut v_loop: Py = Py::None;
    let mut v_ng: Py = Py::None;
    let mut v_nh: Py = Py::None;
    let t3526 = (f__tf_in(v_num_g.clone(), v_den_g.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t3526.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ng = t3526[0].clone();
    v_dg = t3526[1].clone();
    v_nh = (if ({ let t3527 = v_num_h.clone(); Py::Bool({ let t3528 = Py::None; if !py::is(&t3527, &t3528) { true } else { false } }) }).truthy() { f__ptrim(f__vec(v_num_h.clone(), Py::str("num_h"), Py::Int(1i64), Py::Int(9i64))?)? } else { Py::list(vec![Py::Float(1.0)]) });
    v_dh = (if ({ let t3529 = v_den_h.clone(); Py::Bool({ let t3530 = Py::None; if !py::is(&t3529, &t3530) { true } else { false } }) }).truthy() { f__ptrim(f__vec(v_den_h.clone(), Py::str("den_h"), Py::Int(1i64), Py::Int(9i64))?)? } else { Py::list(vec![Py::Float(1.0)]) });
    let _ = f__need({ let t3537 = Py::Bool(({ let mut out = vec![]; for t3531 in py::iter(&(v_nh.clone()))? { let mut v_x: Py; 
v_x = t3531;
out.push({ let t3532 = v_x.clone(); Py::Bool({ let t3533 = Py::Int(0i64); if py::ne(&t3532, &t3533) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())); if !t3537.truthy() { t3537 } else { Py::Bool(({ let mut out = vec![]; for t3534 in py::iter(&(v_dh.clone()))? { let mut v_x: Py; 
v_x = t3534;
out.push({ let t3535 = v_x.clone(); Py::Bool({ let t3536 = Py::Int(0i64); if py::ne(&t3535, &t3536) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }, Py::str("H(s) must have a nonzero numerator and denominator"))?;
    let _ = f__need({ let t3538 = v_sign.clone(); Py::Bool({ let t3539 = Py::tuple(vec![Py::str("negative"), Py::str("positive")]); if py::contains(&t3539, &t3538)? { true } else { false } }) }, Py::str("sign must be negative or positive"))?;
    v_loop = f__pmul(v_ng.clone(), v_nh.clone())?;
    v_den = (if ({ let t3546 = v_sign.clone(); Py::Bool({ let t3547 = Py::str("negative"); if py::eq(&t3546, &t3547) { true } else { false } }) }).truthy() { f__padd(f__pmul(v_dg.clone(), v_dh.clone())?, v_loop.clone())? } else { f__psub(f__pmul(v_dg.clone(), v_dh.clone())?, v_loop.clone())? });
    v_den = f__ptrim(v_den.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3551 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t3551;
out.push({ let t3552 = v_x.clone(); Py::Bool({ let t3553 = Py::Int(0i64); if py::ne(&t3552, &t3553) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())), Py::str("the closed loop is singular (1 + G H = 0)"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("num"), f__pmul(v_ng.clone(), v_dh.clone())?), (Py::str("den"), v_den.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_final_value_theorem(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_d0: Py = Py::None;
    let mut v_kd: Py = Py::None;
    let mut v_kn: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let mut v_net: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3619 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t3619.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t3619[0].clone();
    v_den = t3619[1].clone();
    let t3625 = (f__strip_origin(v_num.clone())?).items()?;
    if t3625.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n0 = t3625[0].clone();
    v_kn = t3625[1].clone();
    let t3626 = (f__strip_origin(v_den.clone())?).items()?;
    if t3626.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_d0 = t3626[0].clone();
    v_kd = t3626[1].clone();
    v_net = py::sub(&v_kd.clone(), &v_kn.clone())?;
    let _ = f__need({ let t3627 = v_net.clone(); Py::Bool({ let t3628 = Py::Int(1i64); if py::le(&t3627, &t3628)? { true } else { false } }) }, Py::str("Y(s) has a repeated pole at the origin: y(t) grows without bound"))?;
    if ({ let t3629 = py::len(&v_d0.clone())?; Py::Bool({ let t3630 = Py::Int(1i64); if py::gt(&t3629, &t3630)? { true } else { false } }) }).truthy() {
        let _ = f__need({ let t3631 = py::minmax(&[{ let mut out = vec![]; for t3714 in py::iter(&(f__roots(v_d0.clone())?))? { let mut v_r: Py; 
v_r = t3714;
out.push(py::real(&py::complex(&[v_r.clone()])?)?); } Py::list(out) }], true)?; Py::Bool({ let t3632 = Py::Int(0i64); if py::lt(&t3631, &t3632)? { true } else { false } }) }, Py::str("s Y(s) has a pole in the right half plane or on the imaginary axis: the final value theorem does not apply"))?;
    }
    if ({ let t3715 = v_net.clone(); Py::Bool({ let t3716 = Py::Int(1i64); if py::eq(&t3715, &t3716) { true } else { false } }) }).truthy() {
        v_v = py::truediv(&py::getitem(&v_n0.clone(), &py::neg(&Py::Int(1i64))?)?, &py::getitem(&v_d0.clone(), &py::neg(&Py::Int(1i64))?)?)?;
    } else {
        v_v = Py::Float(0.0);
    }
    return Ok(f__rnd(Py::dict(vec![(Py::str("final_value"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_freq(mut v_K: Py, mut v_tau: Py, mut v_w: Py) -> R {
    let mut v_mag: Py = Py::None;
    let t3764 = (Py::tuple(vec![f__pos(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__nonneg(v_w.clone(), Py::str("w"))?])).items()?;
    if t3764.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t3764[0].clone();
    v_tau = t3764[1].clone();
    v_w = t3764[2].clone();
    v_mag = py::truediv(&v_K.clone(), &py::math1("sqrt", &py::add(&Py::Float(1.0), &py::pow(&py::mul(&v_w.clone(), &v_tau.clone())?, &Py::Int(2i64))?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("magnitude"), v_mag.clone()), (Py::str("magnitude_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_mag.clone())?)?), (Py::str("phase_deg"), py::neg(&py::math1("degrees", &py::math1("atan", &py::mul(&v_w.clone(), &v_tau.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_specs(mut v_K: Py, mut v_tau: Py) -> R {
    let t3810 = (Py::tuple(vec![f__num(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?])).items()?;
    if t3810.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_K = t3810[0].clone();
    v_tau = t3810[1].clone();
    let _ = f__need({ let t3811 = v_K.clone(); Py::Bool({ let t3812 = Py::Int(0i64); if py::ne(&t3811, &t3812) { true } else { false } }) }, Py::str("K must not be zero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("time_constant"), v_tau.clone()), (Py::str("rise_time"), py::mul(&v_tau.clone(), &py::math1("log", &Py::Float(9.0))?)?), (Py::str("settling_time_2"), py::mul(&v_tau.clone(), &py::math1("log", &Py::Float(50.0))?)?), (Py::str("settling_time_5"), py::mul(&v_tau.clone(), &py::math1("log", &Py::Float(20.0))?)?), (Py::str("bandwidth"), py::truediv(&Py::Float(1.0), &v_tau.clone())?), (Py::str("dc_gain"), v_K.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_first_order_step(mut v_K: Py, mut v_tau: Py, mut v_t: Py, mut v_A: Py) -> R {
    let mut v_frac: Py = Py::None;
    let t3860 = (Py::tuple(vec![f__num(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__nonneg(v_t.clone(), Py::str("t"))?, f__num(v_A.clone(), Py::str("A"))?])).items()?;
    if t3860.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_K = t3860[0].clone();
    v_tau = t3860[1].clone();
    v_t = t3860[2].clone();
    v_A = t3860[3].clone();
    let _ = f__need({ let t3865 = { let t3861 = v_K.clone(); Py::Bool({ let t3862 = Py::Int(0i64); if py::ne(&t3861, &t3862) { true } else { false } }) }; if !t3865.truthy() { t3865 } else { { let t3863 = v_A.clone(); Py::Bool({ let t3864 = Py::Int(0i64); if py::ne(&t3863, &t3864) { true } else { false } }) } } }, Py::str("K and A must not be zero"))?;
    v_frac = py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_t.clone())?, &v_tau.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("y"), py::mul(&py::mul(&v_K.clone(), &v_A.clone())?, &v_frac.clone())?), (Py::str("fraction"), v_frac.clone()), (Py::str("y_final"), py::mul(&v_K.clone(), &v_A.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fopdt_smith(mut v_t28: Py, mut v_t63: Py, mut v_dy: Py, mut v_du: Py) -> R {
    let mut v_tau: Py = Py::None;
    let mut v_theta: Py = Py::None;
    let t3911 = (Py::tuple(vec![f__pos(v_t28.clone(), Py::str("t28"))?, f__pos(v_t63.clone(), Py::str("t63"))?, f__num(v_dy.clone(), Py::str("dy"))?, f__num(v_du.clone(), Py::str("du"))?])).items()?;
    if t3911.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_t28 = t3911[0].clone();
    v_t63 = t3911[1].clone();
    v_dy = t3911[2].clone();
    v_du = t3911[3].clone();
    let _ = f__need({ let t3912 = v_t63.clone(); Py::Bool({ let t3913 = v_t28.clone(); if py::gt(&t3912, &t3913)? { true } else { false } }) }, Py::str("t63 must be later than t28"))?;
    let _ = f__need({ let t3918 = { let t3914 = v_du.clone(); Py::Bool({ let t3915 = Py::Int(0i64); if py::ne(&t3914, &t3915) { true } else { false } }) }; if !t3918.truthy() { t3918 } else { { let t3916 = v_dy.clone(); Py::Bool({ let t3917 = Py::Int(0i64); if py::ne(&t3916, &t3917) { true } else { false } }) } } }, Py::str("the input and output changes must not be zero"))?;
    v_tau = py::mul(&Py::Float(1.5), &py::sub(&v_t63.clone(), &v_t28.clone())?)?;
    v_theta = py::sub(&v_t63.clone(), &v_tau.clone())?;
    let _ = f__need({ let t3919 = v_theta.clone(); Py::Bool({ let t3920 = Py::Int(0i64); if py::ge(&t3919, &t3920)? { true } else { false } }) }, Py::str("the times give a negative dead time: the response is not first order plus dead time"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("K"), py::truediv(&v_dy.clone(), &v_du.clone())?), (Py::str("tau"), v_tau.clone()), (Py::str("theta"), v_theta.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_fopdt_step(mut v_K: Py, mut v_tau: Py, mut v_theta: Py, mut v_t: Py, mut v_A: Py) -> R {
    let mut v_y: Py = Py::None;
    let t3968 = (Py::tuple(vec![f__num(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__nonneg(v_theta.clone(), Py::str("theta"))?, f__nonneg(v_t.clone(), Py::str("t"))?, f__num(v_A.clone(), Py::str("A"))?])).items()?;
    if t3968.len() != 5 { return Err(PyErr::Value("unpack".into())); }
    v_K = t3968[0].clone();
    v_tau = t3968[1].clone();
    v_theta = t3968[2].clone();
    v_t = t3968[3].clone();
    v_A = t3968[4].clone();
    let _ = f__need({ let t3973 = { let t3969 = v_K.clone(); Py::Bool({ let t3970 = Py::Int(0i64); if py::ne(&t3969, &t3970) { true } else { false } }) }; if !t3973.truthy() { t3973 } else { { let t3971 = v_A.clone(); Py::Bool({ let t3972 = Py::Int(0i64); if py::ne(&t3971, &t3972) { true } else { false } }) } } }, Py::str("K and A must not be zero"))?;
    v_y = (if ({ let t3974 = v_t.clone(); Py::Bool({ let t3975 = v_theta.clone(); if py::le(&t3974, &t3975)? { true } else { false } }) }).truthy() { Py::Float(0.0) } else { py::mul(&py::mul(&py::neg(&v_K.clone())?, &v_A.clone())?, &py::math1("expm1", &py::truediv(&py::neg(&py::sub(&v_t.clone(), &v_theta.clone())?)?, &v_tau.clone())?)?)? });
    return Ok(f__rnd(Py::dict(vec![(Py::str("y"), v_y.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_forward_euler(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let mut v_n: Py = Py::None;
    let t4044 = (f__disc_tf_in(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t4044.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_num = t4044[0].clone();
    v_den = t4044[1].clone();
    v_T = t4044[2].clone();
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    return Ok(f__disc_out(f__subst(v_num.clone(), v_n.clone(), py::truediv(&Py::Float(1.0), &v_T.clone())?, Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0)]))?, f__subst(v_den.clone(), v_n.clone(), py::truediv(&Py::Float(1.0), &v_T.clone())?, Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0)]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_h2_norm(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_Af: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_Bf: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_Cf: Py = Py::None;
    let mut v_Wo: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_h2sq: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4134 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(true), Py::Int(9i64))?).items()?;
    if t4134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t4134[0].clone();
    v_den = t4134[1].clone();
    let _ = f__need(f__routh_ok(v_den.clone())?, Py::str("the system must be stable"))?;
    let t4155 = (f__ccf(v_num.clone(), v_den.clone())?).items()?;
    if t4155.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4155[0].clone();
    v_B = t4155[1].clone();
    v_C = t4155[2].clone();
    v__ = t4155[3].clone();
    let t4159 = (Py::tuple(vec![f__frac(v_A.clone())?, f__frac(v_B.clone())?, f__frac(v_C.clone())?])).items()?;
    if t4159.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Af = t4159[0].clone();
    v_Bf = t4159[1].clone();
    v_Cf = t4159[2].clone();
    v_n = py::len(&v_A.clone())?;
    v_Wo = f__lyap(v_Af.clone(), { let mut out = vec![]; for t4194 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4194;
out.push({ let mut out = vec![]; for t4195 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4195;
out.push(py::mul(&py::getitem(&v_Cf.clone(), &v_i.clone())?, &py::getitem(&v_Cf.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) }, Py::Bool(true))?;
    v_h2sq = py::sum(&{ let mut out = vec![]; for t4196 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t4196;
for t4197 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t4197;
out.push(py::mul(&py::mul(&py::getitem(&v_Bf.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_Wo.clone(), &v_i.clone())?, &v_j.clone())?)?, &py::getitem(&v_Bf.clone(), &v_j.clone())?)?); }} Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("h2"), f__r12(py::math1("sqrt", &py::float(&v_h2sq.clone())?)?)?), (Py::str("h2_squared"), f__r12(py::float(&v_h2sq.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hinf_norm(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_best_w: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t4254 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t4254.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t4254[0].clone();
    v_den = t4254[1].clone();
    let _ = f__need(f__routh_ok(v_den.clone())?, Py::str("the system must be stable"))?;
    let t4294 = (Py::tuple(vec![f__mag2(v_num.clone())?, f__mag2(v_den.clone())?])).items()?;
    if t4294.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t4294[0].clone();
    v_g = t4294[1].clone();
    v_P = f__ptrim(f__psub(f__pmul(f__pder(v_f.clone())?, v_g.clone())?, f__pmul(v_f.clone(), f__pder(v_g.clone())?)?)?)?;
    v_cands = Py::list(vec![Py::Float(0.0)]);
    if ({ let t4304 = { let t4299 = py::len(&v_P.clone())?; Py::Bool({ let t4300 = Py::Int(2i64); if py::ge(&t4299, &t4300)? { true } else { false } }) }; if !t4304.truthy() { t4304 } else { Py::Bool(({ let mut out = vec![]; for t4301 in py::iter(&(v_P.clone()))? { let mut v_x: Py; 
v_x = t4301;
out.push({ let t4302 = v_x.clone(); Py::Bool({ let t4303 = Py::Int(0i64); if py::ne(&t4302, &t4303) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        v_cands = py::iadd(&v_cands, &f__real_pos_roots(v_P.clone(), Py::Float(1e-07))?)?;
    }
    let t4385 = (Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?])).items()?;
    if t4385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best_w = t4385[0].clone();
    v_best = t4385[1].clone();
    for t4386 in py::iter(&(v_cands.clone()))? {
        v_w = t4386;
        v_v = py::math1("sqrt", &py::truediv(&f__pval(v_f.clone(), v_w.clone())?, &f__pval(v_g.clone(), v_w.clone())?)?)?;
        if ({ let t4387 = v_v.clone(); Py::Bool({ let t4388 = v_best.clone(); if py::gt(&t4387, &t4388)? { true } else { false } }) }).truthy() {
            let t4389 = (Py::tuple(vec![v_v.clone(), v_w.clone()])).items()?;
            if t4389.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_best = t4389[0].clone();
            v_best_w = t4389[1].clone();
        }
    }
    if ({ let t4390 = py::len(&v_num.clone())?; Py::Bool({ let t4391 = py::len(&v_den.clone())?; if py::eq(&t4390, &t4391) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t4392 = py::abs(&py::truediv(&py::getitem(&v_num.clone(), &Py::Int(0i64))?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?)?; Py::Bool({ let t4393 = py::mul(&v_best.clone(), &py::sub(&Py::Int(1i64), &Py::Float(1e-12))?)?; if py::lt(&t4392, &t4393)? { true } else { false } }) }, Py::str("the peak gain is approached only as w -> infinity"))?;
    }
    return Ok(f__rnd(Py::dict(vec![(Py::str("hinf"), v_best.clone()), (Py::str("hinf_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_best.clone())?)?), (Py::str("w_peak"), v_best_w.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_identify_second_order(mut v_y_peak: Py, mut v_y_final: Py, mut v_t_peak: Py, mut v_A: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_yf: Py = Py::None;
    let mut v_yp: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    let t4439 = (Py::tuple(vec![f__num(v_y_peak.clone(), Py::str("y_peak"))?, f__num(v_y_final.clone(), Py::str("y_final"))?, f__pos(v_t_peak.clone(), Py::str("t_peak"))?, f__num(v_A.clone(), Py::str("A"))?])).items()?;
    if t4439.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_yp = t4439[0].clone();
    v_yf = t4439[1].clone();
    v_tp = t4439[2].clone();
    v_A = t4439[3].clone();
    let _ = f__need({ let t4444 = { let t4440 = v_A.clone(); Py::Bool({ let t4441 = Py::Int(0i64); if py::ne(&t4440, &t4441) { true } else { false } }) }; if !t4444.truthy() { t4444 } else { { let t4442 = v_yf.clone(); Py::Bool({ let t4443 = Py::Int(0i64); if py::ne(&t4442, &t4443) { true } else { false } }) } } }, Py::str("the step size and the final value must not be zero"))?;
    v_mp = py::truediv(&py::sub(&v_yp.clone(), &v_yf.clone())?, &v_yf.clone())?;
    let _ = f__need({ let t4445 = Py::Int(0i64); Py::Bool({ let t4446 = v_mp.clone(); if py::lt(&t4445, &t4446)? { { let t4447 = Py::Int(1i64); if py::lt(&t4446, &t4447)? { true } else { false } } } else { false } }) }, Py::str("the peak must exceed the final value by less than 100 %"))?;
    v_L = py::math1("log", &v_mp.clone())?;
    v_zeta = py::truediv(&py::neg(&v_L.clone())?, &py::math1("sqrt", &py::add(&py::mul(&c_PI()?, &c_PI()?)?, &py::mul(&v_L.clone(), &v_L.clone())?)?)?)?;
    v_wn = py::truediv(&c_PI()?, &py::mul(&v_tp.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("K"), py::truediv(&v_yf.clone(), &v_A.clone())?), (Py::str("PO"), py::mul(&Py::Int(100i64), &v_mp.clone())?), (Py::str("zeta"), v_zeta.clone()), (Py::str("wn"), v_wn.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_initial_value_theorem(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_c: Py = Py::None;
    let t4513 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(true), Py::Int(9i64))?).items()?;
    if t4513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t4513[0].clone();
    v_den = t4513[1].clone();
    v_c = f__series_inf(v_num.clone(), v_den.clone(), Py::Int(2i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("y0"), py::getitem(&v_c.clone(), &Py::Int(0i64))?), (Py::str("dy0"), py::getitem(&v_c.clone(), &Py::Int(1i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_jury_test(mut v_coefficients: Py) -> R {
    let mut v_P1: Py = Py::None;
    let mut v_Pm1: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rev: Py = Py::None;
    let mut v_stable: Py = Py::None;
    v_c = f__ptrim(f__vec(v_coefficients.clone(), Py::str("coefficients"), Py::Int(2i64), Py::Int(12i64))?)?;
    let _ = f__need({ let t4573 = py::len(&v_c.clone())?; Py::Bool({ let t4574 = Py::Int(2i64); if py::ge(&t4573, &t4574)? { true } else { false } }) }, Py::str("give a polynomial of degree 1 or more"))?;
    v_f = { let mut out = vec![]; for t4575 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t4575;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    if ({ let t4576 = py::getitem(&v_f.clone(), &Py::Int(0i64))?; Py::Bool({ let t4577 = Py::Int(0i64); if py::lt(&t4576, &t4577)? { true } else { false } }) }).truthy() {
        v_f = { let mut out = vec![]; for t4578 in py::iter(&(v_f.clone()))? { let mut v_x: Py; 
v_x = t4578;
out.push(py::neg(&v_x.clone())?); } Py::list(out) };
    }
    v_n = py::sub(&py::len(&v_f.clone())?, &Py::Int(1i64))?;
    v_P1 = py::sum(&v_f.clone(), None)?;
    v_Pm1 = py::mul(&py::sum(&{ let mut out = vec![]; for t4579 in py::iter(&(py::enumerate(&v_f.clone(), 0)?))? { let mut v_i: Py; let mut v_x: Py; 
let t4580 = (t4579).items()?;
if t4580.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t4580[0].clone();
v_x = t4580[1].clone();
out.push(py::mul(&v_x.clone(), &py::pow(&py::neg(&Py::Int(1i64))?, &py::sub(&v_n.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None)?, &py::pow(&py::neg(&Py::Int(1i64))?, &v_n.clone())?)?;
    v_q = Py::list((v_f.clone()).items()?);
    v_stable = Py::Bool(true);
    while ({ let t4587 = py::len(&v_q.clone())?; Py::Bool({ let t4588 = Py::Int(2i64); if py::ge(&t4587, &t4588)? { true } else { false } }) }).truthy() {
        let t4581 = (Py::tuple(vec![py::getitem(&v_q.clone(), &Py::Int(0i64))?, py::getitem(&v_q.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t4581.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a0 = t4581[0].clone();
        v_an = t4581[1].clone();
        let _ = f__need({ let t4582 = py::abs(&v_an.clone())?; Py::Bool({ let t4583 = py::abs(&v_a0.clone())?; if py::ne(&t4582, &t4583) { true } else { false } }) }, Py::str("the Jury table is singular (|a_n| = |a_0| at a step): a root lies on the unit circle or the test is degenerate"))?;
        if ({ let t4584 = py::abs(&v_an.clone())?; Py::Bool({ let t4585 = py::abs(&v_a0.clone())?; if py::gt(&t4584, &t4585)? { true } else { false } }) }).truthy() {
            v_stable = Py::Bool(false);
            break;
        }
        v_k = py::truediv(&v_an.clone(), &v_a0.clone())?;
        v_rev = py::slice(&v_q.clone(), None, None, Some((py::neg(&Py::Int(1i64))?).i()?))?;
        v_q = py::slice(&{ let mut out = vec![]; for t4586 in py::range_iter(&[(py::len(&v_q.clone())?).i()?]) { let mut v_i: Py; 
v_i = t4586;
out.push(py::sub(&py::getitem(&v_q.clone(), &v_i.clone())?, &py::mul(&v_k.clone(), &py::getitem(&v_rev.clone(), &v_i.clone())?)?)?); } Py::list(out) }, None, Some((py::neg(&Py::Int(1i64))?).i()?), None)?;
    }
    return Ok(Py::dict(vec![(Py::str("stable"), v_stable.clone()), (Py::str("P1"), f__r12(py::float(&v_P1.clone())?)?), (Py::str("Pm1"), f__r12(py::float(&v_Pm1.clone())?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kalman_bucy(mut v_A: Py, mut v_C: Py, mut v_W: Py, mut v_V: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t4648 = (f__ss_in(v_A.clone(), Py::None, v_C.clone(), Py::None)?).items()?;
    if t4648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t4648[0].clone();
    v_C = t4648[1].clone();
    v_W = f__square(v_W.clone(), Py::str("W"), Py::Int(6i64))?;
    let _ = f__need({ let t4649 = py::len(&v_W.clone())?; Py::Bool({ let t4650 = py::len(&v_A.clone())?; if py::eq(&t4649, &t4650) { true } else { false } }) }, Py::str("W must be n x n"))?;
    let _ = f__sym(v_W.clone(), Py::str("W"))?;
    let _ = f__need(f__posdef(f__frac(v_W.clone())?)?, Py::str("W must be positive definite"))?;
    v_V = f__pos(v_V.clone(), Py::str("V"))?;
    let _ = f__need({ let t4679 = f__rank(f__ctrb(f__frac(f__mT(v_A.clone())?)?, f__frac(v_C.clone())?)?)?; Py::Bool({ let t4680 = py::len(&v_A.clone())?; if py::eq(&t4679, &t4680) { true } else { false } }) }, Py::str("(A, C) must be observable"))?;
    let t4798 = (f__care(f__mT(v_A.clone())?, v_C.clone(), v_W.clone(), v_V.clone())?).items()?;
    if t4798.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t4798[0].clone();
    v_L = t4798[1].clone();
    v_n = py::len(&v_A.clone())?;
    return Ok(Py::dict(vec![(Py::str("L"), f__rnd(v_L.clone())?), (Py::str("P"), f__rnd(v_P.clone())?), (Py::str("poles"), f__cl_poles(f__mT(v_A.clone())?, v_C.clone(), v_L.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kalman_scalar_steady(mut v_a: Py, mut v_c: Py, mut v_q: Py, mut v_r: Py) -> R {
    let mut v_Bq: Py = Py::None;
    let mut v_K: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let t4938 = (Py::tuple(vec![f__num(v_a.clone(), Py::str("a"))?, f__num(v_c.clone(), Py::str("c"))?, f__pos(v_q.clone(), Py::str("q"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t4938.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t4938[0].clone();
    v_c = t4938[1].clone();
    v_q = t4938[2].clone();
    v_r = t4938[3].clone();
    let _ = f__need({ let t4939 = v_c.clone(); Py::Bool({ let t4940 = Py::Int(0i64); if py::ne(&t4939, &t4940) { true } else { false } }) }, Py::str("c must not be zero"))?;
    v_Bq = py::sub(&py::sub(&v_r.clone(), &py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &v_r.clone())?)?, &py::mul(&py::mul(&v_q.clone(), &v_c.clone())?, &v_c.clone())?)?;
    v_disc = py::math1("sqrt", &py::add(&py::mul(&v_Bq.clone(), &v_Bq.clone())?, &py::mul(&py::mul(&py::mul(&py::mul(&Py::Int(4i64), &v_c.clone())?, &v_c.clone())?, &v_q.clone())?, &v_r.clone())?)?)?;
    v_P = (if ({ let t4941 = v_Bq.clone(); Py::Bool({ let t4942 = Py::Int(0i64); if py::gt(&t4941, &t4942)? { true } else { false } }) }).truthy() { py::truediv(&py::mul(&py::mul(&Py::Int(2i64), &v_q.clone())?, &v_r.clone())?, &py::add(&v_Bq.clone(), &v_disc.clone())?)? } else { py::truediv(&py::sub(&v_disc.clone(), &v_Bq.clone())?, &py::mul(&py::mul(&Py::Int(2i64), &v_c.clone())?, &v_c.clone())?)? });
    v_K = py::truediv(&py::mul(&v_P.clone(), &v_c.clone())?, &py::add(&py::mul(&py::mul(&v_c.clone(), &v_c.clone())?, &v_P.clone())?, &v_r.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("P_prior"), v_P.clone()), (Py::str("gain"), v_K.clone()), (Py::str("P_post"), py::mul(&py::sub(&Py::Int(1i64), &py::mul(&v_K.clone(), &v_c.clone())?)?, &v_P.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_kalman_scalar_step(mut v_x: Py, mut v_P: Py, mut v_a: Py, mut v_q: Py, mut v_r: Py, mut v_z: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_Pp: Py = Py::None;
    let mut v_xp: Py = Py::None;
    let t4990 = (Py::tuple(vec![f__num(v_x.clone(), Py::str("x"))?, f__nonneg(v_P.clone(), Py::str("P"))?, f__num(v_a.clone(), Py::str("a"))?, f__nonneg(v_q.clone(), Py::str("q"))?, f__pos(v_r.clone(), Py::str("r"))?, f__num(v_z.clone(), Py::str("z"))?])).items()?;
    if t4990.len() != 6 { return Err(PyErr::Value("unpack".into())); }
    v_x = t4990[0].clone();
    v_P = t4990[1].clone();
    v_a = t4990[2].clone();
    v_q = t4990[3].clone();
    v_r = t4990[4].clone();
    v_z = t4990[5].clone();
    v_xp = py::mul(&v_a.clone(), &v_x.clone())?;
    v_Pp = py::add(&py::mul(&py::mul(&v_a.clone(), &v_a.clone())?, &v_P.clone())?, &v_q.clone())?;
    let _ = f__need({ let t4991 = v_Pp.clone(); Py::Bool({ let t4992 = Py::Int(0i64); if py::gt(&t4991, &t4992)? { true } else { false } }) }, Py::str("the predicted variance is zero"))?;
    v_K = py::truediv(&v_Pp.clone(), &py::add(&v_Pp.clone(), &v_r.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("x_pred"), v_xp.clone()), (Py::str("P_pred"), v_Pp.clone()), (Py::str("gain"), v_K.clone()), (Py::str("x_post"), py::add(&v_xp.clone(), &py::mul(&v_K.clone(), &py::sub(&v_z.clone(), &v_xp.clone())?)?)?), (Py::str("P_post"), py::mul(&py::sub(&Py::Int(1i64), &v_K.clone())?, &v_Pp.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lag_design(mut v_beta: Py, mut v_w_c: Py, mut v_ratio: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_wc: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t5038 = (Py::tuple(vec![f__num(v_beta.clone(), Py::str("beta"))?, f__pos(v_w_c.clone(), Py::str("w_c"))?, f__num(v_ratio.clone(), Py::str("ratio"))?])).items()?;
    if t5038.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_beta = t5038[0].clone();
    v_wc = t5038[1].clone();
    v_ratio = t5038[2].clone();
    let _ = f__need({ let t5039 = v_beta.clone(); Py::Bool({ let t5040 = Py::Int(1i64); if py::gt(&t5039, &t5040)? { true } else { false } }) }, Py::str("beta must exceed 1 (the factor by which the low-frequency gain is raised)"))?;
    let _ = f__need({ let t5041 = v_ratio.clone(); Py::Bool({ let t5042 = Py::Int(2i64); if py::ge(&t5041, &t5042)? { true } else { false } }) }, Py::str("the zero should be at least a factor 2 below the crossover (ratio >= 2)"))?;
    v_z = py::truediv(&v_wc.clone(), &v_ratio.clone())?;
    v_p = py::truediv(&v_z.clone(), &v_beta.clone())?;
    v_g = py::complex(&[Py::Float(0.0), v_wc.clone()])?;
    v_c = py::truediv(&py::add(&v_g.clone(), &v_z.clone())?, &py::add(&v_g.clone(), &v_p.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("zero"), v_z.clone()), (Py::str("pole"), v_p.clone()), (Py::str("dc_gain"), v_beta.clone()), (Py::str("phase_deg"), f__deg(v_c.clone())?), (Py::str("magnitude_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &py::hypot(&[py::real(&v_c.clone())?, py::imag(&v_c.clone())?])?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lead_design(mut v_phi_max_deg: Py, mut v_w_m: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_alpha: Py = Py::None;
    let mut v_phi: Py = Py::None;
    let mut v_sn: Py = Py::None;
    let mut v_wm: Py = Py::None;
    let t5088 = (Py::tuple(vec![f__num(v_phi_max_deg.clone(), Py::str("phi_max_deg"))?, f__pos(v_w_m.clone(), Py::str("w_m"))?])).items()?;
    if t5088.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_phi = t5088[0].clone();
    v_wm = t5088[1].clone();
    let _ = f__need({ let t5089 = Py::Int(0i64); Py::Bool({ let t5090 = v_phi.clone(); if py::lt(&t5089, &t5090)? { { let t5091 = Py::Int(90i64); if py::lt(&t5090, &t5091)? { true } else { false } } } else { false } }) }, Py::str("the phase lead must lie between 0 and 90 degrees"))?;
    v_sn = py::math1("sin", &py::math1("radians", &v_phi.clone())?)?;
    v_alpha = py::truediv(&py::sub(&Py::Int(1i64), &v_sn.clone())?, &py::add(&Py::Int(1i64), &v_sn.clone())?)?;
    v_T = py::truediv(&Py::Float(1.0), &py::mul(&v_wm.clone(), &py::math1("sqrt", &v_alpha.clone())?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("alpha"), v_alpha.clone()), (Py::str("T"), v_T.clone()), (Py::str("zero"), py::truediv(&Py::Float(1.0), &v_T.clone())?), (Py::str("pole"), py::truediv(&Py::Float(1.0), &py::mul(&v_alpha.clone(), &v_T.clone())?)?), (Py::str("gain_db"), py::mul(&py::neg(&Py::Int(10i64))?, &py::math1("log10", &v_alpha.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lead_max_phase(mut v_z: Py, mut v_p: Py) -> R {
    let mut v_wm: Py = Py::None;
    let t5137 = (Py::tuple(vec![f__pos(v_z.clone(), Py::str("z"))?, f__pos(v_p.clone(), Py::str("p"))?])).items()?;
    if t5137.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_z = t5137[0].clone();
    v_p = t5137[1].clone();
    let _ = f__need({ let t5138 = v_z.clone(); Py::Bool({ let t5139 = v_p.clone(); if py::lt(&t5138, &t5139)? { true } else { false } }) }, Py::str("a lead compensator has its zero below its pole (z < p)"))?;
    v_wm = py::math1("sqrt", &py::mul(&v_z.clone(), &v_p.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("phi_max_deg"), py::math1("degrees", &py::math1("asin", &py::truediv(&py::sub(&v_p.clone(), &v_z.clone())?, &py::add(&v_p.clone(), &v_z.clone())?)?)?)?), (Py::str("w_max"), v_wm.clone()), (Py::str("alpha"), py::truediv(&v_z.clone(), &v_p.clone())?), (Py::str("magnitude_db"), py::mul(&Py::Int(10i64), &py::math1("log10", &py::truediv(&v_z.clone(), &v_p.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_locus_asymptotes(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_sp_: Py = Py::None;
    let mut v_sz: Py = Py::None;
    let t5205 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t5205.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t5205[0].clone();
    v_den = t5205[1].clone();
    let t5206 = (Py::tuple(vec![py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?, py::sub(&py::len(&v_num.clone())?, &Py::Int(1i64))?])).items()?;
    if t5206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n = t5206[0].clone();
    v_m = t5206[1].clone();
    let _ = f__need({ let t5207 = v_n.clone(); Py::Bool({ let t5208 = v_m.clone(); if py::gt(&t5207, &t5208)? { true } else { false } }) }, Py::str("asymptotes need more poles than zeros"))?;
    v_sp_ = py::truediv(&py::neg(&py::getitem(&v_den.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?;
    v_sz = (if ({ let t5209 = v_m.clone(); Py::Bool({ let t5210 = Py::Int(1i64); if py::ge(&t5209, &t5210)? { true } else { false } }) }).truthy() { py::truediv(&py::neg(&py::getitem(&v_num.clone(), &Py::Int(1i64))?)?, &py::getitem(&v_num.clone(), &Py::Int(0i64))?)? } else { Py::Float(0.0) });
    v_q = py::sub(&v_n.clone(), &v_m.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("n_asymptotes"), v_q.clone()), (Py::str("centroid"), py::truediv(&py::sub(&v_sp_.clone(), &v_sz.clone())?, &v_q.clone())?), (Py::str("angles_deg"), { let mut out = vec![]; for t5220 in py::range_iter(&[(v_q.clone()).i()?]) { let mut v_k: Py; 
v_k = t5220;
out.push(py::truediv(&py::mul(&py::add(&py::mul(&Py::Int(2i64), &v_k.clone())?, &Py::Int(1i64))?, &Py::Float(180.0))?, &v_q.clone())?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_locus_gain_at_point(mut v_num: Py, mut v_den: Py, mut v_s_re: Py, mut v_s_im: Py) -> R {
    let mut v_ang: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t5277 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t5277.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t5277[0].clone();
    v_den = t5277[1].clone();
    v_s = py::complex(&[f__num(v_s_re.clone(), Py::str("s_re"))?, f__num(v_s_im.clone(), Py::str("s_im"))?])?;
    let t5279 = (Py::tuple(vec![f__pval(v_num.clone(), v_s.clone())?, f__pval(v_den.clone(), v_s.clone())?])).items()?;
    if t5279.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_nv = t5279[0].clone();
    v_dv = t5279[1].clone();
    let _ = f__need({ let t5280 = py::abs(&v_nv.clone())?; Py::Bool({ let t5281 = Py::Int(0i64); if py::gt(&t5280, &t5281)? { true } else { false } }) }, Py::str("the point is a zero of G(s)"))?;
    let _ = f__need({ let t5282 = py::abs(&v_dv.clone())?; Py::Bool({ let t5283 = Py::Int(0i64); if py::gt(&t5282, &t5283)? { true } else { false } }) }, Py::str("the point is a pole of G(s): the gain there is 0"))?;
    v_g = py::truediv(&v_nv.clone(), &v_dv.clone())?;
    v_ang = f__deg(v_g.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("K"), py::truediv(&py::hypot(&[py::real(&v_dv.clone())?, py::imag(&v_dv.clone())?])?, &py::hypot(&[py::real(&v_nv.clone())?, py::imag(&v_nv.clone())?])?)?), (Py::str("angle_deg"), v_ang.clone()), (Py::str("angle_error_deg"), py::sub(&Py::Float(180.0), &py::abs(&v_ang.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_log_decrement(mut v_x1: Py, mut v_xn: Py, mut v_n: Py) -> R {
    let mut v_d: Py = Py::None;
    let t5329 = (Py::tuple(vec![f__pos(v_x1.clone(), Py::str("x1"))?, f__pos(v_xn.clone(), Py::str("xn"))?])).items()?;
    if t5329.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_x1 = t5329[0].clone();
    v_xn = t5329[1].clone();
    v_n = py::int(&v_n.clone())?;
    let _ = f__need({ let t5330 = v_n.clone(); Py::Bool({ let t5331 = Py::Int(1i64); if py::ge(&t5330, &t5331)? { true } else { false } }) }, Py::str("n must be at least 1"))?;
    let _ = f__need({ let t5332 = v_xn.clone(); Py::Bool({ let t5333 = v_x1.clone(); if py::lt(&t5332, &t5333)? { true } else { false } }) }, Py::str("the later peak must be smaller"))?;
    v_d = py::truediv(&py::math1("log", &py::truediv(&v_x1.clone(), &v_xn.clone())?)?, &v_n.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("delta"), v_d.clone()), (Py::str("zeta"), py::truediv(&v_d.clone(), &py::math1("sqrt", &py::add(&py::mul(&py::mul(&Py::Int(4i64), &c_PI()?)?, &c_PI()?)?, &py::mul(&v_d.clone(), &v_d.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lqr(mut v_A: Py, mut v_B: Py, mut v_Q: Py, mut v_R: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_P: Py = Py::None;
    let t5433 = (f__riccati_in(v_A.clone(), v_B.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t5433.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5433[0].clone();
    v_B = t5433[1].clone();
    v_Q = t5433[2].clone();
    v_R = t5433[3].clone();
    let t5551 = (f__care(v_A.clone(), v_B.clone(), v_Q.clone(), v_R.clone())?).items()?;
    if t5551.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_P = t5551[0].clone();
    v_K = t5551[1].clone();
    return Ok(Py::dict(vec![(Py::str("K"), f__rnd(v_K.clone())?), (Py::str("P"), f__rnd(v_P.clone())?), (Py::str("poles"), f__cl_poles(v_A.clone(), v_B.clone(), v_K.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_lyapunov(mut v_A: Py, mut v_Q: Py) -> R {
    let mut v_P: Py = Py::None;
    let t5714 = (f__ss_in(v_A.clone(), Py::None, Py::None, Py::None)?).items()?;
    if t5714.len() != 1 { return Err(PyErr::Value("unpack".into())); }
    v_A = t5714[0].clone();
    v_Q = f__square(v_Q.clone(), Py::str("Q"), Py::Int(6i64))?;
    let _ = f__need({ let t5715 = py::len(&v_Q.clone())?; Py::Bool({ let t5716 = py::len(&v_A.clone())?; if py::eq(&t5715, &t5716) { true } else { false } }) }, Py::str("Q must be n x n"))?;
    let _ = f__sym(v_Q.clone(), Py::str("Q"))?;
    let _ = f__need(f__is_hurwitz(v_A.clone())?, Py::str("A must be stable (every eigenvalue in the left half plane)"))?;
    v_P = f__lyap(f__frac(v_A.clone())?, f__frac(v_Q.clone())?, Py::Bool(true))?;
    return Ok(Py::dict(vec![(Py::str("P"), { let mut out = vec![]; for t5867 in py::iter(&(v_P.clone()))? { let mut v_r: Py; 
v_r = t5867;
out.push({ let mut out = vec![]; for t5868 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t5868;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("positive_definite"), f__posdef(v_P.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_mass_spring_damper(mut v_m: Py, mut v_c: Py, mut v_k: Py) -> R {
    let mut v_wd: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    let t5926 = (Py::tuple(vec![f__pos(v_m.clone(), Py::str("m"))?, f__nonneg(v_c.clone(), Py::str("c"))?, f__pos(v_k.clone(), Py::str("k"))?])).items()?;
    if t5926.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t5926[0].clone();
    v_c = t5926[1].clone();
    v_k = t5926[2].clone();
    v_wn = py::math1("sqrt", &py::truediv(&v_k.clone(), &v_m.clone())?)?;
    v_zeta = py::truediv(&v_c.clone(), &py::mul(&Py::Int(2i64), &py::math1("sqrt", &py::mul(&v_k.clone(), &v_m.clone())?)?)?)?;
    if ({ let t5927 = py::mul(&v_c.clone(), &v_c.clone())?; Py::Bool({ let t5928 = py::mul(&py::mul(&Py::Int(4i64), &v_k.clone())?, &v_m.clone())?; if py::eq(&t5927, &t5928) { true } else { false } }) }).truthy() {
        v_zeta = Py::Float(1.0);
    }
    v_wd = (if ({ let t5929 = v_zeta.clone(); Py::Bool({ let t5930 = Py::Int(1i64); if py::lt(&t5929, &t5930)? { true } else { false } }) }).truthy() { py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)? } else { Py::Float(0.0) });
    return Ok(f__rnd(Py::dict(vec![(Py::str("wn"), v_wn.clone()), (Py::str("zeta"), v_zeta.clone()), (Py::str("wd"), v_wd.clone()), (Py::str("dc_gain"), py::truediv(&Py::Float(1.0), &v_k.clone())?), (Py::str("damping"), f__class(v_zeta.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_nyquist_encirclements(mut v_num: Py, mut v_den: Py, mut v_K: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_Z: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_op: Py = Py::None;
    let t6002 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t6002.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t6002[0].clone();
    v_den = t6002[1].clone();
    v_K = f__num(v_K.clone(), Py::str("K"))?;
    let _ = f__need({ let t6003 = v_K.clone(); Py::Bool({ let t6004 = Py::Int(0i64); if py::ne(&t6003, &t6004) { true } else { false } }) }, Py::str("K must not be zero"))?;
    v_op = f__roots(v_den.clone())?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t6086 in py::iter(&(v_op.clone()))? { let mut v_p: Py; 
v_p = t6086;
out.push({ let t6087 = py::abs(&py::real(&py::complex(&[v_p.clone()])?)?)?; Py::Bool({ let t6088 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_p.clone()])?)?], true)?)?; if py::gt(&t6087, &t6088)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the open loop has a pole on the imaginary axis (the contour needs an indentation; not handled here)"))?;
    v_cl = f__roots(f__padd(v_den.clone(), f__pscale(v_num.clone(), v_K.clone())?)?)?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t6092 in py::iter(&(v_cl.clone()))? { let mut v_p: Py; 
v_p = t6092;
out.push({ let t6093 = py::abs(&py::real(&py::complex(&[v_p.clone()])?)?)?; Py::Bool({ let t6094 = py::mul(&Py::Float(1e-09), &py::minmax(&[Py::Float(1.0), py::abs(&py::complex(&[v_p.clone()])?)?], true)?)?; if py::gt(&t6093, &t6094)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("a closed-loop pole lies on the imaginary axis: the Nyquist curve passes through -1"))?;
    v_P = py::sum(&{ let mut out = vec![]; for t6095 in py::iter(&(v_op.clone()))? { let mut v_p: Py; 
v_p = t6095;
if ({ let t6096 = py::real(&py::complex(&[v_p.clone()])?)?; Py::Bool({ let t6097 = Py::Int(0i64); if py::gt(&t6096, &t6097)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_Z = py::sum(&{ let mut out = vec![]; for t6098 in py::iter(&(v_cl.clone()))? { let mut v_p: Py; 
v_p = t6098;
if ({ let t6099 = py::real(&py::complex(&[v_p.clone()])?)?; Py::Bool({ let t6100 = Py::Int(0i64); if py::gt(&t6099, &t6100)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("Z"), v_Z.clone()), (Py::str("N"), py::sub(&v_Z.clone(), &v_P.clone())?), (Py::str("stable"), { let t6101 = v_Z.clone(); Py::Bool({ let t6102 = Py::Int(0i64); if py::eq(&t6101, &t6102) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_observability(mut v_A: Py, mut v_C: Py) -> R {
    let mut v_Af: Py = Py::None;
    let mut v_At: Py = Py::None;
    let mut v_Cf: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let t6162 = (f__ss_in(v_A.clone(), Py::None, v_C.clone(), Py::None)?).items()?;
    if t6162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t6162[0].clone();
    v_C = t6162[1].clone();
    let t6166 = (Py::tuple(vec![f__frac(v_A.clone())?, f__frac(v_C.clone())?])).items()?;
    if t6166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Af = t6166[0].clone();
    v_Cf = t6166[1].clone();
    v_n = py::len(&v_A.clone())?;
    v_rows = Py::list(vec![Py::list((v_Cf.clone()).items()?)]);
    v_At = f__mT(v_Af.clone())?;
    for t6169 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t6169;
        let _ = py::method(&v_rows.clone(), "append", &[f__mv(v_At.clone(), py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?)?])?;
    }
    v_r = f__rank(v_rows.clone())?;
    return Ok(Py::dict(vec![(Py::str("Wo"), { let mut out = vec![]; for t6192 in py::iter(&(v_rows.clone()))? { let mut v_row: Py; 
v_row = t6192;
out.push({ let mut out = vec![]; for t6193 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t6193;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("rank"), v_r.clone()), (Py::str("observable"), { let t6194 = v_r.clone(); Py::Bool({ let t6195 = v_n.clone(); if py::eq(&t6194, &t6195) { true } else { false } }) }), (Py::str("det_Wo"), f__r12(py::float(&f__det(v_rows.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_observer_gain(mut v_A: Py, mut v_C: Py, mut v_alpha: Py) -> R {
    let t6270 = (f__ss_in(v_A.clone(), Py::None, v_C.clone(), Py::None)?).items()?;
    if t6270.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t6270[0].clone();
    v_C = t6270[1].clone();
    v_alpha = f__vec(v_alpha.clone(), Py::str("alpha"), Py::Int(2i64), Py::Int(7i64))?;
    return Ok(Py::dict(vec![(Py::str("L"), { let mut out = vec![]; for t6353 in py::iter(&(f__acker(f__mT(v_A.clone())?, v_C.clone(), v_alpha.clone())?))? { let mut v_x: Py; 
v_x = t6353;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_obsv_gramian(mut v_A: Py, mut v_C: Py) -> R {
    let mut v_Cf: Py = Py::None;
    let mut v_W: Py = Py::None;
    let mut v_d: Py = Py::None;
    let t6413 = (f__ss_in(v_A.clone(), Py::None, v_C.clone(), Py::None)?).items()?;
    if t6413.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t6413[0].clone();
    v_C = t6413[1].clone();
    let _ = f__need(f__is_hurwitz(v_A.clone())?, Py::str("A must be stable for the infinite-horizon Gramian"))?;
    v_Cf = f__frac(v_C.clone())?;
    v_W = f__lyap(f__frac(v_A.clone())?, { let mut out = vec![]; for t6560 in py::range_iter(&[(py::len(&v_C.clone())?).i()?]) { let mut v_i: Py; 
v_i = t6560;
out.push({ let mut out = vec![]; for t6561 in py::range_iter(&[(py::len(&v_C.clone())?).i()?]) { let mut v_j: Py; 
v_j = t6561;
out.push(py::mul(&py::getitem(&v_Cf.clone(), &v_i.clone())?, &py::getitem(&v_Cf.clone(), &v_j.clone())?)?); } Py::list(out) }); } Py::list(out) }, Py::Bool(true))?;
    v_d = f__det(v_W.clone())?;
    return Ok(Py::dict(vec![(Py::str("Wo"), { let mut out = vec![]; for t6577 in py::iter(&(v_W.clone()))? { let mut v_r: Py; 
v_r = t6577;
out.push({ let mut out = vec![]; for t6578 in py::iter(&(v_r.clone()))? { let mut v_x: Py; 
v_x = t6578;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }); } Py::list(out) }), (Py::str("observable"), { let t6579 = v_d.clone(); Py::Bool({ let t6580 = Py::Int(0i64); if py::ne(&t6579, &t6580) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pade(mut v_theta: Py, mut v_n: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lead: Py = Py::None;
    let mut v_num: Py = Py::None;
    v_theta = f__pos(v_theta.clone(), Py::str("theta"))?;
    v_n = py::int(&v_n.clone())?;
    let _ = f__need({ let t6617 = Py::Int(1i64); Py::Bool({ let t6618 = v_n.clone(); if py::le(&t6617, &t6618)? { { let t6619 = Py::Int(6i64); if py::le(&t6618, &t6619)? { true } else { false } } } else { false } }) }, Py::str("the order must be between 1 and 6"))?;
    v_c = Py::list(vec![]);
    for t6620 in py::range_iter(&[(py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t6620;
        let _ = py::method(&v_c.clone(), "append", &[py::fraction(&[py::mul(&py::factorial(&py::sub(&py::mul(&Py::Int(2i64), &v_n.clone())?, &v_k.clone())?)?, &py::factorial(&v_n.clone())?)?, py::mul(&py::mul(&py::factorial(&py::mul(&Py::Int(2i64), &v_n.clone())?)?, &py::factorial(&v_k.clone())?)?, &py::factorial(&py::sub(&v_n.clone(), &v_k.clone())?)?)?])?])?;
    }
    v_den = { let mut out = vec![]; for t6621 in py::range_iter(&[(v_n.clone()).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t6621;
out.push(py::mul(&py::float(&py::getitem(&v_c.clone(), &v_k.clone())?)?, &py::pow(&v_theta.clone(), &v_k.clone())?)?); } Py::list(out) };
    v_num = { let mut out = vec![]; for t6622 in py::range_iter(&[(v_n.clone()).i()?, (py::neg(&Py::Int(1i64))?).i()?, (py::neg(&Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t6622;
out.push(py::mul(&py::float(&py::getitem(&v_c.clone(), &v_k.clone())?)?, &py::pow(&py::neg(&v_theta.clone())?, &v_k.clone())?)?); } Py::list(out) };
    v_lead = py::getitem(&v_den.clone(), &Py::Int(0i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("num"), { let mut out = vec![]; for t6632 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t6632;
out.push(py::truediv(&v_x.clone(), &v_lead.clone())?); } Py::list(out) }), (Py::str("den"), { let mut out = vec![]; for t6633 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t6633;
out.push(py::truediv(&v_x.clone(), &v_lead.clone())?); } Py::list(out) })]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_parallel_connection(mut v_num1: Py, mut v_den1: Py, mut v_num2: Py, mut v_den2: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t6690 = (f__tf_in(v_num1.clone(), v_den1.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t6690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t6690[0].clone();
    v_d1 = t6690[1].clone();
    let t6691 = (f__tf_in(v_num2.clone(), v_den2.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t6691.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n2 = t6691[0].clone();
    v_d2 = t6691[1].clone();
    v_num = f__ptrim(f__padd(f__pmul(v_n1.clone(), v_d2.clone())?, f__pmul(v_n2.clone(), v_d1.clone())?)?)?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t6700 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t6700;
out.push({ let t6701 = v_x.clone(); Py::Bool({ let t6702 = Py::Int(0i64); if py::ne(&t6701, &t6702) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())), Py::str("the two blocks cancel: the sum is zero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("num"), v_num.clone()), (Py::str("den"), f__pmul(v_d1.clone(), v_d2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_peak_time(mut v_zeta: Py, mut v_wn: Py) -> R {
    let mut v_wd: Py = Py::None;
    let t6750 = (Py::tuple(vec![f__nonneg(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?])).items()?;
    if t6750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t6750[0].clone();
    v_wn = t6750[1].clone();
    let _ = f__need({ let t6751 = v_zeta.clone(); Py::Bool({ let t6752 = Py::Int(1i64); if py::lt(&t6751, &t6752)? { true } else { false } }) }, Py::str("a peak needs an underdamped system: 0 <= zeta < 1"))?;
    v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Tp"), py::truediv(&c_PI()?, &v_wd.clone())?), (Py::str("wd"), v_wd.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_percent_overshoot(mut v_zeta: Py) -> R {
    let mut v_mp: Py = Py::None;
    v_zeta = f__nonneg(v_zeta.clone(), Py::str("zeta"))?;
    let _ = f__need({ let t6798 = v_zeta.clone(); Py::Bool({ let t6799 = Py::Int(1i64); if py::lt(&t6798, &t6799)? { true } else { false } }) }, Py::str("overshoot needs an underdamped system: 0 <= zeta < 1"))?;
    v_mp = py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_zeta.clone())?, &c_PI()?)?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("PO"), py::mul(&Py::Int(100i64), &v_mp.clone())?), (Py::str("Mp"), v_mp.clone()), (Py::str("pole_angle_deg"), py::math1("degrees", &py::math1("acos", &v_zeta.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pi_pole_placement(mut v_K: Py, mut v_tau: Py, mut v_zeta: Py, mut v_wn: Py) -> R {
    let t6845 = (Py::tuple(vec![f__num(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__pos(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?])).items()?;
    if t6845.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_K = t6845[0].clone();
    v_tau = t6845[1].clone();
    v_zeta = t6845[2].clone();
    v_wn = t6845[3].clone();
    let _ = f__need({ let t6846 = v_K.clone(); Py::Bool({ let t6847 = Py::Int(0i64); if py::ne(&t6846, &t6847) { true } else { false } }) }, Py::str("K must not be zero"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Kp"), py::truediv(&py::sub(&py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &v_wn.clone())?, &v_tau.clone())?, &Py::Int(1i64))?, &v_K.clone())?), (Py::str("Ki"), py::truediv(&py::mul(&py::mul(&v_wn.clone(), &v_wn.clone())?, &v_tau.clone())?, &v_K.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pid_closed_loop(mut v_num: Py, mut v_den: Py, mut v_Kp: Py, mut v_Ki: Py, mut v_Kd: Py) -> R {
    let mut v_char: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let t6913 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(true), Py::Int(9i64))?).items()?;
    if t6913.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t6913[0].clone();
    v_den = t6913[1].clone();
    let t6914 = (Py::tuple(vec![f__num(v_Kp.clone(), Py::str("Kp"))?, f__num(v_Ki.clone(), Py::str("Ki"))?, f__num(v_Kd.clone(), Py::str("Kd"))?])).items()?;
    if t6914.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Kp = t6914[0].clone();
    v_Ki = t6914[1].clone();
    v_Kd = t6914[2].clone();
    if ({ let t6915 = v_Ki.clone(); Py::Bool({ let t6916 = Py::Int(0i64); if py::ne(&t6915, &t6916) { true } else { false } }) }).truthy() {
        v_char = f__padd(f__pmul(v_den.clone(), Py::list(vec![Py::Float(1.0), Py::Float(0.0)]))?, f__pmul(Py::list(vec![v_Kd.clone(), v_Kp.clone(), v_Ki.clone()]), v_num.clone())?)?;
    } else {
        v_char = f__padd(v_den.clone(), f__pmul(Py::list(vec![v_Kd.clone(), v_Kp.clone()]), v_num.clone())?)?;
    }
    v_char = f__ptrim(v_char.clone())?;
    let _ = f__need({ let t6925 = py::len(&v_char.clone())?; Py::Bool({ let t6926 = Py::Int(2i64); if py::ge(&t6925, &t6926)? { true } else { false } }) }, Py::str("the closed loop has no poles"))?;
    v_poles = f__roots(v_char.clone())?;
    v_mr = py::minmax(&[{ let mut out = vec![]; for t7008 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t7008;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("char_poly"), { let mut out = vec![]; for t7009 in py::iter(&(v_char.clone()))? { let mut v_x: Py; 
v_x = t7009;
out.push(f__r12(v_x.clone())?); } Py::list(out) }), (Py::str("poles"), v_poles.clone()), (Py::str("max_real_part"), f__r12(v_mr.clone())?), (Py::str("stable"), { let t7010 = v_mr.clone(); Py::Bool({ let t7011 = Py::Int(0i64); if py::lt(&t7010, &t7011)? { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pid_parallel_to_standard(mut v_Kp: Py, mut v_Ki: Py, mut v_Kd: Py) -> R {
    let t7050 = (Py::tuple(vec![f__pos(v_Kp.clone(), Py::str("Kp"))?, f__pos(v_Ki.clone(), Py::str("Ki"))?, f__nonneg(v_Kd.clone(), Py::str("Kd"))?])).items()?;
    if t7050.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Kp = t7050[0].clone();
    v_Ki = t7050[1].clone();
    v_Kd = t7050[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("Ti"), py::truediv(&v_Kp.clone(), &v_Ki.clone())?), (Py::str("Td"), py::truediv(&v_Kd.clone(), &v_Kp.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pid_standard_to_parallel(mut v_Kp: Py, mut v_Ti: Py, mut v_Td: Py) -> R {
    let t7098 = (Py::tuple(vec![f__num(v_Kp.clone(), Py::str("Kp"))?, f__pos(v_Ti.clone(), Py::str("Ti"))?, f__nonneg(v_Td.clone(), Py::str("Td"))?])).items()?;
    if t7098.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Kp = t7098[0].clone();
    v_Ti = t7098[1].clone();
    v_Td = t7098[2].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("Ki"), py::truediv(&v_Kp.clone(), &v_Ti.clone())?), (Py::str("Kd"), py::mul(&v_Kp.clone(), &v_Td.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pole_stability(mut v_den: Py) -> R {
    let mut v_poles: Py = Py::None;
    v_den = f__ptrim(f__vec(v_den.clone(), Py::str("den"), Py::Int(2i64), Py::Int(10i64))?)?;
    let _ = f__need({ let t7152 = py::len(&v_den.clone())?; Py::Bool({ let t7153 = Py::Int(2i64); if py::ge(&t7152, &t7153)? { true } else { false } }) }, Py::str("the polynomial needs degree 1 or more"))?;
    v_poles = f__roots(v_den.clone())?;
    return Ok(Py::dict(vec![(Py::str("poles"), v_poles.clone()), (Py::str("max_real_part"), f__r12(py::minmax(&[{ let mut out = vec![]; for t7235 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t7235;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?)?), (Py::str("stability"), f__classify(v_poles.clone())?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_poles_zeros(mut v_num: Py, mut v_den: Py) -> R {
    let t7306 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t7306.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t7306[0].clone();
    v_den = t7306[1].clone();
    return Ok(Py::dict(vec![(Py::str("zeros"), f__roots(v_num.clone())?), (Py::str("poles"), f__roots(v_den.clone())?), (Py::str("gain"), f__r12(py::truediv(&py::getitem(&v_num.clone(), &Py::Int(0i64))?, &py::getitem(&v_den.clone(), &Py::Int(0i64))?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_resonant_peak(mut v_zeta: Py, mut v_wn: Py) -> R {
    let mut v_mr: Py = Py::None;
    let t7424 = (Py::tuple(vec![f__pos(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?])).items()?;
    if t7424.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t7424[0].clone();
    v_wn = t7424[1].clone();
    let _ = f__need({ let t7425 = v_zeta.clone(); Py::Bool({ let t7426 = py::truediv(&Py::Int(1i64), &py::math1("sqrt", &Py::Int(2i64))?)?; if py::lt(&t7425, &t7426)? { true } else { false } }) }, Py::str("there is no resonant peak unless zeta < 0.7071"))?;
    v_mr = py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Mr"), v_mr.clone()), (Py::str("Mr_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_mr.clone())?)?), (Py::str("wr"), py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &v_zeta.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_rise_time_second(mut v_zeta: Py, mut v_wn: Py) -> R {
    let mut v_t10: Py = Py::None;
    let mut v_t90: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let t7472 = (Py::tuple(vec![f__num(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?])).items()?;
    if t7472.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t7472[0].clone();
    v_wn = t7472[1].clone();
    let _ = f__need({ let t7473 = Py::Int(0i64); Py::Bool({ let t7474 = v_zeta.clone(); if py::lt(&t7473, &t7474)? { { let t7475 = Py::Int(1i64); if py::lt(&t7474, &t7475)? { true } else { false } } } else { false } }) }, Py::str("the rise-time formulas need an underdamped system: 0 < zeta < 1"))?;
    v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?;
    v_tp = py::truediv(&c_PI()?, &v_wd.clone())?;
    v_t10 = f__bisect({ let v_tp = v_tp.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__y2(v_zeta.clone(), v_wn.clone(), v_t.clone())?, &Py::Float(0.1))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_tp.clone())?;
    v_t90 = f__bisect({ let v_t10 = v_t10.clone(); let v_tp = v_tp.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__y2(v_zeta.clone(), v_wn.clone(), v_t.clone())?, &Py::Float(0.9))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_tp.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Tr_0_100"), py::truediv(&py::sub(&c_PI()?, &py::math1("acos", &v_zeta.clone())?)?, &v_wd.clone())?), (Py::str("Tr_10_90"), py::sub(&v_t90.clone(), &v_t10.clone())?), (Py::str("t10"), v_t10.clone()), (Py::str("t90"), v_t90.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_routh_hurwitz(mut v_coefficients: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_changes: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_first: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_r0: Py = Py::None;
    let mut v_r1: Py = Py::None;
    let mut v_rows: Py = Py::None;
    let mut v_width: Py = Py::None;
    v_c = f__ptrim(f__vec(v_coefficients.clone(), Py::str("coefficients"), Py::Int(2i64), Py::Int(12i64))?)?;
    let _ = f__need({ let t7549 = py::len(&v_c.clone())?; Py::Bool({ let t7550 = Py::Int(2i64); if py::ge(&t7549, &t7550)? { true } else { false } }) }, Py::str("give a polynomial of degree 1 or more"))?;
    v_f = { let mut out = vec![]; for t7551 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t7551;
out.push(py::fraction(&[v_x.clone()])?); } Py::list(out) };
    v_n = py::sub(&py::len(&v_f.clone())?, &Py::Int(1i64))?;
    v_width = py::add(&py::floordiv(&v_n.clone(), &Py::Int(2i64))?, &Py::Int(1i64))?;
    v_r0 = py::add(&py::slice(&v_f.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_f.clone(), Some((Py::Int(0i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_r1 = py::add(&py::slice(&v_f.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?, &py::mul(&Py::list(vec![py::fraction(&[Py::Int(0i64)])?]), &py::sub(&v_width.clone(), &py::len(&py::slice(&v_f.clone(), Some((Py::Int(1i64)).i()?), None, Some((Py::Int(2i64)).i()?))?)?)?)?)?;
    v_rows = Py::list(vec![v_r0.clone(), v_r1.clone()]);
    for t7552 in py::range_iter(&[(py::sub(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v__ = t7552;
        let t7553 = (Py::tuple(vec![py::getitem(&v_rows.clone(), &py::neg(&Py::Int(2i64))?)?, py::getitem(&v_rows.clone(), &py::neg(&Py::Int(1i64))?)?])).items()?;
        if t7553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_a = t7553[0].clone();
        v_b = t7553[1].clone();
        let _ = f__need({ let t7554 = py::getitem(&v_b.clone(), &Py::Int(0i64))?; Py::Bool({ let t7555 = Py::Int(0i64); if py::ne(&t7554, &t7555) { true } else { false } }) }, Py::str("a zero appears in the first column of the Routh array (a special case: roots on or symmetric about the imaginary axis); this capability does not handle it"))?;
        let _ = py::method(&v_rows.clone(), "append", &[py::add(&{ let mut out = vec![]; for t7556 in py::range_iter(&[(py::sub(&v_width.clone(), &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t7556;
out.push(py::truediv(&py::sub(&py::mul(&py::getitem(&v_b.clone(), &Py::Int(0i64))?, &py::getitem(&v_a.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &py::mul(&py::getitem(&v_a.clone(), &Py::Int(0i64))?, &py::getitem(&v_b.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?)?, &py::getitem(&v_b.clone(), &Py::Int(0i64))?)?); } Py::list(out) }, &Py::list(vec![py::fraction(&[Py::Int(0i64)])?]))?])?;
    }
    v_first = { let mut out = vec![]; for t7557 in py::iter(&(v_rows.clone()))? { let mut v_r: Py; 
v_r = t7557;
out.push(py::getitem(&v_r.clone(), &Py::Int(0i64))?); } Py::list(out) };
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t7558 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t7558;
out.push({ let t7559 = v_x.clone(); Py::Bool({ let t7560 = Py::Int(0i64); if py::ne(&t7559, &t7560) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("a zero appears in the first column of the Routh array (a special case)"))?;
    v_changes = py::sum(&{ let mut out = vec![]; for t7561 in py::iter(&(py::zip(&[v_first.clone(), py::slice(&v_first.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_x: Py; let mut v_y: Py; 
let t7562 = (t7561).items()?;
if t7562.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t7562[0].clone();
v_y = t7562[1].clone();
if ({ let t7563 = { let t7565 = v_x.clone(); Py::Bool({ let t7566 = Py::Int(0i64); if py::gt(&t7565, &t7566)? { true } else { false } }) }; Py::Bool({ let t7564 = { let t7567 = v_y.clone(); Py::Bool({ let t7568 = Py::Int(0i64); if py::gt(&t7567, &t7568)? { true } else { false } }) }; if py::ne(&t7563, &t7564) { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    return Ok(Py::dict(vec![(Py::str("first_column"), { let mut out = vec![]; for t7569 in py::iter(&(v_first.clone()))? { let mut v_x: Py; 
v_x = t7569;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }), (Py::str("rhp_roots"), v_changes.clone()), (Py::str("stable"), { let t7570 = v_changes.clone(); Py::Bool({ let t7571 = Py::Int(0i64); if py::eq(&t7570, &t7571) { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_s_to_z_pole(mut v_s_re: Py, mut v_s_im: Py, mut v_T: Py) -> R {
    let mut v_r: Py = Py::None;
    let mut v_th: Py = Py::None;
    let t7610 = (Py::tuple(vec![f__num(v_s_re.clone(), Py::str("s_re"))?, f__nonneg(v_s_im.clone(), Py::str("s_im"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t7610.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s_re = t7610[0].clone();
    v_s_im = t7610[1].clone();
    v_T = t7610[2].clone();
    let _ = f__need({ let t7611 = py::mul(&v_s_im.clone(), &v_T.clone())?; Py::Bool({ let t7612 = c_PI()?; if py::lt(&t7611, &t7612)? { true } else { false } }) }, Py::str("the pole's frequency is above the Nyquist limit (s_im T must be below pi)"))?;
    v_r = py::math1("exp", &py::mul(&v_s_re.clone(), &v_T.clone())?)?;
    v_th = py::mul(&v_s_im.clone(), &v_T.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("z_re"), py::mul(&v_r.clone(), &py::math1("cos", &v_th.clone())?)?), (Py::str("z_im"), py::mul(&v_r.clone(), &py::math1("sin", &v_th.clone())?)?), (Py::str("z_mag"), v_r.clone()), (Py::str("z_angle"), v_th.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_scalar_lqr(mut v_a: Py, mut v_b: Py, mut v_q: Py, mut v_r: Py) -> R {
    let mut v_K: Py = Py::None;
    let mut v_P: Py = Py::None;
    let mut v_root: Py = Py::None;
    let t7658 = (Py::tuple(vec![f__num(v_a.clone(), Py::str("a"))?, f__num(v_b.clone(), Py::str("b"))?, f__pos(v_q.clone(), Py::str("q"))?, f__pos(v_r.clone(), Py::str("r"))?])).items()?;
    if t7658.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_a = t7658[0].clone();
    v_b = t7658[1].clone();
    v_q = t7658[2].clone();
    v_r = t7658[3].clone();
    let _ = f__need({ let t7659 = v_b.clone(); Py::Bool({ let t7660 = Py::Int(0i64); if py::ne(&t7659, &t7660) { true } else { false } }) }, Py::str("b must not be zero"))?;
    v_root = py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &py::truediv(&py::mul(&py::mul(&v_b.clone(), &v_b.clone())?, &v_q.clone())?, &v_r.clone())?)?)?;
    v_P = py::truediv(&v_q.clone(), &py::sub(&v_root.clone(), &v_a.clone())?)?;
    v_K = py::truediv(&py::mul(&v_b.clone(), &v_P.clone())?, &v_r.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("P"), v_P.clone()), (Py::str("K"), v_K.clone()), (Py::str("pole"), py::neg(&v_root.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_freq(mut v_zeta: Py, mut v_wn: Py, mut v_w: Py) -> R {
    let mut v_im: Py = Py::None;
    let mut v_mag: Py = Py::None;
    let mut v_re_: Py = Py::None;
    let t7708 = (Py::tuple(vec![f__nonneg(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?, f__pos(v_w.clone(), Py::str("w"))?])).items()?;
    if t7708.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t7708[0].clone();
    v_wn = t7708[1].clone();
    v_w = t7708[2].clone();
    let t7709 = (Py::tuple(vec![py::sub(&py::mul(&v_wn.clone(), &v_wn.clone())?, &py::mul(&v_w.clone(), &v_w.clone())?)?, py::mul(&py::mul(&py::mul(&Py::Int(2i64), &v_zeta.clone())?, &v_wn.clone())?, &v_w.clone())?])).items()?;
    if t7709.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_re_ = t7709[0].clone();
    v_im = t7709[1].clone();
    let _ = f__need({ let t7714 = { let t7710 = v_re_.clone(); Py::Bool({ let t7711 = Py::Int(0i64); if py::ne(&t7710, &t7711) { true } else { false } }) }; if t7714.truthy() { t7714 } else { { let t7712 = v_im.clone(); Py::Bool({ let t7713 = Py::Int(0i64); if py::ne(&t7712, &t7713) { true } else { false } }) } } }, Py::str("the frequency is at an undamped resonance"))?;
    v_mag = py::truediv(&py::mul(&v_wn.clone(), &v_wn.clone())?, &py::hypot(&[v_re_.clone(), v_im.clone()])?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("magnitude"), v_mag.clone()), (Py::str("magnitude_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_mag.clone())?)?), (Py::str("phase_deg"), py::neg(&py::math1("degrees", &py::atan2(&v_im.clone(), &v_re_.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_from_poles(mut v_sigma: Py, mut v_wd: Py) -> R {
    let mut v_wn: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    let t7760 = (Py::tuple(vec![f__pos(v_sigma.clone(), Py::str("sigma"))?, f__pos(v_wd.clone(), Py::str("wd"))?])).items()?;
    if t7760.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sigma = t7760[0].clone();
    v_wd = t7760[1].clone();
    v_wn = py::math1("sqrt", &py::add(&py::mul(&v_sigma.clone(), &v_sigma.clone())?, &py::mul(&v_wd.clone(), &v_wd.clone())?)?)?;
    v_zeta = py::truediv(&v_sigma.clone(), &v_wn.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("wn"), v_wn.clone()), (Py::str("zeta"), v_zeta.clone()), (Py::str("Tp"), py::truediv(&c_PI()?, &v_wd.clone())?), (Py::str("Ts"), py::truediv(&Py::Float(4.0), &v_sigma.clone())?), (Py::str("PO"), py::mul(&Py::Int(100i64), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_sigma.clone())?, &c_PI()?)?, &v_wd.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_params(mut v_den: Py, mut v_K: Py) -> R {
    let mut v_a0: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    v_d = f__vec(v_den.clone(), Py::str("den"), Py::Int(3i64), Py::Int(3i64))?;
    let t7808 = (v_d.clone()).items()?;
    if t7808.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a2 = t7808[0].clone();
    v_a1 = t7808[1].clone();
    v_a0 = t7808[2].clone();
    let _ = f__need({ let t7809 = v_a2.clone(); Py::Bool({ let t7810 = Py::Int(0i64); if py::ne(&t7809, &t7810) { true } else { false } }) }, Py::str("the s^2 coefficient must not be zero"))?;
    let t7811 = (Py::tuple(vec![py::truediv(&v_a1.clone(), &v_a2.clone())?, py::truediv(&v_a0.clone(), &v_a2.clone())?])).items()?;
    if t7811.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t7811[0].clone();
    v_a0 = t7811[1].clone();
    let _ = f__need({ let t7816 = { let t7812 = v_a0.clone(); Py::Bool({ let t7813 = Py::Int(0i64); if py::gt(&t7812, &t7813)? { true } else { false } }) }; if !t7816.truthy() { t7816 } else { { let t7814 = v_a1.clone(); Py::Bool({ let t7815 = Py::Int(0i64); if py::ge(&t7814, &t7815)? { true } else { false } }) } } }, Py::str("the polynomial must be s^2 + 2 zeta wn s + wn^2 with wn^2 > 0 and zeta >= 0"))?;
    v_wn = py::math1("sqrt", &v_a0.clone())?;
    v_zeta = py::truediv(&v_a1.clone(), &py::mul(&Py::Int(2i64), &v_wn.clone())?)?;
    if ({ let t7817 = py::mul(&v_a1.clone(), &v_a1.clone())?; Py::Bool({ let t7818 = py::mul(&Py::Int(4i64), &v_a0.clone())?; if py::eq(&t7817, &t7818) { true } else { false } }) }).truthy() {
        v_zeta = Py::Float(1.0);
    }
    v_wd = (if ({ let t7819 = v_zeta.clone(); Py::Bool({ let t7820 = Py::Int(1i64); if py::lt(&t7819, &t7820)? { true } else { false } }) }).truthy() { py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)? } else { Py::Float(0.0) });
    v_out = Py::dict(vec![(Py::str("wn"), v_wn.clone()), (Py::str("zeta"), v_zeta.clone()), (Py::str("sigma"), py::mul(&v_zeta.clone(), &v_wn.clone())?), (Py::str("wd"), v_wd.clone()), (Py::str("damping"), f__class(v_zeta.clone())?)]);
    py::setitem(&v_out.clone(), &Py::str("dc_gain"), (if ({ let t7827 = v_K.clone(); Py::Bool({ let t7828 = Py::None; if !py::is(&t7827, &t7828) { true } else { false } }) }).truthy() { py::truediv(&f__num(v_K.clone(), Py::str("K"))?, &py::getitem(&v_d.clone(), &Py::Int(2i64))?)? } else { Py::Float(1.0) }))?;
    return Ok(f__rnd(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_second_order_step(mut v_zeta: Py, mut v_wn: Py, mut v_t: Py, mut v_K: Py) -> R {
    let mut v_y: Py = Py::None;
    let t7876 = (Py::tuple(vec![f__nonneg(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?, f__nonneg(v_t.clone(), Py::str("t"))?, f__num(v_K.clone(), Py::str("K"))?])).items()?;
    if t7876.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t7876[0].clone();
    v_wn = t7876[1].clone();
    v_t = t7876[2].clone();
    v_K = t7876[3].clone();
    let _ = f__need({ let t7877 = v_K.clone(); Py::Bool({ let t7878 = Py::Int(0i64); if py::ne(&t7877, &t7878) { true } else { false } }) }, Py::str("K must not be zero"))?;
    v_y = f__y2(v_zeta.clone(), v_wn.clone(), v_t.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("y"), py::mul(&v_K.clone(), &v_y.clone())?), (Py::str("error"), py::mul(&v_K.clone(), &py::sub(&Py::Int(1i64), &v_y.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sensitivity_peak(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_P: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_best_w: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t7951 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(true), Py::Int(9i64))?).items()?;
    if t7951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t7951[0].clone();
    v_den = t7951[1].clone();
    v_cl = f__padd(v_den.clone(), v_num.clone())?;
    let _ = f__need(f__routh_ok(v_cl.clone())?, Py::str("the closed loop must be stable"))?;
    let t7991 = (Py::tuple(vec![f__mag2(v_den.clone())?, f__mag2(v_cl.clone())?])).items()?;
    if t7991.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_f = t7991[0].clone();
    v_g = t7991[1].clone();
    v_P = f__ptrim(f__psub(f__pmul(f__pder(v_f.clone())?, v_g.clone())?, f__pmul(v_f.clone(), f__pder(v_g.clone())?)?)?)?;
    v_cands = Py::list(vec![Py::Float(0.0)]);
    if ({ let t8001 = { let t7996 = py::len(&v_P.clone())?; Py::Bool({ let t7997 = Py::Int(2i64); if py::ge(&t7996, &t7997)? { true } else { false } }) }; if !t8001.truthy() { t8001 } else { Py::Bool(({ let mut out = vec![]; for t7998 in py::iter(&(v_P.clone()))? { let mut v_x: Py; 
v_x = t7998;
out.push({ let t7999 = v_x.clone(); Py::Bool({ let t8000 = Py::Int(0i64); if py::ne(&t7999, &t8000) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        v_cands = py::iadd(&v_cands, &f__real_pos_roots(v_P.clone(), Py::Float(1e-07))?)?;
    }
    let t8082 = (Py::tuple(vec![Py::Float(0.0), py::neg(&Py::Float(1.0))?])).items()?;
    if t8082.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_best_w = t8082[0].clone();
    v_best = t8082[1].clone();
    for t8083 in py::iter(&(v_cands.clone()))? {
        v_w = t8083;
        v_v = py::math1("sqrt", &py::truediv(&f__pval(v_f.clone(), v_w.clone())?, &f__pval(v_g.clone(), v_w.clone())?)?)?;
        if ({ let t8084 = v_v.clone(); Py::Bool({ let t8085 = v_best.clone(); if py::gt(&t8084, &t8085)? { true } else { false } }) }).truthy() {
            let t8086 = (Py::tuple(vec![v_v.clone(), v_w.clone()])).items()?;
            if t8086.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_best = t8086[0].clone();
            v_best_w = t8086[1].clone();
        }
    }
    let _ = f__need({ let t8087 = v_best.clone(); Py::Bool({ let t8088 = py::add(&Py::Int(1i64), &Py::Float(1e-09))?; if py::gt(&t8087, &t8088)? { true } else { false } }) }, Py::str("|S(jw)| never exceeds 1: its supremum is approached only as w -> infinity"))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Ms"), v_best.clone()), (Py::str("Ms_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_best.clone())?)?), (Py::str("w_ms"), v_best_w.clone()), (Py::str("sm"), py::truediv(&Py::Float(1.0), &v_best.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_series_connection(mut v_num1: Py, mut v_den1: Py, mut v_num2: Py, mut v_den2: Py) -> R {
    let mut v_d1: Py = Py::None;
    let mut v_d2: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let t8154 = (f__tf_in(v_num1.clone(), v_den1.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t8154.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t8154[0].clone();
    v_d1 = t8154[1].clone();
    let t8155 = (f__tf_in(v_num2.clone(), v_den2.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t8155.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n2 = t8155[0].clone();
    v_d2 = t8155[1].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("num"), f__pmul(v_n1.clone(), v_n2.clone())?), (Py::str("den"), f__pmul(v_d1.clone(), v_d2.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_settling_time(mut v_zeta: Py, mut v_wn: Py, mut v_band: Py) -> R {
    let mut v_env: Py = Py::None;
    let mut v_rule: Py = Py::None;
    let mut v_s: Py = Py::None;
    let t8207 = (Py::tuple(vec![f__num(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?, f__num(v_band.clone(), Py::str("band"))?])).items()?;
    if t8207.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t8207[0].clone();
    v_wn = t8207[1].clone();
    v_band = t8207[2].clone();
    let _ = f__need({ let t8208 = Py::Int(0i64); Py::Bool({ let t8209 = v_zeta.clone(); if py::lt(&t8208, &t8209)? { { let t8210 = Py::Int(1i64); if py::lt(&t8209, &t8210)? { true } else { false } } } else { false } }) }, Py::str("the settling-time rule is for an underdamped system: 0 < zeta < 1"))?;
    let _ = f__need({ let t8211 = v_band.clone(); Py::Bool({ let t8212 = Py::tuple(vec![Py::Float(0.02), Py::Float(0.05)]); if py::contains(&t8212, &t8211)? { true } else { false } }) }, Py::str("the rule of thumb is stated for a 2 % or a 5 % band"))?;
    v_s = py::mul(&v_zeta.clone(), &v_wn.clone())?;
    v_rule = py::truediv(&(if ({ let t8213 = v_band.clone(); Py::Bool({ let t8214 = Py::Float(0.02); if py::eq(&t8213, &t8214) { true } else { false } }) }).truthy() { Py::Float(4.0) } else { Py::Float(3.0) }), &v_s.clone())?;
    v_env = py::truediv(&py::neg(&py::math1("log", &py::mul(&v_band.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?)?)?, &v_s.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Ts"), v_rule.clone()), (Py::str("Ts_envelope"), v_env.clone()), (Py::str("time_constant"), py::truediv(&Py::Float(1.0), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_simc_pi(mut v_K: Py, mut v_tau: Py, mut v_theta: Py, mut v_tau_c: Py) -> R {
    let mut v_Kp: Py = Py::None;
    let mut v_Ti: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let t8260 = (Py::tuple(vec![f__pos(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__pos(v_theta.clone(), Py::str("theta"))?])).items()?;
    if t8260.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t8260[0].clone();
    v_tau = t8260[1].clone();
    v_theta = t8260[2].clone();
    v_tc = (if ({ let t8261 = v_tau_c.clone(); Py::Bool({ let t8262 = Py::None; if py::is(&t8261, &t8262) { true } else { false } }) }).truthy() { v_theta.clone() } else { f__pos(v_tau_c.clone(), Py::str("tau_c"))? });
    v_Kp = py::truediv(&v_tau.clone(), &py::mul(&v_K.clone(), &py::add(&v_tc.clone(), &v_theta.clone())?)?)?;
    v_Ti = py::minmax(&[v_tau.clone(), py::mul(&Py::Int(4i64), &py::add(&v_tc.clone(), &v_theta.clone())?)?], false)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Kp"), v_Kp.clone()), (Py::str("Ti"), v_Ti.clone()), (Py::str("Ki"), py::truediv(&v_Kp.clone(), &v_Ti.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_specs_to_poles(mut v_OS: Py, mut v_Ts: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_mp: Py = Py::None;
    let mut v_sigma: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let mut v_zeta: Py = Py::None;
    let t8308 = (Py::tuple(vec![f__num(v_OS.clone(), Py::str("OS"))?, f__pos(v_Ts.clone(), Py::str("Ts"))?])).items()?;
    if t8308.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mp = t8308[0].clone();
    v_Ts = t8308[1].clone();
    let _ = f__need({ let t8309 = Py::Int(0i64); Py::Bool({ let t8310 = v_mp.clone(); if py::lt(&t8309, &t8310)? { { let t8311 = Py::Int(1i64); if py::lt(&t8310, &t8311)? { true } else { false } } } else { false } }) }, Py::str("the overshoot must lie strictly between 0 and 100 %"))?;
    v_L = py::math1("log", &v_mp.clone())?;
    v_zeta = py::truediv(&py::neg(&v_L.clone())?, &py::math1("sqrt", &py::add(&py::mul(&c_PI()?, &c_PI()?)?, &py::mul(&v_L.clone(), &v_L.clone())?)?)?)?;
    v_sigma = py::truediv(&Py::Float(4.0), &v_Ts.clone())?;
    v_wn = py::truediv(&v_sigma.clone(), &v_zeta.clone())?;
    v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("zeta"), v_zeta.clone()), (Py::str("wn"), v_wn.clone()), (Py::str("sigma"), v_sigma.clone()), (Py::str("wd"), v_wd.clone()), (Py::str("char_poly"), Py::list(vec![Py::Float(1.0), py::mul(&Py::Int(2i64), &v_sigma.clone())?, py::mul(&v_wn.clone(), &v_wn.clone())?]))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_dc_gain(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py) -> R {
    let mut v_x: Py = Py::None;
    let t8380 = (f__ss_in(v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone())?).items()?;
    if t8380.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t8380[0].clone();
    v_B = t8380[1].clone();
    v_C = t8380[2].clone();
    v_D = t8380[3].clone();
    let t8410: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_x = f__solve(f__frac(v_A.clone())?, f__frac(v_B.clone())?, Py::Bool(true))?;
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8410 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (matches!(e, PyErr::Value(_))) => {
            return Err(py::value_error(py::to_str(&Py::str("A is singular (an eigenvalue at 0): the DC gain is infinite"))));
        }
        Err(e) => return Err(e),
    }
    return Ok(Py::dict(vec![(Py::str("dc_gain"), f__r12(py::float(&py::sub(&py::fraction(&[v_D.clone()])?, &py::sum(&{ let mut out = vec![]; for t8411 in py::iter(&(py::zip(&[f__frac(v_C.clone())?, v_x.clone()])?))? { let mut v_c: Py; let mut v_v: Py; 
let t8412 = (t8411).items()?;
if t8412.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_c = t8412[0].clone();
v_v = t8412[1].clone();
out.push(py::mul(&v_c.clone(), &v_v.clone())?); } Py::list(out) }, None)?)?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_eigenvalues(mut v_A: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let t8472 = (f__ss_in(v_A.clone(), Py::None, Py::None, Py::None)?).items()?;
    if t8472.len() != 1 { return Err(PyErr::Value("unpack".into())); }
    v_A = t8472[0].clone();
    let t8493 = (f__faddeev(f__frac(v_A.clone())?)?).items()?;
    if t8493.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t8493[0].clone();
    v__ = t8493[1].clone();
    v_cf = { let mut out = vec![]; for t8494 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t8494;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    v_ev = f__roots(v_cf.clone())?;
    v_mr = py::minmax(&[{ let mut out = vec![]; for t8582 in py::iter(&(v_ev.clone()))? { let mut v_x: Py; 
v_x = t8582;
out.push(py::real(&py::complex(&[v_x.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("eigenvalues"), v_ev.clone()), (Py::str("char_poly"), { let mut out = vec![]; for t8583 in py::iter(&(v_cf.clone()))? { let mut v_x: Py; 
v_x = t8583;
out.push(f__r12(v_x.clone())?); } Py::list(out) }), (Py::str("max_real_part"), f__r12(v_mr.clone())?), (Py::str("stable"), { let t8584 = v_mr.clone(); Py::Bool({ let t8585 = Py::Int(0i64); if py::lt(&t8584, &t8585)? { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_response(mut v_A: Py, mut v_B: Py, mut v_x0: Py, mut v_u: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_x: Py = Py::None;
    let t8645 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t8645.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t8645[0].clone();
    v_B = t8645[1].clone();
    v_n = py::len(&v_A.clone())?;
    v_x0 = f__vec(v_x0.clone(), Py::str("x0"), v_n.clone(), v_n.clone())?;
    let t8648 = (Py::tuple(vec![f__num(v_u.clone(), Py::str("u"))?, f__nonneg(v_t.clone(), Py::str("t"))?])).items()?;
    if t8648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_u = t8648[0].clone();
    v_t = t8648[1].clone();
    let _ = f__need({ let t8649 = py::mul(&f__mnorm1(v_A.clone())?, &v_t.clone())?; Py::Bool({ let t8650 = Py::Int(40i64); if py::le(&t8649, &t8650)? { true } else { false } }) }, Py::str("|A| t is too large for an accurate matrix exponential here (limit 40)"))?;
    v_M = py::add(&{ let mut out = vec![]; for t8653 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t8653;
out.push(py::add(&{ let mut out = vec![]; for t8654 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t8654;
out.push(py::mul(&py::getitem(&py::getitem(&v_A.clone(), &v_i.clone())?, &v_j.clone())?, &v_t.clone())?); } Py::list(out) }, &Py::list(vec![py::mul(&py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &v_u.clone())?, &v_t.clone())?]))?); } Py::list(out) }, &Py::list(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?]))?;
    v_E = f__expm(v_M.clone())?;
    v_x = { let mut out = vec![]; for t8675 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t8675;
out.push(py::add(&py::sum(&{ let mut out = vec![]; for t8676 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t8676;
out.push(py::mul(&py::getitem(&py::getitem(&v_E.clone(), &v_i.clone())?, &v_j.clone())?, &py::getitem(&v_x0.clone(), &v_j.clone())?)?); } Py::list(out) }, None)?, &py::getitem(&py::getitem(&v_E.clone(), &v_i.clone())?, &v_n.clone())?)?); } Py::list(out) };
    return Ok(Py::dict(vec![(Py::str("x"), f__rnd(f__snapm(v_x.clone(), Py::Float(1e-11))?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ss_to_tf(mut v_A: Py, mut v_B: Py, mut v_C: Py, mut v_D: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_num: Py = Py::None;
    let t8753 = (f__ss_in(v_A.clone(), v_B.clone(), v_C.clone(), v_D.clone())?).items()?;
    if t8753.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t8753[0].clone();
    v_B = t8753[1].clone();
    v_C = t8753[2].clone();
    v_D = t8753[3].clone();
    let t8781 = (f__ss2tf(f__frac(v_A.clone())?, f__frac(v_B.clone())?, f__frac(v_C.clone())?, py::fraction(&[v_D.clone()])?)?).items()?;
    if t8781.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t8781[0].clone();
    v_den = t8781[1].clone();
    return Ok(Py::dict(vec![(Py::str("num"), { let mut out = vec![]; for t8782 in py::iter(&(v_num.clone()))? { let mut v_x: Py; 
v_x = t8782;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) }), (Py::str("den"), { let mut out = vec![]; for t8783 in py::iter(&(v_den.clone()))? { let mut v_x: Py; 
v_x = t8783;
out.push(f__r12(py::float(&v_x.clone())?)?); } Py::list(out) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stability_margins(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_gc: Py = Py::None;
    let mut v_ggc: Py = Py::None;
    let mut v_gm: Py = Py::None;
    let mut v_gpc: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let t8840 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t8840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t8840[0].clone();
    v_den = t8840[1].clone();
    v_gc = f__gain_crossovers(v_num.clone(), v_den.clone())?;
    v_pc = f__phase_crossovers(v_num.clone(), v_den.clone())?;
    let _ = f__need({ let t8963 = py::len(&v_gc.clone())?; Py::Bool({ let t8964 = Py::Int(1i64); if py::eq(&t8963, &t8964) { true } else { false } }) }, (if (v_gc.clone()).truthy() { Py::str("the loop needs exactly one gain crossover (|L(jw)| = 1) for this capability") } else { Py::str("|L(jw)| never crosses 1: the phase margin is not defined") }))?;
    let _ = f__need({ let t8965 = py::len(&v_pc.clone())?; Py::Bool({ let t8966 = Py::Int(1i64); if py::eq(&t8965, &t8966) { true } else { false } }) }, (if (v_pc.clone()).truthy() { Py::str("the loop needs exactly one phase crossover for this capability") } else { Py::str("the phase never crosses -180 degrees: the gain margin is infinite") }))?;
    v_gpc = f__G(v_num.clone(), v_den.clone(), py::complex(&[Py::Float(0.0), py::getitem(&v_pc.clone(), &Py::Int(0i64))?])?)?;
    v_ggc = f__G(v_num.clone(), v_den.clone(), py::complex(&[Py::Float(0.0), py::getitem(&v_gc.clone(), &Py::Int(0i64))?])?)?;
    v_gm = py::truediv(&Py::Float(1.0), &py::hypot(&[py::real(&v_gpc.clone())?, py::imag(&v_gpc.clone())?])?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("gm"), v_gm.clone()), (Py::str("gm_db"), py::mul(&Py::Int(20i64), &py::math1("log10", &v_gm.clone())?)?), (Py::str("w_pc"), py::getitem(&v_pc.clone(), &Py::Int(0i64))?), (Py::str("pm_deg"), f__wrap180(py::add(&Py::Float(180.0), &f__deg(v_ggc.clone())?)?)?), (Py::str("w_gc"), py::getitem(&v_gc.clone(), &Py::Int(0i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_stable_gain_range(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_Di: Py = Py::None;
    let mut v_Dr: Py = Py::None;
    let mut v_Ni: Py = Py::None;
    let mut v_Nr: Py = Py::None;
    let mut v_cands: Py = Py::None;
    let mut v_dv: Py = Py::None;
    let mut v_edges: Py = Py::None;
    let mut v_hi_idx: Py = Py::None;
    let mut v_imp: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_k0: Py = Py::None;
    let mut v_ks: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_nv: Py = Py::None;
    let mut v_runs: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_stable: Py = Py::None;
    let mut v_tests: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t9036 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t9036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t9036[0].clone();
    v_den = t9036[1].clone();
    v_cands = Py::list(vec![]);
    if ({ let t9037 = py::getitem(&v_num.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t9038 = Py::Int(0i64); if py::ne(&t9037, &t9038) { true } else { false } }) }).truthy() {
        v_k0 = py::truediv(&py::neg(&py::getitem(&v_den.clone(), &py::neg(&Py::Int(1i64))?)?)?, &py::getitem(&v_num.clone(), &py::neg(&Py::Int(1i64))?)?)?;
        if ({ let t9039 = v_k0.clone(); Py::Bool({ let t9040 = Py::Int(0i64); if py::gt(&t9039, &t9040)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_cands.clone(), "append", &[Py::tuple(vec![v_k0.clone(), Py::Float(0.0)])])?;
        }
    }
    let t9057 = (f__jw(v_num.clone())?).items()?;
    if t9057.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Nr = t9057[0].clone();
    v_Ni = t9057[1].clone();
    let t9058 = (f__jw(v_den.clone())?).items()?;
    if t9058.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Dr = t9058[0].clone();
    v_Di = t9058[1].clone();
    v_imp = f__ptrim(f__psub(f__pmul(v_Di.clone(), v_Nr.clone())?, f__pmul(v_Dr.clone(), v_Ni.clone())?)?)?;
    if ({ let t9073 = { let t9068 = py::len(&v_imp.clone())?; Py::Bool({ let t9069 = Py::Int(2i64); if py::ge(&t9068, &t9069)? { true } else { false } }) }; if !t9073.truthy() { t9073 } else { Py::Bool(({ let mut out = vec![]; for t9070 in py::iter(&(v_imp.clone()))? { let mut v_x: Py; 
v_x = t9070;
out.push({ let t9071 = v_x.clone(); Py::Bool({ let t9072 = Py::Int(0i64); if py::ne(&t9071, &t9072) { true } else { false } }) }); } Py::list(out) }).items()?.iter().any(|x| x.truthy())) } }).truthy() {
        for t9157 in py::iter(&(f__real_pos_roots(v_imp.clone(), Py::Float(1e-07))?))? {
            v_w = t9157;
            v_s = py::complex(&[Py::Float(0.0), v_w.clone()])?;
            let t9158 = (Py::tuple(vec![f__pval(v_num.clone(), v_s.clone())?, f__pval(v_den.clone(), v_s.clone())?])).items()?;
            if t9158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_nv = t9158[0].clone();
            v_dv = t9158[1].clone();
            if ({ let t9159 = py::abs(&v_nv.clone())?; Py::Bool({ let t9160 = Py::Int(0i64); if py::eq(&t9159, &t9160) { true } else { false } }) }).truthy() {
                continue;
            }
            v_k = py::truediv(&py::neg(&py::real(&py::mul(&v_dv.clone(), &py::method(&v_nv.clone(), "conjugate", &[])?)?)?)?, &py::pow(&py::abs(&v_nv.clone())?, &Py::Int(2i64))?)?;
            if ({ let t9165 = { let t9161 = v_k.clone(); Py::Bool({ let t9162 = Py::Int(0i64); if py::gt(&t9161, &t9162)? { true } else { false } }) }; if !t9165.truthy() { t9165 } else { { let t9163 = py::abs(&py::add(&v_dv.clone(), &py::mul(&v_k.clone(), &v_nv.clone())?)?)?; Py::Bool({ let t9164 = py::mul(&Py::Float(1e-08), &py::add(&py::abs(&v_dv.clone())?, &py::abs(&py::mul(&v_k.clone(), &v_nv.clone())?)?)?)?; if py::le(&t9163, &t9164)? { true } else { false } }) } } }).truthy() {
                let _ = py::method(&v_cands.clone(), "append", &[Py::tuple(vec![v_k.clone(), v_w.clone()])])?;
            }
        }
    }
    let _ = py::method_sort(&v_cands.clone(), None, false)?;
    v_ks = Py::list(vec![]);
    for t9166 in py::iter(&(v_cands.clone()))? {
        let t9170 = (t9166).items()?;
        if t9170.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9170[0].clone();
        v_w = t9170[1].clone();
        if ({ let t9169 = v_ks.clone(); if !t9169.truthy() { t9169 } else { { let t9167 = py::abs(&py::sub(&v_k.clone(), &py::getitem(&py::getitem(&v_ks.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(0i64))?)?)?; Py::Bool({ let t9168 = py::mul(&Py::Float(1e-09), &v_k.clone())?; if py::le(&t9167, &t9168)? { true } else { false } }) } } }).truthy() {
            continue;
        }
        let _ = py::method(&v_ks.clone(), "append", &[Py::tuple(vec![v_k.clone(), v_w.clone()])])?;
    }
    v_edges = py::add(&Py::list(vec![Py::Float(0.0)]), &{ let mut out = vec![]; for t9171 in py::iter(&(v_ks.clone()))? { let mut v__: Py; let mut v_k: Py; 
let t9172 = (t9171).items()?;
if t9172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t9172[0].clone();
v__ = t9172[1].clone();
out.push(v_k.clone()); } Py::list(out) })?;
    v_tests = py::add(&{ let mut out = vec![]; for t9173 in py::range_iter(&[(py::sub(&py::len(&v_edges.clone())?, &Py::Int(1i64))?).i()?]) { let mut v_i: Py; 
v_i = t9173;
out.push(py::truediv(&py::add(&py::getitem(&v_edges.clone(), &v_i.clone())?, &py::getitem(&v_edges.clone(), &py::add(&v_i.clone(), &Py::Int(1i64))?)?)?, &Py::Int(2i64))?); } Py::list(out) }, &Py::list(vec![py::add(&py::mul(&py::getitem(&v_edges.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(2i64))?, &Py::Float(1.0))?]))?;
    v_stable = { let mut out = vec![]; for t9174 in py::iter(&(v_tests.clone()))? { let mut v_t: Py; 
v_t = t9174;
out.push(f__routh_ok(f__padd(v_den.clone(), f__pscale(v_num.clone(), v_t.clone())?)?)?); } Py::list(out) };
    v_runs = { let mut out = vec![]; for t9190 in py::iter(&(py::enumerate(&v_stable.clone(), 0)?))? { let mut v_i: Py; let mut v_s: Py; 
let t9191 = (t9190).items()?;
if t9191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t9191[0].clone();
v_s = t9191[1].clone();
if (v_s.clone()).truthy() { out.push(v_i.clone()); }} Py::list(out) };
    let _ = f__need({ let t9192 = py::len(&v_runs.clone())?; Py::Bool({ let t9193 = Py::Int(0i64); if py::gt(&t9192, &t9193)? { true } else { false } }) }, Py::str("the closed loop is unstable for every positive gain"))?;
    let _ = f__need({ let t9194 = v_runs.clone(); Py::Bool({ let t9195 = Py::list((Py::list((py::range_iter(&[(py::getitem(&v_runs.clone(), &Py::Int(0i64))?).i()?, (py::add(&py::getitem(&v_runs.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(1i64))?).i()?])).into_iter().collect::<Vec<Py>>())).items()?); if py::eq(&t9194, &t9195) { true } else { false } }) }, Py::str("the stable gains form more than one interval"))?;
    let _ = f__need({ let t9196 = py::getitem(&v_runs.clone(), &py::neg(&Py::Int(1i64))?)?; Py::Bool({ let t9197 = py::sub(&py::len(&v_tests.clone())?, &Py::Int(1i64))?; if py::ne(&t9196, &t9197) { true } else { false } }) }, Py::str("the closed loop is stable for every large gain (no upper limit)"))?;
    v_lo = py::getitem(&v_edges.clone(), &py::getitem(&v_runs.clone(), &Py::Int(0i64))?)?;
    v_hi_idx = py::add(&py::getitem(&v_runs.clone(), &py::neg(&Py::Int(1i64))?)?, &Py::Int(1i64))?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("K_min"), v_lo.clone()), (Py::str("K_max"), py::getitem(&v_edges.clone(), &v_hi_idx.clone())?), (Py::str("w_cross"), py::getitem(&py::getitem(&v_ks.clone(), &py::sub(&v_hi_idx.clone(), &Py::Int(1i64))?)?, &Py::Int(1i64))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_state_feedback_poles(mut v_A: Py, mut v_B: Py, mut v_K: Py) -> R {
    let mut v_Acl: Py = Py::None;
    let mut v_Af: Py = Py::None;
    let mut v_Bf: Py = Py::None;
    let mut v_Kf: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_cf: Py = Py::None;
    let mut v_mr: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_poles: Py = Py::None;
    let t9266 = (f__ss_in(v_A.clone(), v_B.clone(), Py::None, Py::None)?).items()?;
    if t9266.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_A = t9266[0].clone();
    v_B = t9266[1].clone();
    v_n = py::len(&v_A.clone())?;
    v_K = f__vec(v_K.clone(), Py::str("K"), v_n.clone(), v_n.clone())?;
    let t9270 = (Py::tuple(vec![f__frac(v_A.clone())?, f__frac(v_B.clone())?, f__frac(v_K.clone())?])).items()?;
    if t9270.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Af = t9270[0].clone();
    v_Bf = t9270[1].clone();
    v_Kf = t9270[2].clone();
    v_Acl = { let mut out = vec![]; for t9271 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t9271;
out.push({ let mut out = vec![]; for t9272 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t9272;
out.push(py::sub(&py::getitem(&py::getitem(&v_Af.clone(), &v_i.clone())?, &v_j.clone())?, &py::mul(&py::getitem(&v_Bf.clone(), &v_i.clone())?, &py::getitem(&v_Kf.clone(), &v_j.clone())?)?)?); } Py::list(out) }); } Py::list(out) };
    let t9290 = (f__faddeev(v_Acl.clone())?).items()?;
    if t9290.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t9290[0].clone();
    v__ = t9290[1].clone();
    v_cf = { let mut out = vec![]; for t9291 in py::iter(&(v_c.clone()))? { let mut v_x: Py; 
v_x = t9291;
out.push(py::float(&v_x.clone())?); } Py::list(out) };
    v_poles = f__roots(v_cf.clone())?;
    v_mr = py::minmax(&[{ let mut out = vec![]; for t9379 in py::iter(&(v_poles.clone()))? { let mut v_p: Py; 
v_p = t9379;
out.push(py::real(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?;
    return Ok(Py::dict(vec![(Py::str("char_poly"), { let mut out = vec![]; for t9380 in py::iter(&(v_cf.clone()))? { let mut v_x: Py; 
v_x = t9380;
out.push(f__r12(v_x.clone())?); } Py::list(out) }), (Py::str("poles"), v_poles.clone()), (Py::str("stable"), { let t9381 = v_mr.clone(); Py::Bool({ let t9382 = Py::Int(0i64); if py::lt(&t9381, &t9382)? { true } else { false } }) })]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_state_transition(mut v_A: Py, mut v_t: Py) -> R {
    let t9442 = (f__ss_in(v_A.clone(), Py::None, Py::None, Py::None)?).items()?;
    if t9442.len() != 1 { return Err(PyErr::Value("unpack".into())); }
    v_A = t9442[0].clone();
    v_t = f__num(v_t.clone(), Py::str("t"))?;
    let _ = f__need({ let t9443 = py::mul(&f__mnorm1(v_A.clone())?, &py::abs(&v_t.clone())?)?; Py::Bool({ let t9444 = Py::Int(40i64); if py::le(&t9443, &t9444)? { true } else { false } }) }, Py::str("|A| t is too large for an accurate matrix exponential here (limit 40)"))?;
    return Ok(Py::dict(vec![(Py::str("Phi"), f__mexp_out(f__expm(f__mscale(v_A.clone(), v_t.clone())?)?)?)]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_step_response(mut v_num: Py, mut v_den: Py, mut v_t: Py, mut v_A: Py) -> R {
    let mut v_Am: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let mut v_E: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_y: Py = Py::None;
    let t9540 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t9540.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t9540[0].clone();
    v_den = t9540[1].clone();
    let t9543 = (Py::tuple(vec![f__nonneg(v_t.clone(), Py::str("t"))?, f__num(v_A.clone(), Py::str("A"))?])).items()?;
    if t9543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_t = t9543[0].clone();
    v_A = t9543[1].clone();
    let _ = f__need({ let t9544 = py::mul(&py::minmax(&[{ let mut out = vec![]; for t9627 in py::iter(&(f__roots(v_den.clone())?))? { let mut v_p: Py; 
v_p = t9627;
out.push(py::abs(&py::complex(&[v_p.clone()])?)?); } Py::list(out) }], true)?, &v_t.clone())?; Py::Bool({ let t9545 = Py::Int(40i64); if py::le(&t9544, &t9545)? { true } else { false } }) }, Py::str("|pole| t is too large (limit 40)"))?;
    let t9634 = (f__ccf(v_num.clone(), v_den.clone())?).items()?;
    if t9634.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_Am = t9634[0].clone();
    v_B = t9634[1].clone();
    v_C = t9634[2].clone();
    v_D = t9634[3].clone();
    v_n = py::len(&v_Am.clone())?;
    v_M = py::add(&{ let mut out = vec![]; for t9635 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t9635;
out.push(py::add(&{ let mut out = vec![]; for t9636 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_j: Py; 
v_j = t9636;
out.push(py::mul(&py::getitem(&py::getitem(&v_Am.clone(), &v_i.clone())?, &v_j.clone())?, &v_t.clone())?); } Py::list(out) }, &Py::list(vec![py::mul(&py::getitem(&v_B.clone(), &v_i.clone())?, &v_t.clone())?]))?); } Py::list(out) }, &Py::list(vec![py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?]))?;
    v_E = f__expm(v_M.clone())?;
    v_y = py::add(&py::sum(&{ let mut out = vec![]; for t9659 in py::range_iter(&[(v_n.clone()).i()?]) { let mut v_i: Py; 
v_i = t9659;
out.push(py::mul(&py::getitem(&v_C.clone(), &v_i.clone())?, &py::getitem(&py::getitem(&v_E.clone(), &v_i.clone())?, &v_n.clone())?)?); } Py::list(out) }, None)?, &v_D.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("y"), py::mul(&v_A.clone(), &v_y.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tf_evaluate(mut v_num: Py, mut v_den: Py, mut v_s_re: Py, mut v_s_im: Py) -> R {
    let mut v_d: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let t9725 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(false), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t9725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t9725[0].clone();
    v_den = t9725[1].clone();
    v_s = py::complex(&[f__num(v_s_re.clone(), Py::str("s_re"))?, f__num(v_s_im.clone(), Py::str("s_im"))?])?;
    v_d = f__pval(v_den.clone(), v_s.clone())?;
    v_scale = py::sum(&{ let mut out = vec![]; for t9727 in py::iter(&(py::enumerate(&v_den.clone(), 0)?))? { let mut v_a: Py; let mut v_i: Py; 
let t9728 = (t9727).items()?;
if t9728.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t9728[0].clone();
v_a = t9728[1].clone();
out.push(py::mul(&py::abs(&v_a.clone())?, &py::pow(&py::abs(&v_s.clone())?, &py::sub(&py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?, &v_i.clone())?)?)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t9729 = py::abs(&v_d.clone())?; Py::Bool({ let t9730 = py::mul(&Py::Float(1e-12), &py::minmax(&[v_scale.clone(), Py::Float(1e-300)], true)?)?; if py::gt(&t9729, &t9730)? { true } else { false } }) }, Py::str("the point is a pole of G(s)"))?;
    v_g = py::truediv(&f__pval(v_num.clone(), v_s.clone())?, &v_d.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("G_re"), py::real(&v_g.clone())?), (Py::str("G_im"), py::imag(&v_g.clone())?), (Py::str("magnitude"), py::hypot(&[py::real(&v_g.clone())?, py::imag(&v_g.clone())?])?), (Py::str("angle_deg"), f__deg(v_g.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tf_to_ss(mut v_num: Py, mut v_den: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_C: Py = Py::None;
    let mut v_D: Py = Py::None;
    let t9796 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t9796.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t9796[0].clone();
    v_den = t9796[1].clone();
    let t9803 = (f__ccf(v_num.clone(), v_den.clone())?).items()?;
    if t9803.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_A = t9803[0].clone();
    v_B = t9803[1].clone();
    v_C = t9803[2].clone();
    v_D = t9803[3].clone();
    return Ok(f__rnd(Py::dict(vec![(Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("C"), v_C.clone()), (Py::str("D"), v_D.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_true_settling_time(mut v_zeta: Py, mut v_wn: Py, mut v_band: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_tk: Py = Py::None;
    let mut v_ts: Py = Py::None;
    let mut v_wd: Py = Py::None;
    let mut v_zk: Py = Py::None;
    let t9849 = (Py::tuple(vec![f__num(v_zeta.clone(), Py::str("zeta"))?, f__pos(v_wn.clone(), Py::str("wn"))?, f__num(v_band.clone(), Py::str("band"))?])).items()?;
    if t9849.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_zeta = t9849[0].clone();
    v_wn = t9849[1].clone();
    v_band = t9849[2].clone();
    let _ = f__need({ let t9850 = Py::Int(0i64); Py::Bool({ let t9851 = v_zeta.clone(); if py::lt(&t9850, &t9851)? { { let t9852 = Py::Int(1i64); if py::lt(&t9851, &t9852)? { true } else { false } } } else { false } }) }, Py::str("this capability is for an underdamped system: 0 < zeta < 1"))?;
    let _ = f__need({ let t9853 = Py::Int(0i64); Py::Bool({ let t9854 = v_band.clone(); if py::lt(&t9853, &t9854)? { { let t9855 = Py::Float(0.5); if py::lt(&t9854, &t9855)? { true } else { false } } } else { false } }) }, Py::str("the band must lie between 0 and 50 %"))?;
    v_s = py::mul(&v_zeta.clone(), &v_wn.clone())?;
    v_wd = py::mul(&v_wn.clone(), &py::math1("sqrt", &py::sub(&Py::Int(1i64), &py::mul(&v_zeta.clone(), &v_zeta.clone())?)?)?)?;
    v_step = py::truediv(&c_PI()?, &v_wd.clone())?;
    v_k = py::int(&py::math1("floor", &py::truediv(&py::math1("log", &py::truediv(&Py::Float(1.0), &v_band.clone())?)?, &py::mul(&v_s.clone(), &v_step.clone())?)?)?)?;
    while ({ let t9860 = { let t9856 = v_k.clone(); Py::Bool({ let t9857 = Py::Int(0i64); if py::gt(&t9856, &t9857)? { true } else { false } }) }; if !t9860.truthy() { t9860 } else { { let t9858 = py::math1("exp", &py::mul(&py::mul(&py::neg(&v_s.clone())?, &v_k.clone())?, &v_step.clone())?)?; Py::Bool({ let t9859 = v_band.clone(); if py::le(&t9858, &t9859)? { true } else { false } }) } } }).truthy() {
        v_k = py::sub(&v_k, &Py::Int(1i64))?;
    }
    while ({ let t9861 = py::math1("exp", &py::mul(&py::mul(&py::neg(&v_s.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?, &v_step.clone())?)?; Py::Bool({ let t9862 = v_band.clone(); if py::gt(&t9861, &t9862)? { true } else { false } }) }).truthy() {
        v_k = py::iadd(&v_k, &Py::Int(1i64))?;
    }
    v_tk = py::mul(&v_k.clone(), &v_step.clone())?;
    v_zk = f__bisect({ let v_band = v_band.clone(); let v_k = v_k.clone(); let v_s = v_s.clone(); let v_step = v_step.clone(); let v_tk = v_tk.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_zeta = v_zeta.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__y2(v_zeta.clone(), v_wn.clone(), v_t.clone())?, &Py::Float(1.0))?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_tk.clone(), py::add(&v_tk.clone(), &v_step.clone())?)?;
    v_ts = f__bisect({ let v_band = v_band.clone(); let v_k = v_k.clone(); let v_s = v_s.clone(); let v_step = v_step.clone(); let v_tk = v_tk.clone(); let v_wd = v_wd.clone(); let v_wn = v_wn.clone(); let v_zeta = v_zeta.clone(); let v_zk = v_zk.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_t: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::abs(&py::sub(&f__y2(v_zeta.clone(), v_wn.clone(), v_t.clone())?, &Py::Float(1.0))?)?, &v_band.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, v_tk.clone(), v_zk.clone())?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("Ts"), v_ts.clone()), (Py::str("last_extremum"), v_k.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tustin(mut v_num: Py, mut v_den: Py, mut v_T: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t9951 = (f__disc_tf_in(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t9951.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_num = t9951[0].clone();
    v_den = t9951[1].clone();
    v_T = t9951[2].clone();
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    v_c = py::truediv(&Py::Float(2.0), &v_T.clone())?;
    return Ok(f__disc_out(f__subst(v_num.clone(), v_n.clone(), v_c.clone(), Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0), Py::Float(1.0)]))?, f__subst(v_den.clone(), v_n.clone(), v_c.clone(), Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0), Py::Float(1.0)]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_tustin_prewarp(mut v_num: Py, mut v_den: Py, mut v_T: Py, mut v_w: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t10044 = (f__disc_tf_in(v_num.clone(), v_den.clone(), v_T.clone())?).items()?;
    if t10044.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_num = t10044[0].clone();
    v_den = t10044[1].clone();
    v_T = t10044[2].clone();
    v_w = f__pos(v_w.clone(), Py::str("w"))?;
    let _ = f__need({ let t10045 = py::mul(&v_w.clone(), &v_T.clone())?; Py::Bool({ let t10046 = c_PI()?; if py::lt(&t10045, &t10046)? { true } else { false } }) }, Py::str("the prewarping frequency must be below the Nyquist frequency (w T < pi)"))?;
    v_n = py::sub(&py::len(&v_den.clone())?, &Py::Int(1i64))?;
    v_c = py::truediv(&v_w.clone(), &py::math1("tan", &py::truediv(&py::mul(&v_w.clone(), &v_T.clone())?, &Py::Int(2i64))?)?)?;
    return Ok(f__disc_out(f__subst(v_num.clone(), v_n.clone(), v_c.clone(), Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0), Py::Float(1.0)]))?, f__subst(v_den.clone(), v_n.clone(), v_c.clone(), Py::list(vec![Py::Float(1.0), py::neg(&Py::Float(1.0))?]), Py::list(vec![Py::Float(1.0), Py::Float(1.0)]))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_z_to_s_pole(mut v_z_re: Py, mut v_z_im: Py, mut v_T: Py) -> R {
    let mut v_mag: Py = Py::None;
    let mut v_s_im: Py = Py::None;
    let mut v_s_re: Py = Py::None;
    let mut v_wn: Py = Py::None;
    let t10118 = (Py::tuple(vec![f__num(v_z_re.clone(), Py::str("z_re"))?, f__nonneg(v_z_im.clone(), Py::str("z_im"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t10118.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_z_re = t10118[0].clone();
    v_z_im = t10118[1].clone();
    v_T = t10118[2].clone();
    let _ = f__need({ let t10123 = { let t10119 = v_z_im.clone(); Py::Bool({ let t10120 = Py::Int(0i64); if py::gt(&t10119, &t10120)? { true } else { false } }) }; if t10123.truthy() { t10123 } else { { let t10121 = v_z_re.clone(); Py::Bool({ let t10122 = Py::Int(0i64); if py::gt(&t10121, &t10122)? { true } else { false } }) } } }, Py::str("a pole on the negative real axis (or at 0) has no unique continuous equivalent"))?;
    v_mag = py::hypot(&[v_z_re.clone(), v_z_im.clone()])?;
    v_s_re = py::truediv(&py::math1("log", &v_mag.clone())?, &v_T.clone())?;
    v_s_im = py::truediv(&py::atan2(&v_z_im.clone(), &v_z_re.clone())?, &v_T.clone())?;
    v_wn = py::hypot(&[v_s_re.clone(), v_s_im.clone()])?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("s_re"), v_s_re.clone()), (Py::str("s_im"), v_s_im.clone()), (Py::str("wn"), v_wn.clone()), (Py::str("zeta"), py::truediv(&py::neg(&v_s_re.clone())?, &v_wn.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zn_from_plant(mut v_num: Py, mut v_den: Py, mut v_controller: Py) -> R {
    let mut v_Ku: Py = Py::None;
    let mut v_Pu: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let t10189 = (f__tf_in(v_num.clone(), v_den.clone(), Py::Bool(true), Py::Bool(false), Py::Int(9i64))?).items()?;
    if t10189.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t10189[0].clone();
    v_den = t10189[1].clone();
    let _ = f__need({ let t10190 = v_controller.clone(); Py::Bool({ let t10191 = Py::tuple(vec![Py::str("P"), Py::str("PI"), Py::str("PID")]); if py::contains(&t10191, &t10190)? { true } else { false } }) }, Py::str("controller must be P, PI or PID"))?;
    v_pc = f__phase_crossovers(v_num.clone(), v_den.clone())?;
    let _ = f__need({ let t10313 = py::len(&v_pc.clone())?; Py::Bool({ let t10314 = Py::Int(1i64); if py::eq(&t10313, &t10314) { true } else { false } }) }, (if (v_pc.clone()).truthy() { Py::str("the plant needs exactly one phase crossover (a -180 degree frequency) for the ultimate-gain test") } else { Py::str("the phase never reaches -180 degrees: there is no ultimate gain") }))?;
    v_g = f__G(v_num.clone(), v_den.clone(), py::complex(&[Py::Float(0.0), py::getitem(&v_pc.clone(), &Py::Int(0i64))?])?)?;
    v_Ku = py::truediv(&Py::Float(1.0), &py::hypot(&[py::real(&v_g.clone())?, py::imag(&v_g.clone())?])?)?;
    v_Pu = py::truediv(&py::mul(&Py::Int(2i64), &c_PI()?)?, &py::getitem(&v_pc.clone(), &Py::Int(0i64))?)?;
    v_out = f_zn_ultimate(v_Ku.clone(), v_Pu.clone(), v_controller.clone())?;
    let _ = py::method(&v_out.clone(), "update", &[Py::dict(vec![(Py::str("Ku"), f__r12(v_Ku.clone())?), (Py::str("Pu"), f__r12(v_Pu.clone())?)])])?;
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zn_reaction(mut v_K: Py, mut v_L: Py, mut v_T: Py, mut v_controller: Py) -> R {
    let mut v_base: Py = Py::None;
    let t10371 = (Py::tuple(vec![f__pos(v_K.clone(), Py::str("K"))?, f__pos(v_L.clone(), Py::str("L"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t10371.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t10371[0].clone();
    v_L = t10371[1].clone();
    v_T = t10371[2].clone();
    let _ = f__need({ let t10372 = v_controller.clone(); Py::Bool({ let t10373 = Py::tuple(vec![Py::str("P"), Py::str("PI"), Py::str("PID")]); if py::contains(&t10373, &t10372)? { true } else { false } }) }, Py::str("controller must be P, PI or PID"))?;
    v_base = py::truediv(&v_T.clone(), &py::mul(&v_K.clone(), &v_L.clone())?)?;
    if ({ let t10374 = v_controller.clone(); Py::Bool({ let t10375 = Py::str("P"); if py::eq(&t10374, &t10375) { true } else { false } }) }).truthy() {
        return Ok(f__pid_out(v_base.clone(), Py::Float(0.0), Py::Float(0.0))?);
    }
    if ({ let t10387 = v_controller.clone(); Py::Bool({ let t10388 = Py::str("PI"); if py::eq(&t10387, &t10388) { true } else { false } }) }).truthy() {
        return Ok(f__pid_out(py::mul(&Py::Float(0.9), &v_base.clone())?, py::truediv(&v_L.clone(), &Py::Float(0.3))?, Py::Float(0.0))?);
    }
    return Ok(f__pid_out(py::mul(&Py::Float(1.2), &v_base.clone())?, py::mul(&Py::Float(2.0), &v_L.clone())?, py::mul(&Py::Float(0.5), &v_L.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zn_ultimate(mut v_Ku: Py, mut v_Pu: Py, mut v_controller: Py) -> R {
    let t10425 = (Py::tuple(vec![f__pos(v_Ku.clone(), Py::str("Ku"))?, f__pos(v_Pu.clone(), Py::str("Pu"))?])).items()?;
    if t10425.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Ku = t10425[0].clone();
    v_Pu = t10425[1].clone();
    let _ = f__need({ let t10426 = v_controller.clone(); Py::Bool({ let t10427 = Py::tuple(vec![Py::str("P"), Py::str("PI"), Py::str("PID")]); if py::contains(&t10427, &t10426)? { true } else { false } }) }, Py::str("controller must be P, PI or PID"))?;
    if ({ let t10428 = v_controller.clone(); Py::Bool({ let t10429 = Py::str("P"); if py::eq(&t10428, &t10429) { true } else { false } }) }).truthy() {
        return Ok(f__pid_out(py::mul(&Py::Float(0.5), &v_Ku.clone())?, Py::Float(0.0), Py::Float(0.0))?);
    }
    if ({ let t10441 = v_controller.clone(); Py::Bool({ let t10442 = Py::str("PI"); if py::eq(&t10441, &t10442) { true } else { false } }) }).truthy() {
        return Ok(f__pid_out(py::mul(&Py::Float(0.45), &v_Ku.clone())?, py::truediv(&v_Pu.clone(), &Py::Float(1.2))?, Py::Float(0.0))?);
    }
    return Ok(f__pid_out(py::mul(&Py::Float(0.6), &v_Ku.clone())?, py::truediv(&v_Pu.clone(), &Py::Float(2.0))?, py::truediv(&v_Pu.clone(), &Py::Float(8.0))?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_zoh_first_order(mut v_K: Py, mut v_tau: Py, mut v_T: Py) -> R {
    let mut v_a: Py = Py::None;
    let t10479 = (Py::tuple(vec![f__num(v_K.clone(), Py::str("K"))?, f__pos(v_tau.clone(), Py::str("tau"))?, f__pos(v_T.clone(), Py::str("T"))?])).items()?;
    if t10479.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_K = t10479[0].clone();
    v_tau = t10479[1].clone();
    v_T = t10479[2].clone();
    v_a = py::math1("exp", &py::truediv(&py::neg(&v_T.clone())?, &v_tau.clone())?)?;
    return Ok(f__rnd(Py::dict(vec![(Py::str("b"), py::mul(&py::neg(&v_K.clone())?, &py::math1("expm1", &py::truediv(&py::neg(&v_T.clone())?, &v_tau.clone())?)?)?), (Py::str("a"), v_a.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_10080(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t10082: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t10081 = v_inputs.clone(); f_tustin_prewarp(py::getitem(&t10081, &Py::str("num"))?, py::getitem(&t10081, &Py::str("den"))?, py::getitem(&t10081, &Py::str("T"))?, py::getitem(&t10081, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10082 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t10083 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10111 = (t10083).items()?;
        if t10111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10111[0].clone();
        v_w = t10111[1].clone();
        if ({ let t10084 = v_k.clone(); Py::Bool({ let t10085 = v_outputs.clone(); if !py::contains(&t10085, &t10084)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_10133(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t10135: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t10134 = v_inputs.clone(); f_z_to_s_pole(py::getitem(&t10134, &Py::str("z_re"))?, py::getitem(&t10134, &Py::str("z_im"))?, py::getitem(&t10134, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10135 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t10136 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10164 = (t10136).items()?;
        if t10164.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10164[0].clone();
        v_w = t10164[1].clone();
        if ({ let t10137 = v_k.clone(); Py::Bool({ let t10138 = v_outputs.clone(); if !py::contains(&t10138, &t10137)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_10335(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t10337: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t10336 = v_inputs.clone(); f_zn_from_plant(py::getitem(&t10336, &Py::str("num"))?, py::getitem(&t10336, &Py::str("den"))?, py::getitem(&t10336, &Py::str("controller"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10337 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t10338 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10366 = (t10338).items()?;
        if t10366.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10366[0].clone();
        v_w = t10366[1].clone();
        if ({ let t10339 = v_k.clone(); Py::Bool({ let t10340 = v_outputs.clone(); if !py::contains(&t10340, &t10339)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_10389(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t10391: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t10390 = v_inputs.clone(); f_zn_reaction(py::getitem(&t10390, &Py::str("K"))?, py::getitem(&t10390, &Py::str("L"))?, py::getitem(&t10390, &Py::str("T"))?, py::getitem(&t10390, &Py::str("controller"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10391 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t10392 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10420 = (t10392).items()?;
        if t10420.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10420[0].clone();
        v_w = t10420[1].clone();
        if ({ let t10393 = v_k.clone(); Py::Bool({ let t10394 = v_outputs.clone(); if !py::contains(&t10394, &t10393)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_10443(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t10445: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t10444 = v_inputs.clone(); f_zn_ultimate(py::getitem(&t10444, &Py::str("Ku"))?, py::getitem(&t10444, &Py::str("Pu"))?, py::getitem(&t10444, &Py::str("controller"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10445 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t10446 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10474 = (t10446).items()?;
        if t10474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10474[0].clone();
        v_w = t10474[1].clone();
        if ({ let t10447 = v_k.clone(); Py::Bool({ let t10448 = v_outputs.clone(); if !py::contains(&t10448, &t10447)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1046(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1048: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1047 = v_inputs.clone(); f_c2d_foh(py::getitem(&t1047, &Py::str("num"))?, py::getitem(&t1047, &Py::str("den"))?, py::getitem(&t1047, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1048 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1049 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1077 = (t1049).items()?;
        if t1077.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1077[0].clone();
        v_w = t1077[1].clone();
        if ({ let t1050 = v_k.clone(); Py::Bool({ let t1051 = v_outputs.clone(); if !py::contains(&t1051, &t1050)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_10489(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t10491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t10490 = v_inputs.clone(); f_zoh_first_order(py::getitem(&t10490, &Py::str("K"))?, py::getitem(&t10490, &Py::str("tau"))?, py::getitem(&t10490, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t10491 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t10492 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10520 = (t10492).items()?;
        if t10520.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10520[0].clone();
        v_w = t10520[1].clone();
        if ({ let t10493 = v_k.clone(); Py::Bool({ let t10494 = v_outputs.clone(); if !py::contains(&t10494, &t10493)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_112(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t114: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t113 = v_inputs.clone(); f_ackermann(py::getitem(&t113, &Py::str("A"))?, py::getitem(&t113, &Py::str("B"))?, py::getitem(&t113, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t114 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t115 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t143 = (t115).items()?;
        if t143.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t143[0].clone();
        v_w = t143[1].clone();
        if ({ let t116 = v_k.clone(); Py::Bool({ let t117 = v_outputs.clone(); if !py::contains(&t117, &t116)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1254(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1256: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1255 = v_inputs.clone(); f_c2d_zoh(py::getitem(&t1255, &Py::str("num"))?, py::getitem(&t1255, &Py::str("den"))?, py::getitem(&t1255, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1256 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1257 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1285 = (t1257).items()?;
        if t1285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1285[0].clone();
        v_w = t1285[1].clone();
        if ({ let t1258 = v_k.clone(); Py::Bool({ let t1259 = v_outputs.clone(); if !py::contains(&t1259, &t1258)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1302(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1304: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1303 = v_inputs.clone(); f_closed_loop_first_order_p(py::getitem(&t1303, &Py::str("K"))?, py::getitem(&t1303, &Py::str("tau"))?, py::getitem(&t1303, &Py::str("Kp"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1304 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1305 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1333 = (t1305).items()?;
        if t1333.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1333[0].clone();
        v_w = t1333[1].clone();
        if ({ let t1306 = v_k.clone(); Py::Bool({ let t1307 = v_outputs.clone(); if !py::contains(&t1307, &t1306)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1356(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1358: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1357 = v_inputs.clone(); f_cohen_coon(py::getitem(&t1357, &Py::str("K"))?, py::getitem(&t1357, &Py::str("tau"))?, py::getitem(&t1357, &Py::str("theta"))?, py::getitem(&t1357, &Py::str("controller"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1358 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1359 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1387 = (t1359).items()?;
        if t1387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1387[0].clone();
        v_w = t1387[1].clone();
        if ({ let t1360 = v_k.clone(); Py::Bool({ let t1361 = v_outputs.clone(); if !py::contains(&t1361, &t1360)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1463(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1465: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1464 = v_inputs.clone(); f_controllability(py::getitem(&t1464, &Py::str("A"))?, py::getitem(&t1464, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1465 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1466 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1494 = (t1466).items()?;
        if t1494.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1494[0].clone();
        v_w = t1494[1].clone();
        if ({ let t1467 = v_k.clone(); Py::Bool({ let t1468 = v_outputs.clone(); if !py::contains(&t1468, &t1467)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1692(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1694: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1693 = v_inputs.clone(); f_ctrb_gramian(py::getitem(&t1693, &Py::str("A"))?, py::getitem(&t1693, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1694 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1695 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1723 = (t1695).items()?;
        if t1723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1723[0].clone();
        v_w = t1723[1].clone();
        if ({ let t1696 = v_k.clone(); Py::Bool({ let t1697 = v_outputs.clone(); if !py::contains(&t1697, &t1696)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1738(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1740: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1739 = v_inputs.clone(); f_damping_from_overshoot(py::getitem(&t1739, &Py::str("OS"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1740 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1741 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1769 = (t1741).items()?;
        if t1769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1769[0].clone();
        v_w = t1769[1].clone();
        if ({ let t1742 = v_k.clone(); Py::Bool({ let t1743 = v_outputs.clone(); if !py::contains(&t1743, &t1742)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1890(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t1892: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t1891 = v_inputs.clone(); f_dc_gain(py::getitem(&t1891, &Py::str("num"))?, py::getitem(&t1891, &Py::str("den"))?, py::method(&t1891, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t1892 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t1893 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1921 = (t1893).items()?;
        if t1921.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1921[0].clone();
        v_w = t1921[1].clone();
        if ({ let t1894 = v_k.clone(); Py::Bool({ let t1895 = v_outputs.clone(); if !py::contains(&t1895, &t1894)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2017(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2019: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2018 = v_inputs.clone(); f_dc_motor(py::getitem(&t2018, &Py::str("J"))?, py::getitem(&t2018, &Py::str("b"))?, py::getitem(&t2018, &Py::str("Kt"))?, py::getitem(&t2018, &Py::str("R"))?, py::getitem(&t2018, &Py::str("L"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2019 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2020 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2048 = (t2020).items()?;
        if t2048.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2048[0].clone();
        v_w = t2048[1].clone();
        if ({ let t2021 = v_k.clone(); Py::Bool({ let t2022 = v_outputs.clone(); if !py::contains(&t2022, &t2021)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2160(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2162: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2161 = v_inputs.clone(); f_deadbeat_gain(py::getitem(&t2161, &Py::str("A"))?, py::getitem(&t2161, &Py::str("B"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2162 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2163 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2191 = (t2163).items()?;
        if t2191.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2191[0].clone();
        v_w = t2191[1].clone();
        if ({ let t2164 = v_k.clone(); Py::Bool({ let t2165 = v_outputs.clone(); if !py::contains(&t2165, &t2164)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2322(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2324: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2323 = v_inputs.clone(); f_departure_angle(py::getitem(&t2323, &Py::str("num"))?, py::getitem(&t2323, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2324 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2325 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2353 = (t2325).items()?;
        if t2353.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2353[0].clone();
        v_w = t2353[1].clone();
        if ({ let t2326 = v_k.clone(); Py::Bool({ let t2327 = v_outputs.clone(); if !py::contains(&t2327, &t2326)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2474(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2476: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2475 = v_inputs.clone(); f_discrete_dc_gain(py::getitem(&t2475, &Py::str("num"))?, py::getitem(&t2475, &Py::str("den"))?, py::method(&t2475, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2476 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2477 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2505 = (t2477).items()?;
        if t2505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2505[0].clone();
        v_w = t2505[1].clone();
        if ({ let t2478 = v_k.clone(); Py::Bool({ let t2479 = v_outputs.clone(); if !py::contains(&t2479, &t2478)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2698(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2700: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2699 = v_inputs.clone(); f_discrete_kalman_gain(py::getitem(&t2699, &Py::str("A"))?, py::getitem(&t2699, &Py::str("C"))?, py::getitem(&t2699, &Py::str("Q"))?, py::getitem(&t2699, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2700 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2701 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2729 = (t2701).items()?;
        if t2729.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2729[0].clone();
        v_w = t2729[1].clone();
        if ({ let t2702 = v_k.clone(); Py::Bool({ let t2703 = v_outputs.clone(); if !py::contains(&t2703, &t2702)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_274(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t276: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t275 = v_inputs.clone(); f_arrival_angle(py::getitem(&t275, &Py::str("num"))?, py::getitem(&t275, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t276 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t277 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t305 = (t277).items()?;
        if t305.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t305[0].clone();
        v_w = t305[1].clone();
        if ({ let t278 = v_k.clone(); Py::Bool({ let t279 = v_outputs.clone(); if !py::contains(&t279, &t278)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2932(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2934: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2933 = v_inputs.clone(); f_discrete_lyapunov(py::getitem(&t2933, &Py::str("A"))?, py::getitem(&t2933, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2934 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2935 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2963 = (t2935).items()?;
        if t2963.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2963[0].clone();
        v_w = t2963[1].clone();
        if ({ let t2936 = v_k.clone(); Py::Bool({ let t2937 = v_outputs.clone(); if !py::contains(&t2937, &t2936)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2980(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t2982: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t2981 = v_inputs.clone(); f_discrete_pid(py::getitem(&t2981, &Py::str("Kp"))?, py::getitem(&t2981, &Py::str("Ki"))?, py::getitem(&t2981, &Py::str("Kd"))?, py::getitem(&t2981, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t2982 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t2983 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3011 = (t2983).items()?;
        if t3011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3011[0].clone();
        v_w = t3011[1].clone();
        if ({ let t2984 = v_k.clone(); Py::Bool({ let t2985 = v_outputs.clone(); if !py::contains(&t2985, &t2984)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3058(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3060: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3059 = v_inputs.clone(); f_discrete_step(py::getitem(&t3059, &Py::str("numz"))?, py::getitem(&t3059, &Py::str("denz"))?, py::getitem(&t3059, &Py::str("k"))?, py::method(&t3059, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3060 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3061 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3089 = (t3061).items()?;
        if t3089.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3089[0].clone();
        v_w = t3089[1].clone();
        if ({ let t3062 = v_k.clone(); Py::Bool({ let t3063 = v_outputs.clone(); if !py::contains(&t3063, &t3062)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3371(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3373: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3372 = v_inputs.clone(); f_dlqr(py::getitem(&t3372, &Py::str("A"))?, py::getitem(&t3372, &Py::str("B"))?, py::getitem(&t3372, &Py::str("Q"))?, py::getitem(&t3372, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3373 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3374 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3402 = (t3374).items()?;
        if t3402.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3402[0].clone();
        v_w = t3402[1].clone();
        if ({ let t3375 = v_k.clone(); Py::Bool({ let t3376 = v_outputs.clone(); if !py::contains(&t3376, &t3375)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3470(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3472: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3471 = v_inputs.clone(); f_error_constants(py::getitem(&t3471, &Py::str("num"))?, py::getitem(&t3471, &Py::str("den"))?, py::getitem(&t3471, &Py::str("input"))?, py::method(&t3471, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3472 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3473 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3501 = (t3473).items()?;
        if t3501.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3501[0].clone();
        v_w = t3501[1].clone();
        if ({ let t3474 = v_k.clone(); Py::Bool({ let t3475 = v_outputs.clone(); if !py::contains(&t3475, &t3474)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3563(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3565: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3564 = v_inputs.clone(); f_feedback_connection(py::getitem(&t3564, &Py::str("num_g"))?, py::getitem(&t3564, &Py::str("den_g"))?, py::method(&t3564, "get", &[Py::str("num_h"), Py::None])?, py::method(&t3564, "get", &[Py::str("den_h"), Py::None])?, py::method(&t3564, "get", &[Py::str("sign"), Py::str("negative")])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3565 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3566 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3594 = (t3566).items()?;
        if t3594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3594[0].clone();
        v_w = t3594[1].clone();
        if ({ let t3567 = v_k.clone(); Py::Bool({ let t3568 = v_outputs.clone(); if !py::contains(&t3568, &t3567)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_367(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t369: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t368 = v_inputs.clone(); f_backward_euler(py::getitem(&t368, &Py::str("num"))?, py::getitem(&t368, &Py::str("den"))?, py::getitem(&t368, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t369 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t370 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t398 = (t370).items()?;
        if t398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t398[0].clone();
        v_w = t398[1].clone();
        if ({ let t371 = v_k.clone(); Py::Bool({ let t372 = v_outputs.clone(); if !py::contains(&t372, &t371)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3726(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3728: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3727 = v_inputs.clone(); f_final_value_theorem(py::getitem(&t3727, &Py::str("num"))?, py::getitem(&t3727, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3728 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3729 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3757 = (t3729).items()?;
        if t3757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3757[0].clone();
        v_w = t3757[1].clone();
        if ({ let t3730 = v_k.clone(); Py::Bool({ let t3731 = v_outputs.clone(); if !py::contains(&t3731, &t3730)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3774(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3775 = v_inputs.clone(); f_first_order_freq(py::getitem(&t3775, &Py::str("K"))?, py::getitem(&t3775, &Py::str("tau"))?, py::getitem(&t3775, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3776 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3777 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3805 = (t3777).items()?;
        if t3805.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3805[0].clone();
        v_w = t3805[1].clone();
        if ({ let t3778 = v_k.clone(); Py::Bool({ let t3779 = v_outputs.clone(); if !py::contains(&t3779, &t3778)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3822(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3824: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3823 = v_inputs.clone(); f_first_order_specs(py::getitem(&t3823, &Py::str("K"))?, py::getitem(&t3823, &Py::str("tau"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3824 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3825 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3853 = (t3825).items()?;
        if t3853.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3853[0].clone();
        v_w = t3853[1].clone();
        if ({ let t3826 = v_k.clone(); Py::Bool({ let t3827 = v_outputs.clone(); if !py::contains(&t3827, &t3826)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3875(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3877: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3876 = v_inputs.clone(); f_first_order_step(py::getitem(&t3876, &Py::str("K"))?, py::getitem(&t3876, &Py::str("tau"))?, py::getitem(&t3876, &Py::str("t"))?, py::method(&t3876, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3877 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3878 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3906 = (t3878).items()?;
        if t3906.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3906[0].clone();
        v_w = t3906[1].clone();
        if ({ let t3879 = v_k.clone(); Py::Bool({ let t3880 = v_outputs.clone(); if !py::contains(&t3880, &t3879)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3930(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3932: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3931 = v_inputs.clone(); f_fopdt_smith(py::getitem(&t3931, &Py::str("t28"))?, py::getitem(&t3931, &Py::str("t63"))?, py::getitem(&t3931, &Py::str("dy"))?, py::getitem(&t3931, &Py::str("du"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3932 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3933 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3961 = (t3933).items()?;
        if t3961.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3961[0].clone();
        v_w = t3961[1].clone();
        if ({ let t3934 = v_k.clone(); Py::Bool({ let t3935 = v_outputs.clone(); if !py::contains(&t3935, &t3934)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t3987: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t3986 = v_inputs.clone(); f_fopdt_step(py::getitem(&t3986, &Py::str("K"))?, py::getitem(&t3986, &Py::str("tau"))?, py::getitem(&t3986, &Py::str("theta"))?, py::getitem(&t3986, &Py::str("t"))?, py::method(&t3986, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t3987 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t3988 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4016 = (t3988).items()?;
        if t4016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4016[0].clone();
        v_w = t4016[1].clone();
        if ({ let t3989 = v_k.clone(); Py::Bool({ let t3990 = v_outputs.clone(); if !py::contains(&t3990, &t3989)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4078(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4080: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4079 = v_inputs.clone(); f_forward_euler(py::getitem(&t4079, &Py::str("num"))?, py::getitem(&t4079, &Py::str("den"))?, py::getitem(&t4079, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4080 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4081 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4109 = (t4081).items()?;
        if t4109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4109[0].clone();
        v_w = t4109[1].clone();
        if ({ let t4082 = v_k.clone(); Py::Bool({ let t4083 = v_outputs.clone(); if !py::contains(&t4083, &t4082)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4198(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4200: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4199 = v_inputs.clone(); f_h2_norm(py::getitem(&t4199, &Py::str("num"))?, py::getitem(&t4199, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4200 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4201 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4229 = (t4201).items()?;
        if t4229.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4229[0].clone();
        v_w = t4229[1].clone();
        if ({ let t4202 = v_k.clone(); Py::Bool({ let t4203 = v_outputs.clone(); if !py::contains(&t4203, &t4202)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4403(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4405: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4404 = v_inputs.clone(); f_hinf_norm(py::getitem(&t4404, &Py::str("num"))?, py::getitem(&t4404, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4405 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4406 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4434 = (t4406).items()?;
        if t4434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4434[0].clone();
        v_w = t4434[1].clone();
        if ({ let t4407 = v_k.clone(); Py::Bool({ let t4408 = v_outputs.clone(); if !py::contains(&t4408, &t4407)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4457(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4459: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4458 = v_inputs.clone(); f_identify_second_order(py::getitem(&t4458, &Py::str("y_peak"))?, py::getitem(&t4458, &Py::str("y_final"))?, py::getitem(&t4458, &Py::str("t_peak"))?, py::method(&t4458, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4459 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4460 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4488 = (t4460).items()?;
        if t4488.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4488[0].clone();
        v_w = t4488[1].clone();
        if ({ let t4461 = v_k.clone(); Py::Bool({ let t4462 = v_outputs.clone(); if !py::contains(&t4462, &t4461)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4529(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4531: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4530 = v_inputs.clone(); f_initial_value_theorem(py::getitem(&t4530, &Py::str("num"))?, py::getitem(&t4530, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4531 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4532 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4560 = (t4532).items()?;
        if t4560.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4560[0].clone();
        v_w = t4560[1].clone();
        if ({ let t4533 = v_k.clone(); Py::Bool({ let t4534 = v_outputs.clone(); if !py::contains(&t4534, &t4533)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4589(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4591: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4590 = v_inputs.clone(); f_jury_test(py::getitem(&t4590, &Py::str("coefficients"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4591 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4592 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4620 = (t4592).items()?;
        if t4620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4620[0].clone();
        v_w = t4620[1].clone();
        if ({ let t4593 = v_k.clone(); Py::Bool({ let t4594 = v_outputs.clone(); if !py::contains(&t4594, &t4593)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4902(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4904: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4903 = v_inputs.clone(); f_kalman_bucy(py::getitem(&t4903, &Py::str("A"))?, py::getitem(&t4903, &Py::str("C"))?, py::getitem(&t4903, &Py::str("W"))?, py::getitem(&t4903, &Py::str("V"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4904 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4905 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4933 = (t4905).items()?;
        if t4933.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4933[0].clone();
        v_w = t4933[1].clone();
        if ({ let t4906 = v_k.clone(); Py::Bool({ let t4907 = v_outputs.clone(); if !py::contains(&t4907, &t4906)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4952(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t4954: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t4953 = v_inputs.clone(); f_kalman_scalar_steady(py::getitem(&t4953, &Py::str("a"))?, py::getitem(&t4953, &Py::str("c"))?, py::getitem(&t4953, &Py::str("q"))?, py::getitem(&t4953, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t4954 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t4955 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4983 = (t4955).items()?;
        if t4983.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4983[0].clone();
        v_w = t4983[1].clone();
        if ({ let t4956 = v_k.clone(); Py::Bool({ let t4957 = v_outputs.clone(); if !py::contains(&t4957, &t4956)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5002(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5004: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5003 = v_inputs.clone(); f_kalman_scalar_step(py::getitem(&t5003, &Py::str("x"))?, py::getitem(&t5003, &Py::str("P"))?, py::getitem(&t5003, &Py::str("a"))?, py::getitem(&t5003, &Py::str("q"))?, py::getitem(&t5003, &Py::str("r"))?, py::getitem(&t5003, &Py::str("z"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5004 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5005 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5033 = (t5005).items()?;
        if t5033.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5033[0].clone();
        v_w = t5033[1].clone();
        if ({ let t5006 = v_k.clone(); Py::Bool({ let t5007 = v_outputs.clone(); if !py::contains(&t5007, &t5006)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5052(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5054: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5053 = v_inputs.clone(); f_lag_design(py::getitem(&t5053, &Py::str("beta"))?, py::getitem(&t5053, &Py::str("w_c"))?, py::method(&t5053, "get", &[Py::str("ratio"), Py::Float(10.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5054 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5055 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5083 = (t5055).items()?;
        if t5083.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5083[0].clone();
        v_w = t5083[1].clone();
        if ({ let t5056 = v_k.clone(); Py::Bool({ let t5057 = v_outputs.clone(); if !py::contains(&t5057, &t5056)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5101(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5103: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5102 = v_inputs.clone(); f_lead_design(py::getitem(&t5102, &Py::str("phi_max_deg"))?, py::getitem(&t5102, &Py::str("w_m"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5103 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5104 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5132 = (t5104).items()?;
        if t5132.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5132[0].clone();
        v_w = t5132[1].clone();
        if ({ let t5105 = v_k.clone(); Py::Bool({ let t5106 = v_outputs.clone(); if !py::contains(&t5106, &t5105)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5149(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5151: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5150 = v_inputs.clone(); f_lead_max_phase(py::getitem(&t5150, &Py::str("z"))?, py::getitem(&t5150, &Py::str("p"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5151 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5152 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5180 = (t5152).items()?;
        if t5180.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5180[0].clone();
        v_w = t5180[1].clone();
        if ({ let t5153 = v_k.clone(); Py::Bool({ let t5154 = v_outputs.clone(); if !py::contains(&t5154, &t5153)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5221(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5223: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5222 = v_inputs.clone(); f_locus_asymptotes(py::getitem(&t5222, &Py::str("num"))?, py::getitem(&t5222, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5223 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5224 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5252 = (t5224).items()?;
        if t5252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5252[0].clone();
        v_w = t5252[1].clone();
        if ({ let t5225 = v_k.clone(); Py::Bool({ let t5226 = v_outputs.clone(); if !py::contains(&t5226, &t5225)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5293(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5295: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5294 = v_inputs.clone(); f_locus_gain_at_point(py::getitem(&t5294, &Py::str("num"))?, py::getitem(&t5294, &Py::str("den"))?, py::getitem(&t5294, &Py::str("s_re"))?, py::getitem(&t5294, &Py::str("s_im"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5295 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5296 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5324 = (t5296).items()?;
        if t5324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5324[0].clone();
        v_w = t5324[1].clone();
        if ({ let t5297 = v_k.clone(); Py::Bool({ let t5298 = v_outputs.clone(); if !py::contains(&t5298, &t5297)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5343(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5345: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5344 = v_inputs.clone(); f_log_decrement(py::getitem(&t5344, &Py::str("x1"))?, py::getitem(&t5344, &Py::str("xn"))?, py::getitem(&t5344, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5345 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5346 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5374 = (t5346).items()?;
        if t5374.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5374[0].clone();
        v_w = t5374[1].clone();
        if ({ let t5347 = v_k.clone(); Py::Bool({ let t5348 = v_outputs.clone(); if !py::contains(&t5348, &t5347)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_561(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t563: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t562 = v_inputs.clone(); f_bandwidth_general(py::getitem(&t562, &Py::str("num"))?, py::getitem(&t562, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t563 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t564 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t592 = (t564).items()?;
        if t592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t592[0].clone();
        v_w = t592[1].clone();
        if ({ let t565 = v_k.clone(); Py::Bool({ let t566 = v_outputs.clone(); if !py::contains(&t566, &t565)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5655(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5657: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5656 = v_inputs.clone(); f_lqr(py::getitem(&t5656, &Py::str("A"))?, py::getitem(&t5656, &Py::str("B"))?, py::getitem(&t5656, &Py::str("Q"))?, py::getitem(&t5656, &Py::str("R"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5657 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5658 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5686 = (t5658).items()?;
        if t5686.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5686[0].clone();
        v_w = t5686[1].clone();
        if ({ let t5659 = v_k.clone(); Py::Bool({ let t5660 = v_outputs.clone(); if !py::contains(&t5660, &t5659)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5888(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5890: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5889 = v_inputs.clone(); f_lyapunov(py::getitem(&t5889, &Py::str("A"))?, py::getitem(&t5889, &Py::str("Q"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5890 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5891 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5919 = (t5891).items()?;
        if t5919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5919[0].clone();
        v_w = t5919[1].clone();
        if ({ let t5892 = v_k.clone(); Py::Bool({ let t5893 = v_outputs.clone(); if !py::contains(&t5893, &t5892)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5946(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t5948: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t5947 = v_inputs.clone(); f_mass_spring_damper(py::getitem(&t5947, &Py::str("m"))?, py::getitem(&t5947, &Py::str("c"))?, py::getitem(&t5947, &Py::str("k"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t5948 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t5949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5977 = (t5949).items()?;
        if t5977.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5977[0].clone();
        v_w = t5977[1].clone();
        if ({ let t5950 = v_k.clone(); Py::Bool({ let t5951 = v_outputs.clone(); if !py::contains(&t5951, &t5950)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_607(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t609: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t608 = v_inputs.clone(); f_bandwidth_second(py::getitem(&t608, &Py::str("zeta"))?, py::getitem(&t608, &Py::str("wn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t609 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t610 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t638 = (t610).items()?;
        if t638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t638[0].clone();
        v_w = t638[1].clone();
        if ({ let t611 = v_k.clone(); Py::Bool({ let t612 = v_outputs.clone(); if !py::contains(&t612, &t611)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6103(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6105: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6104 = v_inputs.clone(); f_nyquist_encirclements(py::getitem(&t6104, &Py::str("num"))?, py::getitem(&t6104, &Py::str("den"))?, py::method(&t6104, "get", &[Py::str("K"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6105 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6106 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6134 = (t6106).items()?;
        if t6134.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6134[0].clone();
        v_w = t6134[1].clone();
        if ({ let t6107 = v_k.clone(); Py::Bool({ let t6108 = v_outputs.clone(); if !py::contains(&t6108, &t6107)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6211(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6213: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6212 = v_inputs.clone(); f_observability(py::getitem(&t6212, &Py::str("A"))?, py::getitem(&t6212, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6213 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6214 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6242 = (t6214).items()?;
        if t6242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6242[0].clone();
        v_w = t6242[1].clone();
        if ({ let t6215 = v_k.clone(); Py::Bool({ let t6216 = v_outputs.clone(); if !py::contains(&t6216, &t6215)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6354(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6356: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6355 = v_inputs.clone(); f_observer_gain(py::getitem(&t6355, &Py::str("A"))?, py::getitem(&t6355, &Py::str("C"))?, py::getitem(&t6355, &Py::str("alpha"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6356 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6357 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6385 = (t6357).items()?;
        if t6385.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6385[0].clone();
        v_w = t6385[1].clone();
        if ({ let t6358 = v_k.clone(); Py::Bool({ let t6359 = v_outputs.clone(); if !py::contains(&t6359, &t6358)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6581(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6583: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6582 = v_inputs.clone(); f_obsv_gramian(py::getitem(&t6582, &Py::str("A"))?, py::getitem(&t6582, &Py::str("C"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6583 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6584 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6612 = (t6584).items()?;
        if t6612.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6612[0].clone();
        v_w = t6612[1].clone();
        if ({ let t6585 = v_k.clone(); Py::Bool({ let t6586 = v_outputs.clone(); if !py::contains(&t6586, &t6585)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6634(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6636: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6635 = v_inputs.clone(); f_pade(py::getitem(&t6635, &Py::str("theta"))?, py::getitem(&t6635, &Py::str("n"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6636 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6637 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6665 = (t6637).items()?;
        if t6665.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6665[0].clone();
        v_w = t6665[1].clone();
        if ({ let t6638 = v_k.clone(); Py::Bool({ let t6639 = v_outputs.clone(); if !py::contains(&t6639, &t6638)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6712(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6714: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6713 = v_inputs.clone(); f_parallel_connection(py::getitem(&t6713, &Py::str("num1"))?, py::getitem(&t6713, &Py::str("den1"))?, py::getitem(&t6713, &Py::str("num2"))?, py::getitem(&t6713, &Py::str("den2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6714 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6715 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6743 = (t6715).items()?;
        if t6743.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6743[0].clone();
        v_w = t6743[1].clone();
        if ({ let t6716 = v_k.clone(); Py::Bool({ let t6717 = v_outputs.clone(); if !py::contains(&t6717, &t6716)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6762(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6764: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6763 = v_inputs.clone(); f_peak_time(py::getitem(&t6763, &Py::str("zeta"))?, py::getitem(&t6763, &Py::str("wn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6764 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6765 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6793 = (t6765).items()?;
        if t6793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6793[0].clone();
        v_w = t6793[1].clone();
        if ({ let t6766 = v_k.clone(); Py::Bool({ let t6767 = v_outputs.clone(); if !py::contains(&t6767, &t6766)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_680(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t682: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t681 = v_inputs.clone(); f_bode_point(py::getitem(&t681, &Py::str("num"))?, py::getitem(&t681, &Py::str("den"))?, py::getitem(&t681, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t682 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t683 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t711 = (t683).items()?;
        if t711.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t711[0].clone();
        v_w = t711[1].clone();
        if ({ let t684 = v_k.clone(); Py::Bool({ let t685 = v_outputs.clone(); if !py::contains(&t685, &t684)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6809(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6811: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6810 = v_inputs.clone(); f_percent_overshoot(py::getitem(&t6810, &Py::str("zeta"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6811 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6812 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6840 = (t6812).items()?;
        if t6840.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6840[0].clone();
        v_w = t6840[1].clone();
        if ({ let t6813 = v_k.clone(); Py::Bool({ let t6814 = v_outputs.clone(); if !py::contains(&t6814, &t6813)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6857(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t6859: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t6858 = v_inputs.clone(); f_pi_pole_placement(py::getitem(&t6858, &Py::str("K"))?, py::getitem(&t6858, &Py::str("tau"))?, py::getitem(&t6858, &Py::str("zeta"))?, py::getitem(&t6858, &Py::str("wn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6859 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t6860 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6888 = (t6860).items()?;
        if t6888.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6888[0].clone();
        v_w = t6888[1].clone();
        if ({ let t6861 = v_k.clone(); Py::Bool({ let t6862 = v_outputs.clone(); if !py::contains(&t6862, &t6861)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7012(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7014: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7013 = v_inputs.clone(); f_pid_closed_loop(py::getitem(&t7013, &Py::str("num"))?, py::getitem(&t7013, &Py::str("den"))?, py::getitem(&t7013, &Py::str("Kp"))?, py::method(&t7013, "get", &[Py::str("Ki"), Py::Float(0.0)])?, py::method(&t7013, "get", &[Py::str("Kd"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7014 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7015 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7043 = (t7015).items()?;
        if t7043.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7043[0].clone();
        v_w = t7043[1].clone();
        if ({ let t7016 = v_k.clone(); Py::Bool({ let t7017 = v_outputs.clone(); if !py::contains(&t7017, &t7016)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7060(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7062: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7061 = v_inputs.clone(); f_pid_parallel_to_standard(py::getitem(&t7061, &Py::str("Kp"))?, py::getitem(&t7061, &Py::str("Ki"))?, py::getitem(&t7061, &Py::str("Kd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7062 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7063 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7091 = (t7063).items()?;
        if t7091.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7091[0].clone();
        v_w = t7091[1].clone();
        if ({ let t7064 = v_k.clone(); Py::Bool({ let t7065 = v_outputs.clone(); if !py::contains(&t7065, &t7064)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7108(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7110: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7109 = v_inputs.clone(); f_pid_standard_to_parallel(py::getitem(&t7109, &Py::str("Kp"))?, py::getitem(&t7109, &Py::str("Ti"))?, py::getitem(&t7109, &Py::str("Td"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7110 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7111 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7139 = (t7111).items()?;
        if t7139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7139[0].clone();
        v_w = t7139[1].clone();
        if ({ let t7112 = v_k.clone(); Py::Bool({ let t7113 = v_outputs.clone(); if !py::contains(&t7113, &t7112)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7250(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7252: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7251 = v_inputs.clone(); f_pole_stability(py::getitem(&t7251, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7252 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7253 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7281 = (t7253).items()?;
        if t7281.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7281[0].clone();
        v_w = t7281[1].clone();
        if ({ let t7254 = v_k.clone(); Py::Bool({ let t7255 = v_outputs.clone(); if !py::contains(&t7255, &t7254)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7388(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7390: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7389 = v_inputs.clone(); f_poles_zeros(py::getitem(&t7389, &Py::str("num"))?, py::getitem(&t7389, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7390 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7391 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7419 = (t7391).items()?;
        if t7419.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7419[0].clone();
        v_w = t7419[1].clone();
        if ({ let t7392 = v_k.clone(); Py::Bool({ let t7393 = v_outputs.clone(); if !py::contains(&t7393, &t7392)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7438: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7437 = v_inputs.clone(); f_resonant_peak(py::getitem(&t7437, &Py::str("zeta"))?, py::getitem(&t7437, &Py::str("wn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7438 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7439 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7467 = (t7439).items()?;
        if t7467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7467[0].clone();
        v_w = t7467[1].clone();
        if ({ let t7440 = v_k.clone(); Py::Bool({ let t7441 = v_outputs.clone(); if !py::contains(&t7441, &t7440)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7505(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7507: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7506 = v_inputs.clone(); f_rise_time_second(py::getitem(&t7506, &Py::str("zeta"))?, py::getitem(&t7506, &Py::str("wn"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7507 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7508 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7536 = (t7508).items()?;
        if t7536.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7536[0].clone();
        v_w = t7536[1].clone();
        if ({ let t7509 = v_k.clone(); Py::Bool({ let t7510 = v_outputs.clone(); if !py::contains(&t7510, &t7509)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7572(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7574: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7573 = v_inputs.clone(); f_routh_hurwitz(py::getitem(&t7573, &Py::str("coefficients"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7574 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7575 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7603 = (t7575).items()?;
        if t7603.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7603[0].clone();
        v_w = t7603[1].clone();
        if ({ let t7576 = v_k.clone(); Py::Bool({ let t7577 = v_outputs.clone(); if !py::contains(&t7577, &t7576)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7622(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7624: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7623 = v_inputs.clone(); f_s_to_z_pole(py::getitem(&t7623, &Py::str("s_re"))?, py::getitem(&t7623, &Py::str("s_im"))?, py::getitem(&t7623, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7624 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7625 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7653 = (t7625).items()?;
        if t7653.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7653[0].clone();
        v_w = t7653[1].clone();
        if ({ let t7626 = v_k.clone(); Py::Bool({ let t7627 = v_outputs.clone(); if !py::contains(&t7627, &t7626)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7670(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7672: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7671 = v_inputs.clone(); f_scalar_lqr(py::getitem(&t7671, &Py::str("a"))?, py::getitem(&t7671, &Py::str("b"))?, py::getitem(&t7671, &Py::str("q"))?, py::getitem(&t7671, &Py::str("r"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7672 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7673 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7701 = (t7673).items()?;
        if t7701.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7701[0].clone();
        v_w = t7701[1].clone();
        if ({ let t7674 = v_k.clone(); Py::Bool({ let t7675 = v_outputs.clone(); if !py::contains(&t7675, &t7674)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7724(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7726: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7725 = v_inputs.clone(); f_second_order_freq(py::getitem(&t7725, &Py::str("zeta"))?, py::getitem(&t7725, &Py::str("wn"))?, py::getitem(&t7725, &Py::str("w"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7726 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7727 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7755 = (t7727).items()?;
        if t7755.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7755[0].clone();
        v_w = t7755[1].clone();
        if ({ let t7728 = v_k.clone(); Py::Bool({ let t7729 = v_outputs.clone(); if !py::contains(&t7729, &t7728)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7770(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7772: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7771 = v_inputs.clone(); f_second_order_from_poles(py::getitem(&t7771, &Py::str("sigma"))?, py::getitem(&t7771, &Py::str("wd"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7772 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7773 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7801 = (t7773).items()?;
        if t7801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7801[0].clone();
        v_w = t7801[1].clone();
        if ({ let t7774 = v_k.clone(); Py::Bool({ let t7775 = v_outputs.clone(); if !py::contains(&t7775, &t7774)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7838(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7840: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7839 = v_inputs.clone(); f_second_order_params(py::getitem(&t7839, &Py::str("den"))?, py::method(&t7839, "get", &[Py::str("K"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7840 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7841 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7869 = (t7841).items()?;
        if t7869.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7869[0].clone();
        v_w = t7869[1].clone();
        if ({ let t7842 = v_k.clone(); Py::Bool({ let t7843 = v_outputs.clone(); if !py::contains(&t7843, &t7842)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7895(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t7897: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t7896 = v_inputs.clone(); f_second_order_step(py::getitem(&t7896, &Py::str("zeta"))?, py::getitem(&t7896, &Py::str("wn"))?, py::getitem(&t7896, &Py::str("t"))?, py::method(&t7896, "get", &[Py::str("K"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7897 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t7898 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7926 = (t7898).items()?;
        if t7926.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7926[0].clone();
        v_w = t7926[1].clone();
        if ({ let t7899 = v_k.clone(); Py::Bool({ let t7900 = v_outputs.clone(); if !py::contains(&t7900, &t7899)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8098(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8100: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8099 = v_inputs.clone(); f_sensitivity_peak(py::getitem(&t8099, &Py::str("num"))?, py::getitem(&t8099, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8100 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8101 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8129 = (t8101).items()?;
        if t8129.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8129[0].clone();
        v_w = t8129[1].clone();
        if ({ let t8102 = v_k.clone(); Py::Bool({ let t8103 = v_outputs.clone(); if !py::contains(&t8103, &t8102)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8171(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8173: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8172 = v_inputs.clone(); f_series_connection(py::getitem(&t8172, &Py::str("num1"))?, py::getitem(&t8172, &Py::str("den1"))?, py::getitem(&t8172, &Py::str("num2"))?, py::getitem(&t8172, &Py::str("den2"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8173 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8174 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8202 = (t8174).items()?;
        if t8202.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8202[0].clone();
        v_w = t8202[1].clone();
        if ({ let t8175 = v_k.clone(); Py::Bool({ let t8176 = v_outputs.clone(); if !py::contains(&t8176, &t8175)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8224(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8226: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8225 = v_inputs.clone(); f_settling_time(py::getitem(&t8225, &Py::str("zeta"))?, py::getitem(&t8225, &Py::str("wn"))?, py::method(&t8225, "get", &[Py::str("band"), Py::Float(0.02)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8226 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8227 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8255 = (t8227).items()?;
        if t8255.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8255[0].clone();
        v_w = t8255[1].clone();
        if ({ let t8228 = v_k.clone(); Py::Bool({ let t8229 = v_outputs.clone(); if !py::contains(&t8229, &t8228)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8272(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8274: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8273 = v_inputs.clone(); f_simc_pi(py::getitem(&t8273, &Py::str("K"))?, py::getitem(&t8273, &Py::str("tau"))?, py::getitem(&t8273, &Py::str("theta"))?, py::method(&t8273, "get", &[Py::str("tau_c"), Py::None])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8274 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8275 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8303 = (t8275).items()?;
        if t8303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8303[0].clone();
        v_w = t8303[1].clone();
        if ({ let t8276 = v_k.clone(); Py::Bool({ let t8277 = v_outputs.clone(); if !py::contains(&t8277, &t8276)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8321(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8323: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8322 = v_inputs.clone(); f_specs_to_poles(py::getitem(&t8322, &Py::str("OS"))?, py::getitem(&t8322, &Py::str("Ts"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8323 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8324 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8352 = (t8324).items()?;
        if t8352.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8352[0].clone();
        v_w = t8352[1].clone();
        if ({ let t8325 = v_k.clone(); Py::Bool({ let t8326 = v_outputs.clone(); if !py::contains(&t8326, &t8325)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_835(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t837: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t836 = v_inputs.clone(); f_breakaway_points(py::getitem(&t836, &Py::str("num"))?, py::getitem(&t836, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t837 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t838 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t866 = (t838).items()?;
        if t866.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t866[0].clone();
        v_w = t866[1].clone();
        if ({ let t839 = v_k.clone(); Py::Bool({ let t840 = v_outputs.clone(); if !py::contains(&t840, &t839)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8413(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8414 = v_inputs.clone(); f_ss_dc_gain(py::getitem(&t8414, &Py::str("A"))?, py::getitem(&t8414, &Py::str("B"))?, py::getitem(&t8414, &Py::str("C"))?, py::method(&t8414, "get", &[Py::str("D"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8415 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8416 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8444 = (t8416).items()?;
        if t8444.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8444[0].clone();
        v_w = t8444[1].clone();
        if ({ let t8417 = v_k.clone(); Py::Bool({ let t8418 = v_outputs.clone(); if !py::contains(&t8418, &t8417)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8586(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8588: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8587 = v_inputs.clone(); f_ss_eigenvalues(py::getitem(&t8587, &Py::str("A"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8588 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8589 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8617 = (t8589).items()?;
        if t8617.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8617[0].clone();
        v_w = t8617[1].clone();
        if ({ let t8590 = v_k.clone(); Py::Bool({ let t8591 = v_outputs.clone(); if !py::contains(&t8591, &t8590)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8694(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8696: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8695 = v_inputs.clone(); f_ss_response(py::getitem(&t8695, &Py::str("A"))?, py::getitem(&t8695, &Py::str("B"))?, py::getitem(&t8695, &Py::str("x0"))?, py::getitem(&t8695, &Py::str("u"))?, py::getitem(&t8695, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8696 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8697 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8725 = (t8697).items()?;
        if t8725.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8725[0].clone();
        v_w = t8725[1].clone();
        if ({ let t8698 = v_k.clone(); Py::Bool({ let t8699 = v_outputs.clone(); if !py::contains(&t8699, &t8698)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8784(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8785 = v_inputs.clone(); f_ss_to_tf(py::getitem(&t8785, &Py::str("A"))?, py::getitem(&t8785, &Py::str("B"))?, py::getitem(&t8785, &Py::str("C"))?, py::method(&t8785, "get", &[Py::str("D"), Py::Float(0.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8786 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8787 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t8815 = (t8787).items()?;
        if t8815.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t8815[0].clone();
        v_w = t8815[1].clone();
        if ({ let t8788 = v_k.clone(); Py::Bool({ let t8789 = v_outputs.clone(); if !py::contains(&t8789, &t8788)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_8980(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t8982: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t8981 = v_inputs.clone(); f_stability_margins(py::getitem(&t8981, &Py::str("num"))?, py::getitem(&t8981, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t8982 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t8983 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9011 = (t8983).items()?;
        if t9011.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9011[0].clone();
        v_w = t9011[1].clone();
        if ({ let t8984 = v_k.clone(); Py::Bool({ let t8985 = v_outputs.clone(); if !py::contains(&t8985, &t8984)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9207(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9209: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9208 = v_inputs.clone(); f_stable_gain_range(py::getitem(&t9208, &Py::str("num"))?, py::getitem(&t9208, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9209 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9210 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9238 = (t9210).items()?;
        if t9238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9238[0].clone();
        v_w = t9238[1].clone();
        if ({ let t9211 = v_k.clone(); Py::Bool({ let t9212 = v_outputs.clone(); if !py::contains(&t9212, &t9211)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9383(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9385: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9384 = v_inputs.clone(); f_state_feedback_poles(py::getitem(&t9384, &Py::str("A"))?, py::getitem(&t9384, &Py::str("B"))?, py::getitem(&t9384, &Py::str("K"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9385 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9386 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9414 = (t9386).items()?;
        if t9414.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9414[0].clone();
        v_w = t9414[1].clone();
        if ({ let t9387 = v_k.clone(); Py::Bool({ let t9388 = v_outputs.clone(); if !py::contains(&t9388, &t9387)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9484(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9486: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9485 = v_inputs.clone(); f_state_transition(py::getitem(&t9485, &Py::str("A"))?, py::getitem(&t9485, &Py::str("t"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9486 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9487 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9515 = (t9487).items()?;
        if t9515.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9515[0].clone();
        v_w = t9515[1].clone();
        if ({ let t9488 = v_k.clone(); Py::Bool({ let t9489 = v_outputs.clone(); if !py::contains(&t9489, &t9488)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9669(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9671: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9670 = v_inputs.clone(); f_step_response(py::getitem(&t9670, &Py::str("num"))?, py::getitem(&t9670, &Py::str("den"))?, py::getitem(&t9670, &Py::str("t"))?, py::method(&t9670, "get", &[Py::str("A"), Py::Float(1.0)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9671 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9672 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9700 = (t9672).items()?;
        if t9700.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9700[0].clone();
        v_w = t9700[1].clone();
        if ({ let t9673 = v_k.clone(); Py::Bool({ let t9674 = v_outputs.clone(); if !py::contains(&t9674, &t9673)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9740(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9742: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9741 = v_inputs.clone(); f_tf_evaluate(py::getitem(&t9741, &Py::str("num"))?, py::getitem(&t9741, &Py::str("den"))?, py::getitem(&t9741, &Py::str("s_re"))?, py::getitem(&t9741, &Py::str("s_im"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9742 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9743 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9771 = (t9743).items()?;
        if t9771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9771[0].clone();
        v_w = t9771[1].clone();
        if ({ let t9744 = v_k.clone(); Py::Bool({ let t9745 = v_outputs.clone(); if !py::contains(&t9745, &t9744)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9813(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9815: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9814 = v_inputs.clone(); f_tf_to_ss(py::getitem(&t9814, &Py::str("num"))?, py::getitem(&t9814, &Py::str("den"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9815 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9816 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9844 = (t9816).items()?;
        if t9844.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9844[0].clone();
        v_w = t9844[1].clone();
        if ({ let t9817 = v_k.clone(); Py::Bool({ let t9818 = v_outputs.clone(); if !py::contains(&t9818, &t9817)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9892(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9894: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9893 = v_inputs.clone(); f_true_settling_time(py::getitem(&t9893, &Py::str("zeta"))?, py::getitem(&t9893, &Py::str("wn"))?, py::method(&t9893, "get", &[Py::str("band"), Py::Float(0.02)])?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9894 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9895 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t9923 = (t9895).items()?;
        if t9923.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t9923[0].clone();
        v_w = t9923[1].clone();
        if ({ let t9896 = v_k.clone(); Py::Bool({ let t9897 = v_outputs.clone(); if !py::contains(&t9897, &t9896)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_9985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    let t9987: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            v_want = { let t9986 = v_inputs.clone(); f_tustin(py::getitem(&t9986, &Py::str("num"))?, py::getitem(&t9986, &Py::str("den"))?, py::getitem(&t9986, &Py::str("T"))?)? };
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t9987 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
            return Ok(c_INF()?);
        }
        Err(e) => return Err(e),
    }
    v_worst = Py::Float(0.0);
    for t9988 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t10016 = (t9988).items()?;
        if t10016.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t10016[0].clone();
        v_w = t10016[1].clone();
        if ({ let t9989 = v_k.clone(); Py::Bool({ let t9990 = v_outputs.clone(); if !py::contains(&t9990, &t9989)? { true } else { false } }) }).truthy() {
            return Ok(c_INF()?);
        }
        v_worst = py::minmax(&[v_worst.clone(), f__diff(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_w.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

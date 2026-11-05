//! Every Control Systems capability's published figure kinds (src/figures.rs, src/figures_charts.rs, and the generic
//! engine's figures for the kinds it already draws): its worked example gives each kind, a changed input changes the
//! drawing, and no drawing holds a non-finite number or text outside its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use control_systems_ai999x::pack;

/// The published figure kinds drawn by the model's own code (src/figures.rs, src/figures_charts.rs), as
/// tools/figure_parity.py lists them (publish/website/models/control-systems-ai999x/figures, except "curve").
const KINDS: &[(&str, &[&str])] = &[
    ("ctrl_ackermann", &["matrix"]),
    ("ctrl_backward_euler", &["vectors"]),
    ("ctrl_bandwidth_second", &["field"]),
    ("ctrl_breakaway_points", &["io"]),
    ("ctrl_c2d_foh", &["vectors"]),
    ("ctrl_c2d_zoh", &["vectors"]),
    ("ctrl_dc_motor", &["complex"]),
    ("ctrl_deadbeat_gain", &["matrix", "vectors"]),
    ("ctrl_discrete_kalman_gain", &["io"]),
    ("ctrl_dlqr", &["complex", "vectors"]),
    ("ctrl_final_value_theorem", &["io"]),
    ("ctrl_forward_euler", &["vectors"]),
    ("ctrl_initial_value_theorem", &["io"]),
    ("ctrl_jury_test", &["data"]),
    ("ctrl_kalman_bucy", &["complex", "vectors"]),
    ("ctrl_lag_design", &["phase-deg-vs-beta"]),
    ("ctrl_lead_max_phase", &["field"]),
    ("ctrl_locus_gain_at_point", &["field"]),
    ("ctrl_lqr", &["complex", "vectors"]),
    ("ctrl_nyquist", &["decision"]),
    ("ctrl_observer_gain", &["matrix"]),
    ("ctrl_parallel_connection", &["vectors"]),
    ("ctrl_peak_time", &["field"]),
    ("ctrl_percent_overshoot", &["po-vs-zeta"]),
    ("ctrl_pid_closed_loop", &["complex", "vectors"]),
    ("ctrl_pole_stability", &["complex"]),
    ("ctrl_poles_zeros", &["complex"]),
    ("ctrl_rise_time_second", &["field"]),
    ("ctrl_s_to_z_pole", &["z-mag-vs-s-re"]),
    ("ctrl_second_order_from_poles", &["field"]),
    ("ctrl_settling_time", &["field"]),
    ("ctrl_specs_to_poles", &["wn-vs-os"]),
    ("ctrl_ss_dc_gain", &["matrix"]),
    ("ctrl_ss_eigenvalues", &["complex", "matrix"]),
    ("ctrl_ss_response", &["matrix", "vectors"]),
    ("ctrl_ss_to_tf", &["matrix"]),
    ("ctrl_state_feedback_poles", &["complex", "vectors"]),
    ("ctrl_tf_evaluate", &["field"]),
    ("ctrl_tf_to_ss", &["matrix"]),
    ("ctrl_true_settling_time", &["field"]),
    ("ctrl_tustin", &["vectors"]),
    ("ctrl_tustin_prewarp", &["vectors"]),
];

/// The published kinds the generic engine already draws (its sweeps, matrices, series). They are checked on the
/// WebAssembly module by tools/figure_parity.py; natively the generic figures stop at a wall-clock limit, so on a
/// loaded test machine one may be left out: here each one found is checked for its drawing, a missing one only
/// reported.
const GENERIC: &[(&str, &[&str])] = &[
    ("ctrl_arrival_angle", &["sensitivity"]),
    ("ctrl_bandwidth", &["sensitivity"]),
    ("ctrl_c2d_zoh", &["series"]),
    ("ctrl_controllability", &["matrix"]),
    ("ctrl_ctrb_gramian", &["matrix"]),
    ("ctrl_dc_gain", &["sensitivity"]),
    ("ctrl_departure_angle", &["sensitivity"]),
    ("ctrl_discrete_dc_gain", &["sensitivity"]),
    ("ctrl_discrete_lyapunov", &["matrix"]),
    ("ctrl_error_constants", &["sensitivity"]),
    ("ctrl_feedback_connection", &["series"]),
    ("ctrl_forward_euler", &["series"]),
    ("ctrl_h2_norm", &["sensitivity"]),
    ("ctrl_hinf_norm", &["sensitivity"]),
    ("ctrl_locus_asymptotes", &["series"]),
    ("ctrl_lyapunov", &["matrix"]),
    ("ctrl_mass_spring_damper", &["decision"]),
    ("ctrl_observability", &["matrix"]),
    ("ctrl_obsv_gramian", &["matrix"]),
    ("ctrl_pade", &["series"]),
    ("ctrl_parallel_connection", &["series"]),
    ("ctrl_routh_hurwitz", &["series"]),
    ("ctrl_second_order_params", &["sensitivity"]),
    ("ctrl_sensitivity_peak", &["sensitivity"]),
    ("ctrl_series_connection", &["series"]),
    ("ctrl_specs_to_poles", &["series"]),
    ("ctrl_ss_to_tf", &["series"]),
    ("ctrl_stability_margins", &["sensitivity"]),
    ("ctrl_stable_gain_range", &["sensitivity"]),
    ("ctrl_state_transition", &["matrix"]),
    ("ctrl_tf_to_ss", &["series"]),
    ("ctrl_zn_from_plant", &["sensitivity"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("ctrl_jury_test", "coefficients", "[1, -0.5, 0.06]"),
    ("ctrl_pole_stability", "den", "[1, 2, 5]"),
    ("ctrl_ss_eigenvalues", "A", "[[0, 1], [-5, -2]]"),
];

/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else {
            // a list or matrix ([[0, 1], [0, 0]]) runs to its closing bracket
            let s0 = i;
            let mut depth = 0i32;
            while i < cs.len() && (depth > 0 || (cs[i] != ',' && cs[i] != ')')) {
                match cs[i] {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    _ => {}
                }
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn kind_of(f: &Figure) -> &str {
    &f.kind
}

/// Every text inside the canvas, no non-finite number (ax_engine::figkit::check_figure).
fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // a non-finite number or text outside the canvas ("inf" alone would match ctrl-hinf-norm)
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        // the model's own figures come first
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

#[test]
fn the_generic_kinds_are_drawn_cleanly() {
    let p = pack();
    let mut left_out = vec![];
    for (key, kinds) in GENERIC {
        let figs = figures(p, key, &example(key), None, None);
        for kind in *kinds {
            match figs.iter().find(|f| kind_of(f) == *kind) {
                Some(f) => check_texts(f),
                None => left_out.push(format!("{key}: {kind}")),
            }
        }
    }
    if !left_out.is_empty() {
        eprintln!("generic figures left out at the wall-clock limit: {left_out:?}");
    }
}

/// Numbers in a text multiplied by f ("12 V" -> "16.44 V"; "3, 6" -> "4.11, 8.22").
fn scale_text(t: &str, f: f64) -> Option<String> {
    let mut out = String::new();
    let mut cur = String::new();
    let mut changed = false;
    let flush = |cur: &mut String, out: &mut String, changed: &mut bool| {
        if !cur.is_empty() {
            match cur.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    *changed = true;
                }
                _ => out.push_str(cur),
            }
            cur.clear();
        }
    };
    for c in t.chars() {
        if c.is_ascii_digit() || c == '.' || (c == '-' && cur.is_empty()) || ((c == 'e' || c == 'E') && !cur.is_empty()) {
            cur.push(c);
        } else {
            flush(&mut cur, &mut out, &mut changed);
            out.push(c);
        }
    }
    flush(&mut cur, &mut out, &mut changed);
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        // the other example: a listed replacement, else each number input scaled until the model solves it
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        if let Some((_, sym, val)) = OTHER.iter().find(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn an_impossible_input_draws_nothing_wrong() {
    // a damping ratio of 1.5 has no peak time: the model refuses it, and no figure is drawn
    let p = pack();
    let given: Vec<(String, Input)> = [("zeta", "1.5"), ("wn", "5")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "ctrl_peak_time", &given, None);
    assert!(!a.solved());
    assert!(figures(p, "ctrl_peak_time", &given, None, None).is_empty());
}

#[test]
fn a_vector_figure_needs_two_entries() {
    // a third-order system's state is not a plane vector: the vectors figure is left out, the matrices stay
    let p = pack();
    let given: Vec<(String, Input)> = [("A", "[[0, 1, 0], [0, 0, 1], [0, 0, 0]]"), ("B", "[0, 0, 1]"),
                                       ("x0", "[0, 0, 0]"), ("u", "1"), ("t", "2")]
        .iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, "ctrl_ss_response", &given, None);
    assert!(a.solved());
    let figs = figures(p, "ctrl_ss_response", &given, None, None);
    assert!(figs.iter().any(|f| f.kind == "matrix") && !figs.iter().any(|f| f.kind == "vectors"));
}

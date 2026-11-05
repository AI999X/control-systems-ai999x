//! Control Systems' own live figures (ax_engine::figkit): every capability's published figure kinds, drawn for the
//! visitor's inputs from what the model computed (system matrices with the gains computed, poles in the complex plane,
//! discretized coefficients as plane vectors, inputs and results, ...). Written by hand for the Rust build. A figure
//! the inputs make impossible (a vector that is not two entries long, a matrix too large) is left out, never drawn
//! wrong. Charts over one or two inputs are in figures_charts.rs.

use crate::figures_charts as ch;
use ax_engine::figkit::{self as k, num, Canvas, Solved, P};
use ax_engine::figures::Figure;
use ax_engine::pack::Val;
use serde_json::Value;

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    use Side::{In, Out};
    let v: Vec<Option<Figure>> = match s.key {
        // the system as matrices, with what was computed from it
        "ctrl_ackermann" => vec![mats(s, &[("A", In), ("B", In), ("K", Out)], "Pole placement by Ackermann's formula")],
        "ctrl_deadbeat_gain" => vec![mats(s, &[("A", In), ("B", In), ("K", Out)], "Deadbeat state feedback"),
                                     vecs(s, &["B"], &["K"])],
        "ctrl_observer_gain" => vec![mats(s, &[("A", In), ("C", In), ("L", Out)], "Observer gain by pole placement")],
        "ctrl_ss_dc_gain" => vec![mats(s, &[("A", In), ("B", In), ("C", In)], "DC gain of a state-space model")],
        "ctrl_ss_eigenvalues" => vec![poles(s, &[("eigenvalues", "λ")], None, "Eigenvalues of the state matrix"),
                                      mats(s, &[("A", In), ("eigenvalues", Out)], "Eigenvalues of the state matrix")],
        "ctrl_ss_response" => vec![mats(s, &[("A", In), ("B", In), ("x0", In), ("x", Out)],
                                        "State response to an initial state and a constant input"),
                                   vecs(s, &["B", "x0"], &["x"])],
        "ctrl_ss_to_tf" => vec![mats(s, &[("A", In), ("B", In), ("C", In)], "Transfer function of a state-space model")],
        "ctrl_tf_to_ss" => vec![mats(s, &[("A", Out), ("B", Out), ("C", Out)], "Controllable canonical form")],
        // discretized and combined transfer functions: their coefficient pairs as plane vectors
        "ctrl_backward_euler" | "ctrl_forward_euler" | "ctrl_tustin" | "ctrl_tustin_prewarp" | "ctrl_c2d_zoh"
        | "ctrl_c2d_foh" => vec![vecs(s, &["num", "den"], &["numz", "denz"])],
        "ctrl_parallel_connection" => vec![vecs(s, &["num1", "den1", "num2", "den2"], &["num", "den"])],
        // poles in the complex plane (with how they move with one input, where there is one)
        "ctrl_dc_motor" => vec![poles(s, &[("poles", "pole")], Some("J"), "Speed transfer function of a DC motor")],
        "ctrl_dlqr" => vec![poles(s, &[("poles", "pole")], Some("R"), "Discrete-time LQR: closed-loop poles"),
                            vecs(s, &["B"], &["K"])],
        "ctrl_lqr" => vec![poles(s, &[("poles", "pole")], Some("R"), "Continuous-time LQR: closed-loop poles"),
                           vecs(s, &["B"], &["K"])],
        "ctrl_kalman_bucy" => vec![poles(s, &[("poles", "pole")], Some("V"), "Kalman-Bucy filter: estimator poles"),
                                   vecs(s, &["C"], &["L"])],
        "ctrl_pid_closed_loop" => vec![poles(s, &[("poles", "pole")], Some("Kp"), "Closed-loop poles of a PID loop"),
                                       vecs(s, &["den"], &["poles"])],
        "ctrl_pole_stability" => vec![poles(s, &[("poles", "pole")], None, "Poles of a characteristic polynomial")],
        "ctrl_poles_zeros" => vec![poles(s, &[("zeros", "zero"), ("poles", "pole")], None,
                                         "Poles and zeros of a transfer function")],
        "ctrl_state_feedback_poles" => vec![poles(s, &[("poles", "pole")], None, "Closed-loop poles under state feedback"),
                                            vecs(s, &["B", "K"], &["poles"])],
        // inputs and results
        "ctrl_breakaway_points" => vec![io(s, "Root-locus breakaway and break-in points")],
        "ctrl_discrete_kalman_gain" => vec![io(s, "Steady-state discrete Kalman filter gain")],
        "ctrl_final_value_theorem" => vec![io(s, "Final value theorem")],
        "ctrl_initial_value_theorem" => vec![io(s, "Initial value theorem")],
        "ctrl_jury_test" => vec![jury(s)],
        _ => ch::live(s),
    };
    v.into_iter().flatten().collect()
}

#[derive(Clone, Copy, PartialEq)]
pub enum Side {
    In,
    Out,
}

// ------------------------------------------------------------------------------------------------ values as text
/// A returned value as text: numbers to 4 digits, lists in brackets, complex numbers as a + bj.
pub fn jtxt(v: &Value) -> String {
    match v {
        Value::Number(n) => n.as_f64().map(num).unwrap_or_default(),
        Value::Bool(b) => if *b { "yes".into() } else { "no".into() },
        Value::String(t) => t.clone(),
        Value::Null => "none".into(),
        Value::Array(a) => format!("[{}]", a.iter().map(jtxt).collect::<Vec<_>>().join(", ")),
        Value::Object(o) => match (o.get("re").and_then(|x| x.as_f64()), o.get("im").and_then(|x| x.as_f64())) {
            (Some(re), Some(im)) => cx((re, im)),
            _ => v.to_string(),
        },
    }
}

/// An input as read, as text.
pub fn vtxt(v: &Val) -> String {
    let l = |x: &[f64]| x.iter().map(|y| num(*y)).collect::<Vec<_>>().join(", ");
    match v {
        Val::Num(x) => num(*x),
        Val::Int(i) => i.to_string(),
        Val::Text(t) => t.clone(),
        Val::Bool(b) => if *b { "yes".into() } else { "no".into() },
        Val::Vec(x) => format!("[{}]", l(x)),
        Val::Mat(m) => format!("[{}]", m.iter().map(|r| format!("[{}]", l(r))).collect::<Vec<_>>().join(", ")),
        Val::Edges(e) => format!("{} edges", e.len()),
    }
}

/// A complex number as text: "-2", "-0.866 + 0.5j".
pub fn cx((re, im): P) -> String {
    let re = k::snap(re, im);
    let im = k::snap(im, re);
    if im == 0.0 {
        num(re)
    } else if im > 0.0 {
        format!("{} + {}j", num(re), num(im))
    } else {
        format!("{} - {}j", num(re), num(-im))
    }
}

// ------------------------------------------------------------------------------------------------ matrices
/// A named entry as a matrix: a matrix as is, a list as a column (a row for gains K and output vectors C), a list of
/// complex numbers as two columns (real, imaginary).
fn as_matrix(s: &Solved, sym: &str, side: Side) -> Option<(Vec<Vec<f64>>, Vec<String>)> {
    let row = matches!(sym, "K" | "C");
    let vec_to = |v: Vec<f64>| -> (Vec<Vec<f64>>, Vec<String>) {
        if row {
            let n = v.len();
            (vec![v], (1..=n).map(|j| format!("{sym}{j}")).collect())
        } else {
            (v.into_iter().map(|x| vec![x]).collect(), vec![if sym == "eigenvalues" { "λ".into() } else { sym.to_string() }])
        }
    };
    match side {
        Side::In => {
            if let Some(m) = s.mat(sym) {
                let c = m[0].len();
                return Some((m, (1..=c).map(|j| format!("{sym}{j}")).collect()));
            }
            s.vec(sym).map(vec_to)
        }
        Side::Out => {
            let v = s.outj(sym)?.as_array()?;
            if v.is_empty() {
                return None;
            }
            if v.iter().all(|r| r.is_array()) {
                let m: Vec<Vec<f64>> = v.iter().map(|r| r.as_array().map(|x| x.iter().filter_map(|y| y.as_f64())
                    .collect()).unwrap_or_default()).collect();
                let c = m[0].len();
                if c == 0 || m.iter().any(|r| r.len() != c) {
                    return None;
                }
                return Some((m, (1..=c).map(|j| format!("{sym}{j}")).collect()));
            }
            if let Some(l) = s.out_list(sym) {
                return Some(vec_to(l));
            }
            let z = k::cx_list(s.a, sym)?;
            Some((z.iter().map(|p| vec![p.0, p.1]).collect(), vec!["Re".into(), "Im".into()]))
        }
    }
}

/// The scalar and list outputs not drawn as a panel, as "name = value" (the header and caption).
fn others(s: &Solved, drawn: &[&str]) -> Vec<String> {
    s.a.values.iter().filter(|x| !drawn.contains(&x.0.as_str())).map(|x| format!("{} = {}", x.0, jtxt(&x.1))).collect()
}

/// The given and computed matrices side by side as heat maps (one gap between panels), each cell shaded by its
/// magnitude.
fn mats(s: &Solved, panels: &[(&str, Side)], what: &str) -> Option<Figure> {
    let mut ps = vec![];
    for (sym, side) in panels {
        let Some((m, cols)) = as_matrix(s, sym, *side) else { continue };
        if m.iter().flatten().any(|x| !x.is_finite()) {
            return None;
        }
        ps.push((*sym, *side, m, cols));
    }
    if !ps.iter().any(|p| p.0 == "A") || ps.len() < 2 && panels.len() > 1 {
        return None;
    }
    let rows = ps.iter().map(|p| p.2.len()).max()?;
    let ncol: usize = ps.iter().map(|p| p.3.len()).sum::<usize>() + ps.len() - 1;
    if rows == 0 || rows > 10 || ncol > 16 {
        return None;
    }
    let mut m = vec![vec![]; rows];
    let mut cols = vec![];
    for (i, (_, _, pm, pc)) in ps.iter().enumerate() {
        if i > 0 {
            cols.push(String::new());
            for r in m.iter_mut() {
                r.push(f64::NAN);
            }
        }
        cols.extend(pc.iter().cloned());
        for (r, row) in m.iter_mut().enumerate() {
            for j in 0..pc.len() {
                row.push(pm.get(r).and_then(|x| x.get(j)).copied().unwrap_or(f64::NAN));
            }
        }
    }
    let names: Vec<&str> = ps.iter().map(|p| p.0).collect();
    let given: Vec<&str> = ps.iter().filter(|p| p.1 == Side::In).map(|p| p.0).collect();
    let got: Vec<&str> = ps.iter().filter(|p| p.1 == Side::Out).map(|p| p.0).collect();
    let rest = others(s, &names);
    let header = if !got.is_empty() {
        got.iter().map(|g| format!("{g} = {}", s.outj(g).map(jtxt).unwrap_or_default())).collect::<Vec<_>>().join("; ")
    } else {
        rest.first().cloned().unwrap_or_default()
    };
    let rows_l: Vec<String> = (1..=rows).map(|i| i.to_string()).collect();
    let body = k::heat(&m, &rows_l, &cols, &hdr(&header));
    let mut parts = vec![];
    if !given.is_empty() {
        parts.push(format!("{} as given", given.join(", ")));
    }
    if !got.is_empty() {
        parts.push(format!("{} as computed by the model", got.join(", ")));
    }
    let also = if rest.is_empty() { String::new() } else { format!(" Also computed: {}.", rest.join("; ")) };
    Some(s.chart("matrix", &format!("{what}: the matrices"),
                 &format!("The matrices side by side as heat maps ({}), rows and columns numbered, each cell shaded by \
                           its magnitude (negative values in the second colour).{}{also}", parts.join("; "),
                          if got.is_empty() { String::new() } else { format!(" Computed: {header}.") }), &body))
}

// ------------------------------------------------------------------------------------------------ vectors
/// A list of two entries as a plane vector (complex entries only when both are real).
fn pair(s: &Solved, sym: &str, side: Side) -> Option<P> {
    let v = match side {
        Side::In => s.vec(sym)?,
        Side::Out => match s.out_list(sym) {
            Some(l) => l,
            None => {
                let z = k::cx_list(s.a, sym)?;
                if z.iter().any(|p| k::snap(p.1, p.0) != 0.0) {
                    return None;
                }
                z.iter().map(|p| p.0).collect()
            }
        },
    };
    (v.len() == 2 && (v[0] != 0.0 || v[1] != 0.0)).then(|| (v[0], v[1]))
}

/// The two-entry inputs and outputs as arrows from the origin; drawn when at least one output has two entries.
fn vecs(s: &Solved, ins: &[&str], outs: &[&str]) -> Option<Figure> {
    let mut v = vec![];
    for sym in ins {
        if let Some(p) = pair(s, sym, Side::In) {
            v.push((sym.to_string(), p, false));
        }
    }
    let mut got = vec![];
    for sym in outs {
        if let Some(p) = pair(s, sym, Side::Out) {
            v.push((sym.to_string(), p, true));
            got.push(format!("{sym} = ({}, {})", num(p.0), num(p.1)));
        }
    }
    if got.is_empty() {
        return None;
    }
    let given: Vec<String> = v.iter().filter(|x| !x.2).map(|x| format!("{} = ({}, {})", x.0, num(x.1 .0), num(x.1 .1)))
        .collect();
    let body = k::arrows(&v, &hdr(&got.join(", ")));
    let name = s.p.procedure(s.key).map(|p| p.name.clone()).unwrap_or_default();
    Some(s.chart("vectors", &format!("{name}: the vectors"),
                 &format!("The two-entry lists as plane vectors from the origin (first entry across, second up): {}\
                           computed by the model: {}; the tip of each computed vector is marked.",
                          if given.is_empty() { String::new() } else { format!("given {}; ", given.join(", ")) },
                          got.join(", ")), &body))
}

// ------------------------------------------------------------------------------------------------ complex plane
/// The returned roots in the complex plane, each labelled; with `sweep`, also the roots for 25 values of that input
/// from a thirtieth to thirty times the one given (each set one solve of the model), as small dots.
fn poles(s: &Solved, syms: &[(&str, &str)], sweep: Option<&str>, what: &str) -> Option<Figure> {
    // each root marked with a short tag (p1, p2, z1, ...), its value in the heading and the caption
    let mut mine = vec![];
    let mut list = vec![];
    for (sym, label) in syms {
        if let Some(z) = k::cx_list(s.a, sym) {
            let tag = if *label == "λ" { "λ" } else { &label[..1] };
            for (i, p) in z.into_iter().enumerate() {
                mine.push((format!("{tag}{}", i + 1), p));
                let named = if *label == tag { String::new() } else { format!("{label} ") };
                list.push(format!("{named}{tag}{} = {}", i + 1, cx(p)));
            }
        }
    }
    if mine.is_empty() || mine.len() > 12 {
        return None;
    }
    let mut dots = vec![];
    let mut swept = String::new();
    if let Some(par) = sweep {
        if let Some(p0) = s.inp(par).filter(|x| *x > 0.0) {
            for v in k::span(p0 / 5.0, p0 * 5.0, 25, true) {
                if let Some(a) = s.with(&[(par, v)]) {
                    if let Some(z) = k::cx_list(&a, syms[0].0) {
                        dots.extend(z);
                    }
                }
            }
            if !dots.is_empty() {
                swept = format!(" Small dots: the {} for 25 values of {par} from {} to {} (logarithmic steps), each set \
                                 computed by the model.", syms[0].0, num(p0 / 5.0), num(p0 * 5.0));
            }
        }
    }
    let header = mine.iter().map(|m| format!("{} = {}", m.0, cx(m.1))).collect::<Vec<_>>().join(", ");
    let verdict = ["stable", "stability"].iter().find_map(|e| s.out_text(e).map(|t| format!(" ({e}: {t})")))
        .unwrap_or_default();
    let shown: Vec<(String, P)> = mine.iter().take(6).cloned().collect();
    let body = k::plane(&dots, &shown, "Real part", "Imaginary part", &hdr(&header));
    Some(s.chart("complex", &format!("{what}, in the complex plane"),
                 &format!("The {} computed by the model, drawn in the complex plane (real part across, imaginary part \
                           up): {}{verdict}.{swept}", syms.iter().map(|x| x.0).collect::<Vec<_>>().join(" and "),
                          list.join(", ")), &body))
}

// ------------------------------------------------------------------------------------------------ inputs and results
/// The inputs as given (left) and the results the model computed (right), joined by an arrow through the
/// capability.
fn io(s: &Solved, what: &str) -> Option<Figure> {
    let spec = s.p.procedure(s.key)?;
    let ins: Vec<(String, String)> = spec.inputs.iter()
        .filter_map(|i| s.val(&i.symbol).map(|v| vtxt(&v)).or_else(|| s.inp(&i.symbol).map(num))
            .map(|t| (i.symbol.clone(), t))).collect();
    let outs: Vec<(String, String)> = s.a.values.iter().map(|x| (x.0.clone(), jtxt(&x.1))).collect();
    if ins.is_empty() || outs.is_empty() || ins.len() > 8 || outs.len() > 8 {
        return None;
    }
    let n = ins.len().max(outs.len());
    let (top, bot) = (52.0, 52.0 + 40.0 * n.max(2) as f64 + 8.0);
    let h = bot + 18.0;
    let mut c = Canvas::new(h);
    c.bold(110.0, 30.0, "middle", "Given");
    c.bold(410.0, 30.0, "middle", "Computed");
    for (x0, list) in [(18.0, &ins), (318.0, &outs)] {
        c.raw(&format!("<rect class=\"axp-k\" x=\"{x0:.1}\" y=\"{top:.1}\" width=\"184\" height=\"{:.1}\" rx=\"6\"/>",
                       bot - top));
        for (i, (name, val)) in list.iter().enumerate() {
            let y = top + 22.0 + 40.0 * i as f64;
            c.name(x0 + 10.0, y, "start", name);
            c.value(x0 + 10.0, y + 16.0, "start", &clip_chars(val, 21));
        }
    }
    let mid = (top + bot) / 2.0;
    c.block(222.0, mid - 26.0, 76.0, 52.0, "model");
    c.arrow((202.0, mid), (222.0, mid));
    c.arrow((298.0, mid), (318.0, mid));
    let body = c.body().to_string();
    let lst = |v: &[(String, String)]| v.iter().map(|x| format!("{} = {}", x.0, x.1)).collect::<Vec<_>>().join(", ");
    Some(s.figure("io", &format!("{what}: inputs and results"),
                  &format!("The inputs as given ({}) and the results computed by the model ({}).", lst(&ins), lst(&outs)),
                  &body, h))
}

/// A chart heading short enough for the frame (it starts at the plot's left edge).
pub fn hdr(t: &str) -> String {
    clip_chars(t, 46)
}

/// At most n characters, an ellipsis marking a cut.
pub fn clip_chars(t: &str, n: usize) -> String {
    if t.chars().count() <= n { t.to_string() } else { format!("{}…", t.chars().take(n - 1).collect::<String>()) }
}

/// The polynomial's coefficients as bars by power of z, with the test values the model returned.
fn jury(s: &Solved) -> Option<Figure> {
    let c = s.vec("coefficients")?;
    let n = c.len();
    if n < 2 || n > 16 {
        return None;
    }
    let (p1, pm1) = (s.out("P1")?, s.out("Pm1")?);
    let stable = s.out_text("stable").unwrap_or_default();
    let labels: Vec<String> = (0..n).map(|i| match n - 1 - i {
        0 => "1".to_string(),
        1 => "z".to_string(),
        p => format!("z^{p}"),
    }).collect();
    let body = k::bars(&labels, &c, &[], "Power of z", "Coefficient",
                       &format!("P(1) = {}, P(-1) = {}, stable: {stable}", num(p1), num(pm1)));
    Some(s.chart("data", "Jury stability test: the polynomial's coefficients",
                 &format!("The coefficients of the polynomial in z as given, highest power first ({n} values), one bar \
                           each; computed by the model: P(1) = {}, P(-1) = {}, stable: {stable}.", num(p1), num(pm1)),
                 &body))
}

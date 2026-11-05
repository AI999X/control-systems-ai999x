//! Control Systems' own live figures, part two: charts over one or two inputs (value fields of second-order
//! specifications, curves such as the percent overshoot against the damping ratio, the Nyquist verdict over the gain).
//! Every point is one further solve of the model itself (ax_engine::figkit::Solved), never a formula written here.

use crate::figures::hdr;
use ax_engine::figkit::{self as k, num, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;
use serde_json::json;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    use Rng::{Decade, Fixed, Near};
    match s.key {
        "ctrl_bandwidth_second" => vec![field2(s, "wn", Near, "zeta", Fixed(0.02, 0.98), "wb",
                                               "Bandwidth of a second-order system over natural frequency and damping")],
        "ctrl_lead_max_phase" => vec![field2(s, "z", Decade, "p", Decade, "phi_max_deg",
                                             "Maximum phase lead over the zero and pole corner frequencies")],
        "ctrl_locus_gain_at_point" => vec![field2(s, "s_im", Decade, "s_re", Near, "K",
                                                  "Root-locus gain over the test point")],
        "ctrl_peak_time" => vec![field2(s, "wn", Near, "zeta", Fixed(0.02, 0.98), "Tp",
                                        "Peak time over natural frequency and damping")],
        "ctrl_rise_time_second" => vec![field2(s, "wn", Near, "zeta", Fixed(0.02, 0.98), "Tr_10_90",
                                               "10-90 % rise time over natural frequency and damping")],
        "ctrl_second_order_from_poles" => vec![field2(s, "sigma", Decade, "wd", Decade, "zeta",
                                                      "Damping ratio over the pole location")],
        "ctrl_settling_time" => vec![field2(s, "zeta", Fixed(0.02, 0.98), "wn", Near, "Ts",
                                            "Settling time over damping and natural frequency")],
        "ctrl_true_settling_time" => vec![field2(s, "zeta", Fixed(0.02, 0.98), "wn", Near, "Ts",
                                                 "Exact settling time over damping and natural frequency")],
        "ctrl_tf_evaluate" => vec![field2(s, "s_im", Decade, "s_re", Near, "magnitude",
                                          "Magnitude of G(s) over the complex point s")],
        "ctrl_lag_design" => vec![curve1(s, "phase-deg-vs-beta", "beta", Span::LogFrom(1.05), "phase_deg",
                                         "Phase added at the crossover against the gain increase")],
        "ctrl_percent_overshoot" => vec![curve1(s, "po-vs-zeta", "zeta", Span::Fixed(0.0, 0.99), "PO",
                                                "Percent overshoot against the damping ratio")],
        "ctrl_s_to_z_pole" => vec![curve1(s, "z-mag-vs-s-re", "s_re", Span::Around, "z_mag",
                                          "Distance of the z-plane pole from the origin against the s-plane real part")],
        "ctrl_specs_to_poles" => vec![curve1(s, "wn-vs-os", "OS", Span::Fixed(0.01, 0.6), "wn",
                                             "Natural frequency of the desired poles against the overshoot")],
        "ctrl_nyquist" => vec![nyquist(s)],
        _ => vec![],
    }
}

/// The unit of an input or output as the pack names it ("" for a pure number).
fn unit_of(s: &Solved, sym: &str) -> String {
    let Some(p) = s.p.procedure(s.key) else { return String::new() };
    let kind = p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.kind.clone())
        .or_else(|| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.kind.clone()));
    match kind.and_then(|kd| s.p.kinds.get(&kd).map(|x| x.unit.clone())) {
        Some(u) if u != "1" => u,
        _ => String::new(),
    }
}

fn label(sym: &str, unit: &str) -> String {
    if unit.is_empty() { sym.to_string() } else { format!("{sym} ({unit})") }
}

fn with_unit(x: f64, unit: &str) -> String {
    if unit.is_empty() { num(x) } else { format!("{} {unit}", num(x)) }
}

fn meaning(s: &Solved, sym: &str) -> String {
    let p = s.p.procedure(s.key);
    p.and_then(|p| p.outputs.iter().find(|o| o.symbol == sym).map(|o| o.meaning.clone())
        .or_else(|| p.inputs.iter().find(|i| i.symbol == sym).map(|i| i.meaning.clone())))
        .unwrap_or_else(|| sym.to_string())
}

// ------------------------------------------------------------------------------------------------ value fields
pub enum Rng {
    /// half to one and a half times the value (-1 to 1 for zero)
    Near,
    /// a tenth to ten times (logarithmic)
    Decade,
    /// a fixed span, widened to hold the value given
    Fixed(f64, f64),
}

fn range_of(x0: f64, r: &Rng) -> (f64, f64, bool) {
    match r {
        Rng::Near if x0 != 0.0 => {
            let (a, b) = (x0 * 0.5, x0 * 1.5);
            (a.min(b), a.max(b), false)
        }
        Rng::Near => (-1.0, 1.0, false),
        Rng::Decade if x0 > 0.0 => (x0 / 10.0, x0 * 10.0, true),
        Rng::Decade => (x0 - 1.0, x0 + 1.0, false),
        Rng::Fixed(lo, hi) => (lo.min(x0), hi.max(x0), false),
    }
}

fn field2(s: &Solved, xs_: &str, xr: Rng, ys_: &str, yr: Rng, out: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(xs_)?, s.inp(ys_)?);
    let z0 = s.out(out)?;
    let (xl, xh, xlog) = range_of(x0, &xr);
    let (yl, yh, ylog) = range_of(y0, &yr);
    let xs = k::span(xl, xh, 13, xlog);
    let ys = k::span(yl, yh, 11, ylog);
    let mut vals = vec![];
    let mut n = 0;
    for x in &xs {
        let mut col = vec![];
        for y in &ys {
            let v = s.with(&[(xs_, *x), (ys_, *y)]).and_then(|a| k::num_of(&a, out));
            n += v.is_some() as usize;
            col.push(v);
        }
        vals.push(col);
    }
    if n < 4 {
        return None;
    }
    let (ux, uy, uz) = (unit_of(s, xs_), unit_of(s, ys_), unit_of(s, out));
    let body = k::field(&xs, &ys, &vals, (x0, y0), &label(xs_, &ux), &label(ys_, &uy),
                        &hdr(&format!("{out} = {}", with_unit(z0, &uz))), (xlog, ylog));
    Some(s.chart("field", title,
                 &format!("{} ({out}) over {xs_} from {} to {} and {ys_} from {} to {}{}: each of the {n} cells is one \
                           computation of the model, shaded by its value (darker is larger; a blank cell: inputs the \
                           model refuses). Marked: the inputs given, {out} = {}.", meaning(s, out), num(xl), num(xh),
                          num(yl), num(yh), if xlog || ylog { " (logarithmic axes)" } else { "" }, with_unit(z0, &uz)),
                 &body))
}

// ------------------------------------------------------------------------------------------------ curves
pub enum Span {
    /// from this lower bound (or a fifth of the value, whichever is larger) to five times the value, logarithmic
    LogFrom(f64),
    /// a fixed span, widened to hold the value given
    Fixed(f64, f64),
    /// the value plus or minus one and a half times its size (at least 1)
    Around,
}

fn curve1(s: &Solved, kind: &str, x: &str, sp: Span, y: &str, title: &str) -> Option<Figure> {
    let (x0, y0) = (s.inp(x)?, s.out(y)?);
    let (lo, hi, log) = match sp {
        Span::LogFrom(b) if x0 > 0.0 => ((x0 / 5.0).max(b), (x0 * 5.0).max(b * 10.0), true),
        Span::LogFrom(_) => return None,
        Span::Fixed(a, b) => (a.min(x0), b.max(x0), false),
        Span::Around => {
            let d = (x0.abs() * 1.5).max(1.0);
            (x0 - d, x0 + d, false)
        }
    };
    let xs = k::span(lo, hi, 31, log);
    let pts = s.curve(x, &xs, y, &[]);
    if pts.len() < 2 {
        return None;
    }
    let (ux, uy) = (unit_of(s, x), unit_of(s, y));
    let body = k::lines_y(&[(y.to_string(), pts.clone())], &label(x, &ux), &label(y, &uy),
                          &hdr(&format!("{y} = {} at {x} = {}", with_unit(y0, &uy), with_unit(x0, &ux))),
                          Some((x0, y0)), &format!("{y} = {}", num(y0)), log, true);
    Some(s.chart(kind, title,
                 &format!("{} ({y}) against {} ({x}) from {} to {}{}: {} points, each computed by the model (points it \
                           refuses are left out). Marked: the inputs given, {x} = {}, {y} = {}.", meaning(s, y),
                          meaning(s, x), num(lo), num(hi), if log { " (logarithmic axis)" } else { "" }, pts.len(),
                          with_unit(x0, &ux), with_unit(y0, &uy)), &body))
}

// ------------------------------------------------------------------------------------------------ Nyquist verdict
/// The stability verdict of the closed loop over the loop gain K and the constant coefficient of the open-loop
/// denominator, each cell one run of the model's Nyquist criterion.
fn nyquist(s: &Solved) -> Option<Figure> {
    let k0 = s.inp("K").filter(|x| *x > 0.0)?;
    let den = s.vec("den")?;
    let d0 = *den.last()?;
    let stable0 = s.out_text("stable")?;
    let xs = k::span(k0 * 0.05, k0 * 2.5, 21, false);
    let dd = d0.abs().max(1.0);
    let ys = k::span(d0 - dd, d0 + dd, 13, false);
    let (yes, no) = ("stable (Z = 0)".to_string(), "unstable".to_string());
    let mut cells = vec![];
    for (i, kx) in xs.iter().enumerate() {
        for (j, dy) in ys.iter().enumerate() {
            let mut d = den.clone();
            if let Some(last) = d.last_mut() {
                *last = *dy;
            }
            let Some(a) = s.with_input(&[("K", Input::Float(*kx)), ("den", Input::Json(json!(d)))]) else { continue };
            let v = match a.values.iter().find(|x| x.0 == "stable").map(|x| &x.1) {
                Some(serde_json::Value::Bool(b)) => *b,
                _ => continue,
            };
            cells.push((i, j, if v { yes.clone() } else { no.clone() }));
        }
    }
    if cells.len() < 4 {
        return None;
    }
    let n = den.len() - 1;
    let coef = if n == 0 { "constant".to_string() } else { "constant coefficient".to_string() };
    let zt = s.out("Z").map(num).unwrap_or_default();
    let body = k::decision(&xs, &ys, &cells, &[yes, no], (k0, d0), "Loop gain K",
                           &format!("Denominator {coef}"), &hdr(&format!("K = {}: Z = {zt}, stable: {stable0}", num(k0))));
    Some(s.chart("decision", "Nyquist criterion: where the closed loop is stable",
                 &format!("The verdict of the Nyquist criterion (stable when no closed-loop pole lies in the right half \
                           plane, Z = 0) over the loop gain K from {} to {} and the denominator's {coef} from {} to {}, \
                           the other coefficients as given: each of the {} cells is one run of the model, coloured by \
                           its verdict. Marked: the inputs given, K = {}, Z = {zt}, stable: {stable0}.", num(xs[0]),
                          num(xs[20]), num(ys[0]), num(ys[12]), cells.len(), num(k0)), &body))
}

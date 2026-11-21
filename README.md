# Control Systems (control-systems-ai999x)

[![tests](https://github.com/AI999X/control-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/control-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/control-systems-ai999x)](https://pypi.org/project/control-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/control-systems-ai999x)

Control Systems covers transfer functions, poles and zeros, step and frequency response, stability (Routh-Hurwitz, gain and phase margins, Nyquist), root locus, state space, controllability and observability, pole placement, LQR, Kalman filters, discretization and PID tuning: 97 calculations in 16 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Control Systems desktop app: Gain and phase margins, with its worked example and live scene](https://www.ai999x.com/?ax_shot=control-systems-ai999x/hero.webp)](https://www.ai999x.com/models/control-systems-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-control-systems-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-control-systems-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `control-systems` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/control-systems-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install control-systems-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Control Systems desktop app: every scene and chart is drawn from the computed answer.

[![Control Systems desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=control-systems-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=control-systems-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=control-systems-ai999x/clip.mp4)*

| | |
|---|---|
| [![Gain and phase margins: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-1.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_stability_margins)<br>Gain and phase margins | [![Percent overshoot from the damping ratio: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-2.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_percent_overshoot)<br>Percent overshoot from the damping ratio |
| [![Continuous-time LQR gain (Riccati equation): the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-3.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_lqr)<br>Continuous-time LQR gain (Riccati equation) | [![Tustin (bilinear) discretization: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-4.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_tustin)<br>Tustin (bilinear) discretization |
| [![Desired closed-loop poles from overshoot and settling time: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-5.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_specs_to_poles)<br>Desired closed-loop poles from overshoot and settling time | [![Steady-state discrete Kalman filter gain: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-6.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_discrete_kalman_gain)<br>Steady-state discrete Kalman filter gain |
| [![Damping ratio from the logarithmic decrement: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-7.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_log_decrement)<br>Damping ratio from the logarithmic decrement | [![Speed transfer function of a DC motor: the live scene in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-8.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_dc_motor)<br>Speed transfer function of a DC motor |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![Gain and phase margins: a chart in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/chart-1.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_stability_margins)<br>Gain and phase margins | [![Percent overshoot from the damping ratio: a chart in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/chart-2.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_percent_overshoot)<br>Percent overshoot from the damping ratio | [![Continuous-time LQR gain (Riccati equation): a chart in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/chart-3.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_lqr)<br>Continuous-time LQR gain (Riccati equation) | [![Tustin (bilinear) discretization: a chart in the Control Systems desktop app](https://www.ai999x.com/?ax_shot=control-systems-ai999x/chart-4.webp)](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_tustin)<br>Tustin (bilinear) discretization |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import control_systems_ai999x as control_systems

a = control_systems.ask("bandwidth: num = [1], den = [1, 1]")
print(a.outcome, a.values)
# SOLVED {'wb': 1.0, 'dc_gain': 1.0}

a = control_systems.ask(
    "logarithmic decrement: x1 = 1, xn = 0.5, n = 1"
)
print(a.outcome, a.values)
# SOLVED {'delta': 0.69314718056, 'zeta': 0.109652580999}

a = control_systems.ask(
    "steady-state error: num = [10], den = [1, 3, 2], input = step"
)
print(a.outcome, a.values)
# SOLVED {'system_type': 0, 'K_static': 5.0, 'ess': 0.166666666667}

a = control_systems.ask("damping ratio from overshoot: OS = 0.2")
print(a.outcome, a.values)
# SOLVED {'zeta': 0.455949810769, 'pole_angle_deg': 62.8739366084}

a = control_systems.ask(
    "Smith FOPDT fit: t28 = 4, t63 = 10, dy = 6, du = 2"
)
print(a.outcome, a.values)
# SOLVED {'K': 3.0, 'tau': 9.0, 'theta': 1.0}

a = control_systems.ask("exact settling time: zeta = 0.5, wn = 1")
print(a.outcome, a.values)
# SOLVED {'Ts': 8.07634897393, 'last_extremum': 2}

a = control_systems.ask("DC gain: num = [10], den = [1, 3, 2]")
print(a.outcome, a.values)
# SOLVED {'dc_gain': 5.0, 'final_value': 5.0}

a = control_systems.ask(
    "discrete DC gain: num = [0.5], den = [1, -0.5]"
)
print(a.outcome, a.values)
# SOLVED {'dc_gain': 1.0, 'final_value': 1.0}

a = control_systems.ask("scalar LQR: a = 1, b = 1, q = 3, r = 1")
print(a.outcome, a.values)
# SOLVED {'P': 3.0, 'K': 3.0, 'pole': -2.0}

a = control_systems.ask(
    "PI pole placement: K = 2, tau = 5, zeta = 0.7, wn = 1"
)
print(a.outcome, a.values)
# SOLVED {'Kp': 3.0, 'Ki': 2.5}

a = control_systems.ask(
    "final value theorem: num = [10], den = [1, 3, 2, 0]"
)
print(a.outcome, a.values)
# SOLVED {'final_value': 5.0}

a = control_systems.ask(
    "discrete PID: Kp = 2, Ki = 1, Kd = 0.5, T = 0.1"
)
print(a.outcome, a.values)
# SOLVED {'q0': 7.1, 'q1': -12.0, 'q2': 5.0}

a = control_systems.ask(
    "PID parallel to standard: Kp = 6, Ki = 3, Kd = 3"
)
print(a.outcome, a.values)
# SOLVED {'Ti': 2.0, 'Td': 0.5}

a = control_systems.ask("Jury test: coefficients = [1, -1.2, 0.35]")
print(a.outcome, a.values)
# SOLVED {'stable': True, 'P1': 0.15, 'Pm1': 2.55}

a = control_systems.ask(
    "initial value theorem: num = [2, 3], den = [1, 5, 6]"
)
print(a.outcome, a.values)
# SOLVED {'y0': 2.0, 'dy0': -7.0}
```

## Command line: 15 examples

Each command below was run with `control-systems` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ control-systems solve ctrl_lag_design beta=10 w_c=1
zero = 0.1
pole = 0.01
dc_gain = 10
phase_deg = -5.13765444
magnitude_db = 0.04277946506

$ control-systems solve ctrl_specs_to_poles OS=0.1630335348 Ts=2
zeta = 0.5
wn = 4 rad/s
sigma = 2
wd = 3.464101615 rad/s
char_poly = [1.0,4.0,15.9999999982]

$ control-systems solve ctrl_backward_euler "num=[1]" "den=[1, 1]" T=0.1
numz = [0.0909090909091,0.0]
denz = [1.0,-0.909090909091]

$ control-systems solve ctrl_kalman_scalar_step x=0 P=1 a=1 q=1 r=2 z=4
x_pred = 0
P_pred = 2
gain = 0.5
x_post = 2
P_post = 1

$ control-systems solve ctrl_bandwidth "num=[1]" "den=[1, 1]"
wb = 1 rad/s
dc_gain = 1

$ control-systems solve ctrl_log_decrement x1=1 xn=0.5 n=1
delta = 0.6931471806
zeta = 0.109652581

$ control-systems solve ctrl_dc_motor J=0.01 b=0.1 Kt=0.01 R=1 L=0.5
num = [0.01]
den = [0.005,0.06,0.1001]
dc_gain = 0.0999000999
poles = [-9.99749921826,-2.00250078174]

$ control-systems solve ctrl_lqr "A=[[0, 1], [0, 0]]" "B=[0, 1]" "Q=[[1, 0], [0, 1]]" R=1
K = [1.0,1.73205080757]
P = [[1.73205080757,1.0],[1.0,1.73205080757]]
poles = [{"re":-0.866025403784,"im":-0.5},{"re":-0.866025403784,"im":0.5}]

$ control-systems solve ctrl_pid_closed_loop "num=[1]" "den=[1, 1]" Kp=3 Ki=2
char_poly = [1.0,4.0,2.0]
poles = [-3.41421356237,-0.585786437627]
max_real_part = -0.5857864376
stable = true

$ control-systems solve ctrl_arrival_angle "num=[1, 2, 2]" "den=[1, 0]"
zero = {"re":-1.0,"im":1.0}
arrival_deg = -135

$ control-systems solve ctrl_lyapunov "A=[[-1, 0], [0, -2]]" "Q=[[2, 0], [0, 4]]"
P = [[1.0,0.0],[0.0,1.0]]
positive_definite = true

$ control-systems solve ctrl_state_feedback_poles "A=[[0, 1], [0, 0]]" "B=[0, 1]" "K=[4, 4]"
char_poly = [1.0,4.0,4.0]
poles = [-2.0,-2.0]
stable = true

$ control-systems solve ctrl_error_constants "num=[10]" "den=[1, 3, 2]" input=step
system_type = 0
K_static = 5
ess = 0.1666666667

$ control-systems solve ctrl_pade theta=2 n=1
num = [-1.0,1.0]
den = [1.0,1.0]

$ control-systems solve ctrl_damping_from_overshoot OS=0.2
zeta = 0.4559498108
pole_angle_deg = 62.87393661
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Control Systems on ai999x.com](https://www.ai999x.com/models/control-systems-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/control-systems-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/control-systems-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `control-systems` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/control-systems`) and its data (`rust/models/control_systems/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p control-systems-ai999x
./target/release/control-systems --help
cargo test -p control-systems-ai999x
```

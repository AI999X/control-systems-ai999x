# control-systems-ai999x — offline, verified control systems solver with step-by-step answers

[![tests](https://github.com/AI999X/control-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/control-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/control-systems-ai999x)](https://pypi.org/project/control-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/control-systems-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/control-systems-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/control-systems-ai999x/)** · **[Guides](https://huggingface.co/AI999X/control-systems-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/control-systems-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

control-systems-ai999x is an offline, deterministic Python solver that answers control systems questions with verified results from 97 certified capabilities, gives 686 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**control-systems-ai999x** is a compact, deterministic control systems engine for Python. It reads a question in plain English (or a typed call), picks a certified control systems capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified control systems capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 686 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10–3.14).
- **Coverage:** transfer functions, time and frequency response, stability margins, root locus, state space, LQR, Kalman filters, discretization and PID tuning.

## Model overview

| Property | Value |
|---|---|
| Package | `control-systems-ai999x` 1.0.0 (import `control_systems_ai999x`) |
| Type | Deterministic neuro-symbolic control systems engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 97 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 686 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python ) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install control-systems-ai999x
```

```python
import control_systems_ai999x as control_systems

control_systems.ask("stability margins: num = [10], den = [1, 3, 2, 0]").show()
# -> SOLVED — gm = 0.6, gm_db = -4.43697, w_pc = 1.41421, pm_deg = -12.9972  (ctrl_stability_margins)
control_systems.ask("Routh-Hurwitz: coefficients = [1, 2, 3, 4, 5]").show()
# -> SOLVED — first_column = [1.0, 2.0, 1.0, -6.0, 5.0], rhp_roots = 2, stable = 0  (ctrl_routh_hurwitz)
control_systems.solve("ctrl_percent_overshoot", zeta=0.5).show()
# -> SOLVED — PO = 16.3034, Mp = 0.163034, pole_angle_deg = 60  (ctrl_percent_overshoot)
control_systems.solve("ctrl_lqr", A=[[0, 1], [0, 0]], B=[0, 1], Q=[[1, 0], [0, 1]], R=1).show()
# -> SOLVED — K = [1.0, 1.73205080757], P = [[1.73205080757, 1.0], [1.0, 1.73205080757]], poles = [(-0.866025403784-0.5j), (-0.866025403784+0.5j)]  (ctrl_lqr)
control_systems.ask("Tustin discretization: num = [1], den = [1, 1], T = 0.1").show()
# -> SOLVED — numz = [0.047619047619, 0.047619047619], denz = [1.0, -0.904761904762]  (ctrl_tustin)
control_systems.ask("Who created you?").message
# -> control-systems-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

The Python package also installs its own command: `control-systems-ai999x "<question>"`, `control-systems-ai999x --capabilities`. The outputs above were produced by running this exact code on control-systems-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

97 certified procedures. A few of them:

- **Procedures:** -3 dB bandwidth of a transfer function; Backward Euler (backward difference) discretization; Bandwidth of a standard second-order system; Bode magnitude and phase at one frequency; Closed-loop poles of a PID loop; Closed-loop poles under state feedback u = -K x; Closed-loop transfer function of a feedback loop; Cohen-Coon tuning of a first-order-plus-dead-time process; Continuous Lyapunov equation; Continuous equivalent of a discrete pole (s = ln z / T); Continuous-time LQR gain (Riccati equation); Controllability Gramian; Controllability matrix and rank; Controllable canonical form of a transfer function; DC gain and final value of a discrete transfer function; DC gain and final value of a step response; DC gain of a state-space model; Damping ratio from percent overshoot; Damping ratio from the logarithmic decrement; Deadbeat state feedback (discrete time); Desired closed-loop poles from overshoot and settling time; Discrete Lyapunov equation; Discrete PID in velocity (incremental) form; Discrete-time LQR gain (discrete Riccati equation) …

`control_systems_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified control systems capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (transfer functions, time and frequency response, stability margins, root locus, state space, LQR, Kalman filters, discretization and PID tuning); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{control_systems_ai999x_1_0_0,
  title  = {control-systems-ai999x: an offline, verified control systems solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.0},
  url    = {https://huggingface.co/AI999X/control-systems-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/control-systems-ai999x/discussions or visit https://ai999x.com.

## Support the project

control-systems-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/control-systems-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/control-systems-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/control-systems-ai999x/tree/main/onnx).

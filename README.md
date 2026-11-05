# control-systems-ai999x — offline, verified control systems solver with step-by-step answers

[![tests](https://github.com/AI999X/control-systems-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/control-systems-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/control-systems-ai999x)](https://pypi.org/project/control-systems-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/control-systems-ai999x)

**[Desktop app & command line](https://www.ai999x.com/desktop-cli/)** · **[Compute in your browser](https://www.ai999x.com/models/control-systems-ai999x/#try)** · **[Install (Python)](https://pypi.org/project/control-systems-ai999x/)** · **[Guides](https://huggingface.co/AI999X/control-systems-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/control-systems-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.0

Field: **Engineering** · Tasks: Question Answering, Sentence Similarity, Feature Extraction

control-systems-ai999x is an offline, deterministic Python solver that answers control systems questions with verified results from 97 certified capabilities, gives 686 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**control-systems-ai999x** is a compact, deterministic control systems engine for Python. It reads a question in plain English (or a typed call), picks a certified control systems capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green) ![Rust](https://img.shields.io/badge/Rust-Windows%20%7C%20Linux%20%7C%20WebAssembly-orange)

**Run it your way.** control-systems-ai999x comes as the AI999X desktop app and as the command-line program `control-systems` for Windows and Linux, runs in your browser as WebAssembly, and installs as a Python package. The desktop app, the command line and the browser run the same compiled Rust program on your own device: offline, no Python needed.

## Get it

| Way to run it | For | Get it |
|---|---|---|
| **AI999X Control Systems desktop app** 1.2.0 | Windows, Linux — this model in its own app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-control-systems-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-control-systems-1.2.0-linux-x86_64.tar.gz) |
| **AI999X desktop app** 1.2.0 | Windows, Linux — every AI999X model in one app: search, ask in words, forms, live figures, explanations, offline | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-1.2.0-linux-x86_64.tar.gz) |
| **Command line** (`control-systems`) | Windows, Linux — one program per model, offline, no Python | [Windows (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) · [Linux (x86-64)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) |
| **In your browser** | any device — nothing to install | [Compute on the model page](https://www.ai999x.com/models/control-systems-ai999x/#try) |
| **Python** | Python 3.10+ | `pip install control-systems-ai999x` |

Checksums: [SHA256SUMS](https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS) · every version: [https://www.ai999x.com/downloads/desktop/latest/](https://www.ai999x.com/downloads/desktop/latest/) · install guide: [desktop & command line](https://www.ai999x.com/desktop-cli/)

## Rust versions

Control Systems also comes as native programs compiled from Rust, version 1.2.0 of the AI999X desktop app and command line: they run offline on your computer and need no Python. Each file is in this repository's `rust/` folder, on the [GitHub release v1.0.0](https://github.com/AI999X/control-systems-ai999x/releases/tag/v1.0.0) and on the website.

| Program | System | File | Size | Download | Use |
|---|---|---|---|---|---|
| Desktop app (Control Systems) | Windows x86-64 | `ai999x-control-systems-1.2.0-windows-x86_64.zip` | 7.4 MB | [Hub](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/rust/ai999x-control-systems-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/control-systems-ai999x/releases/download/v1.0.0/ai999x-control-systems-1.2.0-windows-x86_64.zip) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-control-systems-1.2.0-windows-x86_64.zip) | unzip, then double-click `ai999x-control-systems.exe` (no installer) |
| Desktop app (Control Systems) | Linux x86-64 | `ai999x-control-systems-1.2.0-linux-x86_64.tar.gz` | 8.4 MB | [Hub](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/rust/ai999x-control-systems-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/control-systems-ai999x/releases/download/v1.0.0/ai999x-control-systems-1.2.0-linux-x86_64.tar.gz) · [website](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-control-systems-1.2.0-linux-x86_64.tar.gz) | extract, then run `./ai999x-control-systems` (X11 or Wayland desktop) |
| Command line (`control-systems`) | Windows x86-64 | `ai999x-control-systems-cli-1.2.0-windows-x86_64.zip` | 3.2 MB | [Hub](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/rust/ai999x-control-systems-cli-1.2.0-windows-x86_64.zip) · [GitHub](https://github.com/AI999X/control-systems-ai999x/releases/download/v1.0.0/ai999x-control-systems-cli-1.2.0-windows-x86_64.zip) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-windows-x86_64.zip) | unzip, then `control-systems.exe ask "<question>"` or `control-systems.exe solve <capability> name=value` |
| Command line (`control-systems`) | Linux x86-64 | `ai999x-control-systems-cli-1.2.0-linux-x86_64.tar.gz` | 3.1 MB | [Hub](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/rust/ai999x-control-systems-cli-1.2.0-linux-x86_64.tar.gz) · [GitHub](https://github.com/AI999X/control-systems-ai999x/releases/download/v1.0.0/ai999x-control-systems-cli-1.2.0-linux-x86_64.tar.gz) · [website (all models)](https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz) | extract, then `./control-systems ask "<question>"` or `./control-systems solve <capability> name=value` |

The command-line archive holds `control-systems` with a README of its commands and examples, the licence and the notice. Checksums: [`rust/SHA256SUMS`](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/rust/SHA256SUMS) (`sha256sum -c SHA256SUMS`).

## Quick start

### Desktop app

Download **AI999X Control Systems** for your system (above), unpack it and start it; type a question in words or pick a capability, fill in the form and press Compute. Figures and an explanation of the answer appear beside it. The AI999X desktop app has the same, with Control Systems among all AI999X models.

Linux, from a terminal:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-control-systems-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-control-systems-1.2.0-linux-x86_64.tar.gz
./ai999x-control-systems-1.2.0-linux-x86_64/ai999x-control-systems
```

### Command line

Linux:

```bash
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/ai999x-cli-1.2.0-linux-x86_64.tar.gz
curl -LO https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.2.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.2.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
control-systems --version
```

Windows (PowerShell):

```powershell
$F = "ai999x-cli-1.2.0-windows-x86_64.zip"
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/$F" -OutFile $F
Invoke-WebRequest "https://www.ai999x.com/downloads/desktop/1.2.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.2.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
control-systems --version
```

Every command below was run as shown; the text under it is what the program printed.

**Ask in words** — Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```console
$ control-systems ask "stability margins: num = [10], den = [1, 3, 2, 0]"
Read as: Gain and phase margins
gm = 0.6
gm_db = -4.436974992
w_pc = 1.414213562 rad/s
pm_deg = -12.99720802
w_gc = 1.802203305 rad/s
  by: Gain and phase margins  (crossovers as positive real roots of |N|^2 - |D|^2 and Im(N(jw) conj D(jw)), Newton-polished)
  assumes: exactly one gain crossover and one phase crossover
```

**What a capability needs** — The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```console
$ control-systems info ctrl_stability_margins
Gain and phase margins
  crossovers as positive real roots of |N|^2 - |D|^2 and Im(N(jw) conj D(jw)), Newton-polished
  in   num      numerator of the open loop L(s) (vector) []
  in   den      denominator of L(s) (vector) []
  out  gm       gain margin (ratio) []
  out  gm_db    gain margin in dB []
  out  w_pc     phase-crossover frequency [rad/s]
  out  pm_deg   phase margin in degrees []
  out  w_gc     gain-crossover frequency [rad/s]
```

**Solve** — Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```console
$ control-systems solve ctrl_stability_margins "num=[10]" "den=[1, 3, 2, 0]"
gm = 0.6
gm_db = -4.436974992
w_pc = 1.414213562 rad/s
pm_deg = -12.99720802
w_gc = 1.802203305 rad/s
  by: Gain and phase margins  (crossovers as positive real roots of |N|^2 - |D|^2 and Im(N(jw) conj D(jw)), Newton-polished)
  assumes: exactly one gain crossover and one phase crossover
```

**Many questions at once** — One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | control-systems batch

`cases.jsonl`:

```json
{"key": "ctrl_stability_margins", "inputs": {"num": "[10]", "den": "[1, 3, 2, 0]"}}
{"key": "ctrl_routh_hurwitz", "inputs": {"coefficients": "[1, 2, 3, 4, 5]"}}
```

```console
$ control-systems batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":-12.9972080155,"unit":"","solved_for":"pm_deg","values":[["gm",0.6,""],["gm_db",-4.43697499233,""],["w_pc",1.41421356237,"rad/s"],["pm_deg",-12.9972080155,""],["w_gc",1.80220330461,"rad/s"]],"target":"pm_deg","phrase":"gain margin (ratio)","assumptions":["exactly one gain crossover and one phase crossover"],"name":"Gain and phase margins","equation":"crossovers as positive real roots of |N|^2 - |D|^2 and Im(N(jw) conj D(jw)), Newton-polished"}
{"outcome":"SOLVED","message":"","value":2.0,"unit":"","solved_for":"rhp_roots","values":[["first_column",[1.0,2.0,1.0,-6.0,5.0],""],["rhp_roots",2,""],["stable",false,""]],"target":"rhp_roots","phrase":"first column of the Routh array","assumptions":["no zero in the first column (special cases are refused)"],"name":"Routh-Hurwitz stability test","equation":"Routh array in exact fractions; sign changes of the first column"}
```

### In your browser

Open [Control Systems on ai999x.com](https://www.ai999x.com/models/control-systems-ai999x/#try), choose a capability and press **Compute**: the model runs on your own device as WebAssembly; nothing is sent anywhere. Change any input (any unit of the right kind) and the answer, its explanation and the figures follow.

In your own page: `control_systems_ai999x.wasm` and the loader `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js).

### Python

```bash
pip install control-systems-ai999x
```

## Worked examples in every form

The same worked example from the command line, from Python and in the browser. Each command-line output is what the program printed.

### Lag compensator placement

C(s) = (s + z)/(s + p) with z = w_c/ratio (default a decade below the crossover) and p = z/beta: zero, pole, DC gain beta, and the phase lag and gain it adds at w_c.

Command line:

```console
$ control-systems solve ctrl_lag_design beta=10 w_c=1
zero = 0.1
pole = 0.01
dc_gain = 10
phase_deg = -5.13765444
magnitude_db = 0.04277946506
  by: Lag compensator placement  (zero a decade below crossover, pole beta times lower)
  beta = 10
  w_c = 1
  assumes: beta > 1; ratio >= 2
```

Python:

```python
import control_systems_ai999x as m

a = m.solve("ctrl_lag_design", beta="10", w_c="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_lag_design) and press **Compute in your browser**.

### Lead compensator design for a required phase lead

alpha = (1 - sin phi)/(1 + sin phi) and T = 1/(w_m sqrt(alpha)) so that C(s) = (T s + 1)/(alpha T s + 1) adds the phase lead phi at w_m; its zero, pole and gain at w_m.

Command line:

```console
$ control-systems solve ctrl_lead_design phi_max_deg=30 w_m=1
alpha = 0.3333333333
T = 1.732050808 s
zero = 0.5773502692
pole = 1.732050808
gain_db = 4.771212547
  by: Lead compensator design for a required phase lead  (sin phi_max = (1 - alpha)/(1 + alpha), w_m = 1/(T sqrt(alpha)))
  phi_max_deg = 30
  w_m = 1
  assumes: 0 < phi < 90 degrees; w_m > 0
```

Python:

```python
import control_systems_ai999x as m

a = m.solve("ctrl_lead_design", phi_max_deg="30", w_m="1")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_lead_design) and press **Compute in your browser**.

### Maximum phase lead of a lead compensator

For C(s) = (s + z)/(s + p) with z < p: the maximum phase lead asin((p - z)/(p + z)), the frequency sqrt(z p) where it occurs, alpha = z/p and the gain there, 10 log10(z/p) dB.

Command line:

```console
$ control-systems solve ctrl_lead_max_phase z=1 p=9
phi_max_deg = 53.13010235
w_max = 3 rad/s
alpha = 0.1111111111
magnitude_db = -9.542425094
  by: Maximum phase lead of a lead compensator  (stationary point of the phase)
  z = 1
  p = 9
  assumes: 0 < z < p
```

Python:

```python
import control_systems_ai999x as m

a = m.solve("ctrl_lead_max_phase", z="1", p="9")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_lead_max_phase) and press **Compute in your browser**.

### Desired closed-loop poles from overshoot and settling time

zeta from the overshoot, sigma = 4/Ts from the 2 % settling time, wn = sigma/zeta, the poles -sigma +/- j wd and the desired characteristic polynomial.

Command line:

```console
$ control-systems solve ctrl_specs_to_poles OS=0.1630335348 Ts=2
zeta = 0.5
wn = 4 rad/s
sigma = 2
wd = 3.464101615 rad/s
char_poly = [1.0,4.0,15.9999999982]
  by: Desired closed-loop poles from overshoot and settling time  (zeta from the overshoot formula; sigma = 4/Ts)
  OS = 0.1630335348
  Ts = 2
  assumes: 0 < OS < 100 %; Ts > 0
```

Python:

```python
import control_systems_ai999x as m

a = m.solve("ctrl_specs_to_poles", OS="0.1630335348", Ts="2")
print(a.values or (a.value, a.unit))
```

Browser: [open this capability on the model page](https://www.ai999x.com/models/control-systems-ai999x/#cap-ctrl_specs_to_poles) and press **Compute in your browser**.

## Documentation

- **Rust user guide** — the command line, every capability with a command and what it printed: [PDF](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_rust_guide.pdf) · [Markdown](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_rust_guide.md)
- **Python guide** — every capability with a Python example: [PDF](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_capabilities.pdf) · [Markdown](https://huggingface.co/AI999X/control-systems-ai999x/blob/main/docs/control_systems_ai999x_capabilities.md)
- **Desktop app & command line** — downloads, install, ask and search, figures, offline use, checksums: https://www.ai999x.com/desktop-cli/
- **Model page** — compute in your browser, every capability explained: https://www.ai999x.com/models/control-systems-ai999x/

## Highlights

- **Verified, not generated.** Every answer is computed by a certified control systems capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 686 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Native and in the browser.** Every capability also computes natively in Rust: the desktop app and the `control-systems` command line for Windows and Linux, and WebAssembly on the model page — no Python, no network.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** transfer functions, time and frequency response, stability margins, root locus, state space, LQR, Kalman filters, discretization and PID tuning.

## Model overview

| Property | Value |
|---|---|
| Package | `control-systems-ai999x` 1.0.0 (import `control_systems_ai999x`) |
| Type | Deterministic neuro-symbolic control systems engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 97 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 686 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Python quick start

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

The Python package also installs a Python command, `control-systems-ai999x "<question>"`; for the native program use `control-systems` (above). The outputs above were produced by running this exact code on control-systems-ai999x 1.0.0.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Python worked examples, with their steps

Each question was put to `control-systems-ai999x`; the answer is the one it returned, with the steps it took.

**Gain and phase margins** · _Stability_

> stability margins: num = [10], den = [1, 3, 2, 0]

→ **gm = 0.6, gm_db = -4.436974992, w_pc = 1.414213562 rad/s, pm_deg = -12.99720802, w_gc = 1.802203305 rad/s**

<details><summary>Steps</summary>

- Gain and phase margins: num = [10], den = [1, 3, 2, 0] → gm = 0.6, gm_db = -4.436974992, w_pc = 1.414213562 rad/s, pm_deg = -12.99720802, w_gc = 1.802203305 rad/s

</details>

**Routh-Hurwitz stability test** · _Stability_

> Routh-Hurwitz: coefficients = [1, 2, 3, 4, 5]

→ **first_column = [1, 2, 1, -6, 5], rhp_roots = 2, stable = False**

<details><summary>Steps</summary>

- Routh-Hurwitz stability test: coefficients = [1, 2, 3, 4, 5] → first_column = [1, 2, 1, -6, 5], rhp_roots = 2, stable = False

</details>

**Percent overshoot from the damping ratio** · _Time response_

`m.solve("ctrl_percent_overshoot", zeta=0.5)`

→ **PO = 16.30335348, Mp = 0.1630335348, pole_angle_deg = 60**

<details><summary>Steps</summary>

- Percent overshoot from the damping ratio: zeta = 0.5 → PO = 16.30335348, Mp = 0.1630335348, pole_angle_deg = 60

</details>

**Continuous-time LQR gain (Riccati equation)** · _Optimal control_

`m.solve("ctrl_lqr", A=[[0, 1], [0, 0]], B=[0, 1], Q=[[1, 0], [0, 1]], R=1)`

→ **K = [1, 1.732050808], P = [[1.732050808, 1], [1, 1.732050808]], poles = [-0.8660254038-0.5i, -0.8660254038+0.5i]**

<details><summary>Steps</summary>

- Continuous-time LQR gain (Riccati equation): A = [0 1; 0 0], B = [0, 1], Q = [1 0; 0 1], R = 1 → K = [1, 1.732050808], P = [[1.732050808, 1], [1, 1.732050808]], poles = [-0.8660254038-0.5i, -0.8660254038+0.5i]

</details>

**Tustin (bilinear) discretization** · _Discretization_

> Tustin discretization: num = [1], den = [1, 1], T = 0.1

→ **numz = [0.04761904762, 0.04761904762], denz = [1, -0.9047619048]**

<details><summary>Steps</summary>

- Tustin (bilinear) discretization: num = [1], den = [1, 1], T = 0.1 → numz = [0.04761904762, 0.04761904762], denz = [1, -0.9047619048]

</details>

**Lag compensator placement** · _Compensators_

> lag compensator design: beta = 10, w_c = 1

→ **zero = 0.1, pole = 0.01, dc_gain = 10, phase_deg = -5.13765444, magnitude_db = 0.04277946506**

<details><summary>Steps</summary>

- Lag compensator placement: beta = 10, w_c = 1 → zero = 0.1, pole = 0.01, dc_gain = 10, phase_deg = -5.13765444, magnitude_db = 0.04277946506

</details>

## Figures

Every point was computed by `control-systems-ai999x`.

**Lag compensator placement**

![Lag compensator placement — computed by control-systems-ai999x](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/figures/ctrl-lag-design-phase-deg-vs-beta.svg)

_phase_deg (phase added at w_c in degrees (negative)) against beta (required low-frequency gain increase (> 1)), with w_c = 1. 31 points computed by control-systems-ai999x; marked: beta = 10, phase_deg = -5.13765._

**Desired closed-loop poles from overshoot and settling time**

![Desired closed-loop poles from overshoot and settling time — computed by control-systems-ai999x](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/figures/ctrl-specs-to-poles-wn-vs-os.svg)

_wn (natural frequency) against OS (maximum overshoot (write 10% or 0.1)), with Ts = 2. 31 points computed by control-systems-ai999x; marked: OS = 0.163034, wn = 4 rad/s._

**Percent overshoot from the damping ratio**

![Percent overshoot from the damping ratio — computed by control-systems-ai999x](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/figures/ctrl-percent-overshoot-po-vs-zeta.svg)

_PO (percent overshoot (%)) against zeta (damping ratio (0 <= zeta < 1)). 31 points computed by control-systems-ai999x; marked: zeta = 0.5, PO = 16.3034._

**Mapping an s-plane pole to the z-plane (z = e^{sT})**

![Mapping an s-plane pole to the z-plane (z = e^{sT}) — computed by control-systems-ai999x](https://huggingface.co/AI999X/control-systems-ai999x/resolve/main/figures/ctrl-s-to-z-pole-z-mag-vs-s-re.svg)

_z_mag (|z| = e^{s_re T}) against s_re (real part of the s-plane pole (1/s)), with T = 0.1, s_im = 0. 31 points computed by control-systems-ai999x; marked: s_re = -2, z_mag = 0.818731._

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

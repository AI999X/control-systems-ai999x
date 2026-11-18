# Control Systems: the desktop app, user guide

**Control Systems**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Control Systems covers transfer functions, poles and zeros, step and frequency response, stability (Routh-Hurwitz, gain and phase margins, Nyquist), root locus, state space, controllability and observability, pole placement, LQR, Kalman filters, discretization and PID tuning. Its desktop app holds all 97 calculations in 16 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-control-systems-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-control-systems-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-control-systems-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-control-systems.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-control-systems-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-control-systems-1.6.5-linux-x86_64.tar.gz
./ai999x-control-systems-1.6.5-linux-x86_64/ai999x-control-systems
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Control Systems app, opened on Gain and phase margins with its worked example.](https://www.ai999x.com/?ax_shot=control-systems-ai999x/hero.webp)

*The Control Systems app, opened on Gain and phase margins with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Stability › Gain and phase margins (or type part of its name in the search box). It opens with a worked example filled in: “stability margins: num = [10], den = [1, 3, 2, 0]” The answer: gm = 0.6, gm_db = -4.436974992, w_pc = 1.414213562 rad/s, pm_deg = -12.99720802.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![Gain and phase margins: the live scene of the worked example.](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-1.webp)

*Gain and phase margins: the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “stability margins: num = [10], den = [1, 3, 2, 0]”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: Gain and phase margins
gm = 0.6
gm_db = -4.436974992
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![Percent overshoot from the damping ratio](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-2.webp)

*Percent overshoot from the damping ratio*

![Continuous-time LQR gain (Riccati equation)](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-3.webp)

*Continuous-time LQR gain (Riccati equation)*

![Tustin (bilinear) discretization](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-4.webp)

*Tustin (bilinear) discretization*

![Desired closed-loop poles from overshoot and settling time](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-5.webp)

*Desired closed-loop poles from overshoot and settling time*

![Steady-state discrete Kalman filter gain](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-6.webp)

*Steady-state discrete Kalman filter gain*

![Damping ratio from the logarithmic decrement](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-7.webp)

*Damping ratio from the logarithmic decrement*

![Speed transfer function of a DC motor](https://www.ai999x.com/?ax_shot=control-systems-ai999x/scene-8.webp)

*Speed transfer function of a DC motor*

![Gain and phase margins: a chart, enlarged.](https://www.ai999x.com/?ax_shot=control-systems-ai999x/chart-1.webp)

*Gain and phase margins: a chart, enlarged.*

![Percent overshoot from the damping ratio: a chart, enlarged.](https://www.ai999x.com/?ax_shot=control-systems-ai999x/chart-2.webp)

*Percent overshoot from the damping ratio: a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Control Systems app in the dark theme.](https://www.ai999x.com/?ax_shot=control-systems-ai999x/hero-dark.webp)

*The Control Systems app in the dark theme.*

## 7. Every calculation

97 calculations in 16 topics; each has its own entry on the model's page (https://www.ai999x.com/models/control-systems-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Compensators | Lag compensator placement, Lead compensator design for a required phase lead, Maximum phase lead of a lead compensator |
| Design | Desired closed-loop poles from overshoot and settling time |
| Discretization | Backward Euler (backward difference) discretization, Continuous equivalent of a discrete pole (s = ln z / T), DC gain and final value of a discrete transfer function, Discrete PID in velocity (incremental) form, First-order-hold (triangle-hold) discretization, Forward Euler (forward difference) discretization, Mapping an s-plane pole to the z-plane (z = e^{sT}), Step response of a discrete transfer function at sample k, Tustin (bilinear) discretization, Tustin discretization with frequency prewarping, Zero-order-hold discretization of a first-order system, Zero-order-hold discretization of a transfer function |
| Estimation | One step of a scalar Kalman filter, Steady-state discrete Kalman filter gain, Steady-state gain of a scalar Kalman filter, Steady-state Kalman-Bucy filter gain |
| Frequency response | -3 dB bandwidth of a transfer function, Bandwidth of a standard second-order system, Bode magnitude and phase at one frequency, Frequency response of a first-order lag, Frequency response of a standard second-order system, H-infinity norm (peak gain) of a stable transfer function, H2 norm of a stable transfer function, Resonant peak and resonant frequency |
| Identification | Damping ratio from the logarithmic decrement, First-order-plus-dead-time fit by Smith's two-point method, Second-order model from a step response |
| Modelling | Speed transfer function of a DC motor |
| Optimal control | Continuous-time LQR gain (Riccati equation), Discrete-time LQR gain (discrete Riccati equation), LQR for a first-order system |
| PID tuning | Closed-loop poles of a PID loop, Cohen-Coon tuning of a first-order-plus-dead-time process, PI gains placing the closed-loop poles of a first-order plant, PID: parallel gains to standard form, PID: standard form to parallel gains, SIMC (Skogestad) PI tuning, Ziegler-Nichols tuning from a plant model, Ziegler-Nichols tuning from the process reaction curve, Ziegler-Nichols tuning from the ultimate gain and period |
| Root locus | Root-locus angle of arrival at a complex zero, Root-locus angle of departure from a complex pole, Root-locus asymptotes (centroid and angles), Root-locus breakaway and break-in points, Root-locus gain and angle at a test point |
| Stability | Continuous Lyapunov equation, Discrete Lyapunov equation, Gain and phase margins, Jury (Schur-Cohn) stability test for a discrete polynomial, Nyquist criterion: encirclements of -1, Peak sensitivity Ms and modulus margin, Poles and stability of a characteristic polynomial, Range of gain for closed-loop stability (Routh, j-omega crossing), Routh-Hurwitz stability test |
| State space | Closed-loop poles under state feedback u = -K x, Controllability Gramian, Controllability matrix and rank, Controllable canonical form of a transfer function, DC gain of a state-space model, Deadbeat state feedback (discrete time), Eigenvalues (poles) of a state matrix, Observability Gramian, Observability matrix and rank, Observer (estimator) gain by pole placement, Pole placement by Ackermann's formula, State response to an initial state and a constant input, State-transition matrix e^{At}, Transfer function of a state-space model |
| Steady-state error | System type, static error constant and steady-state error |
| Time delay | Pade approximation of a time delay |
| Time response | Damping ratio from percent overshoot, Exact settling time of a second-order step response, Final value theorem, Initial value theorem (value and slope at t = 0+), Mass-spring-damper natural frequency and damping ratio, Natural frequency and damping ratio of a second-order denominator, Peak time of a second-order step response, Percent overshoot from the damping ratio, Proportional control of a first-order plant, Rise time of an underdamped second-order system, Second-order specifications from the pole location, Settling time of a second-order system (rule and envelope), Step response of a first-order system, Step response of a first-order-plus-dead-time process, Step response of a standard second-order system, Step response of a transfer function at a given time, Time and frequency specifications of a first-order system |
| Transfer functions | Closed-loop transfer function of a feedback loop, DC gain and final value of a step response, Parallel connection of two transfer functions, Poles, zeros and gain of a transfer function, Series (cascade) connection of two transfer functions, Value of a transfer function at a complex point |

## 8. Other ways to use it

The same engine runs on the command line (`control-systems`, its own guide), in Python (`pip install control-systems-ai999x`, `import control_systems_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/control-systems-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/control-systems-ai999x/

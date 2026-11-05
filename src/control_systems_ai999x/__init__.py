"""control-systems-ai999x — offline control systems answers with their formula, steps and verification.

    import control_systems_ai999x as m
    m.ask("stability margins: num = [10], den = [1, 3, 2, 0]").show()      # gain margin 0.6, phase margin -13.0 deg
    m.solve("ctrl_lqr", A=[[0, 1], [0, 0]], B=[0, 1], Q=[[1, 0], [0, 1]], R=1).show()   # K = [1, 1.732]
    m.capabilities()                   # every certified capability

Covers transfer functions (poles, zeros, DC gain, series/parallel/feedback connections), first- and second-order step
responses and their metrics (overshoot, rise, peak and settling times), final and initial value theorems, error
constants and steady-state error, Bode magnitude and phase, resonance and bandwidth, gain and phase margins, Nyquist
encirclements, sensitivity peak, H2 and H-infinity norms, Routh-Hurwitz and the stable gain range, the Jury test, root
locus (asymptotes, gain at a point, breakaway points, departure and arrival angles), state space (conversions,
eigenvalues, controllability, observability, Gramians, Lyapunov equations, state feedback, Ackermann pole placement,
observers, deadbeat control, e^{At}), LQR (continuous, discrete, scalar), Kalman filters (scalar step, steady-state
discrete and Kalman-Bucy gains), discretization (ZOH, FOH, Tustin with and without prewarping, Euler, pole mapping,
discrete step response, digital PID) and PID tuning (Ziegler-Nichols, Cohen-Coon, SIMC, PI pole placement, lead and
lag compensators, Pade approximation, FOPDT fits). Questions use one documented form, '<capability phrase>: name =
value, ...'; a question that is not determined is refused rather than guessed.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.5.0"
_P = Product("control_systems", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__, "engine": _P.engine()})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
figures, engine = _P.figures, _P.engine   # live figures and the engine in use: the Rust engine on Windows/Linux x86_64
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "figures", "engine", "__version__"]

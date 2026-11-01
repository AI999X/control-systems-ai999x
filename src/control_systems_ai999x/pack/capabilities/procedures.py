"""Control systems procedures: deterministic capabilities with typed inputs and several outputs (domain code, loaded
by the pack; enabled only once tools/certify_procedures.py has certified them).

Topics: transfer functions (poles and zeros, DC gain, evaluation at a point, series, parallel and feedback
connections), time response (first- and second-order step responses and their metrics: rise, peak and settling time,
overshoot, final and initial values, dead time), steady-state error and error constants, frequency response (Bode
magnitude and phase, resonance, bandwidth, gain and phase margins, Nyquist encirclements, sensitivity peak, H2 and
H-infinity norms), stability (pole classification, Routh-Hurwitz, the stable gain range, Jury/Schur test), root locus
(asymptotes, gain at a point, breakaway points, departure and arrival angles), state space (conversions to and from
transfer functions, eigenvalues, controllability, observability, state feedback, Ackermann pole placement, observers,
deadbeat control, the state-transition matrix, Lyapunov equations and Gramians), optimal control and estimation
(continuous and discrete LQR, scalar LQR, Kalman filter steps and steady-state gains), discretization (ZOH, FOH,
Tustin with and without prewarping, forward and backward Euler, pole mapping z = e^{sT}, discrete step responses,
discrete PID) and PID tuning (Ziegler-Nichols, Cohen-Coon, SIMC, PI pole placement, lead and lag compensators, Pade
approximation of dead time, FOPDT fits).

Conventions (fixed so that every answer is unique and reproducible):
- polynomials are lists of coefficients, highest power first (s^2 + 3 s + 2 is [1, 3, 2]); vectors and matrices are
  written [1, 2] and [[1, 2], [3, 4]]; B is a column (single input), C a row (single output);
- roots and poles: real roots as numbers, complex roots as complex numbers; sorted by real part, then imaginary part;
  conjugate pairs have exactly equal real parts; roots within 1e-9 (relative) of the real axis are real; a root
  smaller than 1e-12 of the largest root is reported as 0;
- every real output is rounded to 12 significant digits (identical answers on every platform and Python version);
  in matrix exponentials and state responses, entries smaller than 1e-11 of the largest entry are reported as 0;
- phase angles are in degrees in (-180, 180] (principal value) unless a capability states otherwise; the phase
  margin is 180 + arg L(j w_gc) wrapped into (-180, 180];
- a discrete transfer function is normalised so that its denominator's leading coefficient is 1, and its numerator
  is padded to the denominator's length;
- the controllable canonical form is scipy's: A has -a1 ... -an in its first row and ones on the subdiagonal,
  B = [1, 0, ..., 0];
- PID: parallel form u = Kp e + Ki int e + Kd de/dt; standard form Kp (1 + 1/(Ti s) + Td s); Ki = Kp/Ti,
  Kd = Kp Td; an integral time of 0 means the controller has no integral action;
- the settling time rule is 4/(zeta wn) for a 2 % band and 3/(zeta wn) for 5 %; the envelope settling time solves
  e^{-zeta wn t}/sqrt(1 - zeta^2) = band exactly; the true settling time is the last time |y - 1| equals the band;
- exact arithmetic (fractions) is used where the result is rational in the inputs (Routh array, Faddeev-LeVerrier
  characteristic polynomials and transfer functions, ranks, Ackermann gains, Lyapunov equations, Gramians, Jury).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the one documented question form it parses
('<phrase>: name = value, ...'), worked examples, an INDEPENDENT reference computation (used only by certification:
python-control and scipy are imported inside the reference functions, never shipped) and a deliberately wrong
variant that the checks must reject. The shipped code (run, verify, parse) is plain Python (math, cmath, fractions,
re).

How the references differ from the shipped code: step responses and crossing times from scipy's matrix exponential
(state space) and brentq; overshoot and peak times from the numerically computed response; frequency responses from
scipy.signal.freqs and python-control evaluation; poles and zeros from python-control (companion-matrix eigenvalues);
margins, encirclements and sensitivity peaks from python-control's stability_margins and nyquist_response; Routh and
stable gain ranges from numpy roots scanned over the gain; state-space conversions from scipy.signal (ss2tf, tf2ss,
cont2discrete, bilinear) and python-control (sample_system, ctrb, obsv, dcgain); pole placement from scipy's
place_poles (Kautsky-Nichols-Van Dooren) or sympy's linear solve; Riccati and Lyapunov equations from scipy.linalg's
Schur-based solvers; limits (final and initial value theorems, error constants) from sympy; norms by quadrature and
dense search. Weaker references (a table or a definition restated) are listed in procedures_certified.json.
"""
from __future__ import annotations

import cmath
import math
import re
from fractions import Fraction

INF = float("inf")
PI = math.pi


# ================================================================================================ checks
def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _num(v, name):
    if isinstance(v, bool) or not isinstance(v, (int, float)) or not math.isfinite(v):
        raise ValueError(f"{name} must be a finite number")
    return float(v)


def _pos(v, name):
    v = _num(v, name)
    _need(v > 0, f"{name} must be positive")
    return v


def _nonneg(v, name):
    v = _num(v, name)
    _need(v >= 0, f"{name} must be zero or positive")
    return v


def _vec(v, name, lo=1, hi=12):
    _need(isinstance(v, (list, tuple)), f"{name} must be a list of numbers")
    out = [_num(x, name) for x in v]
    _need(lo <= len(out) <= hi, f"{name} needs between {lo} and {hi} entries")
    return out


def _mat(v, name, hi=6):
    _need(isinstance(v, (list, tuple)) and len(v) > 0, f"{name} must be a matrix")
    rows = []
    for r in v:
        _need(isinstance(r, (list, tuple)), f"{name} must be a matrix (a list of rows)")
        rows.append([_num(x, name) for x in r])
    _need(all(len(r) == len(rows[0]) for r in rows), f"{name} must be rectangular")
    _need(len(rows) <= hi and len(rows[0]) <= hi, f"{name} is limited to {hi} x {hi}")
    return rows


def _square(v, name, hi=6):
    m = _mat(v, name, hi)
    _need(len(m) == len(m[0]), f"{name} must be square")
    return m


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


# ================================================================================================ rounding
def _r12(x):
    """12 significant digits (identical on every platform); -0 becomes 0."""
    return float(f"{x:.12g}") + 0.0


def _rnd(v):
    """Round every real number of an output (numbers, complex numbers, lists, matrices) to 12 digits."""
    if isinstance(v, bool) or isinstance(v, str) or v is None:
        return v
    if isinstance(v, int):
        return v
    if isinstance(v, float):
        return _r12(v)
    if isinstance(v, complex):
        im = _r12(v.imag)
        if im == 0:
            return _r12(v.real)
        return complex(_r12(v.real), im)
    if isinstance(v, (list, tuple)):
        return [_rnd(x) for x in v]
    if isinstance(v, dict):
        return {k: _rnd(x) for k, x in v.items()}
    return v


def _snapm(m, rel=1e-11):
    """Entries smaller than rel times the largest entry of a matrix or vector become exactly 0."""
    flat = [abs(x) for r in m for x in (r if isinstance(r, list) else [r])]
    big = max(flat) if flat else 0.0
    out = []
    for r in m:
        if isinstance(r, list):
            out.append([0.0 if abs(x) <= rel * big else x for x in r])
        else:
            out.append(0.0 if abs(r) <= rel * big else r)
    return out


# ================================================================================================ residuals
def _diff(a, b):
    if isinstance(a, str) or isinstance(b, str) or isinstance(a, bool) or isinstance(b, bool):
        return 0.0 if a == b else INF
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return INF
        w = 0.0
        for x, y in zip(a, b):
            w = max(w, _diff(x, y))
        return w
    if a is None or b is None:
        return 0.0 if a is b else INF
    try:
        a, b = complex(a), complex(b)
    except (TypeError, ValueError):
        return INF
    if a == b:
        return 0.0
    d = abs(a - b)
    if d != d:
        return INF
    return d / max(abs(a), abs(b))


def _recheck(run):
    """Residual of a deterministic algorithm: the outputs recomputed from the inputs."""
    def verify(inputs, outputs):
        try:
            want = run(**inputs)
        except Exception:
            return INF
        worst = 0.0
        for k, w in want.items():
            if k not in outputs:
                return INF
            worst = max(worst, _diff(outputs[k], w))
        return worst
    return verify


# ================================================================================================ polynomials
def _ptrim(c):
    c = [float(x) for x in c]
    while len(c) > 1 and c[0] == 0:
        c = c[1:]
    return c


def _pval(c, x):
    acc = 0 * x
    for a in c:
        acc = acc * x + a
    return acc


def _pder(c):
    n = len(c) - 1
    if n <= 0:
        return [0.0]
    return [c[i] * (n - i) for i in range(n)]


def _padd(a, b):
    n = max(len(a), len(b))
    a = [0.0] * (n - len(a)) + list(a)
    b = [0.0] * (n - len(b)) + list(b)
    return [x + y for x, y in zip(a, b)]


def _psub(a, b):
    return _padd(a, [-x for x in b])


def _pmul(a, b):
    out = [0.0] * (len(a) + len(b) - 1)
    for i, x in enumerate(a):
        for j, y in enumerate(b):
            out[i + j] += x * y
    return out


def _ppow(a, k):
    out = [1.0]
    for _ in range(k):
        out = _pmul(out, a)
    return out


def _pscale(a, k):
    return [k * x for x in a]


def _pad(a, n):
    return [0.0] * (n - len(a)) + list(a)


def _tf_in(num, den, proper=True, strict=False, hi=9):
    num = _ptrim(_vec(num, "num", 1, hi))
    den = _ptrim(_vec(den, "den", 1, hi))
    _need(any(x != 0 for x in den), "the denominator must not be zero")
    _need(any(x != 0 for x in num), "the numerator must not be zero")
    _need(len(den) >= 2, "the denominator needs degree 1 or more")
    if strict:
        _need(len(num) < len(den), "the transfer function must be strictly proper (numerator degree below the denominator's)")
    elif proper:
        _need(len(num) <= len(den), "the transfer function must be proper (numerator degree at most the denominator's)")
    return num, den


def _roots_raw(c):
    """Every root of c0 x^n + ... + cn (highest power first): Aberth-Ehrlich simultaneous iteration with Newton
    polishing; roots at 0 split off exactly."""
    c = _ptrim(c)
    if len(c) < 2:
        return []
    zeros = 0
    while len(c) > 1 and c[-1] == 0:
        c = c[:-1]
        zeros += 1
    n = len(c) - 1
    if n == 0:
        return [0.0] * zeros
    a = [x / c[0] for x in c]
    if n == 1:
        return [complex(-a[1], 0.0)] + [0.0] * zeros
    d = _pder(a)
    r = abs(a[-1]) ** (1.0 / n)
    for x in a[1:]:
        r = max(r, abs(x) ** 0.5 * 0.5)
    if r == 0:
        r = 1.0
    z = [r * cmath.exp(1j * (2 * math.pi * k / n + 0.4)) for k in range(n)]
    for _ in range(600):
        biggest = 0.0
        for i in range(n):
            p = _pval(a, z[i])
            if p == 0:
                continue
            dp = _pval(d, z[i])
            ratio = p / dp if dp != 0 else p * 1e-3
            s = 0j
            for j in range(n):
                if j != i and z[i] != z[j]:
                    s += 1.0 / (z[i] - z[j])
            w = ratio / (1 - ratio * s)
            z[i] -= w
            biggest = max(biggest, abs(w) / max(1.0, abs(z[i])))
        if biggest < 1e-17:
            break
    out = []
    for x in z:
        for _ in range(3):
            dp = _pval(d, x)
            if dp != 0:
                step = _pval(a, x) / dp
                if abs(step) < 1e-3 * max(1.0, abs(x)):
                    x = x - step
        out.append(x)
    out = _multiples(a, out)
    return out + [0.0] * zeros


def _multiples(a, roots):
    """Repeated roots: a cluster of m close roots that is a true m-fold root (the first m-1 derivatives vanish there)
    is replaced by the root of the (m-1)-th derivative, which is simple and found to full precision."""
    n = len(roots)
    used = [False] * n
    out = []
    scale = max(abs(x) for x in a)
    for i in range(n):
        if used[i]:
            continue
        group = [i]
        for j in range(i + 1, n):
            if not used[j] and abs(roots[j] - roots[i]) <= 1e-4 * max(1.0, abs(roots[i])):
                group.append(j)
        for j in group:
            used[j] = True
        m = len(group)
        if m == 1:
            out.append(roots[i])
            continue
        c = sum(roots[j] for j in group) / m
        der = a
        for _ in range(m - 1):
            der = _pder(der)
        d1 = _pder(der)
        for _ in range(60):
            dv = _pval(d1, c)
            if dv == 0:
                break
            step = _pval(der, c) / dv
            c = c - step
            if abs(step) <= 1e-16 * max(1.0, abs(c)):
                break
        ok = True
        dd = a
        for k in range(m - 1):
            mag = sum(abs(x) * max(1.0, abs(c)) ** (len(dd) - 1 - t) for t, x in enumerate(dd))
            if abs(_pval(dd, c)) > 1e-9 * max(mag, 1e-300) * (k + 1):
                ok = False
            dd = _pder(dd)
        if ok:
            out.extend([c] * m)
        else:
            out.extend(roots[j] for j in group)
    return out


def _canon(roots, tol=1e-9):
    """Real roots as floats; complex roots paired with their conjugates (equal real parts, opposite imaginary
    parts); rounded to 12 digits; sorted by real part, then imaginary part."""
    real, ups, downs = [], [], []
    big = max([abs(complex(r)) for r in roots], default=0.0)
    for r in roots:
        r = complex(r)
        if abs(r) <= 1e-12 * big:
            r = 0j
        if abs(r.real) <= 1e-12 * abs(r):
            r = complex(0.0, r.imag)
        if abs(r.imag) <= tol * max(1.0, abs(r)):
            real.append(r.real)
        elif r.imag > 0:
            ups.append(r)
        else:
            downs.append(r)
    out = [_r12(x) for x in real]
    ups = sorted(ups, key=lambda x: (x.real, x.imag))
    for u in ups:
        if not downs:
            break
        best = 0
        for k in range(len(downs)):
            if abs(downs[k] - u.conjugate()) < abs(downs[best] - u.conjugate()):
                best = k
        dn = downs.pop(best)
        re_ = _r12((u.real + dn.real) / 2)
        im = _r12((u.imag - dn.imag) / 2)
        out.append(complex(re_, -im))
        out.append(complex(re_, im))
    for d in downs:
        out.append(complex(_r12(d.real), _r12(d.imag)))
    out.sort(key=lambda x: (complex(x).real, complex(x).imag))
    return out


def _roots(c):
    return _canon(_roots_raw(c))


def _resid_roots(c, roots):
    """Largest relative residual of roots substituted into the polynomial."""
    worst = 0.0
    n = len(c) - 1
    if len(roots) != n:
        return INF
    for p in roots:
        p = complex(p)
        terms = [a * p ** (n - i) for i, a in enumerate(c)]
        s = sum(abs(t) for t in terms) or 1.0
        worst = max(worst, abs(sum(terms)) / s)
    return worst


def _real_pos_roots(c, tol=1e-7):
    """Distinct positive real roots of a real polynomial, Newton-polished in real arithmetic, ascending."""
    c = _ptrim(c)
    if len(c) < 2:
        return []
    d = _pder(c)
    found = []
    for r in _roots_raw(c):
        r = complex(r)
        if abs(r.imag) > tol * max(1.0, abs(r)) or r.real <= 0:
            continue
        x = r.real
        for _ in range(4):
            dv = _pval(d, x)
            if dv == 0:
                break
            step = _pval(c, x) / dv
            if abs(step) > 1e-6 * max(1.0, abs(x)):
                break
            x -= step
        if x > 1e-12:
            found.append(x)
    found.sort()
    out = []
    for x in found:
        if out and abs(x - out[-1]) <= 1e-7 * max(1.0, x):
            raise ValueError("a repeated (tangential) crossing: the result is not well defined")
        out.append(x)
    return out


def _jw(p):
    """Real and imaginary parts of p(j w) as real polynomials in w (highest power first)."""
    n = len(p) - 1
    re_ = [0.0] * (n + 1)
    im = [0.0] * (n + 1)
    for k, a in enumerate(p):
        m = n - k                     # the power of s, and of w
        idx = n - m
        q = m % 4
        if q == 0:
            re_[idx] += a
        elif q == 1:
            im[idx] += a
        elif q == 2:
            re_[idx] -= a
        else:
            im[idx] -= a
    return _ptrim(re_), _ptrim(im)


def _mag2(p):
    """|p(j w)|^2 as a real polynomial in w."""
    r, i = _jw(p)
    return _padd(_pmul(r, r), _pmul(i, i))


def _G(num, den, s):
    d = _pval(den, s)
    _need(abs(d) > 1e-300, "the transfer function has a pole at this point")
    return _pval(num, s) / d


def _deg(z):
    return math.degrees(cmath.phase(complex(z.real + 0.0, z.imag + 0.0)))


def _wrap180(a):
    """An angle in degrees wrapped into (-180, 180]."""
    a = math.fmod(a, 360.0)
    if a <= -180.0:
        a += 360.0
    elif a > 180.0:
        a -= 360.0
    return a + 0.0


# ================================================================================================ matrices
def _eye(n, one=1.0):
    return [[one if i == j else 0 * one for j in range(n)] for i in range(n)]


def _mmul(X, Y):
    n, m, p = len(X), len(Y), len(Y[0])
    out = []
    for i in range(n):
        row = []
        for j in range(p):
            acc = 0 * X[0][0]
            for k in range(m):
                acc = acc + X[i][k] * Y[k][j]
            row.append(acc)
        out.append(row)
    return out


def _madd(X, Y):
    return [[a + b for a, b in zip(r, s)] for r, s in zip(X, Y)]


def _msub(X, Y):
    return [[a - b for a, b in zip(r, s)] for r, s in zip(X, Y)]


def _mscale(X, k):
    return [[k * a for a in r] for r in X]


def _mT(X):
    return [[X[i][j] for i in range(len(X))] for j in range(len(X[0]))]


def _mv(X, v):
    return [sum(a * b for a, b in zip(r, v)) for r in X]


def _frac(X):
    if isinstance(X[0], list):
        return [[Fraction(x) for x in r] for r in X]
    return [Fraction(x) for x in X]


def _fl(X):
    if isinstance(X, list):
        return [_fl(x) for x in X]
    return float(X)


def _col(v):
    return [[x] for x in v]


def _trace(X):
    return sum(X[i][i] for i in range(len(X)))


def _solve(M, b, exact=False):
    """Gaussian elimination with partial pivoting; b a vector or a matrix of columns. ValueError when singular."""
    n = len(M)
    multi = isinstance(b[0], list)
    A = [list(M[i]) + (list(b[i]) if multi else [b[i]]) for i in range(n)]
    w = len(A[0])
    scale = max(abs(x) for r in M for x in r) if not exact else 0
    for c in range(n):
        p = c
        for r in range(c + 1, n):
            if abs(A[r][c]) > abs(A[p][c]):
                p = r
        if (exact and A[p][c] == 0) or (not exact and abs(A[p][c]) <= 1e-13 * max(scale, 1e-300)):
            raise ValueError("the matrix is singular")
        A[c], A[p] = A[p], A[c]
        for r in range(n):
            if r != c and A[r][c] != 0:
                f = A[r][c] / A[c][c]
                A[r] = [x - f * y for x, y in zip(A[r], A[c])]
    out = [[A[i][j] / A[i][i] for j in range(n, w)] for i in range(n)]
    return out if multi else [r[0] for r in out]


def _det(M):
    """Exact determinant of a matrix of Fractions (or floats) by elimination."""
    n = len(M)
    A = [list(r) for r in M]
    det = 1 + 0 * A[0][0]
    for c in range(n):
        p = None
        for r in range(c, n):
            if A[r][c] != 0:
                p = r
                break
        if p is None:
            return 0 * det
        if p != c:
            A[c], A[p] = A[p], A[c]
            det = -det
        det = det * A[c][c]
        for r in range(c + 1, n):
            if A[r][c] != 0:
                f = A[r][c] / A[c][c]
                A[r] = [x - f * y for x, y in zip(A[r], A[c])]
    return det


def _rank(M):
    """Exact rank of a matrix of Fractions."""
    A = [list(r) for r in M]
    rows, cols = len(A), len(A[0])
    rk = 0
    for c in range(cols):
        p = None
        for r in range(rk, rows):
            if A[r][c] != 0:
                p = r
                break
        if p is None:
            continue
        A[rk], A[p] = A[p], A[rk]
        for r in range(rows):
            if r != rk and A[r][c] != 0:
                f = A[r][c] / A[rk][c]
                A[r] = [x - f * y for x, y in zip(A[r], A[rk])]
        rk += 1
        if rk == rows:
            break
    return rk


def _faddeev(A):
    """Faddeev-LeVerrier: the characteristic polynomial [1, c1, ..., cn] of A and the matrices M1..Mn with
    adj(sI - A) = sum_k M_k s^(n-k). Exact for Fractions."""
    n = len(A)
    one = 1 + 0 * A[0][0]
    I = _eye(n, one)
    M = [[0 * one] * n for _ in range(n)]
    c = [one]
    Ms = []
    for k in range(1, n + 1):
        M = _madd(_mmul(A, M), _mscale(I, c[-1]))
        Ms.append(M)
        c.append(-_trace(_mmul(A, M)) / k)
    return c, Ms


def _ss2tf(A, B, C, D):
    """Numerator and denominator (monic, length n+1) of C (sI - A)^-1 B + D by Faddeev-LeVerrier."""
    n = len(A)
    c, Ms = _faddeev(A)
    num = [D * c[0]]
    for k in range(n):
        v = _mv(Ms[k], B)
        num.append(sum(a * b for a, b in zip(C, v)) + D * c[k + 1])
    return num, c


def _ccf(num, den):
    """scipy's controllable canonical form of a proper transfer function."""
    den = _ptrim(den)
    num = _ptrim(num)
    n = len(den) - 1
    a0 = den[0]
    den = [x / a0 for x in den]
    num = _pad([x / a0 for x in num], n + 1)
    D = num[0]
    b = [num[i] - D * den[i] for i in range(n + 1)]
    A = [[0.0] * n for _ in range(n)]
    A[0] = [-x for x in den[1:]]
    for i in range(1, n):
        A[i][i - 1] = 1.0
    B = [1.0] + [0.0] * (n - 1)
    C = b[1:]
    return A, B, C, D


def _mnorm1(X):
    return max(sum(abs(x) for x in r) for r in X)


def _expm(M):
    """Matrix exponential (float): scaling and squaring with a Taylor series."""
    n = len(M)
    nrm = _mnorm1(M)
    s = 0
    while nrm > 0.5:
        nrm /= 2.0
        s += 1
    X = _mscale(M, 1.0 / (2.0 ** s))
    E = _eye(n)
    T = _eye(n)
    for k in range(1, 30):
        T = _mscale(_mmul(T, X), 1.0 / k)
        E = _madd(E, T)
        if _mnorm1(T) <= 1e-18 * _mnorm1(E):
            break
    for _ in range(s):
        E = _mmul(E, E)
    return E


def _is_hurwitz(A):
    c, _ = _faddeev(_frac(A))
    return max(complex(r).real for r in _roots(_fl(c))) < 0


def _routh_ok(c):
    """True when every root of the real polynomial c has a negative real part (exact Routh test on Fractions;
    a zero in the first column counts as not stable)."""
    c = [Fraction(x) for x in _ptrim(c)]
    n = len(c) - 1
    if n == 0:
        return True
    if c[0] < 0:
        c = [-x for x in c]
    width = n // 2 + 1
    r0 = c[0::2] + [Fraction(0)] * (width - len(c[0::2]))
    r1 = c[1::2] + [Fraction(0)] * (width - len(c[1::2]))
    rows = [r0, r1]
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        if b[0] == 0:
            return False
        rows.append([(b[0] * a[i + 1] - a[0] * b[i + 1]) / b[0] for i in range(width - 1)] + [Fraction(0)])
    return all(r[0] > 0 for r in rows)


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(%|percent|per cent)?\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[\[.*\]\]|\[[^\]\[]*\])\s*$")


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs, decline=None):
    """The one documented question form, '<phrase>: name = value, ...'.

    Reading guard: the capability phrase must open the question and be followed by a colon (wording that names
    another capability never reaches this reader); the rest is split into 'name = value' pairs and any other text,
    unknown name, repeated name or unit on a plain number refuses the whole question (a stray number is never
    ignored); a fraction input accepts 20% or 0.2 but refuses a bare 20; conventions are inputs, never assumed.

    `decline` (a regex): prose that matches it is declined as OUT_OF_SCOPE by this capability, so that no generic
    fallback reads it instead (the shared typed-formula solver once 'solved' a written matrix Lyapunov equation with
    an empty answer, ask sweep 2026-10-31)."""
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    drx = re.compile(decline, re.I | re.S) if decline else None
    by = {}
    names = {}
    for i in inputs:
        by[i["symbol"]] = i
        names[i["symbol"]] = i["symbol"]
    for i in inputs:
        names.setdefault(i["symbol"].lower(), i["symbol"])

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            if drx is not None and drx.search(question):
                return {"_out_of_scope": f"write the documented form '{phrases[0]}: " +
                                         ", ".join(f"{i['symbol']} = ..." for i in inputs) + "'"}
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") not in ("vector", "matrix"):
                    return None
                text = mv.group(2).strip()
                if by[key]["type"] == "matrix":
                    if not text.startswith("[["):
                        return None
                    rows = re.findall(r"\[([^\[\]]*)\]", text[1:-1])
                    out[key] = "; ".join(r.strip() for r in rows)
                else:
                    if text.startswith("[["):
                        return None
                    out[key] = text
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is None or key in out or by[key].get("type") != "choice":
                    return None
                out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                continue
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            if t in ("vector", "matrix", "choice"):
                return None
            num, pct = ms.group(2), ms.group(3)
            if t == "integer":
                if pct:
                    return None
                out[key] = num
                continue
            if pct:
                if not spec.get("pct"):
                    return None
                out[key] = float(num) / 100.0
                continue
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        return out
    return parse


def _w(v):
    """A number as a user writes it."""
    v = float(v)
    if v == int(v) and abs(v) < 1e15:
        return str(int(v))
    return repr(v)


def _written(spec, v):
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_w(x) for x in v) + "]"
    if t == "matrix":
        return "[" + ", ".join("[" + ", ".join(_w(x) for x in r) + "]" for r in v) + "]"
    if t == "choice":
        return str(v)
    if t == "integer":
        return str(int(v))
    return _w(v)


# ================================================================================================ registry
def _n(sym, meaning, required=True, pct=False):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required}
    if pct:
        d["pct"] = True
    return d


def _int(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _v(sym, meaning, lo=1, hi=10, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _m(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _ch(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, meaning, kind="number"):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}
WEAK: dict = {}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), extra_q=(), verify=None, weak=None, decline=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, decline)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex_inputs):
        return f"{phrases[0]}: " + ", ".join(f"{k} = {_written(by[k], v)}" for k, v in ex_inputs.items())
    templates = [ask_text(e["inputs"]) for e in examples[:2]]
    questions = [{"q": ask_text(e["inputs"]), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        partial = {k: v for k, v in examples[0]["inputs"].items() if k != req[0]}
        questions.append({"q": ask_text(partial), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]["inputs"]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    for q in extra_q:
        if "inputs" in q:
            questions.append({"q": ask_text(q["inputs"]), "outcome": q["outcome"]})
        else:
            questions.append(q)
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": list(examples), "questions": questions}
    LEVELS[key] = level
    if weak:
        WEAK[key] = weak


def _ex(inputs, outputs):
    return {"inputs": inputs, "outputs": outputs}


def _bad(inputs, outcome="INVALID_INPUT"):
    return {"inputs": inputs, "outcome": outcome}


# ================================================================================================ reference helpers
def _ref_canon(vals):
    return _canon([complex(x) for x in vals])


def _ref_step(num, den, t, A=1.0):
    """Step response at time t from scipy's matrix exponential of the augmented state-space model."""
    import numpy as np
    from scipy.linalg import expm
    from scipy.signal import tf2ss
    a, b, c, d = tf2ss(num, den)
    n = a.shape[0]
    M = np.zeros((n + 1, n + 1))
    M[:n, :n] = a
    M[:n, n:] = b
    E = expm(M * t)
    return float(A * ((c @ E[:n, n:])[0, 0] + d[0, 0]))


def _ref_impulse(num, den, t):
    import numpy as np
    from scipy.linalg import expm
    from scipy.signal import tf2ss
    a, b, c, d = tf2ss(num, den)
    return float((c @ expm(a * t) @ b)[0, 0])


def _ref_2nd_peak(zeta, wn=1.0):
    """Peak time and peak value of the unit step response of wn^2/(s^2 + 2 zeta wn s + wn^2), computed numerically:
    the first zero of the impulse response (brentq), then the step response there."""
    from scipy.optimize import brentq
    num, den = [wn * wn], [1.0, 2 * zeta * wn, wn * wn]
    wd = wn * math.sqrt(1 - zeta * zeta)
    tp = brentq(lambda t: _ref_impulse(num, den, t), 0.5 * math.pi / wd, 1.5 * math.pi / wd, xtol=1e-15, rtol=8.9e-16)
    return tp, _ref_step(num, den, tp)


def _ref_freq(num, den, w):
    from scipy.signal import freqs
    _, h = freqs(num, den, worN=[w])
    return complex(h[0])


def _ref_crossing(f, lo, hi):
    from scipy.optimize import brentq
    return brentq(f, lo, hi, xtol=1e-300, rtol=8.9e-16, maxiter=500)


def _rng_stable_poly(rng, n, lo=0.3, hi=6.0, cplx=True):
    """Coefficients of a random stable monic polynomial of degree n with distinct roots (some complex pairs)."""
    p = [1.0]
    k = 0
    used = []
    while k < n:
        if cplx and n - k >= 2 and rng.random() < 0.4:
            a = round(rng.uniform(lo, hi), 1)
            b = round(rng.uniform(0.5, hi), 1)
            p = _pmul(p, [1.0, 2 * a, a * a + b * b])
            k += 2
        else:
            a = round(rng.uniform(lo, hi), 1)
            if any(abs(a - u) < 0.25 for u in used):
                continue
            used.append(a)
            p = _pmul(p, [1.0, a])
            k += 1
    return [_r12(x) for x in p]


# ================================================================================================ first-order systems
def first_order_step(K, tau, t, A=1.0):
    K, tau, t, A = _num(K, "K"), _pos(tau, "tau"), _nonneg(t, "t"), _num(A, "A")
    _need(K != 0 and A != 0, "K and A must not be zero")
    frac = -math.expm1(-t / tau)
    return _rnd({"y": K * A * frac, "fraction": frac, "y_final": K * A})


def ref_first_order_step(K, tau, t, A=1.0):
    y = _ref_step([K], [tau, 1.0], t, A)
    return {"y": y, "fraction": y / (K * A), "y_final": K * A}


_add("ctrl_first_order_step", name="Step response of a first-order system", topic="time response / first order",
     purpose="The output y(t) = K A (1 - e^{-t/tau}) of K/(tau s + 1) at time t after a step of size A, and the fraction of the final value reached.",
     inputs=[_n("K", "steady-state (DC) gain of the system"), _n("tau", "time constant in seconds"),
             _n("t", "time after the step in seconds"), _n("A", "step size (default 1)", required=False)],
     outputs=[_o("y", "output at time t"), _o("fraction", "fraction of the final value reached, 1 - e^{-t/tau}"),
              _o("y_final", "final value K A")],
     primary="y", conditions=["tau > 0", "t >= 0", "K and A not zero"], method="y = K A (1 - exp(-t/tau))",
     latex=r"y(t) = K A\left(1 - e^{-t/\tau}\right)", run=first_order_step, reference=ref_first_order_step,
     wrong=lambda K, tau, t, A=1.0: {"y": K * A * (1 - math.exp(-t * tau)), "fraction": 1 - math.exp(-t * tau), "y_final": K * A},
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.2, 20), 2),
                       "t": round(rng.uniform(0.1, 3.0), 2) * 2, "A": round(rng.uniform(0.5, 5), 1)},
     phrases=["first-order step response", "first order step response"],
     examples=[_ex({"K": 2, "tau": 5, "t": 5}, {"y": 1.26424111766, "fraction": 0.632120558829}),
               _ex({"K": 1, "tau": 2, "t": 4, "A": 10}, {"y": 8.64664716763})],
     extra_q=[_bad({"K": 2, "tau": -5, "t": 5})], level="beginner")


def first_order_specs(K, tau):
    K, tau = _num(K, "K"), _pos(tau, "tau")
    _need(K != 0, "K must not be zero")
    return _rnd({"time_constant": tau, "rise_time": tau * math.log(9.0), "settling_time_2": tau * math.log(50.0),
                 "settling_time_5": tau * math.log(20.0), "bandwidth": 1.0 / tau, "dc_gain": K})


def ref_first_order_specs(K, tau):
    f = lambda lvl: (lambda t: _ref_step([1.0], [tau, 1.0], t) - lvl)
    t10 = _ref_crossing(f(0.1), 1e-9 * tau, 50 * tau)
    t90 = _ref_crossing(f(0.9), 1e-9 * tau, 50 * tau)
    wb = _ref_crossing(lambda w: abs(_ref_freq([1.0], [tau, 1.0], w)) - 1 / math.sqrt(2), 1e-6 / tau, 1e3 / tau)
    return {"time_constant": _ref_crossing(f(1 - math.exp(-1)), 1e-9 * tau, 50 * tau), "rise_time": t90 - t10,
            "settling_time_2": _ref_crossing(f(0.98), 1e-9 * tau, 50 * tau),
            "settling_time_5": _ref_crossing(f(0.95), 1e-9 * tau, 50 * tau), "bandwidth": wb, "dc_gain": K}


_add("ctrl_first_order_specs", name="Time and frequency specifications of a first-order system",
     topic="time response / first order",
     purpose="Time constant, 10-90 % rise time tau ln 9, 2 % and 5 % settling times tau ln 50 and tau ln 20, bandwidth 1/tau and DC gain of K/(tau s + 1).",
     inputs=[_n("K", "steady-state (DC) gain"), _n("tau", "time constant in seconds")],
     outputs=[_o("time_constant", "time constant (63.2 % of the final value)", "time"),
              _o("rise_time", "10-90 % rise time", "time"), _o("settling_time_2", "2 % settling time", "time"),
              _o("settling_time_5", "5 % settling time", "time"), _o("bandwidth", "-3 dB bandwidth", "angular_frequency"),
              _o("dc_gain", "DC gain")],
     primary="rise_time", conditions=["tau > 0"], method="exact crossing times of 1 - exp(-t/tau)",
     latex=r"t_r = \tau\ln 9,\quad t_{s,2\%} = \tau\ln 50,\quad \omega_B = 1/\tau",
     run=first_order_specs, reference=ref_first_order_specs,
     wrong=lambda K, tau: {"time_constant": tau, "rise_time": tau * math.log(10.0), "settling_time_2": 4 * tau,
                           "settling_time_5": 3 * tau, "bandwidth": 2 * math.pi / tau, "dc_gain": K},
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.05, 30), 2)},
     phrases=["first-order specifications", "first order specifications"],
     examples=[_ex({"K": 3, "tau": 2}, {"rise_time": 4.39444915467, "settling_time_2": 7.82404601086, "bandwidth": 0.5}),
               _ex({"K": 1, "tau": 0.1}, {"settling_time_5": 0.299573227355})],
     extra_q=[_bad({"K": 3, "tau": 0})], level="beginner")


def first_order_freq(K, tau, w):
    K, tau, w = _pos(K, "K"), _pos(tau, "tau"), _nonneg(w, "w")
    mag = K / math.sqrt(1.0 + (w * tau) ** 2)
    return _rnd({"magnitude": mag, "magnitude_db": 20 * math.log10(mag), "phase_deg": -math.degrees(math.atan(w * tau))})


def ref_first_order_freq(K, tau, w):
    h = _ref_freq([K], [tau, 1.0], w)
    return {"magnitude": abs(h), "magnitude_db": 20 * math.log10(abs(h)), "phase_deg": math.degrees(cmath.phase(h))}


_add("ctrl_first_order_freq", name="Frequency response of a first-order lag", topic="frequency response / Bode",
     purpose="Magnitude K/sqrt(1 + (w tau)^2), its value in dB and the phase -atan(w tau) of K/(tau s + 1) at the angular frequency w.",
     inputs=[_n("K", "DC gain (positive)"), _n("tau", "time constant in seconds"), _n("w", "angular frequency in rad/s")],
     outputs=[_o("magnitude", "gain |G(jw)|"), _o("magnitude_db", "gain in dB"), _o("phase_deg", "phase in degrees")],
     primary="magnitude", conditions=["K > 0", "tau > 0", "w >= 0"], method="G(jw) = K/(1 + j w tau)",
     latex=r"|G(j\omega)| = \frac{K}{\sqrt{1+\omega^2\tau^2}},\quad \angle G = -\arctan(\omega\tau)",
     run=first_order_freq, reference=ref_first_order_freq,
     wrong=lambda K, tau, w: {"magnitude": K / (1 + w * tau), "magnitude_db": 20 * math.log10(K / (1 + w * tau)),
                              "phase_deg": -math.degrees(math.atan(w / tau))},
     rand=lambda rng: {"K": round(rng.uniform(0.2, 20), 2), "tau": round(rng.uniform(0.05, 10), 2), "w": round(rng.uniform(0.05, 50), 2)},
     phrases=["first-order frequency response", "first order frequency response"],
     examples=[_ex({"K": 10, "tau": 1, "w": 1}, {"magnitude": 7.07106781187, "phase_deg": -45.0}),
               _ex({"K": 1, "tau": 0.5, "w": 4}, {"magnitude_db": -6.98970004336})],
     extra_q=[_bad({"K": 10, "tau": 1, "w": -1})], level="beginner")


# ================================================================================================ second-order basics
def _class(zeta):
    if zeta == 0:
        return "undamped"
    if zeta < 1:
        return "underdamped"
    if zeta == 1:
        return "critically damped"
    return "overdamped"


def mass_spring_damper(m, c, k):
    m, c, k = _pos(m, "m"), _nonneg(c, "c"), _pos(k, "k")
    wn = math.sqrt(k / m)
    zeta = c / (2 * math.sqrt(k * m))
    if c * c == 4 * k * m:
        zeta = 1.0
    wd = wn * math.sqrt(1 - zeta * zeta) if zeta < 1 else 0.0
    return _rnd({"wn": wn, "zeta": zeta, "wd": wd, "dc_gain": 1.0 / k, "damping": _class(zeta)})


def _ref_wn_zeta(den):
    """wn and zeta from the two poles python-control computes (product and sum of the poles)."""
    import control
    p = [complex(x) for x in control.tf([1.0], den).poles()]
    wn = math.sqrt((p[0] * p[1]).real)
    zeta = -(p[0] + p[1]).real / (2 * wn)
    if abs(zeta - 1) < 1e-12:
        zeta = 1.0
    return wn, zeta, max(abs(x.imag) for x in p), p


def ref_mass_spring_damper(m, c, k):
    import control
    wn, zeta, wd, _ = _ref_wn_zeta([m, c, k])
    return {"wn": wn, "zeta": zeta, "wd": wd if zeta < 1 else 0.0,
            "dc_gain": float(control.dcgain(control.tf([1.0], [m, c, k]))), "damping": _class(zeta)}


_add("ctrl_mass_spring_damper", name="Mass-spring-damper natural frequency and damping ratio",
     topic="time response / second order",
     purpose="Natural frequency sqrt(k/m), damping ratio c/(2 sqrt(k m)), damped frequency and static compliance 1/k of m x'' + c x' + k x = F.",
     inputs=[_n("m", "mass in kg"), _n("c", "damping coefficient in N*s/m"), _n("k", "spring stiffness in N/m")],
     outputs=[_o("wn", "undamped natural frequency", "angular_frequency"), _o("zeta", "damping ratio"),
              _o("wd", "damped natural frequency (0 when not underdamped)", "angular_frequency"),
              _o("dc_gain", "static gain x/F = 1/k in m/N"), _o("damping", "damping class", "text")],
     primary="wn", conditions=["m > 0", "c >= 0", "k > 0"], method="G(s) = 1/(m s^2 + c s + k)",
     latex=r"\omega_n = \sqrt{k/m},\quad \zeta = \frac{c}{2\sqrt{km}}",
     run=mass_spring_damper, reference=ref_mass_spring_damper,
     wrong=lambda m, c, k: {"wn": math.sqrt(k / m), "zeta": c / (2 * math.sqrt(k / m)), "wd": 0.0, "dc_gain": 1 / k, "damping": "?"},
     rand=lambda rng: {"m": round(rng.uniform(0.2, 20), 2), "c": round(rng.uniform(0.1, 30), 2), "k": round(rng.uniform(1, 500), 1)},
     phrases=["mass-spring-damper", "mass spring damper"],
     examples=[_ex({"m": 1, "c": 2, "k": 4}, {"wn": 2.0, "zeta": 0.5, "wd": 1.73205080757, "damping": "underdamped"}),
               _ex({"m": 2, "c": 8, "k": 8}, {"zeta": 1.0, "damping": "critically damped"})],
     extra_q=[_bad({"m": 0, "c": 2, "k": 4})], level="beginner")


def second_order_params(den, K=None):
    d = _vec(den, "den", 3, 3)
    a2, a1, a0 = d
    _need(a2 != 0, "the s^2 coefficient must not be zero")
    a1, a0 = a1 / a2, a0 / a2
    _need(a0 > 0 and a1 >= 0, "the polynomial must be s^2 + 2 zeta wn s + wn^2 with wn^2 > 0 and zeta >= 0")
    wn = math.sqrt(a0)
    zeta = a1 / (2 * wn)
    if a1 * a1 == 4 * a0:
        zeta = 1.0
    wd = wn * math.sqrt(1 - zeta * zeta) if zeta < 1 else 0.0
    out = {"wn": wn, "zeta": zeta, "sigma": zeta * wn, "wd": wd, "damping": _class(zeta)}
    out["dc_gain"] = (_num(K, "K") / d[2]) if K is not None else 1.0
    return _rnd(out)


def ref_second_order_params(den, K=None):
    import control
    wn, zeta, wd, p = _ref_wn_zeta(den)
    sig = -p[0].real if zeta < 1 else zeta * wn
    dc = float(control.dcgain(control.tf([K], den))) if K is not None else 1.0
    return {"wn": wn, "zeta": zeta, "sigma": sig, "wd": wd if zeta < 1 else 0.0, "damping": _class(zeta), "dc_gain": dc}


def _rand_den2(rng):
    return {"den": [round(rng.uniform(0.5, 4), 1), round(rng.uniform(0.2, 12), 2), round(rng.uniform(1, 60), 1)],
            "K": round(rng.uniform(0.5, 60), 1)}


_add("ctrl_second_order_params", name="Natural frequency and damping ratio of a second-order denominator",
     topic="time response / second order",
     purpose="wn, zeta, the decay rate sigma = zeta wn, the damped frequency wd and the DC gain of K/(a2 s^2 + a1 s + a0).",
     inputs=[_v("den", "denominator [a2, a1, a0]", 3, 3), _n("K", "numerator constant (default: unit DC gain)", required=False)],
     outputs=[_o("wn", "natural frequency", "angular_frequency"), _o("zeta", "damping ratio"),
              _o("sigma", "decay rate zeta wn (1/s)"), _o("wd", "damped frequency (0 unless underdamped)", "angular_frequency"),
              _o("damping", "damping class", "text"), _o("dc_gain", "DC gain K/a0")],
     primary="wn", conditions=["a0/a2 > 0", "a1/a2 >= 0"], method="compare with s^2 + 2 zeta wn s + wn^2",
     latex=r"s^2 + 2\zeta\omega_n s + \omega_n^2", run=second_order_params, reference=ref_second_order_params,
     wrong=lambda den, K=None: {"wn": den[2] / den[0], "zeta": den[1] / (2 * den[2]), "sigma": 0.0, "wd": 0.0, "damping": "?", "dc_gain": 1.0},
     rand=_rand_den2, phrases=["second-order parameters", "second order parameters"],
     examples=[_ex({"den": [1, 4, 25]}, {"wn": 5.0, "zeta": 0.4, "sigma": 2.0, "wd": 4.58257569496}),
               _ex({"den": [2, 12, 50], "K": 50}, {"wn": 5.0, "zeta": 0.6, "dc_gain": 1.0})],
     extra_q=[_bad({"den": [1, 4, -25]})], level="beginner")


def percent_overshoot(zeta):
    zeta = _nonneg(zeta, "zeta")
    _need(zeta < 1, "overshoot needs an underdamped system: 0 <= zeta < 1")
    mp = math.exp(-zeta * PI / math.sqrt(1 - zeta * zeta))
    return _rnd({"PO": 100 * mp, "Mp": mp, "pole_angle_deg": math.degrees(math.acos(zeta))})


def ref_percent_overshoot(zeta):
    _, yp = _ref_2nd_peak(zeta)
    return {"PO": 100 * (yp - 1), "Mp": yp - 1}


_add("ctrl_percent_overshoot", name="Percent overshoot from the damping ratio", topic="time response / second order",
     purpose="Percent overshoot 100 e^{-zeta pi/sqrt(1 - zeta^2)} of the step response of a standard second-order system, and the pole angle acos(zeta).",
     inputs=[_n("zeta", "damping ratio (0 <= zeta < 1)")],
     outputs=[_o("PO", "percent overshoot (%)"), _o("Mp", "overshoot as a fraction of the final value"),
              _o("pole_angle_deg", "angle of the poles from the negative real axis, degrees")],
     primary="PO", conditions=["0 <= zeta < 1"], method="peak of the step response at t = pi/wd",
     latex=r"\%OS = 100\,e^{-\zeta\pi/\sqrt{1-\zeta^2}}", run=percent_overshoot, reference=ref_percent_overshoot,
     wrong=lambda zeta: {"PO": 100 * math.exp(-zeta * PI / (1 - zeta * zeta)), "Mp": math.exp(-zeta * PI / (1 - zeta * zeta)), "pole_angle_deg": 0.0},
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 0.9), 3)}, phrases=["percent overshoot"],
     examples=[_ex({"zeta": 0.5}, {"PO": 16.3033534821, "pole_angle_deg": 60.0}),
               _ex({"zeta": 0.7}, {"PO": 4.59879102603})],
     extra_q=[_bad({"zeta": 1.2})], level="beginner")


def damping_from_overshoot(OS):
    mp = _num(OS, "OS")
    _need(0 < mp < 1, "the overshoot must lie strictly between 0 and 100 %")
    L = math.log(mp)
    zeta = -L / math.sqrt(PI * PI + L * L)
    return _rnd({"zeta": zeta, "pole_angle_deg": math.degrees(math.acos(zeta))})


def ref_damping_from_overshoot(OS):
    from scipy.optimize import brentq
    z = brentq(lambda zz: (_ref_2nd_peak(zz)[1] - 1) - OS, 1e-6, 0.99, xtol=1e-15, rtol=8.9e-16)
    return {"zeta": z}


_add("ctrl_damping_from_overshoot", name="Damping ratio from percent overshoot", topic="time response / second order",
     purpose="The damping ratio zeta = -ln(OS)/sqrt(pi^2 + ln^2 OS) that gives a stated overshoot.",
     inputs=[_n("OS", "overshoot (write 20% or 0.2)", pct=True)],
     outputs=[_o("zeta", "damping ratio"), _o("pole_angle_deg", "pole angle from the negative real axis, degrees")],
     primary="zeta", conditions=["0 < OS < 100 %"], method="invert OS = exp(-zeta pi / sqrt(1 - zeta^2))",
     latex=r"\zeta = \frac{-\ln(\%OS/100)}{\sqrt{\pi^2 + \ln^2(\%OS/100)}}", run=damping_from_overshoot,
     reference=ref_damping_from_overshoot,
     wrong=lambda OS: {"zeta": -math.log(OS) / math.sqrt(PI + math.log(OS) ** 2), "pole_angle_deg": 0.0},
     rand=lambda rng: {"OS": round(rng.uniform(0.005, 0.8), 4)}, phrases=["damping ratio from overshoot"],
     examples=[_ex({"OS": 0.2}, {"zeta": 0.455949810453}), _ex({"OS": 0.05}, {"zeta": 0.690106730559})],
     extra_q=[{"q": "damping ratio from overshoot: OS = 20", "outcome": "UNRECOGNIZED_FORMAT"},
              {"q": "damping ratio from overshoot: OS = 20%", "outputs": {"zeta": 0.455949810453}},
              _bad({"OS": 0.0})], level="beginner")


def peak_time(zeta, wn):
    zeta, wn = _nonneg(zeta, "zeta"), _pos(wn, "wn")
    _need(zeta < 1, "a peak needs an underdamped system: 0 <= zeta < 1")
    wd = wn * math.sqrt(1 - zeta * zeta)
    return _rnd({"Tp": PI / wd, "wd": wd})


def ref_peak_time(zeta, wn):
    tp, _ = _ref_2nd_peak(zeta, wn)
    return {"Tp": tp}


_add("ctrl_peak_time", name="Peak time of a second-order step response", topic="time response / second order",
     purpose="Peak time Tp = pi/wd and damped frequency wd = wn sqrt(1 - zeta^2) of wn^2/(s^2 + 2 zeta wn s + wn^2).",
     inputs=[_n("zeta", "damping ratio (0 <= zeta < 1)"), _n("wn", "natural frequency in rad/s")],
     outputs=[_o("Tp", "peak time", "time"), _o("wd", "damped frequency", "angular_frequency")],
     primary="Tp", conditions=["0 <= zeta < 1", "wn > 0"], method="first zero of the impulse response",
     latex=r"T_p = \frac{\pi}{\omega_n\sqrt{1-\zeta^2}}", run=peak_time, reference=ref_peak_time,
     wrong=lambda zeta, wn: {"Tp": PI / wn, "wd": wn},
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 0.9), 3), "wn": round(rng.uniform(0.2, 50), 2)},
     phrases=["peak time"],
     examples=[_ex({"zeta": 0.6, "wn": 5}, {"Tp": 0.785398163397, "wd": 4.0}),
               _ex({"zeta": 0, "wn": 2}, {"Tp": 1.57079632679})],
     extra_q=[_bad({"zeta": 1.0, "wn": 5})], level="beginner")


def settling_time(zeta, wn, band=0.02):
    zeta, wn, band = _num(zeta, "zeta"), _pos(wn, "wn"), _num(band, "band")
    _need(0 < zeta < 1, "the settling-time rule is for an underdamped system: 0 < zeta < 1")
    _need(band in (0.02, 0.05), "the rule of thumb is stated for a 2 % or a 5 % band")
    s = zeta * wn
    rule = (4.0 if band == 0.02 else 3.0) / s
    env = -math.log(band * math.sqrt(1 - zeta * zeta)) / s
    return _rnd({"Ts": rule, "Ts_envelope": env, "time_constant": 1.0 / s})


def ref_settling_time(zeta, wn, band=0.02):
    from scipy.optimize import brentq
    s = zeta * wn
    env = brentq(lambda t: math.exp(-s * t) / math.sqrt(1 - zeta * zeta) - band, 0.0, 100.0 / s, xtol=1e-300, rtol=8.9e-16)
    return {"Ts": (4.0 if band == 0.02 else 3.0) / s, "Ts_envelope": env, "time_constant": 1 / s}


_add("ctrl_settling_time", name="Settling time of a second-order system (rule and envelope)",
     topic="time response / second order",
     purpose="Settling time by the rule 4/(zeta wn) (2 % band) or 3/(zeta wn) (5 %), and the time at which the decay envelope enters the band.",
     inputs=[_n("zeta", "damping ratio (0 < zeta < 1)"), _n("wn", "natural frequency in rad/s"),
             _n("band", "settling band: 2% (default) or 5%", required=False, pct=True)],
     outputs=[_o("Ts", "settling time by the rule of thumb", "time"),
              _o("Ts_envelope", "time at which e^{-zeta wn t}/sqrt(1-zeta^2) equals the band", "time"),
              _o("time_constant", "envelope time constant 1/(zeta wn)", "time")],
     primary="Ts", conditions=["0 < zeta < 1", "band 2 % or 5 %"], method="Ts = 4/(zeta wn) or 3/(zeta wn)",
     latex=r"T_s \approx \frac{4}{\zeta\omega_n}\;(2\%),\qquad T_s \approx \frac{3}{\zeta\omega_n}\;(5\%)",
     run=settling_time, reference=ref_settling_time,
     wrong=lambda zeta, wn, band=0.02: {"Ts": 4.0 / wn, "Ts_envelope": 4.0 / wn, "time_constant": 1 / wn},
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 0.95), 3), "wn": round(rng.uniform(0.2, 50), 2),
                       "band": rng.choice([0.02, 0.05])},
     phrases=["settling time"],
     examples=[_ex({"zeta": 0.5, "wn": 4}, {"Ts": 2.0, "Ts_envelope": 2.02793202083}),
               _ex({"zeta": 0.5, "wn": 4, "band": 0.05}, {"Ts": 1.5})],
     extra_q=[_bad({"zeta": 0.5, "wn": 4, "band": 0.1})], level="beginner",
     weak="the rule of thumb is restated; the envelope time is a re-solve of its defining equation (brentq)")


# ================================================================================================ DC gain and poles
def _rand_stable_tf(rng, nmax=4):
    n = rng.randint(1, nmax)
    den = _rng_stable_poly(rng, n)
    m = rng.randint(0, n - 1)
    num = [round(rng.uniform(0.5, 10), 1)]
    for _ in range(m):
        num = _pmul(num, [1.0, round(rng.uniform(0.2, 8), 1)])
    return {"num": [_r12(x) for x in num], "den": den}


def dc_gain(num, den, A=1.0):
    num, den = _tf_in(num, den)
    A = _num(A, "A")
    _need(den[-1] != 0, "G(s) has a pole at s = 0: the DC gain is infinite")
    poles = _roots(den)
    _need(max(complex(p).real for p in poles) < 0, "the system is not stable, so its step response has no final value")
    g = num[-1] / den[-1]
    return _rnd({"dc_gain": g, "final_value": g * A})


def ref_dc_gain(num, den, A=1.0):
    import control
    sys = control.tf(num, den)
    g = float(control.dcgain(sys))
    return {"dc_gain": g, "final_value": g * A}


_add("ctrl_dc_gain", name="DC gain and final value of a step response", topic="transfer functions / DC gain",
     purpose="The DC gain G(0) of a stable transfer function and the final value G(0) A of its response to a step of size A.",
     inputs=[_v("num", "numerator coefficients, highest power of s first"), _v("den", "denominator coefficients, highest power of s first", 2),
             _n("A", "step size (default 1)", required=False)],
     outputs=[_o("dc_gain", "DC gain G(0)"), _o("final_value", "final value of the step response")],
     primary="dc_gain", conditions=["proper G(s)", "every pole in the left half plane"],
     method="G(0) = num(0)/den(0) after a stability check of the poles", latex=r"y(\infty) = A\,G(0)",
     run=dc_gain, reference=ref_dc_gain,
     wrong=lambda num, den, A=1.0: {"dc_gain": num[0] / den[0], "final_value": A * num[0] / den[0]},
     rand=lambda rng: dict(_rand_stable_tf(rng), A=round(rng.uniform(0.5, 10), 1)), phrases=["DC gain"],
     examples=[_ex({"num": [10], "den": [1, 3, 2]}, {"dc_gain": 5.0, "final_value": 5.0}),
               _ex({"num": [2, 6], "den": [1, 5, 4], "A": 3}, {"dc_gain": 1.5, "final_value": 4.5})],
     extra_q=[_bad({"num": [1], "den": [1, -1]}), _bad({"num": [1], "den": [1, 2, 0]})], level="beginner")


def _classify(poles):
    on_axis = [p for p in poles if abs(complex(p).real) <= 1e-9 * max(1.0, abs(complex(p)))]
    rhp = [p for p in poles if complex(p).real > 0 and p not in on_axis]
    if rhp:
        return "unstable"
    if not on_axis:
        return "asymptotically stable"
    for i, p in enumerate(on_axis):
        for q in on_axis[i + 1:]:
            if abs(complex(p) - complex(q)) <= 1e-7 * max(1.0, abs(complex(p))):
                return "unstable"
    return "marginally stable"


def pole_stability(den):
    den = _ptrim(_vec(den, "den", 2, 10))
    _need(len(den) >= 2, "the polynomial needs degree 1 or more")
    poles = _roots(den)
    return {"poles": poles, "max_real_part": _r12(max(complex(p).real for p in poles)), "stability": _classify(poles)}


def ref_pole_stability(den):
    import numpy as np
    r = _ref_canon(np.roots(den))
    return {"poles": r, "max_real_part": max(complex(p).real for p in r), "stability": _classify(r)}


def _rand_pole_poly(rng):
    """Distinct roots only: a repeated root is answered exactly here, but numerical references resolve it to ~1e-8."""
    n = rng.randint(1, 5)
    p = [1.0]
    k = 0
    used = []
    while k < n:
        u = rng.random()
        if n - k >= 2 and u < 0.3:
            a = round(rng.uniform(-3, 5), 1)
            b = round(rng.uniform(0.5, 5), 1)
            if rng.random() < 0.2:
                a = 0.0
                b = float(rng.randint(1, 4))
            if any(abs(complex(-a, b) - x) < 0.2 for x in used):
                continue
            used += [complex(-a, b), complex(-a, -b)]
            p = _pmul(p, [1.0, 2 * a, a * a + b * b])
            k += 2
        else:
            r = round(rng.uniform(-3, 8), 1) or 0.7
            if any(abs(complex(-r, 0) - x) < 0.2 for x in used):
                continue
            used.append(complex(-r, 0))
            p = _pmul(p, [1.0, r])
            k += 1
    return {"den": [_r12(x) for x in _pscale(p, round(rng.uniform(0.5, 4), 1))]}


def _ref_pd(P):
    import numpy as np
    ev = np.linalg.eigvalsh((np.array(P) + np.array(P).T) / 2)
    return bool(np.min(ev) > 1e-10 * max(1.0, float(np.max(np.abs(ev)))))


def _ref_det(W):
    import numpy as np
    W = np.array(W, float)
    d = float(np.linalg.det(W))
    return 0.0 if abs(d) <= 1e-10 * float(np.max(np.abs(W))) ** len(W) else d


_add("ctrl_pole_stability", name="Poles and stability of a characteristic polynomial", topic="stability / poles",
     purpose="The roots (poles) of a characteristic polynomial and whether the system is asymptotically stable, marginally stable or unstable.",
     inputs=[_v("den", "characteristic polynomial (denominator) coefficients, highest power first", 2, 10)],
     outputs=[_o("poles", "the poles"), _o("max_real_part", "largest real part of the poles"),
              _o("stability", "asymptotically stable, marginally stable or unstable", "text")],
     primary="max_real_part", conditions=["degree 1 or more"],
     method="roots by the Aberth-Ehrlich iteration; simple poles on the imaginary axis are marginal, repeated ones unstable",
     latex=r"\text{stable} \iff \operatorname{Re} p_i < 0 \;\forall i", run=pole_stability, reference=ref_pole_stability,
     wrong=lambda den: (lambda r: {"poles": r, "max_real_part": -max(complex(p).real for p in r), "stability": "?"})(_roots(den)),
     rand=_rand_pole_poly, phrases=["pole stability", "stability of poles"],
     examples=[_ex({"den": [1, 6, 11, 6]}, {"poles": [-3.0, -2.0, -1.0], "stability": "asymptotically stable"}),
               _ex({"den": [1, 1, 4, 4]}, {"poles": [-1.0, complex(0, -2), complex(0, 2)], "stability": "marginally stable"}),
               _ex({"den": [1, -1, 2]}, {"stability": "unstable", "max_real_part": 0.5})],
     level="beginner")


def closed_loop_first_order_p(K, tau, Kp):
    K, tau, Kp = _num(K, "K"), _pos(tau, "tau"), _num(Kp, "Kp")
    L = K * Kp
    _need(1 + L > 0, "the closed loop is unstable (1 + K Kp must be positive)")
    return _rnd({"dc_gain_cl": L / (1 + L), "tau_cl": tau / (1 + L), "ess": 1.0 / (1 + L), "pole": -(1 + L) / tau})


def ref_closed_loop_first_order_p(K, tau, Kp):
    import control
    cl = control.feedback(control.tf([Kp * K], [tau, 1.0]), 1)
    p = float(complex(cl.poles()[0]).real)
    g = float(control.dcgain(cl))
    return {"dc_gain_cl": g, "tau_cl": -1 / p, "ess": 1 - g, "pole": p}


_add("ctrl_closed_loop_first_order_p", name="Proportional control of a first-order plant",
     topic="time response / closed loop",
     purpose="Closed-loop DC gain K Kp/(1 + K Kp), time constant tau/(1 + K Kp), steady-state error 1/(1 + K Kp) to a unit step and the closed-loop pole, for K/(tau s + 1) under unity feedback with gain Kp.",
     inputs=[_n("K", "plant DC gain"), _n("tau", "plant time constant in seconds"), _n("Kp", "proportional gain")],
     outputs=[_o("dc_gain_cl", "closed-loop DC gain"), _o("tau_cl", "closed-loop time constant", "time"),
              _o("ess", "steady-state error to a unit step"), _o("pole", "closed-loop pole (1/s)")],
     primary="tau_cl", conditions=["tau > 0", "1 + K Kp > 0"], method="T(s) = K Kp/(tau s + 1 + K Kp)",
     latex=r"T(s) = \frac{KK_p}{\tau s + 1 + KK_p}", run=closed_loop_first_order_p, reference=ref_closed_loop_first_order_p,
     wrong=lambda K, tau, Kp: {"dc_gain_cl": K * Kp / (1 + K * Kp), "tau_cl": tau * (1 + K * Kp), "ess": 1 / (K * Kp), "pole": -1 / tau},
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.1, 20), 2), "Kp": round(rng.uniform(0.1, 20), 2)},
     phrases=["proportional control of a first-order plant", "P control of a first-order plant"],
     examples=[_ex({"K": 2, "tau": 10, "Kp": 4.5}, {"dc_gain_cl": 0.9, "tau_cl": 1.0, "ess": 0.1}),
               _ex({"K": 1, "tau": 3, "Kp": 2}, {"tau_cl": 1.0, "pole": -1.0})],
     extra_q=[_bad({"K": 1, "tau": 3, "Kp": -2})], level="beginner")


def second_order_from_poles(sigma, wd):
    sigma, wd = _pos(sigma, "sigma"), _pos(wd, "wd")
    wn = math.sqrt(sigma * sigma + wd * wd)
    zeta = sigma / wn
    return _rnd({"wn": wn, "zeta": zeta, "Tp": PI / wd, "Ts": 4.0 / sigma, "PO": 100 * math.exp(-sigma * PI / wd)})


def ref_second_order_from_poles(sigma, wd):
    import control
    den = [1.0, 2 * sigma, sigma * sigma + wd * wd]
    wn, z, _ = control.damp(control.tf([1.0], den), doprint=False)
    tp, yp = _ref_2nd_peak(float(z[0]), float(wn[0]))
    return {"wn": float(wn[0]), "zeta": float(z[0]), "Tp": tp, "Ts": 4.0 / sigma, "PO": 100 * (yp - 1)}


_add("ctrl_second_order_from_poles", name="Second-order specifications from the pole location",
     topic="time response / second order",
     purpose="From poles at -sigma +/- j wd: wn, zeta, peak time pi/wd, 2 % settling time 4/sigma and percent overshoot 100 e^{-sigma pi/wd}.",
     inputs=[_n("sigma", "decay rate: the poles' real part is -sigma (sigma > 0)"), _n("wd", "imaginary part of the poles (rad/s)")],
     outputs=[_o("wn", "natural frequency", "angular_frequency"), _o("zeta", "damping ratio"), _o("Tp", "peak time", "time"),
              _o("Ts", "2 % settling time 4/sigma", "time"), _o("PO", "percent overshoot (%)")],
     primary="zeta", conditions=["sigma > 0", "wd > 0"], method="wn = |p|, zeta = sigma/wn",
     latex=r"\omega_n = \sqrt{\sigma^2+\omega_d^2},\quad \zeta = \sigma/\omega_n,\quad \%OS = 100e^{-\sigma\pi/\omega_d}",
     run=second_order_from_poles, reference=ref_second_order_from_poles,
     wrong=lambda sigma, wd: {"wn": sigma + wd, "zeta": sigma / wd, "Tp": PI / wd, "Ts": 4 / sigma, "PO": 100 * math.exp(-PI * wd / sigma)},
     rand=lambda rng: (lambda s: {"sigma": s, "wd": round(s * rng.uniform(0.5, 6), 2)})(round(rng.uniform(0.2, 10), 2)),
     phrases=["second-order specifications from poles", "second order specifications from poles"],
     examples=[_ex({"sigma": 3, "wd": 4}, {"wn": 5.0, "zeta": 0.6, "Tp": 0.785398163397, "PO": 9.47802248422}),
               _ex({"sigma": 2, "wd": 2}, {"zeta": 0.707106781187, "Ts": 2.0})],
     extra_q=[_bad({"sigma": -3, "wd": 4})], level="beginner", weak="the settling time 4/sigma is a restated rule")


def s_to_z_pole(s_re, s_im, T):
    s_re, s_im, T = _num(s_re, "s_re"), _nonneg(s_im, "s_im"), _pos(T, "T")
    _need(s_im * T < PI, "the pole's frequency is above the Nyquist limit (s_im T must be below pi)")
    r = math.exp(s_re * T)
    th = s_im * T
    return _rnd({"z_re": r * math.cos(th), "z_im": r * math.sin(th), "z_mag": r, "z_angle": th})


def ref_s_to_z_pole(s_re, s_im, T):
    import numpy as np
    from scipy.signal import cont2discrete
    if s_im == 0:
        den = [1.0, -s_re]
    else:
        den = [1.0, -2 * s_re, s_re * s_re + s_im * s_im]
    _, a, _ = cont2discrete(([1.0], den), T, method="zoh")
    r = np.roots(a)
    z = complex(r[int(np.argmax(np.imag(r)))])
    return {"z_re": z.real, "z_im": max(z.imag, 0.0), "z_mag": abs(z), "z_angle": cmath.phase(z) if s_im else 0.0}


_add("ctrl_s_to_z_pole", name="Mapping an s-plane pole to the z-plane (z = e^{sT})", topic="discretization / pole mapping",
     purpose="The discrete pole z = e^{sT} of a continuous pole s = s_re + j s_im sampled with period T: real and imaginary parts, magnitude e^{s_re T} and angle s_im T.",
     inputs=[_n("s_re", "real part of the s-plane pole (1/s)"), _n("s_im", "imaginary part of the pole, >= 0 (rad/s)"),
             _n("T", "sampling period in seconds")],
     outputs=[_o("z_re", "real part of z"), _o("z_im", "imaginary part of z"), _o("z_mag", "|z| = e^{s_re T}"),
              _o("z_angle", "angle of z in radians, s_im T")],
     primary="z_mag", conditions=["T > 0", "0 <= s_im T < pi"], method="z = e^{s_re T}(cos s_im T + j sin s_im T)",
     latex=r"z = e^{sT} = e^{\sigma T}\angle\,\omega T", run=s_to_z_pole, reference=ref_s_to_z_pole,
     wrong=lambda s_re, s_im, T: {"z_re": 1 + s_re * T, "z_im": s_im * T, "z_mag": abs(complex(1 + s_re * T, s_im * T)), "z_angle": s_im * T},
     rand=lambda rng: {"s_re": round(rng.uniform(-10, 1), 2), "s_im": rng.choice([0.0, round(rng.uniform(0.1, 20), 2)]),
                       "T": round(rng.uniform(0.01, 0.15), 3)},
     phrases=["s-plane to z-plane pole", "s to z pole"],
     examples=[_ex({"s_re": -2, "s_im": 0, "T": 0.1}, {"z_re": 0.818730753078, "z_mag": 0.818730753078, "z_im": 0.0}),
               _ex({"s_re": -1, "s_im": 3.14159265359, "T": 0.25}, {"z_mag": 0.778800783071, "z_angle": 0.785398163398})],
     extra_q=[_bad({"s_re": -1, "s_im": 40, "T": 0.1})], level="beginner")


# ================================================================================================ PID tables and forms
def _pid_out(Kp, Ti, Td):
    Ki = Kp / Ti if Ti > 0 else 0.0
    return _rnd({"Kp": Kp, "Ti": Ti, "Td": Td, "Ki": Ki, "Kd": Kp * Td})


def zn_ultimate(Ku, Pu, controller):
    Ku, Pu = _pos(Ku, "Ku"), _pos(Pu, "Pu")
    _need(controller in ("P", "PI", "PID"), "controller must be P, PI or PID")
    if controller == "P":
        return _pid_out(0.5 * Ku, 0.0, 0.0)
    if controller == "PI":
        return _pid_out(0.45 * Ku, Pu / 1.2, 0.0)
    return _pid_out(0.6 * Ku, Pu / 2.0, Pu / 8.0)


def ref_zn_ultimate(Ku, Pu, controller):
    table = {"P": (Fraction(1, 2), None, None), "PI": (Fraction(9, 20), Fraction(5, 6), None),
             "PID": (Fraction(3, 5), Fraction(1, 2), Fraction(1, 8))}
    a, b, c = table[controller]
    Kp = float(a * Fraction(Ku))
    Ti = float(b * Fraction(Pu)) if b else 0.0
    Td = float(c * Fraction(Pu)) if c else 0.0
    return {"Kp": Kp, "Ti": Ti, "Td": Td, "Ki": Kp / Ti if Ti else 0.0, "Kd": Kp * Td}


_PID_OUT = [_o("Kp", "proportional gain"), _o("Ti", "integral time (0 when there is no integral action)", "time"),
            _o("Td", "derivative time", "time"), _o("Ki", "integral gain Kp/Ti"), _o("Kd", "derivative gain Kp Td")]


_add("ctrl_zn_ultimate", name="Ziegler-Nichols tuning from the ultimate gain and period", topic="PID tuning / Ziegler-Nichols",
     purpose="P, PI or PID settings from the closed-loop (ultimate-cycle) Ziegler-Nichols table: P 0.5 Ku; PI 0.45 Ku, Ti = Pu/1.2; PID 0.6 Ku, Ti = Pu/2, Td = Pu/8.",
     inputs=[_n("Ku", "ultimate gain (gain at sustained oscillation)"), _n("Pu", "ultimate period in seconds"),
             _ch("controller", ["P", "PI", "PID"], "controller type")],
     outputs=_PID_OUT, primary="Kp", conditions=["Ku > 0", "Pu > 0"], method="Ziegler-Nichols ultimate-cycle table",
     latex=r"\text{PID: } K_p = 0.6K_u,\; T_i = P_u/2,\; T_d = P_u/8", run=zn_ultimate, reference=ref_zn_ultimate,
     wrong=lambda Ku, Pu, controller: {"P": _pid_out(0.6 * Ku, 0.0, 0.0), "PI": _pid_out(0.45 * Ku, Pu / 2.0, 0.0),
                                       "PID": _pid_out(0.6 * Ku, Pu / 2.0, Pu / 4.0)}[controller],
     rand=lambda rng: {"Ku": round(rng.uniform(0.5, 50), 2), "Pu": round(rng.uniform(0.2, 60), 2), "controller": rng.choice(["P", "PI", "PID"])},
     phrases=["Ziegler-Nichols ultimate tuning", "Ziegler Nichols ultimate tuning"],
     examples=[_ex({"Ku": 10, "Pu": 4, "controller": "PID"}, {"Kp": 6.0, "Ti": 2.0, "Td": 0.5, "Ki": 3.0, "Kd": 3.0}),
               _ex({"Ku": 8, "Pu": 3, "controller": "PI"}, {"Kp": 3.6, "Ti": 2.5}),
               _ex({"Ku": 8, "Pu": 3, "controller": "P"}, {"Kp": 4.0, "Ki": 0.0})],
     level="beginner", weak="tuning table restated with exact fractions (no library computes the table)")


def zn_reaction(K, L, T, controller):
    K, L, T = _pos(K, "K"), _pos(L, "L"), _pos(T, "T")
    _need(controller in ("P", "PI", "PID"), "controller must be P, PI or PID")
    base = T / (K * L)
    if controller == "P":
        return _pid_out(base, 0.0, 0.0)
    if controller == "PI":
        return _pid_out(0.9 * base, L / 0.3, 0.0)
    return _pid_out(1.2 * base, 2.0 * L, 0.5 * L)


def ref_zn_reaction(K, L, T, controller):
    base = Fraction(T) / (Fraction(K) * Fraction(L))
    if controller == "P":
        Kp, Ti, Td = base, Fraction(0), Fraction(0)
    elif controller == "PI":
        Kp, Ti, Td = Fraction(9, 10) * base, Fraction(10, 3) * Fraction(L), Fraction(0)
    else:
        Kp, Ti, Td = Fraction(6, 5) * base, 2 * Fraction(L), Fraction(L) / 2
    Kp, Ti, Td = float(Kp), float(Ti), float(Td)
    return {"Kp": Kp, "Ti": Ti, "Td": Td}


_add("ctrl_zn_reaction", name="Ziegler-Nichols tuning from the process reaction curve", topic="PID tuning / Ziegler-Nichols",
     purpose="P, PI or PID settings from the open-loop (reaction-curve) Ziegler-Nichols table for a process with gain K, dead time L and time constant T.",
     inputs=[_n("K", "process gain"), _n("L", "apparent dead time in seconds"), _n("T", "apparent time constant in seconds"),
             _ch("controller", ["P", "PI", "PID"], "controller type")],
     outputs=_PID_OUT, primary="Kp", conditions=["K, L, T > 0"], method="Ziegler-Nichols reaction-curve table",
     latex=r"\text{PID: } K_p = \frac{1.2\,T}{KL},\; T_i = 2L,\; T_d = 0.5L", run=zn_reaction, reference=ref_zn_reaction,
     wrong=lambda K, L, T, controller: {"P": _pid_out(T * K / L, 0.0, 0.0), "PI": _pid_out(0.9 * T / (K * L), 3.0 * L, 0.0),
                                        "PID": _pid_out(1.2 * T / (K * L), 2.0 * L, 0.25 * L)}[controller],
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "L": round(rng.uniform(0.1, 10), 2), "T": round(rng.uniform(0.5, 60), 2),
                       "controller": rng.choice(["P", "PI", "PID"])},
     phrases=["Ziegler-Nichols reaction curve tuning", "Ziegler Nichols reaction curve tuning"],
     examples=[_ex({"K": 2, "L": 1, "T": 10, "controller": "PID"}, {"Kp": 6.0, "Ti": 2.0, "Td": 0.5}),
               _ex({"K": 2, "L": 1.5, "T": 9, "controller": "PI"}, {"Kp": 2.7, "Ti": 5.0})],
     level="beginner", weak="tuning table restated with exact fractions (no library computes the table)")


def pid_standard_to_parallel(Kp, Ti, Td):
    Kp, Ti, Td = _num(Kp, "Kp"), _pos(Ti, "Ti"), _nonneg(Td, "Td")
    return _rnd({"Ki": Kp / Ti, "Kd": Kp * Td})


def ref_pid_standard_to_parallel(Kp, Ti, Td):
    import control
    s = control.tf("s")
    C = Kp * (1 + 1 / (Ti * s) + Td * s)
    num, den = [float(x) for x in C.num[0][0]], [float(x) for x in C.den[0][0]]
    num = [x / den[0] for x in num]                       # C = (Kd s^2 + Kp s + Ki)/s
    num = _pad(num, 3)
    return {"Ki": num[2], "Kd": num[0]}


_add("ctrl_pid_standard_to_parallel", name="PID: standard form to parallel gains", topic="PID tuning / forms",
     purpose="Ki = Kp/Ti and Kd = Kp Td from the standard (ideal ISA) form Kp (1 + 1/(Ti s) + Td s).",
     inputs=[_n("Kp", "proportional gain"), _n("Ti", "integral (reset) time in seconds"), _n("Td", "derivative time in seconds")],
     outputs=[_o("Ki", "integral gain"), _o("Kd", "derivative gain")], primary="Ki", conditions=["Ti > 0", "Td >= 0"],
     method="Ki = Kp/Ti, Kd = Kp Td", latex=r"K_i = K_p/T_i,\quad K_d = K_pT_d",
     run=pid_standard_to_parallel, reference=ref_pid_standard_to_parallel,
     wrong=lambda Kp, Ti, Td: {"Ki": Kp * Ti, "Kd": Kp / Td if Td else 1.0},
     rand=lambda rng: {"Kp": round(rng.uniform(0.2, 20), 2), "Ti": round(rng.uniform(0.2, 30), 2), "Td": round(rng.uniform(0.01, 5), 2)},
     phrases=["PID standard to parallel"],
     examples=[_ex({"Kp": 4, "Ti": 2, "Td": 0.5}, {"Ki": 2.0, "Kd": 2.0}), _ex({"Kp": 1.2, "Ti": 3, "Td": 0}, {"Ki": 0.4, "Kd": 0.0})],
     extra_q=[_bad({"Kp": 4, "Ti": 0, "Td": 0.5})], level="beginner",
     weak="python-control polynomial arithmetic of the same controller (a definition)")


def pid_parallel_to_standard(Kp, Ki, Kd):
    Kp, Ki, Kd = _pos(Kp, "Kp"), _pos(Ki, "Ki"), _nonneg(Kd, "Kd")
    return _rnd({"Ti": Kp / Ki, "Td": Kd / Kp})


def ref_pid_parallel_to_standard(Kp, Ki, Kd):
    import control
    s = control.tf("s")
    C = Kp + Ki / s + Kd * s
    num, den = [float(x) for x in C.num[0][0]], [float(x) for x in C.den[0][0]]
    num = _pad([x / den[0] for x in num], 3)
    return {"Ti": num[1] / num[2], "Td": num[0] / num[1]}


_add("ctrl_pid_parallel_to_standard", name="PID: parallel gains to standard form", topic="PID tuning / forms",
     purpose="The integral time Ti = Kp/Ki and derivative time Td = Kd/Kp of a parallel PID Kp + Ki/s + Kd s.",
     inputs=[_n("Kp", "proportional gain (> 0)"), _n("Ki", "integral gain (> 0)"), _n("Kd", "derivative gain")],
     outputs=[_o("Ti", "integral time", "time"), _o("Td", "derivative time", "time")], primary="Ti",
     conditions=["Kp > 0", "Ki > 0", "Kd >= 0"], method="Ti = Kp/Ki, Td = Kd/Kp", latex=r"T_i = K_p/K_i,\quad T_d = K_d/K_p",
     run=pid_parallel_to_standard, reference=ref_pid_parallel_to_standard,
     wrong=lambda Kp, Ki, Kd: {"Ti": Ki / Kp, "Td": Kd * Kp},
     rand=lambda rng: {"Kp": round(rng.uniform(0.2, 20), 2), "Ki": round(rng.uniform(0.05, 20), 2), "Kd": round(rng.uniform(0.01, 5), 2)},
     phrases=["PID parallel to standard"],
     examples=[_ex({"Kp": 6, "Ki": 3, "Kd": 3}, {"Ti": 2.0, "Td": 0.5}), _ex({"Kp": 2, "Ki": 0.5, "Kd": 0}, {"Ti": 4.0, "Td": 0.0})],
     extra_q=[_bad({"Kp": 6, "Ki": 0, "Kd": 3})], level="beginner",
     weak="python-control polynomial arithmetic of the same controller (a definition)")


# ================================================================================================ connections
def series_connection(num1, den1, num2, den2):
    n1, d1 = _tf_in(num1, den1, proper=False)
    n2, d2 = _tf_in(num2, den2, proper=False)
    return _rnd({"num": _pmul(n1, n2), "den": _pmul(d1, d2)})


def _ref_tfpair(sys):
    import numpy as np
    num = _ptrim([float(x) for x in np.atleast_1d(sys.num[0][0])])
    den = _ptrim([float(x) for x in np.atleast_1d(sys.den[0][0])])
    return num, den


def ref_series_connection(num1, den1, num2, den2):
    import control
    n, d = _ref_tfpair(control.series(control.tf(num1, den1), control.tf(num2, den2)))
    return {"num": n, "den": d}


def _rand_tf_any(rng, nmax=3):
    n = rng.randint(1, nmax)
    den = [1.0] + [round(rng.uniform(-2, 9), 1) for _ in range(n)]
    m = rng.randint(0, n)
    num = [round(rng.uniform(0.5, 9), 1)] + [round(rng.uniform(-3, 9), 1) for _ in range(m)]
    return num, den


def _rand_strict(rng, nmax=4):
    """A random strictly proper transfer function."""
    n = rng.randint(1, nmax)
    den = [round(rng.uniform(0.5, 3), 1)] + [round(rng.uniform(-2, 9), 1) for _ in range(n)]
    m = rng.randint(0, n - 1)
    num = [round(rng.uniform(0.5, 9), 1)] + [round(rng.uniform(-3, 9), 1) for _ in range(m)]
    return {"num": num, "den": den}


_add("ctrl_series_connection", name="Series (cascade) connection of two transfer functions", topic="transfer functions / block diagrams",
     purpose="The transfer function G1 G2 of two blocks in series: numerators multiplied, denominators multiplied (no cancellation).",
     inputs=[_v("num1", "numerator of G1"), _v("den1", "denominator of G1", 2), _v("num2", "numerator of G2"), _v("den2", "denominator of G2", 2)],
     outputs=[_o("num", "numerator of G1 G2"), _o("den", "denominator of G1 G2")], primary="num",
     method="polynomial multiplication", latex=r"G(s) = G_1(s)G_2(s) = \frac{N_1N_2}{D_1D_2}",
     run=series_connection, reference=ref_series_connection,
     wrong=lambda num1, den1, num2, den2: {"num": _padd(_pmul(num1, den2), _pmul(num2, den1)), "den": _pmul(den1, den2)},
     rand=lambda rng: (lambda a, b: {"num1": a[0], "den1": a[1], "num2": b[0], "den2": b[1]})(_rand_tf_any(rng), _rand_tf_any(rng)),
     phrases=["series connection"],
     examples=[_ex({"num1": [1], "den1": [1, 1], "num2": [2], "den2": [1, 3]}, {"num": [2.0], "den": [1.0, 4.0, 3.0]}),
               _ex({"num1": [1, 2], "den1": [1, 0], "num2": [5], "den2": [1, 5]}, {"num": [5.0, 10.0], "den": [1.0, 5.0, 0.0]})],
     level="beginner")


def parallel_connection(num1, den1, num2, den2):
    n1, d1 = _tf_in(num1, den1, proper=False)
    n2, d2 = _tf_in(num2, den2, proper=False)
    num = _ptrim(_padd(_pmul(n1, d2), _pmul(n2, d1)))
    _need(any(x != 0 for x in num), "the two blocks cancel: the sum is zero")
    return _rnd({"num": num, "den": _pmul(d1, d2)})


def ref_parallel_connection(num1, den1, num2, den2):
    import control
    n, d = _ref_tfpair(control.parallel(control.tf(num1, den1), control.tf(num2, den2)))
    return {"num": n, "den": d}


_add("ctrl_parallel_connection", name="Parallel connection of two transfer functions", topic="transfer functions / block diagrams",
     purpose="The transfer function G1 + G2 of two blocks in parallel: (N1 D2 + N2 D1)/(D1 D2).",
     inputs=[_v("num1", "numerator of G1"), _v("den1", "denominator of G1", 2), _v("num2", "numerator of G2"), _v("den2", "denominator of G2", 2)],
     outputs=[_o("num", "numerator of G1 + G2"), _o("den", "denominator of G1 + G2")], primary="num",
     method="N = N1 D2 + N2 D1, D = D1 D2", latex=r"G_1 + G_2 = \frac{N_1D_2 + N_2D_1}{D_1D_2}",
     run=parallel_connection, reference=ref_parallel_connection,
     wrong=lambda num1, den1, num2, den2: {"num": _padd(_pmul(num1, den1), _pmul(num2, den2)), "den": _pmul(den1, den2)},
     rand=lambda rng: (lambda a, b: {"num1": a[0], "den1": a[1], "num2": b[0], "den2": b[1]})(_rand_tf_any(rng), _rand_tf_any(rng)),
     phrases=["parallel connection"],
     examples=[_ex({"num1": [1], "den1": [1, 1], "num2": [2], "den2": [1, 3]}, {"num": [3.0, 5.0], "den": [1.0, 4.0, 3.0]}),
               _ex({"num1": [1], "den1": [1, 0], "num2": [1], "den2": [1, 2]}, {"num": [2.0, 2.0], "den": [1.0, 2.0, 0.0]})],
     level="beginner")


def discrete_dc_gain(num, den, A=1.0):
    num, den = _tf_in(num, den)
    A = _num(A, "A")
    _need(sum(den) != 0, "G(z) has a pole at z = 1: the DC gain is infinite")
    poles = _roots(den)
    _need(max(abs(complex(p)) for p in poles) < 1, "the system is not stable (a pole on or outside the unit circle)")
    g = sum(num) / sum(den)
    return _rnd({"dc_gain": g, "final_value": g * A})


def ref_discrete_dc_gain(num, den, A=1.0):
    import control
    g = float(control.dcgain(control.tf(num, den, True)))
    return {"dc_gain": g, "final_value": g * A}


def _rand_stable_z(rng):
    n = rng.randint(1, 4)
    den = [1.0]
    for _ in range(n):
        den = _pmul(den, [1.0, -round(rng.uniform(-0.9, 0.95), 2)])
    m = rng.randint(0, n)
    num = [round(rng.uniform(0.1, 3), 2)] + [round(rng.uniform(-1, 1), 2) for _ in range(m)]
    return {"num": num, "den": [_r12(x) for x in den]}


_add("ctrl_discrete_dc_gain", name="DC gain and final value of a discrete transfer function", topic="discretization / discrete systems",
     purpose="G(1) = sum(num)/sum(den) of a stable discrete transfer function G(z), and the final value of its response to a step of size A.",
     inputs=[_v("num", "numerator coefficients in z, highest power first"), _v("den", "denominator coefficients in z, highest power first", 2),
             _n("A", "step size (default 1)", required=False)],
     outputs=[_o("dc_gain", "DC gain G(1)"), _o("final_value", "final value of the step response")], primary="dc_gain",
     conditions=["proper G(z)", "every pole inside the unit circle"], method="G(1) after a stability check",
     latex=r"y(\infty) = A\,G(1)", run=discrete_dc_gain, reference=ref_discrete_dc_gain,
     wrong=lambda num, den, A=1.0: {"dc_gain": num[-1] / den[-1], "final_value": A * num[-1] / den[-1]},
     rand=lambda rng: dict(_rand_stable_z(rng), A=round(rng.uniform(0.5, 5), 1)), phrases=["discrete DC gain"],
     examples=[_ex({"num": [0.5], "den": [1, -0.5]}, {"dc_gain": 1.0}),
               _ex({"num": [1, 0.5], "den": [1, -1.2, 0.35], "A": 2}, {"dc_gain": 10.0, "final_value": 20.0})],
     extra_q=[_bad({"num": [1], "den": [1, -1.5]})], level="beginner")


# ================================================================================================ transfer functions
def poles_zeros(num, den):
    num, den = _tf_in(num, den, proper=False)
    return {"zeros": _roots(num), "poles": _roots(den), "gain": _r12(num[0] / den[0])}


def ref_poles_zeros(num, den):
    import control
    from scipy.signal import tf2zpk
    sys = control.tf(num, den)
    _, _, k = tf2zpk(num, den)
    return {"zeros": _ref_canon(sys.zeros()), "poles": _ref_canon(sys.poles()), "gain": float(k)}


def _rand_pz(rng):
    n = rng.randint(1, 5)
    den = [round(rng.uniform(0.5, 3), 1)] + [round(rng.uniform(-3, 12), 1) for _ in range(n)]
    m = rng.randint(0, n)
    num = [round(rng.uniform(0.5, 9), 1)] + [round(rng.uniform(-4, 9), 1) for _ in range(m)]
    return {"num": num, "den": den}


def _pz_ok(rng):
    for _ in range(50):
        x = _rand_pz(rng)
        # keep roots well separated (numerical references are accurate to ~1e-8 for clustered roots)
        ok = True
        for c in (x["num"], x["den"]):
            r = _roots_raw(c)
            for i in range(len(r)):
                for j in range(i + 1, len(r)):
                    if abs(complex(r[i]) - complex(r[j])) < 0.05 * max(1.0, abs(complex(r[i]))):
                        ok = False
        if ok:
            return x
    return {"num": [1.0, 2.0], "den": [1.0, 4.0, 3.0]}


_add("ctrl_poles_zeros", name="Poles, zeros and gain of a transfer function", topic="transfer functions / poles and zeros",
     purpose="The zeros (roots of the numerator), the poles (roots of the denominator) and the zero-pole-gain constant k of G(s) = num/den.",
     inputs=[_v("num", "numerator coefficients, highest power of s first"), _v("den", "denominator coefficients, highest power of s first", 2)],
     outputs=[_o("zeros", "the zeros"), _o("poles", "the poles"), _o("gain", "gain k of the zero-pole-gain form (num[0]/den[0])")],
     primary="gain", method="roots of both polynomials (Aberth-Ehrlich iteration)",
     latex=r"G(s) = k\,\frac{\prod (s - z_i)}{\prod (s - p_j)}", run=poles_zeros, reference=ref_poles_zeros,
     wrong=lambda num, den: {"zeros": _roots(den), "poles": _roots(num), "gain": num[-1] / den[-1]}, rand=_pz_ok,
     phrases=["poles and zeros"],
     examples=[_ex({"num": [1, 2], "den": [1, 4, 3]}, {"zeros": [-2.0], "poles": [-3.0, -1.0], "gain": 1.0}),
               _ex({"num": [5], "den": [1, 2, 5]}, {"zeros": [], "poles": [complex(-1, -2), complex(-1, 2)], "gain": 5.0})],
     level="intermediate")


def tf_evaluate(num, den, s_re, s_im):
    num, den = _tf_in(num, den, proper=False)
    s = complex(_num(s_re, "s_re"), _num(s_im, "s_im"))
    d = _pval(den, s)
    scale = sum(abs(a) * abs(s) ** (len(den) - 1 - i) for i, a in enumerate(den))
    _need(abs(d) > 1e-12 * max(scale, 1e-300), "the point is a pole of G(s)")
    g = _pval(num, s) / d
    return _rnd({"G_re": g.real, "G_im": g.imag, "magnitude": math.hypot(g.real, g.imag), "angle_deg": _deg(g)})


def ref_tf_evaluate(num, den, s_re, s_im):
    import control
    g = complex(control.tf(num, den)(complex(s_re, s_im)))
    return {"G_re": g.real, "G_im": g.imag, "magnitude": abs(g), "angle_deg": math.degrees(cmath.phase(g))}


_add("ctrl_tf_evaluate", name="Value of a transfer function at a complex point", topic="transfer functions / evaluation",
     purpose="G(s0) = num(s0)/den(s0) at s0 = s_re + j s_im: real and imaginary parts, magnitude and angle.",
     inputs=[_v("num", "numerator coefficients"), _v("den", "denominator coefficients", 2), _n("s_re", "real part of s0"),
             _n("s_im", "imaginary part of s0")],
     outputs=[_o("G_re", "real part of G(s0)"), _o("G_im", "imaginary part of G(s0)"), _o("magnitude", "|G(s0)|"),
              _o("angle_deg", "angle of G(s0) in degrees, (-180, 180]")],
     primary="magnitude", conditions=["s0 is not a pole"], method="Horner evaluation in complex arithmetic",
     latex=r"G(s_0) = \frac{N(s_0)}{D(s_0)}", run=tf_evaluate, reference=ref_tf_evaluate,
     wrong=lambda num, den, s_re, s_im: (lambda g: {"G_re": g.real, "G_im": g.imag, "magnitude": abs(g), "angle_deg": _deg(g)})(
         _pval(num, complex(s_re, -s_im)) / _pval(den, complex(s_re, s_im))),
     rand=lambda rng: dict(_rand_pz(rng), s_re=round(rng.uniform(-5, 3), 2), s_im=round(rng.uniform(0.3, 6), 2)),
     phrases=["evaluate the transfer function"],
     examples=[_ex({"num": [1], "den": [1, 1], "s_re": 0, "s_im": 1}, {"G_re": 0.5, "G_im": -0.5, "magnitude": 0.707106781187, "angle_deg": -45.0}),
               _ex({"num": [2, 1], "den": [1, 3, 2], "s_re": -3, "s_im": 1}, {"G_re": -1.1, "G_im": -1.3})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "s_re": -1, "s_im": 0})], level="intermediate")


def feedback_connection(num_g, den_g, num_h=None, den_h=None, sign="negative"):
    ng, dg = _tf_in(num_g, den_g, proper=False)
    nh = _ptrim(_vec(num_h, "num_h", 1, 9)) if num_h is not None else [1.0]
    dh = _ptrim(_vec(den_h, "den_h", 1, 9)) if den_h is not None else [1.0]
    _need(any(x != 0 for x in nh) and any(x != 0 for x in dh), "H(s) must have a nonzero numerator and denominator")
    _need(sign in ("negative", "positive"), "sign must be negative or positive")
    loop = _pmul(ng, nh)
    den = _padd(_pmul(dg, dh), loop) if sign == "negative" else _psub(_pmul(dg, dh), loop)
    den = _ptrim(den)
    _need(any(x != 0 for x in den), "the closed loop is singular (1 + G H = 0)")
    return _rnd({"num": _pmul(ng, dh), "den": den})


def ref_feedback_connection(num_g, den_g, num_h=None, den_h=None, sign="negative"):
    import control
    H = control.tf(num_h if num_h is not None else [1.0], den_h if den_h is not None else [1.0])
    n, d = _ref_tfpair(control.feedback(control.tf(num_g, den_g), H, sign=-1 if sign == "negative" else 1))
    return {"num": n, "den": d}


def _rand_feedback(rng):
    g = _rand_tf_any(rng)
    x = {"num_g": g[0], "den_g": g[1]}
    if rng.random() < 0.6:
        h = _rand_tf_any(rng, 2)
        x["num_h"], x["den_h"] = h
    if rng.random() < 0.3:
        x["sign"] = "positive"
    return x


_add("ctrl_feedback_connection", name="Closed-loop transfer function of a feedback loop", topic="transfer functions / block diagrams",
     purpose="G/(1 + G H) for negative feedback (or G/(1 - G H) for positive feedback): numerator N_G D_H and denominator D_G D_H +/- N_G N_H.",
     inputs=[_v("num_g", "numerator of the forward path G"), _v("den_g", "denominator of G", 1),
             _v("num_h", "numerator of the feedback path H (default 1)", required=False),
             _v("den_h", "denominator of H (default 1)", required=False),
             _ch("sign", ["negative", "positive"], "feedback sign (default negative)", required=False)],
     outputs=[_o("num", "closed-loop numerator"), _o("den", "closed-loop denominator")], primary="num",
     method="T = N_G D_H / (D_G D_H + N_G N_H)", latex=r"T(s) = \frac{G}{1 + GH} = \frac{N_GD_H}{D_GD_H + N_GN_H}",
     run=feedback_connection, reference=ref_feedback_connection,
     wrong=lambda num_g, den_g, num_h=None, den_h=None, sign="negative": feedback_connection(
         num_g, den_g, num_h, den_h, "positive" if sign == "negative" else "negative"),
     rand=_rand_feedback, phrases=["feedback connection", "closed-loop transfer function"],
     examples=[_ex({"num_g": [10], "den_g": [1, 2, 0]}, {"num": [10.0], "den": [1.0, 2.0, 10.0]}),
               _ex({"num_g": [1], "den_g": [1, 1], "num_h": [2], "den_h": [1, 3]}, {"num": [1.0, 3.0], "den": [1.0, 4.0, 5.0]}),
               _ex({"num_g": [1], "den_g": [1, 3], "sign": "positive"}, {"num": [1.0], "den": [1.0, 2.0]})],
     level="intermediate")


# ================================================================================================ second-order time response
def _y2(zeta, wn, t):
    """Unit step response of wn^2/(s^2 + 2 zeta wn s + wn^2)."""
    if t == 0:
        return 0.0
    if zeta < 1:
        wd = wn * math.sqrt(1 - zeta * zeta)
        return 1 - math.exp(-zeta * wn * t) * (math.cos(wd * t) + zeta / math.sqrt(1 - zeta * zeta) * math.sin(wd * t))
    if zeta == 1:
        return 1 - math.exp(-wn * t) * (1 + wn * t)
    q = math.sqrt(zeta * zeta - 1)
    s1, s2 = wn * (zeta - q), wn * (zeta + q)
    return 1 - (s2 * math.exp(-s1 * t) - s1 * math.exp(-s2 * t)) / (s2 - s1)


def second_order_step(zeta, wn, t, K=1.0):
    zeta, wn, t, K = _nonneg(zeta, "zeta"), _pos(wn, "wn"), _nonneg(t, "t"), _num(K, "K")
    _need(K != 0, "K must not be zero")
    y = _y2(zeta, wn, t)
    return _rnd({"y": K * y, "error": K * (1 - y)})


def ref_second_order_step(zeta, wn, t, K=1.0):
    y = _ref_step([K * wn * wn], [1.0, 2 * zeta * wn, wn * wn], t)
    return {"y": y, "error": K - y}


def _rand_2nd_step(rng):
    zeta = rng.choice([round(rng.uniform(0.05, 0.9), 3), round(rng.uniform(1.15, 3), 2), 1.0])
    wn = round(rng.uniform(0.3, 20), 2)
    return {"zeta": zeta, "wn": wn, "t": round(rng.uniform(0.3, 6) / wn, 4), "K": round(rng.uniform(0.5, 10), 2)}


_add("ctrl_second_order_step", name="Step response of a standard second-order system", topic="time response / second order",
     purpose="y(t) of K wn^2/(s^2 + 2 zeta wn s + wn^2) after a unit step, for underdamped, critically damped and overdamped systems, and the remaining error K - y.",
     inputs=[_n("zeta", "damping ratio (>= 0)"), _n("wn", "natural frequency in rad/s"), _n("t", "time in seconds"),
             _n("K", "DC gain (default 1)", required=False)],
     outputs=[_o("y", "output at time t"), _o("error", "K - y(t)")], primary="y",
     conditions=["zeta >= 0", "wn > 0", "t >= 0"], method="closed-form step response for each damping case",
     latex=r"y(t) = 1 - e^{-\zeta\omega_n t}\left(\cos\omega_d t + \frac{\zeta}{\sqrt{1-\zeta^2}}\sin\omega_d t\right)",
     run=second_order_step, reference=ref_second_order_step,
     wrong=lambda zeta, wn, t, K=1.0: (lambda y: {"y": K * y, "error": K * (1 - y)})(1 - math.exp(-zeta * wn * t) * math.cos(wn * t)),
     rand=_rand_2nd_step, phrases=["second-order step response", "second order step response"],
     examples=[_ex({"zeta": 0.5, "wn": 2, "t": 1}, {"y": 0.849425634854}),
               _ex({"zeta": 1, "wn": 1, "t": 2}, {"y": 0.593994150290}),
               _ex({"zeta": 2, "wn": 1, "t": 1, "K": 3}, {"y": 0.533209728295})],
     extra_q=[_bad({"zeta": -0.5, "wn": 2, "t": 1})], level="intermediate")


def _bisect(f, lo, hi):
    flo = f(lo)
    for _ in range(300):
        mid = 0.5 * (lo + hi)
        if mid == lo or mid == hi:
            break
        fm = f(mid)
        if (fm > 0) == (flo > 0):
            lo, flo = mid, fm
        else:
            hi = mid
    return 0.5 * (lo + hi)


def rise_time_second(zeta, wn):
    zeta, wn = _num(zeta, "zeta"), _pos(wn, "wn")
    _need(0 < zeta < 1, "the rise-time formulas need an underdamped system: 0 < zeta < 1")
    wd = wn * math.sqrt(1 - zeta * zeta)
    tp = PI / wd
    t10 = _bisect(lambda t: _y2(zeta, wn, t) - 0.1, 0.0, tp)
    t90 = _bisect(lambda t: _y2(zeta, wn, t) - 0.9, 0.0, tp)
    return _rnd({"Tr_0_100": (PI - math.acos(zeta)) / wd, "Tr_10_90": t90 - t10, "t10": t10, "t90": t90})


def ref_rise_time_second(zeta, wn):
    num, den = [wn * wn], [1.0, 2 * zeta * wn, wn * wn]
    wd = wn * math.sqrt(1 - zeta * zeta)
    tp = math.pi / wd
    f = lambda lvl: (lambda t: _ref_step(num, den, t) - lvl)
    t10, t90 = _ref_crossing(f(0.1), 1e-12 * tp, tp), _ref_crossing(f(0.9), 1e-12 * tp, tp)
    return {"Tr_0_100": _ref_crossing(f(1.0), 1e-9 * tp, tp * (1 - 1e-12)), "Tr_10_90": t90 - t10, "t10": t10, "t90": t90}


_add("ctrl_rise_time_second", name="Rise time of an underdamped second-order system", topic="time response / second order",
     purpose="The 0-100 % rise time (pi - acos zeta)/wd and the 10-90 % rise time (found exactly by bisection on the step response).",
     inputs=[_n("zeta", "damping ratio (0 < zeta < 1)"), _n("wn", "natural frequency in rad/s")],
     outputs=[_o("Tr_0_100", "0-100 % rise time", "time"), _o("Tr_10_90", "10-90 % rise time", "time"),
              _o("t10", "time to 10 %", "time"), _o("t90", "time to 90 %", "time")],
     primary="Tr_10_90", conditions=["0 < zeta < 1", "wn > 0"], method="closed form (0-100 %); bisection on y(t) (10-90 %)",
     latex=r"T_{r,0-100} = \frac{\pi - \cos^{-1}\zeta}{\omega_d}", run=rise_time_second, reference=ref_rise_time_second,
     wrong=lambda zeta, wn: {"Tr_0_100": PI / (wn * math.sqrt(1 - zeta * zeta)), "Tr_10_90": 1.8 / wn, "t10": 0.0, "t90": 1.8 / wn},
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 0.95), 3), "wn": round(rng.uniform(0.3, 40), 2)},
     phrases=["rise time"],
     examples=[_ex({"zeta": 0.5, "wn": 1}, {"Tr_0_100": 2.41839915231, "Tr_10_90": 1.63757294733}),
               _ex({"zeta": 0.7, "wn": 10}, {"Tr_0_100": 0.328532848418})],
     extra_q=[_bad({"zeta": 1.0, "wn": 1})], level="intermediate")


def _strip_origin(c):
    """Remove the factors s at the origin: (polynomial, count)."""
    c = _ptrim(c)
    k = 0
    while len(c) > 1 and c[-1] == 0:
        c = c[:-1]
        k += 1
    return c, k


def final_value_theorem(num, den):
    num, den = _tf_in(num, den, proper=False)
    n0, kn = _strip_origin(num)
    d0, kd = _strip_origin(den)
    net = kd - kn                      # poles at the origin left after cancellation
    _need(net <= 1, "Y(s) has a repeated pole at the origin: y(t) grows without bound")
    if len(d0) > 1:
        _need(max(complex(r).real for r in _roots(d0)) < 0,
              "s Y(s) has a pole in the right half plane or on the imaginary axis: the final value theorem does not apply")
    if net == 1:
        v = n0[-1] / d0[-1]
    else:
        v = 0.0
    return _rnd({"final_value": v})


def ref_final_value_theorem(num, den):
    import sympy as sp
    s = sp.symbols("s")
    N = sum(sp.Rational(str(c)) * s ** (len(num) - 1 - i) for i, c in enumerate(num))
    D = sum(sp.Rational(str(c)) * s ** (len(den) - 1 - i) for i, c in enumerate(den))
    return {"final_value": float(sp.limit(s * N / D, s, 0))}


def _rand_fvt(rng):
    d = _rand_stable_tf(rng, 3)
    den = d["den"]
    if rng.random() < 0.7:
        den = _pmul(den, [1.0, 0.0])
    return {"num": d["num"], "den": den}


_add("ctrl_final_value_theorem", name="Final value theorem", topic="time response / final and initial values",
     purpose="lim_{t->inf} y(t) = lim_{s->0} s Y(s) for Y(s) = num/den, after checking that s Y(s) has every pole in the open left half plane.",
     inputs=[_v("num", "numerator of Y(s)"), _v("den", "denominator of Y(s)", 2)],
     outputs=[_o("final_value", "final value of y(t)")], primary="final_value",
     conditions=["at most one pole at the origin", "all other poles in the open left half plane"],
     method="cancel factors s, then evaluate s Y(s) at s = 0", latex=r"y(\infty) = \lim_{s\to 0} sY(s)",
     run=final_value_theorem, reference=ref_final_value_theorem,
     wrong=lambda num, den: {"final_value": (_strip_origin(den)[0][-1] / _strip_origin(num)[0][-1])},
     rand=_rand_fvt, phrases=["final value theorem"],
     examples=[_ex({"num": [10], "den": [1, 3, 2, 0]}, {"final_value": 5.0}),
               _ex({"num": [1, 2], "den": [1, 4, 3]}, {"final_value": 0.0})],
     extra_q=[_bad({"num": [1], "den": [1, -1, 0]}), _bad({"num": [1], "den": [1, 0, 0]})], level="intermediate")


def _series_inf(num, den, k):
    """The first k coefficients c0..c(k-1) of Y(s) = sum c_i / s^(i+1) for strictly proper num/den (long division)."""
    n = len(den) - 1
    num = _pad(num, n)                 # num has degree <= n-1: coefficients of s^(n-1) ... s^0
    rem = list(num) + [0.0] * k
    out = []
    for i in range(k):
        c = rem[i] / den[0]
        out.append(c)
        for j in range(len(den)):
            if i + j < len(rem):
                rem[i + j] -= c * den[j]
    return out


def initial_value_theorem(num, den):
    num, den = _tf_in(num, den, strict=True)
    c = _series_inf(num, den, 2)
    return _rnd({"y0": c[0], "dy0": c[1]})


def ref_initial_value_theorem(num, den):
    import sympy as sp
    s = sp.symbols("s")
    N = sum(sp.Rational(str(c)) * s ** (len(num) - 1 - i) for i, c in enumerate(num))
    D = sum(sp.Rational(str(c)) * s ** (len(den) - 1 - i) for i, c in enumerate(den))
    y0 = sp.limit(s * N / D, s, sp.oo)
    dy0 = sp.limit(s * (s * N / D - y0), s, sp.oo)
    return {"y0": float(y0), "dy0": float(dy0)}


_add("ctrl_initial_value_theorem", name="Initial value theorem (value and slope at t = 0+)", topic="time response / final and initial values",
     purpose="y(0+) = lim_{s->inf} s Y(s) and y'(0+) = lim_{s->inf} s (s Y(s) - y(0+)) for a strictly proper Y(s).",
     inputs=[_v("num", "numerator of Y(s)"), _v("den", "denominator of Y(s)", 2)],
     outputs=[_o("y0", "initial value y(0+)"), _o("dy0", "initial slope y'(0+)")], primary="y0",
     conditions=["Y(s) strictly proper"], method="long division of num by den in powers of 1/s",
     latex=r"y(0^+) = \lim_{s\to\infty} sY(s)", run=initial_value_theorem, reference=ref_initial_value_theorem,
     wrong=lambda num, den: {"y0": num[-1] / den[-1], "dy0": 0.0},
     rand=lambda rng: _rand_strict(rng),
     phrases=["initial value theorem"],
     examples=[_ex({"num": [2, 3], "den": [1, 5, 6]}, {"y0": 2.0, "dy0": -7.0}),
               _ex({"num": [4], "den": [1, 2, 4]}, {"y0": 0.0, "dy0": 4.0})],
     extra_q=[_bad({"num": [1, 2], "den": [1, 3]})], level="intermediate")


def error_constants(num, den, input, A=1.0):
    num, den = _tf_in(num, den)
    A = _num(A, "A")
    _need(input in ("step", "ramp", "parabola"), "input must be step, ramp or parabola")
    _need(_routh_ok(_padd(den, num)), "the closed loop is not stable: there is no steady-state error")
    n0, kn = _strip_origin(num)
    d0, kd = _strip_origin(den)
    _need(kn == 0, "G(s) has a zero at the origin")
    N = kd
    K = n0[-1] / d0[-1]
    order = {"step": 0, "ramp": 1, "parabola": 2}[input]
    _need(N >= order, "the steady-state error grows without bound (the system type is below the input's order)")
    if N > order:
        ess = 0.0
    elif N == 0:
        ess = A / (1 + K)
    else:
        ess = A / K
    return _rnd({"system_type": N, "K_static": K, "ess": ess})


def ref_error_constants(num, den, input, A=1.0):
    import numpy as np
    import sympy as sp
    s = sp.symbols("s")
    N = sum(sp.Rational(str(c)) * s ** (len(num) - 1 - i) for i, c in enumerate(num))
    D = sum(sp.Rational(str(c)) * s ** (len(den) - 1 - i) for i, c in enumerate(den))
    k = {"step": 1, "ramp": 2, "parabola": 3}[input]
    R = sp.Rational(str(A)) / s ** k
    ess = sp.limit(s * R / (1 + N / D), s, 0)
    typ = int(sum(1 for r in np.roots(den) if abs(r) < 1e-9))
    return {"system_type": typ, "K_static": float(sp.limit(s ** typ * N / D, s, 0)), "ess": float(ess)}


def _rand_errconst(rng):
    for _ in range(100):
        typ = rng.choice([0, 1, 1, 2])
        d = _rand_stable_tf(rng, 3)
        den = d["den"]
        for _ in range(typ):
            den = _pmul(den, [1.0, 0.0])
        num = d["num"]
        if typ == 2:
            num = _pmul(num, [1.0, round(rng.uniform(0.1, 1.0), 2)])
            if len(num) > len(den):
                continue
        x = {"num": [_r12(v) for v in num], "den": den, "input": rng.choice(["step", "ramp", "parabola"]),
             "A": round(rng.uniform(0.5, 5), 1)}
        if typ >= {"step": 0, "ramp": 1, "parabola": 2}[x["input"]] and _routh_ok(_padd(den, num)):
            return x
    return {"num": [10.0], "den": [1.0, 3.0, 2.0], "input": "step", "A": 1.0}


_add("ctrl_error_constants", name="System type, static error constant and steady-state error", topic="steady-state error",
     purpose="For unity feedback around G(s): the system type N (poles at the origin), the static error constant lim s^N G(s) (Kp, Kv or Ka) and the steady-state error to a step, ramp or parabola of size A.",
     inputs=[_v("num", "numerator of the open-loop G(s)"), _v("den", "denominator of G(s)", 2),
             _ch("input", ["step", "ramp", "parabola"], "test input"), _n("A", "input size (default 1)", required=False)],
     outputs=[_o("system_type", "system type N", "integer"), _o("K_static", "static error constant of type N (Kp if 0, Kv if 1, Ka if 2)"),
              _o("ess", "steady-state error")],
     primary="ess", conditions=["closed loop stable", "proper G(s)", "type at least the input's order"],
     method="ess = A/(1 + Kp) (step, type 0), A/Kv (ramp, type 1), A/Ka (parabola, type 2), 0 for a higher type",
     latex=r"e_{ss} = \frac{A}{1+K_p},\; \frac{A}{K_v},\; \frac{A}{K_a}", run=error_constants, reference=ref_error_constants,
     wrong=lambda num, den, input, A=1.0: (lambda r: dict(r, ess=A / r["K_static"] if r["system_type"] == 0 else A / (1 + r["K_static"])))(error_constants(num, den, input, A)),
     rand=_rand_errconst, phrases=["steady-state error", "steady state error"],
     examples=[_ex({"num": [10], "den": [1, 3, 2], "input": "step"}, {"system_type": 0, "K_static": 5.0, "ess": 0.166666666667}),
               _ex({"num": [10], "den": [1, 5, 4, 0], "input": "ramp", "A": 2}, {"system_type": 1, "K_static": 2.5, "ess": 0.8}),
               _ex({"num": [10], "den": [1, 5, 4, 0], "input": "step"}, {"ess": 0.0})],
     extra_q=[_bad({"num": [10], "den": [1, 3, 2], "input": "ramp"})], level="intermediate")


# ================================================================================================ frequency response
def bode_point(num, den, w):
    num, den = _tf_in(num, den)
    w = _pos(w, "w")
    g = _G(num, den, complex(0.0, w))
    mag = math.hypot(g.real, g.imag)
    _need(mag > 0, "G(jw) is zero at this frequency")
    return _rnd({"magnitude": mag, "magnitude_db": 20 * math.log10(mag), "phase_deg": _deg(g)})


def ref_bode_point(num, den, w):
    h = _ref_freq(num, den, w)
    return {"magnitude": abs(h), "magnitude_db": 20 * math.log10(abs(h)), "phase_deg": math.degrees(cmath.phase(h))}


def _rand_bode(rng):
    for _ in range(100):
        x = _rand_stable_tf(rng, 4)
        if rng.random() < 0.3:
            x["den"] = _pmul(x["den"], [1.0, 0.0])
            if len(x["den"]) > 10:
                continue
        x["w"] = round(10 ** rng.uniform(-1.5, 1.5), 3)
        g = _G(x["num"], x["den"], complex(0, x["w"]))
        if abs(abs(_deg(g)) - 180) > 0.5:
            return x
    return {"num": [1.0], "den": [1.0, 1.0], "w": 1.0}


_add("ctrl_bode_point", name="Bode magnitude and phase at one frequency", topic="frequency response / Bode",
     purpose="|G(jw)|, 20 log10 |G(jw)| and the phase of G(jw) (principal value) of a transfer function at the angular frequency w.",
     inputs=[_v("num", "numerator coefficients"), _v("den", "denominator coefficients", 2), _n("w", "angular frequency in rad/s")],
     outputs=[_o("magnitude", "|G(jw)|"), _o("magnitude_db", "magnitude in dB"), _o("phase_deg", "phase in degrees, (-180, 180]")],
     primary="magnitude", conditions=["proper G(s)", "w > 0", "not a pole on the imaginary axis"],
     method="G(jw) by complex evaluation", latex=r"|G(j\omega)|_{dB} = 20\log_{10}|G(j\omega)|",
     run=bode_point, reference=ref_bode_point,
     wrong=lambda num, den, w: (lambda g: {"magnitude": abs(g), "magnitude_db": 10 * math.log10(abs(g)), "phase_deg": _deg(g)})(_G(num, den, complex(0, 2 * math.pi * w))),
     rand=_rand_bode, phrases=["Bode point", "Bode magnitude and phase"],
     examples=[_ex({"num": [10], "den": [1, 1, 0], "w": 1}, {"magnitude": 7.07106781187, "phase_deg": -135.0}),
               _ex({"num": [100], "den": [1, 10, 100], "w": 10}, {"magnitude": 1.0, "magnitude_db": 0.0, "phase_deg": -90.0})],
     extra_q=[_bad({"num": [1], "den": [1, 0, 4], "w": 2})], level="intermediate")


def second_order_freq(zeta, wn, w):
    zeta, wn, w = _nonneg(zeta, "zeta"), _pos(wn, "wn"), _pos(w, "w")
    re_, im = wn * wn - w * w, 2 * zeta * wn * w
    _need(re_ != 0 or im != 0, "the frequency is at an undamped resonance")
    mag = wn * wn / math.hypot(re_, im)
    return _rnd({"magnitude": mag, "magnitude_db": 20 * math.log10(mag), "phase_deg": -math.degrees(math.atan2(im, re_))})


def ref_second_order_freq(zeta, wn, w):
    h = _ref_freq([wn * wn], [1.0, 2 * zeta * wn, wn * wn], w)
    ph = math.degrees(cmath.phase(h))
    if ph > 0:
        ph -= 360.0
    return {"magnitude": abs(h), "magnitude_db": 20 * math.log10(abs(h)), "phase_deg": ph}


_add("ctrl_second_order_freq", name="Frequency response of a standard second-order system", topic="frequency response / Bode",
     purpose="Magnitude, dB gain and phase (between 0 and -180 degrees) of wn^2/(s^2 + 2 zeta wn s + wn^2) at the angular frequency w.",
     inputs=[_n("zeta", "damping ratio"), _n("wn", "natural frequency in rad/s"), _n("w", "angular frequency in rad/s")],
     outputs=[_o("magnitude", "|G(jw)|"), _o("magnitude_db", "magnitude in dB"), _o("phase_deg", "phase in degrees, [-180, 0]")],
     primary="magnitude", conditions=["zeta >= 0", "wn > 0", "w > 0"], method="G(jw) = wn^2/(wn^2 - w^2 + 2 j zeta wn w)",
     latex=r"|G| = \frac{\omega_n^2}{\sqrt{(\omega_n^2-\omega^2)^2 + (2\zeta\omega_n\omega)^2}}",
     run=second_order_freq, reference=ref_second_order_freq,
     wrong=lambda zeta, wn, w: (lambda m: {"magnitude": m, "magnitude_db": 20 * math.log10(m), "phase_deg": -math.degrees(math.atan2(zeta * wn * w, wn * wn - w * w))})(
         wn * wn / math.hypot(wn * wn - w * w, zeta * wn * w)),
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 2), 3), "wn": round(rng.uniform(0.3, 30), 2), "w": round(10 ** rng.uniform(-1, 2), 3)},
     phrases=["second-order frequency response", "second order frequency response"],
     examples=[_ex({"zeta": 0.5, "wn": 10, "w": 10}, {"magnitude": 1.0, "phase_deg": -90.0}),
               _ex({"zeta": 0.1, "wn": 1, "w": 1}, {"magnitude": 5.0, "magnitude_db": 13.9794000867})],
     extra_q=[_bad({"zeta": 0, "wn": 1, "w": 1})], level="intermediate")


def bandwidth_second(zeta, wn):
    zeta, wn = _pos(zeta, "zeta"), _pos(wn, "wn")
    x = 1 - 2 * zeta * zeta
    return _rnd({"wb": wn * math.sqrt(x + math.sqrt(x * x + 1))})


def ref_bandwidth_second(zeta, wn):
    num, den = [wn * wn], [1.0, 2 * zeta * wn, wn * wn]
    return {"wb": _ref_crossing(lambda w: abs(_ref_freq(num, den, w)) - 1 / math.sqrt(2), 1e-3 * wn, 10 * wn)}


_add("ctrl_bandwidth_second", name="Bandwidth of a standard second-order system", topic="frequency response / bandwidth",
     purpose="The -3 dB bandwidth wn sqrt(1 - 2 zeta^2 + sqrt(4 zeta^4 - 4 zeta^2 + 2)) of wn^2/(s^2 + 2 zeta wn s + wn^2).",
     inputs=[_n("zeta", "damping ratio (> 0)"), _n("wn", "natural frequency in rad/s")],
     outputs=[_o("wb", "-3 dB bandwidth", "angular_frequency")], primary="wb", conditions=["zeta > 0", "wn > 0"],
     method="solve |G(jw)| = 1/sqrt(2)", latex=r"\omega_B = \omega_n\sqrt{1-2\zeta^2+\sqrt{4\zeta^4-4\zeta^2+2}}",
     run=bandwidth_second, reference=ref_bandwidth_second,
     wrong=lambda zeta, wn: {"wb": wn * math.sqrt(1 - 2 * zeta * zeta + math.sqrt(4 * zeta ** 4 - 4 * zeta * zeta + 1)) if 1 - 2 * zeta * zeta + math.sqrt(4 * zeta ** 4 - 4 * zeta * zeta + 1) > 0 else wn},
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 2.5), 3), "wn": round(rng.uniform(0.3, 50), 2)},
     phrases=["second-order bandwidth", "second order bandwidth"],
     examples=[_ex({"zeta": 0.7071067811865476, "wn": 1}, {"wb": 1.0}), _ex({"zeta": 0.5, "wn": 10}, {"wb": 12.7201964951})],
     extra_q=[_bad({"zeta": 0, "wn": 10})], level="intermediate")


def resonant_peak(zeta, wn):
    zeta, wn = _pos(zeta, "zeta"), _pos(wn, "wn")
    _need(zeta < 1 / math.sqrt(2), "there is no resonant peak unless zeta < 0.7071")
    mr = 1 / (2 * zeta * math.sqrt(1 - zeta * zeta))
    return _rnd({"Mr": mr, "Mr_db": 20 * math.log10(mr), "wr": wn * math.sqrt(1 - 2 * zeta * zeta)})


def ref_resonant_peak(zeta, wn):
    import numpy as np
    # |D(jw)|^2 = w^4 + (4 zeta^2 - 2) wn^2 w^2 + wn^4: its stationary point in w (numpy roots of the derivative)
    d = np.polyder(np.array([1.0, 0.0, (4 * zeta ** 2 - 2) * wn ** 2, 0.0, wn ** 4]))
    r = [x.real for x in np.roots(d) if abs(x.imag) < 1e-9 and x.real > 0]
    wr = r[0]
    mr = abs(_ref_freq([wn * wn], [1.0, 2 * zeta * wn, wn * wn], wr))
    return {"Mr": mr, "Mr_db": 20 * math.log10(mr), "wr": wr}


_add("ctrl_resonant_peak", name="Resonant peak and resonant frequency", topic="frequency response / resonance",
     purpose="Peak magnitude Mr = 1/(2 zeta sqrt(1 - zeta^2)) and resonant frequency wr = wn sqrt(1 - 2 zeta^2) of a standard second-order system.",
     inputs=[_n("zeta", "damping ratio (0 < zeta < 0.7071)"), _n("wn", "natural frequency in rad/s")],
     outputs=[_o("Mr", "resonant peak (ratio)"), _o("Mr_db", "resonant peak in dB"), _o("wr", "resonant frequency", "angular_frequency")],
     primary="Mr", conditions=["0 < zeta < 1/sqrt(2)"], method="maximum of |G(jw)|",
     latex=r"M_r = \frac{1}{2\zeta\sqrt{1-\zeta^2}},\quad \omega_r = \omega_n\sqrt{1-2\zeta^2}",
     run=resonant_peak, reference=ref_resonant_peak,
     wrong=lambda zeta, wn: {"Mr": 1 / (2 * zeta), "Mr_db": 20 * math.log10(1 / (2 * zeta)), "wr": wn * math.sqrt(1 - zeta * zeta)},
     rand=lambda rng: {"zeta": round(rng.uniform(0.05, 0.65), 3), "wn": round(rng.uniform(0.3, 50), 2)},
     phrases=["resonant peak"],
     examples=[_ex({"zeta": 0.5, "wn": 10}, {"Mr": 1.15470053838, "wr": 7.07106781187}),
               _ex({"zeta": 0.1, "wn": 1}, {"Mr": 5.02518907629})],
     extra_q=[_bad({"zeta": 0.8, "wn": 10})], level="intermediate")


# ================================================================================================ Routh-Hurwitz
def routh_hurwitz(coefficients):
    c = _ptrim(_vec(coefficients, "coefficients", 2, 12))
    _need(len(c) >= 2, "give a polynomial of degree 1 or more")
    f = [Fraction(x) for x in c]
    n = len(f) - 1
    width = n // 2 + 1
    r0 = f[0::2] + [Fraction(0)] * (width - len(f[0::2]))
    r1 = f[1::2] + [Fraction(0)] * (width - len(f[1::2]))
    rows = [r0, r1]
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        _need(b[0] != 0, "a zero appears in the first column of the Routh array (a special case: roots on or symmetric "
                         "about the imaginary axis); this capability does not handle it")
        rows.append([(b[0] * a[i + 1] - a[0] * b[i + 1]) / b[0] for i in range(width - 1)] + [Fraction(0)])
    first = [r[0] for r in rows]
    _need(all(x != 0 for x in first), "a zero appears in the first column of the Routh array (a special case)")
    changes = sum(1 for x, y in zip(first, first[1:]) if (x > 0) != (y > 0))
    return {"first_column": [_r12(float(x)) for x in first], "rhp_roots": changes, "stable": changes == 0}


def ref_routh_hurwitz(coefficients):
    import numpy as np
    r = np.roots(np.array(_ptrim(coefficients), float))
    k = int(np.sum(r.real > 0))
    return {"rhp_roots": k, "stable": k == 0}


def _rand_routh(rng):
    for _ in range(200):
        n = rng.randint(2, 7)
        c = [1.0] + [float(rng.randint(-3, 14)) for _ in range(n)]
        if 0 in c:
            continue
        r = _roots_raw(c)
        if min(abs(complex(x).real) for x in r) < 1e-3:
            continue
        try:
            routh_hurwitz(c)
            return {"coefficients": c}
        except ValueError:
            continue
    return {"coefficients": [1.0, 2.0, 3.0]}


def _routh_wrong(coefficients):
    c = [Fraction(x) for x in _ptrim(coefficients)]
    n = len(c) - 1
    width = n // 2 + 1
    rows = [c[0::2] + [Fraction(0)] * (width - len(c[0::2])), c[1::2] + [Fraction(0)] * (width - len(c[1::2]))]
    for _ in range(n - 1):
        a, b = rows[-2], rows[-1]
        rows.append([(a[0] * b[i + 1] - b[0] * a[i + 1]) / b[0] for i in range(width - 1)] + [Fraction(0)])
    first = [r[0] for r in rows]
    ch = sum(1 for x, y in zip(first, first[1:]) if (x > 0) != (y > 0))
    return {"first_column": [float(x) for x in first], "rhp_roots": ch, "stable": ch == 0}


_add("ctrl_routh_hurwitz", name="Routh-Hurwitz stability test", topic="stability / Routh-Hurwitz",
     purpose="The first column of the Routh array (exact arithmetic), the number of right-half-plane roots (its sign changes) and whether the characteristic polynomial is stable.",
     inputs=[_v("coefficients", "characteristic polynomial coefficients, highest power of s first", 2, 12)],
     outputs=[_o("first_column", "first column of the Routh array"), _o("rhp_roots", "number of roots in the right half plane", "integer"),
              _o("stable", "true when every root has a negative real part", "boolean")],
     primary="rhp_roots", conditions=["no zero in the first column (special cases are refused)"],
     method="Routh array in exact fractions; sign changes of the first column", latex=r"b_i = \frac{a_1a_{0,i+1} - a_0a_{1,i+1}}{a_1}",
     run=routh_hurwitz, reference=ref_routh_hurwitz, wrong=_routh_wrong, rand=_rand_routh, phrases=["Routh-Hurwitz", "Routh test"],
     examples=[_ex({"coefficients": [1, 6, 11, 6]}, {"first_column": [1.0, 6.0, 10.0, 6.0], "rhp_roots": 0, "stable": True}),
               _ex({"coefficients": [1, 2, 3, 4, 5]}, {"rhp_roots": 2, "stable": False}),
               _ex({"coefficients": [1, 1, 2, 24]}, {"first_column": [1.0, 1.0, -22.0, 24.0], "rhp_roots": 2})],
     extra_q=[_bad({"coefficients": [1, 2, 1, 2]})], level="intermediate",
     weak="the first column is checked by recomputation only; the root count against numpy roots")


# ================================================================================================ root locus
def locus_asymptotes(num, den):
    num, den = _tf_in(num, den, proper=False)
    n, m = len(den) - 1, len(num) - 1
    _need(n > m, "asymptotes need more poles than zeros")
    sp_ = -den[1] / den[0]
    sz = -num[1] / num[0] if m >= 1 else 0.0
    q = n - m
    return _rnd({"n_asymptotes": q, "centroid": (sp_ - sz) / q, "angles_deg": [(2 * k + 1) * 180.0 / q for k in range(q)]})


def ref_locus_asymptotes(num, den):
    import numpy as np
    p, z = np.roots(den), np.roots(num) if len(_ptrim(num)) > 1 else np.array([])
    q = len(p) - len(z)
    return {"n_asymptotes": q, "centroid": float((np.sum(p).real - np.sum(z).real) / q)}


_add("ctrl_locus_asymptotes", name="Root-locus asymptotes (centroid and angles)", topic="root locus / asymptotes",
     purpose="Number of asymptotes n - m, their real-axis intercept (sum of poles - sum of zeros)/(n - m) and their angles (2k+1) 180/(n - m) degrees.",
     inputs=[_v("num", "numerator of the open-loop G(s)"), _v("den", "denominator of G(s)", 2)],
     outputs=[_o("n_asymptotes", "number of asymptotes", "integer"), _o("centroid", "real-axis intercept"),
              _o("angles_deg", "asymptote angles in degrees")],
     primary="centroid", conditions=["more poles than zeros"], method="sums of poles and zeros from the coefficients",
     latex=r"\sigma_a = \frac{\sum p_i - \sum z_j}{n-m},\quad \theta_k = \frac{(2k+1)180^\circ}{n-m}",
     run=locus_asymptotes, reference=ref_locus_asymptotes,
     wrong=lambda num, den: (lambda r: dict(r, centroid=(-den[1] / den[0] - (-num[1] / num[0] if len(num) > 1 else 0.0)) / (len(den) - 1)))(locus_asymptotes(num, den)),
     rand=lambda rng: _rand_strict(rng, 5),
     phrases=["root locus asymptotes", "root-locus asymptotes"],
     examples=[_ex({"num": [1], "den": [1, 3, 2, 0]}, {"n_asymptotes": 3, "centroid": -1.0, "angles_deg": [60.0, 180.0, 300.0]}),
               _ex({"num": [1, 3], "den": [1, 6, 8, 0]}, {"n_asymptotes": 2, "centroid": -1.5, "angles_deg": [90.0, 270.0]})],
     extra_q=[_bad({"num": [1, 2], "den": [1, 3]})], level="intermediate",
     weak="the angles are a restated rule; the centroid is checked against numpy roots")


def locus_gain_at_point(num, den, s_re, s_im):
    num, den = _tf_in(num, den, proper=False)
    s = complex(_num(s_re, "s_re"), _num(s_im, "s_im"))
    nv, dv = _pval(num, s), _pval(den, s)
    _need(abs(nv) > 0, "the point is a zero of G(s)")
    _need(abs(dv) > 0, "the point is a pole of G(s): the gain there is 0")
    g = nv / dv
    ang = _deg(g)
    return _rnd({"K": math.hypot(dv.real, dv.imag) / math.hypot(nv.real, nv.imag), "angle_deg": ang,
                 "angle_error_deg": 180.0 - abs(ang)})


def ref_locus_gain_at_point(num, den, s_re, s_im):
    import control
    g = complex(control.tf(num, den)(complex(s_re, s_im)))
    a = math.degrees(cmath.phase(g))
    return {"K": 1 / abs(g), "angle_deg": a, "angle_error_deg": 180 - abs(a)}


_add("ctrl_locus_gain_at_point", name="Root-locus gain and angle at a test point", topic="root locus / magnitude and angle criteria",
     purpose="The gain K = 1/|G(s0)| that would place a closed-loop pole at s0 (magnitude criterion) and the angle of G(s0); s0 lies on the root locus for K > 0 when the angle is 180 degrees (angle error 0).",
     inputs=[_v("num", "numerator of G(s)"), _v("den", "denominator of G(s)", 2), _n("s_re", "real part of the test point"),
             _n("s_im", "imaginary part of the test point")],
     outputs=[_o("K", "gain from the magnitude criterion"), _o("angle_deg", "angle of G(s0) in degrees"),
              _o("angle_error_deg", "180 - |angle|: 0 on the locus")],
     primary="K", conditions=["s0 neither a pole nor a zero"], method="K = |D(s0)|/|N(s0)|",
     latex=r"K = \frac{1}{|G(s_0)|},\quad \angle G(s_0) = \pm 180^\circ", run=locus_gain_at_point, reference=ref_locus_gain_at_point,
     wrong=lambda num, den, s_re, s_im: (lambda g: {"K": abs(g), "angle_deg": _deg(g), "angle_error_deg": 180 - abs(_deg(g))})(_G(num, den, complex(s_re, s_im))),
     rand=lambda rng: dict(_rand_pz(rng), s_re=round(rng.uniform(-5, 1), 2), s_im=round(rng.uniform(0.2, 5), 2)),
     phrases=["root locus gain at a point", "root-locus gain at a point"],
     examples=[_ex({"num": [1], "den": [1, 2, 0], "s_re": -1, "s_im": 1}, {"K": 2.0, "angle_deg": 180.0, "angle_error_deg": 0.0}),
               _ex({"num": [1], "den": [1, 3, 2, 0], "s_re": 0, "s_im": 1.4142135623730951}, {"K": 6.0})],
     level="intermediate")


# ================================================================================================ state space (intermediate)
def _ss_in(A, B=None, C=None, D=None):
    A = _square(A, "A")
    n = len(A)
    out = [A]
    if B is not None:
        B = _vec(B, "B", n, n)
        out.append(B)
    if C is not None:
        C = _vec(C, "C", n, n)
        out.append(C)
    if D is not None:
        out.append(_num(D, "D"))
    return out


def ss_to_tf(A, B, C, D=0.0):
    A, B, C, D = _ss_in(A, B, C, D)
    num, den = _ss2tf(_frac(A), _frac(B), _frac(C), Fraction(D))
    return {"num": [_r12(float(x)) for x in num], "den": [_r12(float(x)) for x in den]}


def _ref_snap_poly(c):
    big = max(abs(x) for x in c) or 1.0
    return [0.0 if abs(x) < 1e-11 * big else float(x) for x in c]


def ref_ss_to_tf(A, B, C, D=0.0):
    import numpy as np
    from scipy.signal import ss2tf
    num, den = ss2tf(np.array(A, float), np.array(B, float).reshape(-1, 1), np.array(C, float).reshape(1, -1), np.array([[D]]))
    n = len(A)
    return {"num": _ref_snap_poly(_pad(list(num[0]), n + 1)), "den": _ref_snap_poly(list(den))}


def _rand_ss(rng, n=None, D=True):
    n = n or rng.randint(2, 4)
    A = [[float(rng.randint(-4, 4)) for _ in range(n)] for _ in range(n)]
    B = [float(rng.randint(-2, 3)) for _ in range(n)]
    C = [float(rng.randint(-2, 3)) for _ in range(n)]
    x = {"A": A, "B": B, "C": C}
    if D and rng.random() < 0.4:
        x["D"] = float(rng.randint(-2, 3))
    return x


_add("ctrl_ss_to_tf", name="Transfer function of a state-space model", topic="state space / conversions",
     purpose="G(s) = C (sI - A)^{-1} B + D of a single-input single-output state-space model, computed exactly (Faddeev-LeVerrier).",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector (column, n entries)"), _v("C", "output vector (row, n entries)"),
             _n("D", "feedthrough (default 0)", required=False)],
     outputs=[_o("num", "numerator (n+1 coefficients, highest power first)"), _o("den", "characteristic polynomial (monic)")],
     primary="num", conditions=["A square, at most 6 x 6"], method="Faddeev-LeVerrier in exact fractions",
     latex=r"G(s) = C(sI-A)^{-1}B + D = \frac{C\,\mathrm{adj}(sI-A)\,B}{\det(sI-A)} + D", run=ss_to_tf, reference=ref_ss_to_tf,
     wrong=lambda A, B, C, D=0.0: ss_to_tf(_mT(A), B, C, D), rand=_rand_ss, phrases=["state space to transfer function", "state-space to transfer function"],
     examples=[_ex({"A": [[0, 1], [-2, -3]], "B": [0, 1], "C": [1, 0]}, {"num": [0.0, 0.0, 1.0], "den": [1.0, 3.0, 2.0]}),
               _ex({"A": [[0, 1], [-6, -5]], "B": [0, 1], "C": [3, 1], "D": 1}, {"num": [1.0, 6.0, 9.0], "den": [1.0, 5.0, 6.0]})],
     extra_q=[{"q": "state space to transfer function: A = [[0, 1], [-2, -3]], B = [0, 1, 2], C = [1, 0]", "outcome": "INVALID_INPUT"}],
     level="intermediate")


def tf_to_ss(num, den):
    num, den = _tf_in(num, den)
    A, B, C, D = _ccf(num, den)
    return _rnd({"A": A, "B": B, "C": C, "D": D})


def ref_tf_to_ss(num, den):
    from scipy.signal import tf2ss
    a, b, c, d = tf2ss(num, den)
    return {"A": [[float(x) for x in r] for r in a], "B": [float(x) for x in b[:, 0]], "C": [float(x) for x in c[0]], "D": float(d[0, 0])}


_add("ctrl_tf_to_ss", name="Controllable canonical form of a transfer function", topic="state space / conversions",
     purpose="A, B, C, D in the controllable canonical form (scipy's convention) of a proper transfer function.",
     inputs=[_v("num", "numerator coefficients"), _v("den", "denominator coefficients", 2)],
     outputs=[_o("A", "state matrix"), _o("B", "input vector"), _o("C", "output vector"), _o("D", "feedthrough")],
     primary="D", conditions=["proper G(s)"], method="normalise den; first row of A = -a1..-an; C from num - D den",
     latex=r"A = \begin{bmatrix}-a_1 & \cdots & -a_n\\ 1 & & 0\\ & \ddots & \end{bmatrix},\; B = e_1",
     run=tf_to_ss, reference=ref_tf_to_ss,
     wrong=lambda num, den: (lambda r: dict(r, A=_mT(r["A"]), D=r["D"] + 1.0))(tf_to_ss(num, den)),
     rand=lambda rng: (lambda a: {"num": a[0], "den": a[1]})(_rand_tf_any(rng, 4)),
     phrases=["controllable canonical form"],
     examples=[_ex({"num": [1], "den": [1, 3, 2]}, {"A": [[-3.0, -2.0], [1.0, 0.0]], "B": [1.0, 0.0], "C": [0.0, 1.0], "D": 0.0}),
               _ex({"num": [2, 1, 4], "den": [2, 6, 4]}, {"A": [[-3.0, -2.0], [1.0, 0.0]], "C": [-2.5, 0.0], "D": 1.0})],
     extra_q=[_bad({"num": [1, 2, 3], "den": [1, 1]})], level="intermediate",
     weak="scipy.signal.tf2ss computes the same canonical form (the convention is the library's)")


def ss_eigenvalues(A):
    (A,) = _ss_in(A)
    c, _ = _faddeev(_frac(A))
    cf = [float(x) for x in c]
    ev = _roots(cf)
    mr = max(complex(x).real for x in ev)
    return {"eigenvalues": ev, "char_poly": [_r12(x) for x in cf], "max_real_part": _r12(mr), "stable": mr < 0}


def ref_ss_eigenvalues(A):
    import numpy as np
    from scipy.linalg import eigvals
    ev = _ref_canon(eigvals(np.array(A, float)))
    return {"eigenvalues": ev, "char_poly": _ref_snap_poly(list(np.poly(np.array(A, float)).real)),
            "max_real_part": max(complex(x).real for x in ev), "stable": max(complex(x).real for x in ev) < 0}


def _rand_eig(rng):
    for _ in range(100):
        x = _rand_ss(rng)
        r = _roots_raw([float(v) for v in _faddeev(_frac(x["A"]))[0]])
        sep = min((abs(complex(r[i]) - complex(r[j])) for i in range(len(r)) for j in range(i + 1, len(r))), default=1)
        if sep > 0.05 and min(abs(complex(v).real) for v in r) > 1e-6:
            return {"A": x["A"]}
    return {"A": [[0.0, 1.0], [-2.0, -3.0]]}


_add("ctrl_ss_eigenvalues", name="Eigenvalues (poles) of a state matrix", topic="state space / stability",
     purpose="The characteristic polynomial det(sI - A) (exact), the eigenvalues of A and whether the system is asymptotically stable.",
     inputs=[_m("A", "state matrix (n x n)")],
     outputs=[_o("eigenvalues", "eigenvalues of A"), _o("char_poly", "characteristic polynomial (monic)"),
              _o("max_real_part", "largest real part"), _o("stable", "true when every eigenvalue has a negative real part", "boolean")],
     primary="max_real_part", conditions=["A square, at most 6 x 6"], method="Faddeev-LeVerrier, then polynomial roots",
     latex=r"\det(sI - A) = 0", run=ss_eigenvalues, reference=ref_ss_eigenvalues,
     wrong=lambda A: ss_eigenvalues(_mscale(A, -1.0)), rand=_rand_eig, phrases=["eigenvalues of the state matrix"],
     examples=[_ex({"A": [[0, 1], [-2, -3]]}, {"eigenvalues": [-2.0, -1.0], "char_poly": [1.0, 3.0, 2.0], "stable": True}),
               _ex({"A": [[0, 1], [-5, -2]]}, {"eigenvalues": [complex(-1, -2), complex(-1, 2)], "stable": True})],
     extra_q=[{"q": "eigenvalues of the state matrix: A = [[0, 1, 2], [-2, -3, 1]]", "outcome": "INVALID_INPUT"}],
     level="intermediate")


def _ctrb(A, B):
    n = len(A)
    cols = [list(B)]
    for _ in range(n - 1):
        cols.append(_mv(A, cols[-1]))
    return [[cols[j][i] for j in range(n)] for i in range(n)]


def controllability(A, B):
    A, B = _ss_in(A, B)
    W = _ctrb(_frac(A), _frac(B))
    r = _rank(W)
    return {"Wc": [[_r12(float(x)) for x in row] for row in W], "rank": r, "controllable": r == len(A),
            "det_Wc": _r12(float(_det(W)))}


def ref_controllability(A, B):
    import control
    import numpy as np
    W = control.ctrb(np.array(A, float), np.array(B, float).reshape(-1, 1))
    r = int(np.linalg.matrix_rank(W))
    return {"Wc": [[float(x) for x in row] for row in W], "rank": r, "controllable": r == len(A), "det_Wc": float(np.linalg.det(W)) if r == len(A) else 0.0}


def _rand_ctrb(rng):
    x = _rand_ss(rng, None, False)
    if rng.random() < 0.3:                       # an uncontrollable pair: B an eigenvector-like or a decoupled block
        n = len(x["A"])
        for j in range(n - 1):
            x["A"][n - 1][j] = 0.0
        x["B"][n - 1] = 0.0
    return {"A": x["A"], "B": x["B"]}


_add("ctrl_controllability", name="Controllability matrix and rank", topic="state space / controllability",
     purpose="The controllability matrix [B, AB, ..., A^{n-1}B], its exact rank and determinant, and whether (A, B) is controllable.",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector (n entries)")],
     outputs=[_o("Wc", "controllability matrix"), _o("rank", "rank of the controllability matrix", "integer"),
              _o("controllable", "true when the rank is n", "boolean"), _o("det_Wc", "determinant of the controllability matrix")],
     primary="rank", conditions=["single input"], method="exact rank by elimination in fractions",
     latex=r"\mathcal{C} = [B\;AB\;\cdots\;A^{n-1}B]", run=controllability, reference=ref_controllability,
     wrong=lambda A, B: (lambda r: dict(r, rank=len(A) if r["rank"] < len(A) else len(A) - 1, controllable=not r["controllable"]))(controllability(A, B)),
     rand=_rand_ctrb, phrases=["controllability"],
     examples=[_ex({"A": [[0, 1], [-2, -3]], "B": [0, 1]}, {"Wc": [[0.0, 1.0], [1.0, -3.0]], "rank": 2, "controllable": True, "det_Wc": -1.0}),
               _ex({"A": [[-1, 0], [0, -2]], "B": [1, 0]}, {"rank": 1, "controllable": False, "det_Wc": 0.0})],
     level="intermediate")


def observability(A, C):
    A, C = _ss_in(A, None, C)
    Af, Cf = _frac(A), _frac(C)
    n = len(A)
    rows = [list(Cf)]
    At = _mT(Af)
    for _ in range(n - 1):
        rows.append(_mv(At, rows[-1]))
    r = _rank(rows)
    return {"Wo": [[_r12(float(x)) for x in row] for row in rows], "rank": r, "observable": r == n,
            "det_Wo": _r12(float(_det(rows)))}


def ref_observability(A, C):
    import control
    import numpy as np
    W = control.obsv(np.array(A, float), np.array(C, float).reshape(1, -1))
    r = int(np.linalg.matrix_rank(W))
    return {"Wo": [[float(x) for x in row] for row in W], "rank": r, "observable": r == len(A), "det_Wo": float(np.linalg.det(W)) if r == len(A) else 0.0}


def _rand_obsv(rng):
    x = _rand_ss(rng, None, False)
    if rng.random() < 0.3:
        n = len(x["A"])
        for i in range(n - 1):
            x["A"][i][n - 1] = 0.0
        x["C"][n - 1] = 0.0
    return {"A": x["A"], "C": x["C"]}


_add("ctrl_observability", name="Observability matrix and rank", topic="state space / observability",
     purpose="The observability matrix [C; CA; ...; CA^{n-1}], its exact rank and determinant, and whether (A, C) is observable.",
     inputs=[_m("A", "state matrix (n x n)"), _v("C", "output vector (n entries)")],
     outputs=[_o("Wo", "observability matrix"), _o("rank", "rank of the observability matrix", "integer"),
              _o("observable", "true when the rank is n", "boolean"), _o("det_Wo", "determinant of the observability matrix")],
     primary="rank", conditions=["single output"], method="exact rank by elimination in fractions",
     latex=r"\mathcal{O} = \begin{bmatrix}C\\ CA\\ \vdots\\ CA^{n-1}\end{bmatrix}", run=observability, reference=ref_observability,
     wrong=lambda A, C: (lambda r: dict(r, rank=len(A) if r["rank"] < len(A) else len(A) - 1, observable=not r["observable"]))(observability(A, C)),
     rand=_rand_obsv, phrases=["observability"],
     examples=[_ex({"A": [[0, 1], [-2, -3]], "C": [1, 0]}, {"Wo": [[1.0, 0.0], [0.0, 1.0]], "rank": 2, "observable": True}),
               _ex({"A": [[-1, 0], [0, -2]], "C": [1, 0]}, {"rank": 1, "observable": False})],
     level="intermediate")


def ss_dc_gain(A, B, C, D=0.0):
    A, B, C, D = _ss_in(A, B, C, D)
    try:
        x = _solve(_frac(A), _frac(B), exact=True)
    except ValueError:
        raise ValueError("A is singular (an eigenvalue at 0): the DC gain is infinite") from None
    return {"dc_gain": _r12(float(Fraction(D) - sum(c * v for c, v in zip(_frac(C), x))))}


def ref_ss_dc_gain(A, B, C, D=0.0):
    import control
    import numpy as np
    sys = control.ss(np.array(A, float), np.array(B, float).reshape(-1, 1), np.array(C, float).reshape(1, -1), [[D]])
    return {"dc_gain": float(np.real(control.dcgain(sys)))}


def _rand_ssdc(rng):
    for _ in range(100):
        x = _rand_ss(rng)
        if _det(_frac(x["A"])) != 0:
            return x
    return {"A": [[0.0, 1.0], [-2.0, -3.0]], "B": [0.0, 1.0], "C": [1.0, 0.0]}


_add("ctrl_ss_dc_gain", name="DC gain of a state-space model", topic="state space / DC gain",
     purpose="The steady-state gain G(0) = D - C A^{-1} B of a state-space model (exact).",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector"), _v("C", "output vector"), _n("D", "feedthrough (default 0)", required=False)],
     outputs=[_o("dc_gain", "DC gain G(0)")], primary="dc_gain", conditions=["A nonsingular"],
     method="solve A x = B exactly, then D - C x", latex=r"G(0) = D - CA^{-1}B", run=ss_dc_gain, reference=ref_ss_dc_gain,
     wrong=lambda A, B, C, D=0.0: {"dc_gain": 2 * D - ss_dc_gain(A, B, C, D)["dc_gain"]},
     rand=_rand_ssdc, phrases=["state-space DC gain", "state space DC gain"],
     examples=[_ex({"A": [[0, 1], [-2, -3]], "B": [0, 1], "C": [1, 0]}, {"dc_gain": 0.5}),
               _ex({"A": [[-1, 0], [0, -4]], "B": [1, 2], "C": [3, 1], "D": 1}, {"dc_gain": 4.5})],
     extra_q=[_bad({"A": [[0, 1], [0, -3]], "B": [0, 1], "C": [1, 0]})], level="intermediate")


# ================================================================================================ discretization
def _subst(poly, n, c, P, Q):
    """sum_k poly(s^k) with s = c P(z)/Q(z), multiplied through by Q(z)^n (P, Q of degree <= 1)."""
    poly = _pad(poly, n + 1)
    out = [0.0]
    for i, a in enumerate(poly):
        k = n - i                      # power of s
        if a == 0:
            continue
        term = _pscale(_pmul(_ppow(P, k), _ppow(Q, n - k)), a * c ** k)
        out = _padd(out, term)
    return out


def _disc_out(numz, denz):
    denz = _ptrim(denz)
    _need(denz[0] != 0, "the discrete denominator vanishes")
    lead = denz[0]
    numz = _pad(_ptrim(numz), len(denz))
    big_n = max(abs(x) for x in numz) or 1.0
    numz = [x / lead for x in numz]
    denz = [x / lead for x in denz]
    big = max(max(abs(x) for x in numz), 1e-300)
    numz = [0.0 if abs(x) <= 1e-13 * big else x for x in numz]
    return _rnd({"numz": numz, "denz": denz})


def _ref_disc(num, den, T, method, **kw):
    import numpy as np
    from scipy.signal import cont2discrete
    b, a, _ = cont2discrete((num, den), T, method=method, **kw)
    b = list(np.atleast_2d(b)[0])
    a = list(a)
    b = _pad([float(x) / a[0] for x in b], len(a))
    a = [float(x) / a[0] for x in a]
    return {"numz": _ref_snap_poly(b), "denz": a}


def _ref_sym_disc(num, den, T, method):
    """Exact symbolic substitution (sympy rationals) of the discretization rule, then normalisation."""
    import sympy as sp
    s, z = sp.symbols("s z")
    Tq = sp.Rational(str(T))
    rule = {"tustin": 2 / Tq * (z - 1) / (z + 1), "euler": (z - 1) / Tq, "backward": (z - 1) / (Tq * z)}[method]
    N = sum(sp.Rational(str(c)) * s ** (len(num) - 1 - i) for i, c in enumerate(num))
    D = sum(sp.Rational(str(c)) * s ** (len(den) - 1 - i) for i, c in enumerate(den))
    n = len(_ptrim(den)) - 1
    mult = {"tustin": (z + 1) ** n, "euler": 1, "backward": z ** n}[method]
    Nz = sp.Poly(sp.expand(sp.cancel(N.subs(s, rule) * mult)), z)
    Dz = sp.Poly(sp.expand(sp.cancel(D.subs(s, rule) * mult)), z)
    dc = [sp.Rational(c) for c in Dz.all_coeffs()]
    nc = [sp.Rational(c) for c in Nz.all_coeffs()]
    nc = [sp.Integer(0)] * (len(dc) - len(nc)) + nc
    return {"numz": [float(c / dc[0]) for c in nc], "denz": [float(c / dc[0]) for c in dc]}


def zoh_first_order(K, tau, T):
    K, tau, T = _num(K, "K"), _pos(tau, "tau"), _pos(T, "T")
    a = math.exp(-T / tau)
    return _rnd({"b": -K * math.expm1(-T / tau), "a": a})


def ref_zoh_first_order(K, tau, T):
    r = _ref_disc([K], [tau, 1.0], T, "zoh")
    return {"b": r["numz"][1], "a": -r["denz"][1]}


_add("ctrl_zoh_first_order", name="Zero-order-hold discretization of a first-order system", topic="discretization / ZOH",
     purpose="G(z) = b/(z - a) with a = e^{-T/tau} and b = K (1 - a): the exact step-invariant (ZOH) equivalent of K/(tau s + 1) sampled every T seconds.",
     inputs=[_n("K", "DC gain"), _n("tau", "time constant in seconds"), _n("T", "sampling period in seconds")],
     outputs=[_o("b", "numerator coefficient b"), _o("a", "discrete pole a")], primary="a", conditions=["tau > 0", "T > 0"],
     method="a = exp(-T/tau), b = K (1 - a)", latex=r"G(z) = \frac{K(1 - e^{-T/\tau})}{z - e^{-T/\tau}}",
     run=zoh_first_order, reference=ref_zoh_first_order,
     wrong=lambda K, tau, T: {"b": K * T / tau, "a": 1 - T / tau},
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.1, 20), 2), "T": round(rng.uniform(0.01, 2), 3)},
     phrases=["ZOH discretization of a first-order system", "zero-order hold of a first-order system"],
     examples=[_ex({"K": 2, "tau": 1, "T": 0.5}, {"a": 0.606530659713, "b": 0.786938680575}),
               _ex({"K": 1, "tau": 10, "T": 1}, {"a": 0.904837418036})],
     extra_q=[_bad({"K": 2, "tau": 1, "T": 0})], level="intermediate")


def _disc_tf_in(num, den, T):
    num, den = _tf_in(num, den)
    return num, den, _pos(T, "T")


def tustin(num, den, T):
    num, den, T = _disc_tf_in(num, den, T)
    n = len(den) - 1
    c = 2.0 / T
    return _disc_out(_subst(num, n, c, [1.0, -1.0], [1.0, 1.0]), _subst(den, n, c, [1.0, -1.0], [1.0, 1.0]))


def _rand_disc(rng):
    x = _rand_stable_tf(rng, 3)
    x["T"] = round(rng.uniform(0.01, 0.5), 3)
    return x


_add("ctrl_tustin", name="Tustin (bilinear) discretization", topic="discretization / Tustin",
     purpose="The discrete transfer function obtained by substituting s = (2/T)(z - 1)/(z + 1) into G(s).",
     inputs=[_v("num", "numerator in s"), _v("den", "denominator in s", 2), _n("T", "sampling period in seconds")],
     outputs=[_o("numz", "numerator in z (padded, normalised)"), _o("denz", "denominator in z (leading coefficient 1)")],
     primary="denz", conditions=["proper G(s)", "T > 0"], method="polynomial substitution, multiplied through by (z + 1)^n",
     latex=r"s \leftarrow \frac{2}{T}\,\frac{z-1}{z+1}", run=tustin, reference=lambda num, den, T: _ref_sym_disc(num, den, T, "tustin"),
     wrong=lambda num, den, T: tustin(num, den, 2 * T), rand=_rand_disc, phrases=["Tustin discretization", "bilinear discretization"],
     examples=[_ex({"num": [1], "den": [1, 1], "T": 0.1}, {"numz": [0.047619047619, 0.047619047619], "denz": [1.0, -0.904761904762]}),
               _ex({"num": [1], "den": [1, 0], "T": 2}, {"numz": [1.0, 1.0], "denz": [1.0, -1.0]})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "T": -0.1})], level="intermediate")


def forward_euler(num, den, T):
    num, den, T = _disc_tf_in(num, den, T)
    n = len(den) - 1
    return _disc_out(_subst(num, n, 1.0 / T, [1.0, -1.0], [1.0]), _subst(den, n, 1.0 / T, [1.0, -1.0], [1.0]))


_add("ctrl_forward_euler", name="Forward Euler (forward difference) discretization", topic="discretization / Euler",
     purpose="The discrete transfer function obtained by substituting s = (z - 1)/T into G(s).",
     inputs=[_v("num", "numerator in s"), _v("den", "denominator in s", 2), _n("T", "sampling period in seconds")],
     outputs=[_o("numz", "numerator in z (padded, normalised)"), _o("denz", "denominator in z (leading coefficient 1)")],
     primary="denz", conditions=["proper G(s)", "T > 0"], method="polynomial substitution s = (z - 1)/T",
     latex=r"s \leftarrow \frac{z-1}{T}", run=forward_euler, reference=lambda num, den, T: _ref_sym_disc(num, den, T, "euler"),
     wrong=lambda num, den, T: backward_euler(num, den, T), rand=_rand_disc, phrases=["forward Euler discretization"],
     examples=[_ex({"num": [1], "den": [1, 1], "T": 0.1}, {"numz": [0.0, 0.1], "denz": [1.0, -0.9]}),
               _ex({"num": [2], "den": [1, 3, 2], "T": 0.5}, {"numz": [0.0, 0.0, 0.5], "denz": [1.0, -0.5, 0.0]})],
     extra_q=[_bad({"num": [1, 1, 1], "den": [1, 1], "T": 0.1})], level="intermediate")


def backward_euler(num, den, T):
    num, den, T = _disc_tf_in(num, den, T)
    n = len(den) - 1
    return _disc_out(_subst(num, n, 1.0 / T, [1.0, -1.0], [1.0, 0.0]), _subst(den, n, 1.0 / T, [1.0, -1.0], [1.0, 0.0]))


_add("ctrl_backward_euler", name="Backward Euler (backward difference) discretization", topic="discretization / Euler",
     purpose="The discrete transfer function obtained by substituting s = (z - 1)/(T z) into G(s).",
     inputs=[_v("num", "numerator in s"), _v("den", "denominator in s", 2), _n("T", "sampling period in seconds")],
     outputs=[_o("numz", "numerator in z (padded, normalised)"), _o("denz", "denominator in z (leading coefficient 1)")],
     primary="denz", conditions=["proper G(s)", "T > 0"], method="polynomial substitution s = (z - 1)/(T z)",
     latex=r"s \leftarrow \frac{z-1}{Tz}", run=backward_euler, reference=lambda num, den, T: _ref_sym_disc(num, den, T, "backward"),
     wrong=lambda num, den, T: forward_euler(num, den, T), rand=_rand_disc, phrases=["backward Euler discretization"],
     examples=[_ex({"num": [1], "den": [1, 1], "T": 0.1}, {"numz": [0.0909090909091, 0.0], "denz": [1.0, -0.909090909091]}),
               _ex({"num": [1], "den": [1, 0], "T": 0.5}, {"numz": [0.5, 0.0], "denz": [1.0, -1.0]})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "T": 0})], level="intermediate")


def discrete_step(numz, denz, k, A=1.0):
    num, den = _tf_in(numz, denz)
    k = int(k)
    _need(0 <= k <= 500, "k must be between 0 and 500")
    A = _num(A, "A")
    a0 = den[0]
    den = [x / a0 for x in den]
    num = _pad([x / a0 for x in num], len(den))
    n = len(den) - 1
    y = []
    for j in range(k + 1):
        acc = 0.0
        for i in range(n + 1):
            if j - i >= 0:
                acc += num[i] * A
        for i in range(1, n + 1):
            if j - i >= 0:
                acc -= den[i] * y[j - i]
        y.append(acc)
    return _rnd({"y": y[k]})


def ref_discrete_step(numz, denz, k, A=1.0):
    from scipy.signal import dstep
    _, (yy,) = dstep((numz, denz, 1), n=k + 1)
    return {"y": float(yy[k, 0]) * A}


def _rand_dstep(rng):
    x = _rand_stable_z(rng)
    for _ in range(50):
        k = rng.randint(1, 60)
        y = discrete_step(x["num"], x["den"], k)["y"]
        if abs(y) > 1e-3:
            return {"numz": x["num"], "denz": x["den"], "k": k, "A": round(rng.uniform(0.5, 5), 1)}
    return {"numz": [0.5], "denz": [1.0, -0.5], "k": 3}


_add("ctrl_discrete_step", name="Step response of a discrete transfer function at sample k", topic="discretization / discrete systems",
     purpose="y[k] of G(z) = numz/denz after a step of size A applied at k = 0, by the difference equation.",
     inputs=[_v("numz", "numerator in z, highest power first"), _v("denz", "denominator in z, highest power first", 2),
             _int("k", "sample index (0 to 500)", 0, 500), _n("A", "step size (default 1)", required=False)],
     outputs=[_o("y", "output at sample k")], primary="y", conditions=["proper (causal) G(z)", "0 <= k <= 500"],
     method="recursion y[j] = sum b_i u[j-i] - sum a_i y[j-i]", latex=r"y[k] = \sum_{i} b_i u[k-i] - \sum_{i\ge 1} a_i y[k-i]",
     run=discrete_step, reference=ref_discrete_step,
     wrong=lambda numz, denz, k, A=1.0: discrete_step(numz, denz, k + 1, A) if k < 500 else {"y": 0.0},
     rand=_rand_dstep, phrases=["discrete step response"],
     examples=[_ex({"numz": [0.5], "denz": [1, -0.5], "k": 2}, {"y": 0.75}),
               _ex({"numz": [1, 0], "denz": [1, -0.5], "k": 3, "A": 2}, {"y": 3.75})],
     extra_q=[_bad({"numz": [0.5], "denz": [1, -0.5], "k": 900})], level="intermediate")


def discrete_pid(Kp, Ki, Kd, T):
    Kp, Ki, Kd, T = _num(Kp, "Kp"), _nonneg(Ki, "Ki"), _nonneg(Kd, "Kd"), _pos(T, "T")
    return _rnd({"q0": Kp + Ki * T + Kd / T, "q1": -Kp - 2 * Kd / T, "q2": Kd / T})


def ref_discrete_pid(Kp, Ki, Kd, T):
    import numpy as np
    rng = np.random.default_rng(7)
    e = rng.normal(size=40)
    integ = np.cumsum(e) * T                     # backward rectangle: the integral includes e[k]
    deriv = np.concatenate([[e[0]], np.diff(e)]) / T
    u = Kp * e + Ki * integ + Kd * deriv
    du = np.diff(u)[2:]                          # u[k] - u[k-1] for k >= 3
    X = np.column_stack([e[3:], e[2:-1], e[1:-2]])
    q = np.linalg.lstsq(X, du, rcond=None)[0]
    return {"q0": float(q[0]), "q1": float(q[1]), "q2": float(q[2])}


_add("ctrl_discrete_pid", name="Discrete PID in velocity (incremental) form", topic="discretization / digital PID",
     purpose="Coefficients of u[k] = u[k-1] + q0 e[k] + q1 e[k-1] + q2 e[k-2] for a PID with backward-difference integral and derivative: q0 = Kp + Ki T + Kd/T, q1 = -Kp - 2 Kd/T, q2 = Kd/T.",
     inputs=[_n("Kp", "proportional gain"), _n("Ki", "integral gain"), _n("Kd", "derivative gain"), _n("T", "sampling period in seconds")],
     outputs=[_o("q0", "coefficient of e[k]"), _o("q1", "coefficient of e[k-1]"), _o("q2", "coefficient of e[k-2]")],
     primary="q0", conditions=["T > 0", "Ki, Kd >= 0"], method="difference of the positional backward-difference PID",
     latex=r"\Delta u_k = (K_p + K_iT + \tfrac{K_d}{T})e_k - (K_p + \tfrac{2K_d}{T})e_{k-1} + \tfrac{K_d}{T}e_{k-2}",
     run=discrete_pid, reference=ref_discrete_pid,
     wrong=lambda Kp, Ki, Kd, T: {"q0": Kp + Ki * T + Kd / T, "q1": -Kp - Kd / T, "q2": Kd / T},
     rand=lambda rng: {"Kp": round(rng.uniform(0.2, 20), 2), "Ki": round(rng.uniform(0.0, 10), 2), "Kd": round(rng.uniform(0.01, 5), 2),
                       "T": round(rng.uniform(0.005, 1), 3)},
     phrases=["discrete PID", "digital PID"],
     examples=[_ex({"Kp": 2, "Ki": 1, "Kd": 0.5, "T": 0.1}, {"q0": 7.1, "q1": -12.0, "q2": 5.0}),
               _ex({"Kp": 1, "Ki": 4, "Kd": 0, "T": 0.25}, {"q0": 2.0, "q1": -1.0, "q2": 0.0})],
     extra_q=[_bad({"Kp": 2, "Ki": 1, "Kd": 0.5, "T": 0})], level="intermediate",
     weak="least-squares fit of the incremental coefficients to a simulated positional PID (same discretization rules)")


def z_to_s_pole(z_re, z_im, T):
    z_re, z_im, T = _num(z_re, "z_re"), _nonneg(z_im, "z_im"), _pos(T, "T")
    _need(z_im > 0 or z_re > 0, "a pole on the negative real axis (or at 0) has no unique continuous equivalent")
    mag = math.hypot(z_re, z_im)
    s_re = math.log(mag) / T
    s_im = math.atan2(z_im, z_re) / T
    wn = math.hypot(s_re, s_im)
    return _rnd({"s_re": s_re, "s_im": s_im, "wn": wn, "zeta": -s_re / wn})


def ref_z_to_s_pole(z_re, z_im, T):
    import control
    z = complex(z_re, z_im)
    den = [1.0, -2 * z_re, abs(z) ** 2] if z_im > 0 else [1.0, -z_re]
    wn, zeta, _ = control.damp(control.tf([1.0], den, T), doprint=False)
    s = cmath.log(z) / T
    return {"s_re": s.real, "s_im": s.imag, "wn": float(wn[0]), "zeta": float(zeta[0])}


_add("ctrl_z_to_s_pole", name="Continuous equivalent of a discrete pole (s = ln z / T)", topic="discretization / pole mapping",
     purpose="The s-plane pole s = ln(z)/T (principal branch) of a discrete pole z = z_re + j z_im, with its natural frequency and damping ratio.",
     inputs=[_n("z_re", "real part of the discrete pole"), _n("z_im", "imaginary part of the discrete pole (>= 0)"),
             _n("T", "sampling period in seconds")],
     outputs=[_o("s_re", "real part of s (1/s)"), _o("s_im", "imaginary part of s (rad/s)"), _o("wn", "natural frequency", "angular_frequency"),
              _o("zeta", "damping ratio")],
     primary="wn", conditions=["T > 0", "z not on the negative real axis"], method="s = (ln|z| + j arg z)/T",
     latex=r"s = \frac{\ln z}{T}", run=z_to_s_pole, reference=ref_z_to_s_pole,
     wrong=lambda z_re, z_im, T: (lambda s: {"s_re": s.real, "s_im": s.imag, "wn": abs(s), "zeta": -s.real / abs(s)})(complex(z_re - 1, z_im) / T),
     rand=lambda rng: (lambda r, th: {"z_re": round(r * math.cos(th), 4), "z_im": round(r * math.sin(th), 4), "T": round(rng.uniform(0.01, 0.5), 3)})(
         rng.uniform(0.2, 0.99), rng.choice([0.0, rng.uniform(0.05, 2.8)])),
     phrases=["z-plane to s-plane pole", "z to s pole"],
     examples=[_ex({"z_re": 0.5, "z_im": 0, "T": 0.1}, {"s_re": -6.93147180560, "s_im": 0.0, "zeta": 1.0}),
               _ex({"z_re": 0, "z_im": 0.5, "T": 1}, {"s_re": -0.69314718056, "s_im": 1.57079632679})],
     extra_q=[_bad({"z_re": -0.5, "z_im": 0, "T": 0.1})], level="intermediate")


# ================================================================================================ PID tuning (intermediate)
def cohen_coon(K, tau, theta, controller):
    K, tau, theta = _pos(K, "K"), _pos(tau, "tau"), _pos(theta, "theta")
    _need(controller in ("P", "PI", "PID"), "controller must be P, PI or PID")
    r = theta / tau
    if controller == "P":
        return _pid_out((1 / K) * (1 / r) * (1 + r / 3), 0.0, 0.0)
    if controller == "PI":
        return _pid_out((1 / K) * (1 / r) * (0.9 + r / 12), theta * (30 + 3 * r) / (9 + 20 * r), 0.0)
    return _pid_out((1 / K) * (1 / r) * (4.0 / 3 + r / 4), theta * (32 + 6 * r) / (13 + 8 * r), 4 * theta / (11 + 2 * r))


def ref_cohen_coon(K, tau, theta, controller):
    K, tau, theta = Fraction(K), Fraction(tau), Fraction(theta)
    if controller == "P":
        Kp, Ti, Td = (tau / (K * theta)) * (1 + theta / (3 * tau)), Fraction(0), Fraction(0)
    elif controller == "PI":
        Kp = (tau / (K * theta)) * (Fraction(9, 10) + theta / (12 * tau))
        Ti, Td = theta * (30 * tau + 3 * theta) / (9 * tau + 20 * theta), Fraction(0)
    else:
        Kp = (tau / (K * theta)) * (Fraction(4, 3) + theta / (4 * tau))
        Ti = theta * (32 * tau + 6 * theta) / (13 * tau + 8 * theta)
        Td = 4 * theta * tau / (11 * tau + 2 * theta)
    return {"Kp": float(Kp), "Ti": float(Ti), "Td": float(Td)}


_add("ctrl_cohen_coon", name="Cohen-Coon tuning of a first-order-plus-dead-time process", topic="PID tuning / Cohen-Coon",
     purpose="P, PI or PID settings from the Cohen-Coon formulas for K e^{-theta s}/(tau s + 1).",
     inputs=[_n("K", "process gain"), _n("tau", "time constant in seconds"), _n("theta", "dead time in seconds"),
             _ch("controller", ["P", "PI", "PID"], "controller type")],
     outputs=_PID_OUT, primary="Kp", conditions=["K, tau, theta > 0"], method="Cohen-Coon formulas (r = theta/tau)",
     latex=r"\text{PID: } K_p = \frac{\tau}{K\theta}\left(\frac{4}{3} + \frac{\theta}{4\tau}\right),\; T_i = \theta\frac{32+6\theta/\tau}{13+8\theta/\tau},\; T_d = \frac{4\theta}{11+2\theta/\tau}",
     run=cohen_coon, reference=ref_cohen_coon,
     wrong=lambda K, tau, theta, controller: (lambda r: dict(r, Kp=r["Kp"] * K * K))(cohen_coon(K, tau, theta, controller)) if K != 1 else _pid_out(0.0, 0.0, 0.0),
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.5, 50), 2), "theta": round(rng.uniform(0.1, 10), 2),
                       "controller": rng.choice(["P", "PI", "PID"])},
     phrases=["Cohen-Coon tuning", "Cohen Coon tuning"],
     examples=[_ex({"K": 2, "tau": 10, "theta": 2, "controller": "PID"}, {"Kp": 3.45833333333, "Ti": 4.54794520548, "Td": 0.701754385965}),
               _ex({"K": 1, "tau": 5, "theta": 1, "controller": "PI"}, {"Kp": 4.58333333333, "Ti": 2.35384615385})],
     extra_q=[_bad({"K": 2, "tau": 10, "theta": 0, "controller": "PID"})], level="intermediate",
     weak="formulas restated in exact fractions (no library computes the Cohen-Coon table)")


def simc_pi(K, tau, theta, tau_c=None):
    K, tau, theta = _pos(K, "K"), _pos(tau, "tau"), _pos(theta, "theta")
    tc = theta if tau_c is None else _pos(tau_c, "tau_c")
    Kp = tau / (K * (tc + theta))
    Ti = min(tau, 4 * (tc + theta))
    return _rnd({"Kp": Kp, "Ti": Ti, "Ki": Kp / Ti})


def ref_simc_pi(K, tau, theta, tau_c=None):
    tc = Fraction(theta if tau_c is None else tau_c)
    Kp = Fraction(tau) / (Fraction(K) * (tc + Fraction(theta)))
    Ti = min(Fraction(tau), 4 * (tc + Fraction(theta)))
    return {"Kp": float(Kp), "Ti": float(Ti), "Ki": float(Kp / Ti)}


_add("ctrl_simc_pi", name="SIMC (Skogestad) PI tuning", topic="PID tuning / SIMC",
     purpose="PI settings Kp = tau/(K (tau_c + theta)), Ti = min(tau, 4 (tau_c + theta)) for K e^{-theta s}/(tau s + 1); the default tau_c equals theta.",
     inputs=[_n("K", "process gain"), _n("tau", "time constant in seconds"), _n("theta", "dead time in seconds"),
             _n("tau_c", "desired closed-loop time constant (default theta)", required=False)],
     outputs=[_o("Kp", "proportional gain"), _o("Ti", "integral time", "time"), _o("Ki", "integral gain")], primary="Kp",
     conditions=["K, tau, theta > 0"], method="Skogestad's SIMC rule", latex=r"K_p = \frac{\tau}{K(\tau_c+\theta)},\; T_i = \min(\tau, 4(\tau_c+\theta))",
     run=simc_pi, reference=ref_simc_pi,
     wrong=lambda K, tau, theta, tau_c=None: {"Kp": tau / (K * (theta if tau_c is None else tau_c)), "Ti": tau, "Ki": 1 / (K * (theta if tau_c is None else tau_c))},
     rand=lambda rng: dict({"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.5, 60), 2), "theta": round(rng.uniform(0.1, 10), 2)},
                           **({"tau_c": round(rng.uniform(0.1, 20), 2)} if rng.random() < 0.5 else {})),
     phrases=["SIMC PI tuning", "Skogestad PI tuning"],
     examples=[_ex({"K": 2, "tau": 10, "theta": 1}, {"Kp": 2.5, "Ti": 8.0, "Ki": 0.3125}),
               _ex({"K": 1, "tau": 4, "theta": 0.5, "tau_c": 2}, {"Kp": 1.6, "Ti": 4.0})],
     extra_q=[_bad({"K": 2, "tau": 10, "theta": -1})], level="intermediate", weak="the rule restated in exact fractions")


def pid_closed_loop(num, den, Kp, Ki=0.0, Kd=0.0):
    num, den = _tf_in(num, den, strict=True)
    Kp, Ki, Kd = _num(Kp, "Kp"), _num(Ki, "Ki"), _num(Kd, "Kd")
    if Ki != 0:
        char = _padd(_pmul(den, [1.0, 0.0]), _pmul([Kd, Kp, Ki], num))
    else:
        char = _padd(den, _pmul([Kd, Kp], num))
    char = _ptrim(char)
    _need(len(char) >= 2, "the closed loop has no poles")
    poles = _roots(char)
    mr = max(complex(p).real for p in poles)
    return {"char_poly": [_r12(x) for x in char], "poles": poles, "max_real_part": _r12(mr), "stable": mr < 0}


def ref_pid_closed_loop(num, den, Kp, Ki=0.0, Kd=0.0):
    import control
    s = control.tf("s")
    C = Kp + Kd * s + (Ki / s if Ki != 0 else 0)
    cl = control.feedback(C * control.tf(num, den), 1)
    p = _ref_canon(cl.poles())
    return {"poles": p, "max_real_part": max(complex(x).real for x in p), "stable": max(complex(x).real for x in p) < 0}


def _rand_pidcl(rng):
    for _ in range(100):
        x = _rand_stable_tf(rng, 3)
        if len(x["num"]) >= len(x["den"]):
            continue
        x.update({"Kp": round(rng.uniform(0.2, 20), 2), "Ki": round(rng.uniform(0, 10), 2), "Kd": round(rng.uniform(0, 3), 2)})
        r = _roots_raw(pid_closed_loop(**x)["char_poly"])
        sep = min((abs(complex(r[i]) - complex(r[j])) for i in range(len(r)) for j in range(i + 1, len(r))), default=1)
        if sep > 0.05 * max(1.0, max(abs(complex(v)) for v in r)):
            return x
    return {"num": [1.0], "den": [1.0, 1.0], "Kp": 1.0}


_add("ctrl_pid_closed_loop", name="Closed-loop poles of a PID loop", topic="PID tuning / closed loop",
     purpose="The characteristic polynomial s D(s) + (Kd s^2 + Kp s + Ki) N(s), the closed-loop poles and stability of a plant N/D under a PID controller with unity feedback.",
     inputs=[_v("num", "plant numerator"), _v("den", "plant denominator", 2), _n("Kp", "proportional gain"),
             _n("Ki", "integral gain (default 0)", required=False), _n("Kd", "derivative gain (default 0)", required=False)],
     outputs=[_o("char_poly", "closed-loop characteristic polynomial"), _o("poles", "closed-loop poles"),
              _o("max_real_part", "largest real part"), _o("stable", "true when every pole is in the left half plane", "boolean")],
     primary="max_real_part", conditions=["strictly proper plant"], method="1 + C(s) G(s) = 0, roots by the Aberth-Ehrlich iteration",
     latex=r"sD(s) + (K_ds^2 + K_ps + K_i)N(s) = 0", run=pid_closed_loop, reference=ref_pid_closed_loop,
     wrong=lambda num, den, Kp, Ki=0.0, Kd=0.0: pid_closed_loop(num, den, Kp, Kd, Ki) if Ki != Kd else pid_closed_loop(num, den, -Kp, Ki, Kd),
     rand=_rand_pidcl, phrases=["PID closed-loop poles", "PID closed loop poles"],
     examples=[_ex({"num": [1], "den": [1, 1], "Kp": 3, "Ki": 2}, {"char_poly": [1.0, 4.0, 2.0], "poles": [-3.41421356237, -0.585786437627], "stable": True}),
               _ex({"num": [1], "den": [1, 3, 2], "Kp": 10}, {"char_poly": [1.0, 3.0, 12.0], "stable": True})],
     extra_q=[_bad({"num": [1, 1], "den": [1, 1], "Kp": 3})], level="intermediate")


def lead_max_phase(z, p):
    z, p = _pos(z, "z"), _pos(p, "p")
    _need(z < p, "a lead compensator has its zero below its pole (z < p)")
    wm = math.sqrt(z * p)
    return _rnd({"phi_max_deg": math.degrees(math.asin((p - z) / (p + z))), "w_max": wm, "alpha": z / p,
                 "magnitude_db": 10 * math.log10(z / p)})


def ref_lead_max_phase(z, p):
    import numpy as np
    num, den = np.array([1.0, z]), np.array([1.0, p])
    # d(phase)/dw = Re(C'(jw)/C(jw)) (numpy polynomial derivative); its positive root is the maximum
    f = lambda w: float(np.real(np.polyval(np.polyder(num), 1j * w) / np.polyval(num, 1j * w)
                                - np.polyval(np.polyder(den), 1j * w) / np.polyval(den, 1j * w)))
    wm = _ref_crossing(f, z * 1e-2, p * 1e2)
    h = _ref_freq([1.0, z], [1.0, p], wm)
    return {"phi_max_deg": math.degrees(cmath.phase(h)), "w_max": wm, "magnitude_db": 20 * math.log10(abs(h))}


_add("ctrl_lead_max_phase", name="Maximum phase lead of a lead compensator", topic="compensators / lead",
     purpose="For C(s) = (s + z)/(s + p) with z < p: the maximum phase lead asin((p - z)/(p + z)), the frequency sqrt(z p) where it occurs, alpha = z/p and the gain there, 10 log10(z/p) dB.",
     inputs=[_n("z", "zero corner frequency (rad/s)"), _n("p", "pole corner frequency (rad/s), p > z")],
     outputs=[_o("phi_max_deg", "maximum phase lead in degrees"), _o("w_max", "frequency of the maximum", "angular_frequency"),
              _o("alpha", "alpha = z/p"), _o("magnitude_db", "|C(j w_max)| in dB")],
     primary="phi_max_deg", conditions=["0 < z < p"], method="stationary point of the phase",
     latex=r"\sin\phi_{max} = \frac{p-z}{p+z} = \frac{1-\alpha}{1+\alpha},\quad \omega_m = \sqrt{zp}",
     run=lead_max_phase, reference=ref_lead_max_phase,
     wrong=lambda z, p: {"phi_max_deg": math.degrees(math.atan((p - z) / (p + z))), "w_max": (z + p) / 2, "alpha": z / p, "magnitude_db": 20 * math.log10(z / p)},
     rand=lambda rng: (lambda z: {"z": z, "p": round(z * rng.uniform(1.5, 30), 2)})(round(rng.uniform(0.1, 20), 2)),
     phrases=["lead compensator maximum phase", "maximum phase lead"],
     examples=[_ex({"z": 1, "p": 9}, {"phi_max_deg": 53.1301023542, "w_max": 3.0, "alpha": 0.111111111111}),
               _ex({"z": 2, "p": 8}, {"phi_max_deg": 36.8698976458, "w_max": 4.0, "magnitude_db": -6.02059991328})],
     extra_q=[_bad({"z": 9, "p": 1})], level="intermediate")


# ================================================================================================ specifications and identification
def specs_to_poles(OS, Ts):
    mp, Ts = _num(OS, "OS"), _pos(Ts, "Ts")
    _need(0 < mp < 1, "the overshoot must lie strictly between 0 and 100 %")
    L = math.log(mp)
    zeta = -L / math.sqrt(PI * PI + L * L)
    sigma = 4.0 / Ts
    wn = sigma / zeta
    wd = wn * math.sqrt(1 - zeta * zeta)
    return _rnd({"zeta": zeta, "wn": wn, "sigma": sigma, "wd": wd, "char_poly": [1.0, 2 * sigma, wn * wn]})


def ref_specs_to_poles(OS, Ts):
    z = ref_damping_from_overshoot(OS)["zeta"]
    wn = 4.0 / (z * Ts)
    return {"zeta": z, "wn": wn, "sigma": 4.0 / Ts, "wd": wn * math.sqrt(1 - z * z)}


_add("ctrl_specs_to_poles", name="Desired closed-loop poles from overshoot and settling time", topic="design / specifications",
     purpose="zeta from the overshoot, sigma = 4/Ts from the 2 % settling time, wn = sigma/zeta, the poles -sigma +/- j wd and the desired characteristic polynomial.",
     inputs=[_n("OS", "maximum overshoot (write 10% or 0.1)", pct=True), _n("Ts", "2 % settling time in seconds")],
     outputs=[_o("zeta", "damping ratio"), _o("wn", "natural frequency", "angular_frequency"), _o("sigma", "real part magnitude 4/Ts"),
              _o("wd", "imaginary part of the poles", "angular_frequency"), _o("char_poly", "s^2 + 2 sigma s + wn^2")],
     primary="wn", conditions=["0 < OS < 100 %", "Ts > 0"], method="zeta from the overshoot formula; sigma = 4/Ts",
     latex=r"\zeta = \frac{-\ln OS}{\sqrt{\pi^2+\ln^2 OS}},\quad \sigma = \frac{4}{T_s},\quad \omega_n = \frac{\sigma}{\zeta}",
     run=specs_to_poles, reference=ref_specs_to_poles,
     wrong=lambda OS, Ts: (lambda r: dict(r, wn=r["sigma"] * r["zeta"]))(specs_to_poles(OS, Ts)),
     rand=lambda rng: {"OS": round(rng.uniform(0.01, 0.6), 3), "Ts": round(rng.uniform(0.1, 20), 2)},
     phrases=["poles from overshoot and settling time"],
     examples=[_ex({"OS": 0.163033534821, "Ts": 2}, {"zeta": 0.5, "wn": 4.0, "sigma": 2.0, "wd": 3.46410161514}),
               _ex({"OS": 0.1, "Ts": 1}, {"zeta": 0.591155033798, "sigma": 4.0})],
     extra_q=[_bad({"OS": 0.1, "Ts": -1})], level="intermediate", weak="the settling time rule 4/(zeta wn) is restated")


def log_decrement(x1, xn, n):
    x1, xn = _pos(x1, "x1"), _pos(xn, "xn")
    n = int(n)
    _need(n >= 1, "n must be at least 1")
    _need(xn < x1, "the later peak must be smaller")
    d = math.log(x1 / xn) / n
    return _rnd({"delta": d, "zeta": d / math.sqrt(4 * PI * PI + d * d)})


def ref_log_decrement(x1, xn, n):
    import numpy as np
    from scipy.linalg import expm
    from scipy.optimize import brentq
    target = x1 / xn

    def ratio(z):                               # free response from x(0) = 1 at rest: peaks at multiples of 2 pi / wd
        A = np.array([[0.0, 1.0], [-1.0, -2 * z]])
        wd = math.sqrt(1 - z * z)
        x = expm(A * (2 * math.pi * n / wd)) @ np.array([1.0, 0.0])
        return 1.0 / x[0]
    z = brentq(lambda zz: ratio(zz) - target, 1e-9, 0.999, xtol=1e-16, rtol=8.9e-16)
    return {"zeta": z}


_add("ctrl_log_decrement", name="Damping ratio from the logarithmic decrement", topic="identification / second order",
     purpose="delta = ln(x1/xn)/n from two peak amplitudes n cycles apart, and zeta = delta/sqrt(4 pi^2 + delta^2).",
     inputs=[_n("x1", "first peak amplitude (measured from the final value)"), _n("xn", "peak amplitude n cycles later"),
             _int("n", "number of cycles between the peaks", 1, 1000)],
     outputs=[_o("delta", "logarithmic decrement per cycle"), _o("zeta", "damping ratio")], primary="zeta",
     conditions=["0 < xn < x1", "n >= 1"], method="ratio of successive peaks", latex=r"\delta = \frac{1}{n}\ln\frac{x_1}{x_{n+1}},\quad \zeta = \frac{\delta}{\sqrt{4\pi^2+\delta^2}}",
     run=log_decrement, reference=ref_log_decrement,
     wrong=lambda x1, xn, n: {"delta": math.log(x1 / xn) / n, "zeta": math.log(x1 / xn) / n / (2 * PI)},
     rand=lambda rng: (lambda x1: {"x1": x1, "xn": round(x1 * rng.uniform(0.02, 0.9), 4), "n": rng.randint(1, 5)})(round(rng.uniform(0.5, 10), 3)),
     phrases=["logarithmic decrement"],
     examples=[_ex({"x1": 1, "xn": 0.5, "n": 1}, {"delta": 0.69314718056, "zeta": 0.109652580999}),
               _ex({"x1": 4, "xn": 1, "n": 3}, {"delta": 0.462098120373})],
     extra_q=[_bad({"x1": 1, "xn": 2, "n": 1})], level="intermediate")


def identify_second_order(y_peak, y_final, t_peak, A=1.0):
    yp, yf, tp, A = _num(y_peak, "y_peak"), _num(y_final, "y_final"), _pos(t_peak, "t_peak"), _num(A, "A")
    _need(A != 0 and yf != 0, "the step size and the final value must not be zero")
    mp = (yp - yf) / yf
    _need(0 < mp < 1, "the peak must exceed the final value by less than 100 %")
    L = math.log(mp)
    zeta = -L / math.sqrt(PI * PI + L * L)
    wn = PI / (tp * math.sqrt(1 - zeta * zeta))
    return _rnd({"K": yf / A, "PO": 100 * mp, "zeta": zeta, "wn": wn})


def ref_identify_second_order(y_peak, y_final, t_peak, A=1.0):
    mp = (y_peak - y_final) / y_final
    z = ref_damping_from_overshoot(mp)["zeta"]
    tp1, _ = _ref_2nd_peak(z, 1.0)               # peak time at wn = 1, scaled
    return {"K": y_final / A, "PO": 100 * mp, "zeta": z, "wn": tp1 / t_peak}


_add("ctrl_identify_second_order", name="Second-order model from a step response", topic="identification / second order",
     purpose="Gain K = y_final/A, percent overshoot, zeta and wn of a standard second-order model from the peak value, the final value and the peak time of a measured step response.",
     inputs=[_n("y_peak", "peak value of the response"), _n("y_final", "final (steady) value"), _n("t_peak", "time of the peak in seconds"),
             _n("A", "step size (default 1)", required=False)],
     outputs=[_o("K", "DC gain"), _o("PO", "percent overshoot (%)"), _o("zeta", "damping ratio"), _o("wn", "natural frequency", "angular_frequency")],
     primary="wn", conditions=["0 < overshoot < 100 %", "t_peak > 0"], method="zeta from the overshoot, wn = pi/(tp sqrt(1 - zeta^2))",
     latex=r"\omega_n = \frac{\pi}{T_p\sqrt{1-\zeta^2}}", run=identify_second_order, reference=ref_identify_second_order,
     wrong=lambda y_peak, y_final, t_peak, A=1.0: (lambda r: dict(r, wn=PI / t_peak))(identify_second_order(y_peak, y_final, t_peak, A)),
     rand=lambda rng: (lambda yf: {"y_peak": round(yf * (1 + rng.uniform(0.01, 0.7)), 4), "y_final": yf, "t_peak": round(rng.uniform(0.05, 20), 3),
                                   "A": round(rng.uniform(0.5, 5), 1)})(round(rng.uniform(0.5, 20), 3)),
     phrases=["second-order model from step response", "second order model from step response"],
     examples=[_ex({"y_peak": 1.163033534821, "y_final": 1, "t_peak": 0.906899682118}, {"zeta": 0.5, "wn": 4.0}),
               _ex({"y_peak": 12.5, "y_final": 10, "t_peak": 1.2, "A": 2}, {"K": 5.0, "PO": 25.0})],
     extra_q=[_bad({"y_peak": 0.9, "y_final": 1, "t_peak": 1})], level="intermediate")


def fopdt_step(K, tau, theta, t, A=1.0):
    K, tau, theta, t, A = _num(K, "K"), _pos(tau, "tau"), _nonneg(theta, "theta"), _nonneg(t, "t"), _num(A, "A")
    _need(K != 0 and A != 0, "K and A must not be zero")
    y = 0.0 if t <= theta else -K * A * math.expm1(-(t - theta) / tau)
    return _rnd({"y": y})


def ref_fopdt_step(K, tau, theta, t, A=1.0):
    from scipy.integrate import solve_ivp
    if t <= theta:
        return {"y": 0.0}
    sol = solve_ivp(lambda s, y: [(K * A - y[0]) / tau], (theta, t), [0.0], method="DOP853", rtol=1e-13, atol=1e-16 * abs(K * A))
    return {"y": float(sol.y[0, -1])}


_add("ctrl_fopdt_step", name="Step response of a first-order-plus-dead-time process", topic="time response / dead time",
     purpose="y(t) of K e^{-theta s}/(tau s + 1) after a step of size A: 0 before the dead time, K A (1 - e^{-(t - theta)/tau}) after it.",
     inputs=[_n("K", "process gain"), _n("tau", "time constant in seconds"), _n("theta", "dead time in seconds"), _n("t", "time in seconds"),
             _n("A", "step size (default 1)", required=False)],
     outputs=[_o("y", "output at time t")], primary="y", conditions=["tau > 0", "theta >= 0", "t >= 0"],
     method="delayed first-order response", latex=r"y(t) = KA\left(1 - e^{-(t-\theta)/\tau}\right)\,u(t-\theta)",
     run=fopdt_step, reference=ref_fopdt_step,
     wrong=lambda K, tau, theta, t, A=1.0: {"y": -K * A * math.expm1(-t / tau)},
     rand=lambda rng: (lambda th: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.2, 20), 2), "theta": th,
                                   "t": round(th + rng.uniform(0.1, 30), 2), "A": round(rng.uniform(0.5, 5), 1)})(round(rng.uniform(0.1, 10), 2)),
     phrases=["dead-time step response", "FOPDT step response"],
     examples=[_ex({"K": 2, "tau": 5, "theta": 1, "t": 6}, {"y": 1.26424111766}), _ex({"K": 2, "tau": 5, "theta": 1, "t": 0.5}, {"y": 0.0})],
     extra_q=[_bad({"K": 2, "tau": 0, "theta": 1, "t": 6})], level="intermediate")


def kalman_scalar_step(x, P, a, q, r, z):
    x, P, a, q, r, z = _num(x, "x"), _nonneg(P, "P"), _num(a, "a"), _nonneg(q, "q"), _pos(r, "r"), _num(z, "z")
    xp = a * x
    Pp = a * a * P + q
    _need(Pp > 0, "the predicted variance is zero")
    K = Pp / (Pp + r)
    return _rnd({"x_pred": xp, "P_pred": Pp, "gain": K, "x_post": xp + K * (z - xp), "P_post": (1 - K) * Pp})


def ref_kalman_scalar_step(x, P, a, q, r, z):
    import numpy as np
    Pp = np.array([[a]]) @ np.array([[P]]) @ np.array([[a]]) + q
    info = np.linalg.inv(Pp) + 1.0 / r                    # information form
    Ppost = float(np.linalg.inv(info)[0, 0])
    xpost = Ppost * (float(np.linalg.inv(Pp)[0, 0]) * a * x + z / r)
    return {"x_pred": a * x, "P_pred": float(Pp[0, 0]), "gain": Ppost / r, "x_post": xpost, "P_post": Ppost}


_add("ctrl_kalman_scalar_step", name="One step of a scalar Kalman filter", topic="estimation / Kalman filter",
     purpose="Predict (x- = a x, P- = a^2 P + q) and update with a measurement z of variance r (gain P-/(P- + r)) for x_k = a x_{k-1} + w, z_k = x_k + v.",
     inputs=[_n("x", "previous estimate"), _n("P", "previous error variance"), _n("a", "state transition coefficient"),
             _n("q", "process noise variance"), _n("r", "measurement noise variance"), _n("z", "new measurement")],
     outputs=[_o("x_pred", "predicted estimate"), _o("P_pred", "predicted variance"), _o("gain", "Kalman gain"),
              _o("x_post", "updated estimate"), _o("P_post", "updated variance")],
     primary="x_post", conditions=["P, q >= 0", "r > 0"], method="Kalman predict and update equations",
     latex=r"K = \frac{P^-}{P^- + r},\quad \hat x = \hat x^- + K(z - \hat x^-),\quad P = (1-K)P^-",
     run=kalman_scalar_step, reference=ref_kalman_scalar_step,
     wrong=lambda x, P, a, q, r, z: (lambda Pp: (lambda K: {"x_pred": a * x, "P_pred": Pp, "gain": K, "x_post": a * x + K * (z - a * x), "P_post": (1 - K) * Pp})(Pp / r))(a * P + q),
     rand=lambda rng: {"x": round(rng.uniform(-10, 10), 2), "P": round(rng.uniform(0.1, 5), 2), "a": round(rng.uniform(0.5, 1.2), 2),
                       "q": round(rng.uniform(0.01, 2), 3), "r": round(rng.uniform(0.1, 5), 2), "z": round(rng.uniform(-12, 12), 2)},
     phrases=["scalar Kalman filter step", "Kalman filter step"],
     examples=[_ex({"x": 0, "P": 1, "a": 1, "q": 1, "r": 2, "z": 4}, {"x_pred": 0.0, "P_pred": 2.0, "gain": 0.5, "x_post": 2.0, "P_post": 1.0}),
               _ex({"x": 1, "P": 0.5, "a": 2, "q": 0, "r": 2, "z": 5}, {"gain": 0.5, "x_post": 3.5})],
     extra_q=[_bad({"x": 0, "P": 1, "a": 1, "q": 1, "r": 0, "z": 4})], level="intermediate",
     weak="the information-filter form of the same update (an algebraic rearrangement)")


def dc_motor(J, b, Kt, R, L):
    J, b, Kt, R, L = _pos(J, "J"), _nonneg(b, "b"), _pos(Kt, "Kt"), _pos(R, "R"), _pos(L, "L")
    den = [J * L, b * L + J * R, b * R + Kt * Kt]
    return {"num": [_r12(Kt)], "den": [_r12(x) for x in den], "dc_gain": _r12(Kt / den[2]), "poles": _roots(den)}


def ref_dc_motor(J, b, Kt, R, L):
    import control
    import numpy as np
    A = np.array([[-b / J, Kt / J], [-Kt / L, -R / L]])
    sys = control.ss(A, [[0.0], [1.0 / L]], [[1.0, 0.0]], [[0.0]])
    return {"dc_gain": float(np.real(control.dcgain(sys))), "poles": _ref_canon(np.linalg.eigvals(A))}


_add("ctrl_dc_motor", name="Speed transfer function of a DC motor", topic="modelling / DC motor",
     purpose="omega(s)/V(s) = Kt/((J s + b)(L s + R) + Kt^2) of an armature-controlled DC motor (Ke = Kt in SI units): coefficients, DC gain and poles.",
     inputs=[_n("J", "rotor inertia in kg*m^2"), _n("b", "viscous friction in N*m*s"), _n("Kt", "torque (= back-emf) constant in N*m/A"),
             _n("R", "armature resistance in ohm"), _n("L", "armature inductance in H")],
     outputs=[_o("num", "numerator"), _o("den", "denominator [J L, b L + J R, b R + Kt^2]"), _o("dc_gain", "speed per volt at steady state (rad/s per V)"),
              _o("poles", "poles")],
     primary="dc_gain", conditions=["J, Kt, R, L > 0", "b >= 0"], method="electrical and mechanical equations combined",
     latex=r"\frac{\Omega(s)}{V(s)} = \frac{K_t}{(Js+b)(Ls+R)+K_t^2}", run=dc_motor, reference=ref_dc_motor,
     wrong=lambda J, b, Kt, R, L: (lambda d: {"num": [Kt], "den": d, "dc_gain": Kt / (b * R), "poles": _roots(d)})([J * L, b * L + J * R, b * R]),
     rand=lambda rng: {"J": round(rng.uniform(0.001, 0.1), 4), "b": round(rng.uniform(0.001, 0.2), 4), "Kt": round(rng.uniform(0.01, 1), 3),
                       "R": round(rng.uniform(0.5, 10), 2), "L": round(rng.uniform(0.01, 1), 3)},
     phrases=["DC motor transfer function"],
     examples=[_ex({"J": 0.01, "b": 0.1, "Kt": 0.01, "R": 1, "L": 0.5}, {"den": [0.005, 0.06, 0.1001], "dc_gain": 0.0999000999001}),
               _ex({"J": 1, "b": 0, "Kt": 1, "R": 2, "L": 1}, {"dc_gain": 1.0, "poles": [-1.0, -1.0]})],
     extra_q=[_bad({"J": 0, "b": 0.1, "Kt": 0.01, "R": 1, "L": 0.5})], level="intermediate")


# ================================================================================================ margins and bandwidth
def _phase_crossovers(num, den):
    """Positive frequencies where G(jw) is real and negative."""
    Nr, Ni = _jw(num)
    Dr, Di = _jw(den)
    imp = _ptrim(_psub(_pmul(Ni, Dr), _pmul(Nr, Di)))
    out = []
    if len(imp) < 2 or all(x == 0 for x in imp):
        return out
    for w in _real_pos_roots(imp):
        if _G(num, den, complex(0.0, w)).real < 0:
            out.append(w)
    return out


def _gain_crossovers(num, den):
    return _real_pos_roots(_ptrim(_psub(_mag2(num), _mag2(den))))


def stability_margins(num, den):
    num, den = _tf_in(num, den)
    gc = _gain_crossovers(num, den)
    pc = _phase_crossovers(num, den)
    _need(len(gc) == 1, "the loop needs exactly one gain crossover (|L(jw)| = 1) for this capability" if gc else
          "|L(jw)| never crosses 1: the phase margin is not defined")
    _need(len(pc) == 1, "the loop needs exactly one phase crossover for this capability" if pc else
          "the phase never crosses -180 degrees: the gain margin is infinite")
    gpc = _G(num, den, complex(0.0, pc[0]))
    ggc = _G(num, den, complex(0.0, gc[0]))
    gm = 1.0 / math.hypot(gpc.real, gpc.imag)
    return _rnd({"gm": gm, "gm_db": 20 * math.log10(gm), "w_pc": pc[0], "pm_deg": _wrap180(180.0 + _deg(ggc)), "w_gc": gc[0]})


def ref_stability_margins(num, den):
    import control
    gm, pm, _, wpc, wgc, _ = control.stability_margins(control.tf(num, den))
    return {"gm": float(gm), "gm_db": 20 * math.log10(float(gm)), "w_pc": float(wpc), "pm_deg": float(pm), "w_gc": float(wgc)}


def _rand_loop(rng):
    k = round(10 ** rng.uniform(-0.3, 2.5), 2)
    form = rng.randint(0, 2)
    a, b, c = sorted(rng.sample([0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 8.0, 10.0], 3))
    if form == 0:
        return {"num": [k], "den": [_r12(x) for x in _pmul(_pmul([1.0, 0.0], [1.0, a]), [1.0, b])]}
    if form == 1:
        return {"num": [k], "den": [_r12(x) for x in _pmul(_pmul([1.0, a], [1.0, b]), [1.0, c])]}
    z = round(rng.uniform(0.2, 3), 1)
    return {"num": [k, _r12(k * z)], "den": [_r12(x) for x in _pmul(_pmul(_pmul([1.0, 0.0], [1.0, a]), [1.0, b]), [1.0, c])]}


_add("ctrl_stability_margins", name="Gain and phase margins", topic="stability / margins",
     purpose="Gain margin 1/|L(j w_pc)| (ratio and dB) at the phase crossover and phase margin 180 + arg L(j w_gc) at the gain crossover of an open loop L(s).",
     inputs=[_v("num", "numerator of the open loop L(s)"), _v("den", "denominator of L(s)", 2)],
     outputs=[_o("gm", "gain margin (ratio)"), _o("gm_db", "gain margin in dB"), _o("w_pc", "phase-crossover frequency", "angular_frequency"),
              _o("pm_deg", "phase margin in degrees"), _o("w_gc", "gain-crossover frequency", "angular_frequency")],
     primary="pm_deg", conditions=["exactly one gain crossover and one phase crossover"],
     method="crossovers as positive real roots of |N|^2 - |D|^2 and Im(N(jw) conj D(jw)), Newton-polished",
     latex=r"GM = \frac{1}{|L(j\omega_{pc})|},\quad PM = 180^\circ + \angle L(j\omega_{gc})",
     run=stability_margins, reference=ref_stability_margins,
     wrong=lambda num, den: (lambda r: dict(r, gm=1 / r["gm"], gm_db=-r["gm_db"], pm_deg=_wrap180(r["pm_deg"] + 180.0)))(stability_margins(num, den)),
     rand=_rand_loop, phrases=["stability margins", "gain and phase margins"],
     examples=[_ex({"num": [10], "den": [1, 3, 2, 0]}, {"gm": 0.6, "w_pc": 1.41421356237, "pm_deg": -12.9972080155}),
               _ex({"num": [2], "den": [1, 3, 2, 0]}, {"gm": 3.0, "gm_db": 9.54242509439})],
     extra_q=[_bad({"num": [1], "den": [1, 1]})], level="advanced")


def bandwidth_general(num, den):
    num, den = _tf_in(num, den, strict=True)
    _need(_routh_ok(den), "the system must be stable")
    g0 = num[-1] / den[-1]
    _need(g0 != 0, "the DC gain is zero")
    roots = _real_pos_roots(_ptrim(_psub(_pscale(_mag2(num), 2.0), _pscale(_mag2(den), g0 * g0))))
    _need(len(roots) > 0, "the gain never falls 3 dB below its DC value")
    return _rnd({"wb": roots[0], "dc_gain": g0})


def ref_bandwidth_general(num, den):
    import numpy as np
    g0 = num[-1] / den[-1]
    lvl = abs(g0) / math.sqrt(2)
    ws = np.logspace(-5, 5, 20001)
    from scipy.signal import freqs
    _, h = freqs(num, den, worN=ws)
    i = int(np.argmax(np.abs(h) < lvl))
    wb = _ref_crossing(lambda w: abs(_ref_freq(num, den, w)) - lvl, ws[i - 1], ws[i])
    return {"wb": wb, "dc_gain": g0}


_add("ctrl_bandwidth", name="-3 dB bandwidth of a transfer function", topic="frequency response / bandwidth",
     purpose="The lowest frequency at which |G(jw)| falls to |G(0)|/sqrt(2) for a stable, strictly proper G(s), and the DC gain.",
     inputs=[_v("num", "numerator"), _v("den", "denominator", 2)],
     outputs=[_o("wb", "-3 dB bandwidth", "angular_frequency"), _o("dc_gain", "DC gain")], primary="wb",
     conditions=["stable", "strictly proper", "nonzero DC gain"], method="smallest positive root of 2|N(jw)|^2 - G(0)^2 |D(jw)|^2",
     latex=r"|G(j\omega_B)| = \frac{|G(0)|}{\sqrt 2}", run=bandwidth_general, reference=ref_bandwidth_general,
     wrong=lambda num, den: (lambda r: dict(r, wb=_real_pos_roots(_ptrim(_psub(_mag2(num), _pscale(_mag2(den), r["dc_gain"] ** 2 / 4))))[0]))(bandwidth_general(num, den)),
     rand=lambda rng: (lambda x: x if len(x["num"]) < len(x["den"]) else {"num": x["num"][:1], "den": x["den"]})(_rand_stable_tf(rng, 4)),
     phrases=["bandwidth"],
     examples=[_ex({"num": [1], "den": [1, 1]}, {"wb": 1.0, "dc_gain": 1.0}),
               _ex({"num": [100], "den": [1, 10, 100]}, {"wb": 12.7201964951})],
     extra_q=[_bad({"num": [1], "den": [1, -1]})], level="advanced")


def stable_gain_range(num, den):
    num, den = _tf_in(num, den)
    cands = []
    if num[-1] != 0:
        k0 = -den[-1] / num[-1]
        if k0 > 0:
            cands.append((k0, 0.0))
    Nr, Ni = _jw(num)
    Dr, Di = _jw(den)
    imp = _ptrim(_psub(_pmul(Di, Nr), _pmul(Dr, Ni)))
    if len(imp) >= 2 and any(x != 0 for x in imp):
        for w in _real_pos_roots(imp):
            s = complex(0.0, w)
            nv, dv = _pval(num, s), _pval(den, s)
            if abs(nv) == 0:
                continue
            k = -(dv * nv.conjugate()).real / (abs(nv) ** 2)
            if k > 0 and abs(dv + k * nv) <= 1e-8 * (abs(dv) + abs(k * nv)):
                cands.append((k, w))
    cands.sort()
    ks = []
    for k, w in cands:
        if ks and abs(k - ks[-1][0]) <= 1e-9 * k:
            continue
        ks.append((k, w))
    edges = [0.0] + [k for k, _ in ks]
    tests = [(edges[i] + edges[i + 1]) / 2 for i in range(len(edges) - 1)] + [edges[-1] * 2 + 1.0]
    stable = [_routh_ok(_padd(den, _pscale(num, t))) for t in tests]
    runs = [i for i, s in enumerate(stable) if s]
    _need(len(runs) > 0, "the closed loop is unstable for every positive gain")
    _need(runs == list(range(runs[0], runs[-1] + 1)), "the stable gains form more than one interval")
    _need(runs[-1] != len(tests) - 1, "the closed loop is stable for every large gain (no upper limit)")
    lo = edges[runs[0]]
    hi_idx = runs[-1] + 1
    return _rnd({"K_min": lo, "K_max": edges[hi_idx], "w_cross": ks[hi_idx - 1][1]})


def ref_stable_gain_range(num, den):
    import numpy as np

    def f(K):
        return float(np.max(np.roots(np.polyadd(den, K * np.array(num))).real))
    Ks = np.concatenate([[1e-9], np.logspace(-5, 6, 1500)])
    vals = [f(k) for k in Ks]
    st = [v < 0 for v in vals]
    lo = 0.0
    if not st[0]:
        i = st.index(True)
        lo = _ref_crossing(f, Ks[i - 1], Ks[i])
    j = st.index(True)
    j2 = j + st[j:].index(False)
    hi = _ref_crossing(f, Ks[j2 - 1], Ks[j2])
    r = np.roots(np.polyadd(den, hi * np.array(num)))
    w = float(abs(r[int(np.argmax(r.real))].imag))
    return {"K_min": lo, "K_max": hi, "w_cross": 0.0 if w < 1e-6 else w}


def _rand_gain_range(rng):
    x = _rand_loop(rng)
    x["num"] = [_r12(v / x["num"][0]) for v in x["num"]]
    return x


_add("ctrl_stable_gain_range", name="Range of gain for closed-loop stability (Routh, j-omega crossing)",
     topic="stability / Routh-Hurwitz",
     purpose="The interval K_min < K < K_max of positive gains for which 1 + K G(s) = 0 has every root in the left half plane, and the frequency at which the root locus crosses the imaginary axis at K_max.",
     inputs=[_v("num", "numerator of G(s)"), _v("den", "denominator of G(s)", 2)],
     outputs=[_o("K_min", "lower end of the stable range (0 when stable from small gains)"), _o("K_max", "critical gain (upper end)"),
              _o("w_cross", "imaginary-axis crossing frequency at K_max (0 for a crossing at s = 0)", "angular_frequency")],
     primary="K_max", conditions=["one bounded interval of stable positive gains"],
     method="critical gains from D(jw) + K N(jw) = 0 and D(0) + K N(0) = 0; each interval tested by an exact Routh array",
     latex=r"D(j\omega) + K N(j\omega) = 0", run=stable_gain_range, reference=ref_stable_gain_range,
     wrong=lambda num, den: (lambda r: dict(r, K_max=r["K_max"] * 2.0, w_cross=r["w_cross"] * math.sqrt(2)))(stable_gain_range(num, den)),
     rand=_rand_gain_range, phrases=["stable gain range", "range of K for stability"],
     examples=[_ex({"num": [1], "den": [1, 3, 2, 0]}, {"K_min": 0.0, "K_max": 6.0, "w_cross": 1.41421356237}),
               _ex({"num": [1], "den": [1, 6, 11, 6]}, {"K_max": 60.0, "w_cross": 3.31662479036})],
     extra_q=[_bad({"num": [1], "den": [1, 1]})], level="advanced")


# ================================================================================================ root locus (advanced)
def breakaway_points(num, den):
    num, den = _tf_in(num, den)
    P = _ptrim(_psub(_pmul(num, _pder(den)), _pmul(_pder(num), den)))
    pts = []
    if len(P) >= 2:
        dP = _pder(P)
        for r in _roots_raw(P):
            r = complex(r)
            if abs(r.imag) > 1e-7 * max(1.0, abs(r)):
                continue
            x = r.real
            for _ in range(3):
                dv = _pval(dP, x)
                if dv != 0:
                    x -= _pval(P, x) / dv
            nv = _pval(num, x)
            if nv == 0:
                continue
            k = -_pval(den, x) / nv
            if k > 1e-9 * max(1.0, abs(_pval(den, x))):
                pts.append((x, k))
    pts.sort()
    return _rnd({"points": [p for p, _ in pts], "gains": [k for _, k in pts], "count": len(pts)})


def ref_breakaway_points(num, den):
    import sympy as sp
    s = sp.symbols("s")
    N = sum(sp.Rational(str(c)) * s ** (len(num) - 1 - i) for i, c in enumerate(num))
    D = sum(sp.Rational(str(c)) * s ** (len(den) - 1 - i) for i, c in enumerate(den))
    dK = sp.together(sp.diff(-D / N, s))
    numer = sp.Poly(sp.numer(dK), s)
    out = []
    for r in numer.nroots(n=30, maxsteps=200):
        r = complex(r)
        if abs(r.imag) < 1e-12:
            k = float((-D / N).subs(s, sp.Float(r.real, 30)))
            if k > 0:
                out.append((r.real, k))
    out.sort()
    return {"points": [p for p, _ in out], "gains": [k for _, k in out], "count": len(out)}


def _rand_breakaway(rng):
    a, b, c = sorted(rng.sample([0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0], 3))
    form = rng.randint(0, 2)
    if form == 0:
        return {"num": [1.0], "den": _pmul(_pmul([1.0, 0.0], [1.0, a]), [1.0, b])}
    if form == 1:
        return {"num": [1.0, b], "den": _pmul(_pmul([1.0, 0.0], [1.0, a]), [1.0, c])}
    return {"num": [1.0, c], "den": _pmul([1.0, a], [1.0, b])}


_add("ctrl_breakaway_points", name="Root-locus breakaway and break-in points", topic="root locus / breakaway points",
     purpose="Real points where root-locus branches meet and leave (or enter) the real axis: real roots of N D' - N' D = 0 with a positive gain K = -D/N there.",
     inputs=[_v("num", "numerator of G(s)"), _v("den", "denominator of G(s)", 2)],
     outputs=[_o("points", "breakaway/break-in points, ascending"), _o("gains", "gain K at each point"), _o("count", "number of points", "integer")],
     primary="count", conditions=["proper G(s)"], method="dK/ds = 0 with K = -D(s)/N(s)",
     latex=r"\frac{dK}{ds} = 0 \iff N D' - N' D = 0", run=breakaway_points, reference=ref_breakaway_points,
     wrong=lambda num, den: (lambda r: dict(r, points=[p * 0.5 for p in r["points"]], count=r["count"] + 1))(breakaway_points(num, den)),
     rand=_rand_breakaway, phrases=["breakaway points", "breakaway point"],
     examples=[_ex({"num": [1], "den": [1, 3, 2, 0]}, {"points": [-0.422649730810], "gains": [0.384900179460], "count": 1}),
               _ex({"num": [1, 3], "den": [1, 3, 2]}, {"points": [-4.41421356237, -1.58578643763], "count": 2})],
     level="advanced")


def _angle_rule(target, others_plus, others_minus):
    a = 180.0
    for z in others_plus:
        a += _deg(complex(target) - complex(z))
    for p in others_minus:
        a -= _deg(complex(target) - complex(p))
    return _wrap180(a)


def _one_pair(roots, what):
    cpx = [r for r in roots if isinstance(r, complex) and r.imag > 0]
    _need(len(cpx) == 1, f"this capability needs exactly one complex-conjugate pair of {what}")
    return cpx[0]


def departure_angle(num, den):
    num, den = _tf_in(num, den)
    _need(num[0] * den[0] > 0, "the leading coefficients must have the same sign (the K > 0 locus)")
    poles, zeros = _roots_raw(den), _roots_raw(num)
    p = _one_pair(_canon(poles), "poles")
    others = list(poles)
    k = min(range(len(others)), key=lambda i: abs(complex(others[i]) - p))
    others.pop(k)
    return _rnd({"pole": p, "departure_deg": _angle_rule(p, zeros, others)})


def ref_departure_angle(num, den):
    import numpy as np
    r = np.roots(den)
    p = complex(r[int(np.argmax(r.imag))])
    v = -np.polyval(num, p) / np.polyval(np.polyder(den), p)
    return {"pole": p, "departure_deg": math.degrees(cmath.phase(complex(v)))}


def _rand_departure(rng):
    a = round(rng.uniform(0.3, 4), 1)
    b = round(rng.uniform(0.5, 5), 1)
    den = [1.0, 2 * a, a * a + b * b]
    for _ in range(rng.randint(0, 2)):
        den = _pmul(den, [1.0, round(rng.uniform(0, 8), 1)])
    num = [1.0]
    if rng.random() < 0.6:
        num = [1.0, round(rng.uniform(0.2, 9), 1)]
    return {"num": num, "den": [_r12(x) for x in den]}


_add("ctrl_departure_angle", name="Root-locus angle of departure from a complex pole", topic="root locus / departure and arrival angles",
     purpose="The angle at which the root locus leaves the complex pole p (upper half plane): 180 + sum of angles from the zeros - sum of angles from the other poles.",
     inputs=[_v("num", "numerator of G(s)"), _v("den", "denominator of G(s) (exactly one complex pole pair)", 3)],
     outputs=[_o("pole", "the complex pole (positive imaginary part)"), _o("departure_deg", "angle of departure in degrees, (-180, 180]")],
     primary="departure_deg", conditions=["exactly one complex pole pair", "same-sign leading coefficients"],
     method="angle criterion at the pole", latex=r"\theta_d = 180^\circ + \sum\angle(p - z_i) - \sum_{j\ne}\angle(p - p_j)",
     run=departure_angle, reference=ref_departure_angle,
     wrong=lambda num, den: (lambda r: dict(r, departure_deg=_wrap180(-r["departure_deg"] + 90.0)))(departure_angle(num, den)),
     rand=_rand_departure, phrases=["angle of departure", "departure angle"],
     examples=[_ex({"num": [1], "den": [1, 2, 2, 0]}, {"pole": complex(-1, 1), "departure_deg": -45.0}),
               _ex({"num": [1, 2], "den": [1, 2, 5]}, {"departure_deg": 153.434948823})],
     extra_q=[_bad({"num": [1], "den": [1, 3, 2]})], level="advanced")


def arrival_angle(num, den):
    num, den = _tf_in(num, den, proper=False)
    _need(num[0] * den[0] > 0, "the leading coefficients must have the same sign (the K > 0 locus)")
    poles, zeros = _roots_raw(den), _roots_raw(num)
    z = _one_pair(_canon(zeros), "zeros")
    others = list(zeros)
    k = min(range(len(others)), key=lambda i: abs(complex(others[i]) - z))
    others.pop(k)
    return _rnd({"zero": z, "arrival_deg": _angle_rule(z, poles, others)})


def ref_arrival_angle(num, den):
    import numpy as np
    r = np.roots(num)
    z = complex(r[int(np.argmax(r.imag))])
    v = -np.polyval(den, z) / np.polyval(np.polyder(num), z)
    return {"zero": z, "arrival_deg": math.degrees(cmath.phase(complex(v)))}


def _rand_arrival(rng):
    a = round(rng.uniform(0.3, 4), 1)
    b = round(rng.uniform(0.5, 5), 1)
    num = [1.0, 2 * a, a * a + b * b]
    den = [1.0, 0.0]
    for _ in range(rng.randint(1, 3)):
        den = _pmul(den, [1.0, round(rng.uniform(0.5, 8), 1)])
    return {"num": [_r12(x) for x in num], "den": [_r12(x) for x in den]}


_add("ctrl_arrival_angle", name="Root-locus angle of arrival at a complex zero", topic="root locus / departure and arrival angles",
     purpose="The angle at which the root locus arrives at the complex zero z (upper half plane): 180 + sum of angles from the poles - sum of angles from the other zeros.",
     inputs=[_v("num", "numerator of G(s) (exactly one complex zero pair)", 3), _v("den", "denominator of G(s)", 2)],
     outputs=[_o("zero", "the complex zero (positive imaginary part)"), _o("arrival_deg", "angle of arrival in degrees, (-180, 180]")],
     primary="arrival_deg", conditions=["exactly one complex zero pair", "same-sign leading coefficients"],
     method="angle criterion at the zero", latex=r"\theta_a = 180^\circ + \sum\angle(z - p_j) - \sum_{i\ne}\angle(z - z_i)",
     run=arrival_angle, reference=ref_arrival_angle,
     wrong=lambda num, den: (lambda r: dict(r, arrival_deg=_wrap180(-r["arrival_deg"] + 90.0)))(arrival_angle(num, den)),
     rand=_rand_arrival, phrases=["angle of arrival", "arrival angle"],
     examples=[_ex({"num": [1, 2, 2], "den": [1, 0]}, {"zero": complex(-1, 1), "arrival_deg": -135.0}),
               _ex({"num": [1, 2, 5], "den": [1, 3, 0]}, {"arrival_deg": -108.434948823})],
     extra_q=[_bad({"num": [1, 3, 2], "den": [1, 2, 0]})], level="advanced")


# ================================================================================================ pole placement
def state_feedback_poles(A, B, K):
    A, B = _ss_in(A, B)
    n = len(A)
    K = _vec(K, "K", n, n)
    Af, Bf, Kf = _frac(A), _frac(B), _frac(K)
    Acl = [[Af[i][j] - Bf[i] * Kf[j] for j in range(n)] for i in range(n)]
    c, _ = _faddeev(Acl)
    cf = [float(x) for x in c]
    poles = _roots(cf)
    mr = max(complex(p).real for p in poles)
    return {"char_poly": [_r12(x) for x in cf], "poles": poles, "stable": mr < 0}


def ref_state_feedback_poles(A, B, K):
    import numpy as np
    Acl = np.array(A, float) - np.outer(B, K)
    p = _ref_canon(np.linalg.eigvals(Acl))
    return {"char_poly": _ref_snap_poly(list(np.poly(Acl).real)), "poles": p, "stable": max(complex(x).real for x in p) < 0}


def _rand_sfb(rng):
    for _ in range(100):
        x = _rand_ss(rng, None, False)
        n = len(x["A"])
        K = [float(rng.randint(-5, 9)) for _ in range(n)]
        r = _roots_raw(state_feedback_poles(x["A"], x["B"], K)["char_poly"])
        sep = min((abs(complex(r[i]) - complex(r[j])) for i in range(n) for j in range(i + 1, n)), default=1)
        if sep > 0.05:
            return {"A": x["A"], "B": x["B"], "K": K}
    return {"A": [[0.0, 1.0], [-2.0, -3.0]], "B": [0.0, 1.0], "K": [1.0, 1.0]}


_add("ctrl_state_feedback_poles", name="Closed-loop poles under state feedback u = -K x", topic="state space / state feedback",
     purpose="The characteristic polynomial det(sI - (A - B K)) (exact), the closed-loop poles and stability for a state-feedback gain K.",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector"), _v("K", "state-feedback gain (row, n entries)")],
     outputs=[_o("char_poly", "closed-loop characteristic polynomial"), _o("poles", "closed-loop poles"),
              _o("stable", "true when every pole is in the left half plane", "boolean")],
     primary="char_poly", conditions=["single input"], method="Faddeev-LeVerrier on A - B K",
     latex=r"\det(sI - A + BK) = 0", run=state_feedback_poles, reference=ref_state_feedback_poles,
     wrong=lambda A, B, K: state_feedback_poles(A, B, [-k for k in K]), rand=_rand_sfb, phrases=["state feedback poles"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "B": [0, 1], "K": [4, 4]}, {"char_poly": [1.0, 4.0, 4.0], "poles": [-2.0, -2.0], "stable": True}),
               _ex({"A": [[0, 1], [-2, -3]], "B": [0, 1], "K": [3, 1]}, {"char_poly": [1.0, 4.0, 5.0]})],
     level="advanced")


def _acker(A, B, alpha):
    """Ackermann's formula in exact fractions: K = e_n^T Wc^{-1} alpha(A)."""
    n = len(A)
    Af, Bf = _frac(A), _frac(B)
    al = [Fraction(x) for x in alpha]
    _need(len(al) == n + 1, "the desired characteristic polynomial must have degree n")
    _need(al[0] != 0, "the desired polynomial's leading coefficient must not be zero")
    al = [x / al[0] for x in al]
    W = _ctrb(Af, Bf)
    _need(_rank(W) == n, "(A, B) is not controllable: the poles cannot all be placed")
    M = _eye(n, Fraction(1))
    I = _eye(n, Fraction(1))
    for k in range(1, n + 1):                       # Horner: M = A M + alpha_k I
        M = _madd(_mmul(Af, M), _mscale(I, al[k]))
    e = [Fraction(0)] * (n - 1) + [Fraction(1)]
    v = _solve(_mT(W), e, exact=True)               # v^T = e_n^T W^{-1}
    return [sum(v[i] * M[i][j] for i in range(n)) for j in range(n)]


def ackermann(A, B, alpha):
    A, B = _ss_in(A, B)
    alpha = _vec(alpha, "alpha", 2, 7)
    return {"K": [_r12(float(x)) for x in _acker(A, B, alpha)]}


def ref_ackermann(A, B, alpha):
    import numpy as np
    from scipy.signal import place_poles
    poles = np.roots(alpha)
    return {"K": [float(x) for x in place_poles(np.array(A, float), np.array(B, float).reshape(-1, 1), poles).gain_matrix[0]]}


def _rand_placement(rng):
    for _ in range(100):
        x = _rand_ss(rng, rng.randint(2, 4), False)
        n = len(x["A"])
        if _rank(_ctrb(_frac(x["A"]), _frac(x["B"]))) < n:
            continue
        roots = rng.sample([1.0, 1.5, 2.0, 3.0, 4.0, 5.0, 6.0], n)
        alpha = [1.0]
        if n >= 2 and rng.random() < 0.5:
            a, b = rng.choice([1.0, 2.0, 3.0]), rng.choice([1.0, 2.0, 3.0])
            alpha = _pmul(alpha, [1.0, 2 * a, a * a + b * b])
            roots = roots[:n - 2]
        for r in roots:
            alpha = _pmul(alpha, [1.0, r])
        return {"A": x["A"], "B": x["B"], "alpha": alpha}
    return {"A": [[0.0, 1.0], [-2.0, -3.0]], "B": [0.0, 1.0], "alpha": [1.0, 4.0, 4.0]}


_add("ctrl_ackermann", name="Pole placement by Ackermann's formula", topic="state space / pole placement",
     purpose="The state-feedback gain K that gives A - B K the desired characteristic polynomial alpha(s) (single input), computed exactly.",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector"), _v("alpha", "desired characteristic polynomial (n+1 coefficients, highest power first)", 2, 7)],
     outputs=[_o("K", "state-feedback gain (row)")], primary="K", conditions=["(A, B) controllable"],
     method="K = [0 ... 0 1] Wc^{-1} alpha(A) in exact fractions", latex=r"K = \begin{bmatrix}0 & \cdots & 0 & 1\end{bmatrix}\mathcal{C}^{-1}\alpha(A)",
     run=ackermann, reference=ref_ackermann,
     wrong=lambda A, B, alpha: {"K": [_r12(float(x)) for x in _acker(_mT(A), B, alpha)]} if _mT(A) != A else {"K": [0.0] * len(A)},
     rand=_rand_placement, phrases=["Ackermann pole placement", "Ackermann's formula"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "B": [0, 1], "alpha": [1, 4, 4]}, {"K": [4.0, 4.0]}),
               _ex({"A": [[0, 1], [-2, -3]], "B": [0, 1], "alpha": [1, 6, 25]}, {"K": [23.0, 3.0]})],
     extra_q=[_bad({"A": [[-1, 0], [0, -2]], "B": [1, 0], "alpha": [1, 4, 4]})], level="advanced")


def observer_gain(A, C, alpha):
    A, C = _ss_in(A, None, C)
    alpha = _vec(alpha, "alpha", 2, 7)
    return {"L": [_r12(float(x)) for x in _acker(_mT(A), C, alpha)]}


def ref_observer_gain(A, C, alpha):
    import numpy as np
    from scipy.signal import place_poles
    g = place_poles(np.array(A, float).T, np.array(C, float).reshape(-1, 1), np.roots(alpha)).gain_matrix
    return {"L": [float(x) for x in g[0]]}


def _rand_observer(rng):
    x = _rand_placement(rng)
    return {"A": _mT(x["A"]), "C": x["B"], "alpha": x["alpha"]}


_add("ctrl_observer_gain", name="Observer (estimator) gain by pole placement", topic="state space / observers",
     purpose="The observer gain L that gives A - L C the desired characteristic polynomial (single output), by duality with Ackermann's formula.",
     inputs=[_m("A", "state matrix (n x n)"), _v("C", "output vector"), _v("alpha", "desired observer characteristic polynomial (n+1 coefficients)", 2, 7)],
     outputs=[_o("L", "observer gain (column)")], primary="L", conditions=["(A, C) observable"],
     method="L^T = Ackermann(A^T, C^T, alpha)", latex=r"L = \alpha(A)\,\mathcal{O}^{-1}\begin{bmatrix}0\\ \vdots\\ 1\end{bmatrix}",
     run=observer_gain, reference=ref_observer_gain,
     wrong=lambda A, C, alpha: {"L": [_r12(float(x)) for x in _acker(A, C, alpha)]} if _mT(A) != A else {"L": [0.0] * len(A)},
     rand=_rand_observer, phrases=["observer gain", "estimator gain"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "C": [1, 0], "alpha": [1, 10, 25]}, {"L": [10.0, 25.0]}),
               _ex({"A": [[0, 1], [-2, -3]], "C": [1, 0], "alpha": [1, 12, 36]}, {"L": [9.0, 7.0]})],
     extra_q=[_bad({"A": [[-1, 0], [0, -2]], "C": [1, 0], "alpha": [1, 4, 4]})], level="advanced")


def deadbeat_gain(A, B):
    A, B = _ss_in(A, B)
    n = len(A)
    return {"K": [_r12(float(x)) for x in _acker(A, B, [1.0] + [0.0] * n)]}


def ref_deadbeat_gain(A, B):
    import sympy as sp
    n = len(A)
    ks = sp.symbols(f"k0:{n}")
    Am = sp.Matrix([[sp.Rational(str(x)) for x in r] for r in A])
    Bm = sp.Matrix([sp.Rational(str(x)) for x in B])
    z = sp.symbols("z")
    cp = (Am - Bm * sp.Matrix([ks])).charpoly(z).all_coeffs()
    sol = sp.solve(cp[1:], ks, dict=True)[0]
    return {"K": [float(sol[k]) for k in ks]}


def _rand_deadbeat(rng):
    for _ in range(100):
        x = _rand_ss(rng, rng.randint(2, 4), False)
        if _rank(_ctrb(_frac(x["A"]), _frac(x["B"]))) == len(x["A"]):
            return {"A": x["A"], "B": x["B"]}
    return {"A": [[1.0, 1.0], [0.0, 1.0]], "B": [0.0, 1.0]}


_add("ctrl_deadbeat_gain", name="Deadbeat state feedback (discrete time)", topic="state space / pole placement",
     purpose="The gain K that places every eigenvalue of A - B K at z = 0, so the discrete closed loop reaches the origin in at most n steps.",
     inputs=[_m("A", "discrete state matrix (n x n)"), _v("B", "input vector")],
     outputs=[_o("K", "deadbeat state-feedback gain")], primary="K", conditions=["(A, B) controllable"],
     method="Ackermann's formula with alpha(z) = z^n (exact)", latex=r"K = [0\cdots 1]\,\mathcal{C}^{-1}A^n",
     run=deadbeat_gain, reference=ref_deadbeat_gain,
     wrong=lambda A, B: {"K": [_r12(float(x)) for x in _acker(A, B, [1.0] + [0.0] * (len(A) - 1) + [1.0])]},
     rand=_rand_deadbeat, phrases=["deadbeat gain", "deadbeat control"],
     examples=[_ex({"A": [[1, 1], [0, 1]], "B": [0, 1]}, {"K": [1.0, 2.0]}),
               _ex({"A": [[0, 1], [-0.5, 1.5]], "B": [0, 1]}, {"K": [-0.5, 1.5]})],
     extra_q=[_bad({"A": [[1, 0], [0, 2]], "B": [1, 0]})], level="advanced")


# ================================================================================================ matrix exponential and discretization
def _mexp_out(E):
    return _rnd(_snapm(E))


def state_transition(A, t):
    (A,) = _ss_in(A)
    t = _num(t, "t")
    _need(_mnorm1(A) * abs(t) <= 40, "|A| t is too large for an accurate matrix exponential here (limit 40)")
    return {"Phi": _mexp_out(_expm(_mscale(A, t)))}


def ref_state_transition(A, t):
    import numpy as np
    from scipy.linalg import expm
    E = expm(np.array(A, float) * t)
    return {"Phi": _rnd(_snapm([[float(x) for x in r] for r in E]))}


_add("ctrl_state_transition", name="State-transition matrix e^{At}", topic="state space / time response",
     purpose="The state-transition matrix Phi(t) = e^{At} that maps x(0) to x(t) for x' = A x.",
     inputs=[_m("A", "state matrix (n x n)"), _n("t", "time in seconds")],
     outputs=[_o("Phi", "state-transition matrix")], primary="Phi", conditions=["|A| t <= 40"],
     method="scaling and squaring with a Taylor series", latex=r"\Phi(t) = e^{At} = \sum_k \frac{(At)^k}{k!}",
     run=state_transition, reference=ref_state_transition,
     wrong=lambda A, t: {"Phi": _mexp_out(_madd(_eye(len(A)), _mscale(A, t)))},
     rand=lambda rng: {"A": [[float(rng.randint(-3, 2)) for _ in range(n)] for n in [rng.randint(2, 4)] for _ in range(n)],
                       "t": round(rng.uniform(0.1, 1.5), 2)},
     phrases=["state transition matrix", "state-transition matrix"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "t": 2}, {"Phi": [[1.0, 2.0], [0.0, 1.0]]}),
               _ex({"A": [[-1, 0], [0, -2]], "t": 1}, {"Phi": [[0.367879441171, 0.0], [0.0, 0.135335283237]]})],
     extra_q=[_bad({"A": [[100, 0], [0, 1]], "t": 10})], level="advanced")


def ss_response(A, B, x0, u, t):
    A, B = _ss_in(A, B)
    n = len(A)
    x0 = _vec(x0, "x0", n, n)
    u, t = _num(u, "u"), _nonneg(t, "t")
    _need(_mnorm1(A) * t <= 40, "|A| t is too large for an accurate matrix exponential here (limit 40)")
    M = [[A[i][j] * t for j in range(n)] + [B[i] * u * t] for i in range(n)] + [[0.0] * (n + 1)]
    E = _expm(M)
    x = [sum(E[i][j] * x0[j] for j in range(n)) + E[i][n] for i in range(n)]
    return {"x": _rnd(_snapm(x))}


def ref_ss_response(A, B, x0, u, t):
    import numpy as np
    from scipy.integrate import solve_ivp
    Am, Bm = np.array(A, float), np.array(B, float)
    sol = solve_ivp(lambda s, x: Am @ x + Bm * u, (0, t), np.array(x0, float), method="DOP853", rtol=1e-13,
                    atol=1e-15 * max(1.0, float(np.max(np.abs(x0))), abs(u)))
    return {"x": _rnd(_snapm([float(v) for v in sol.y[:, -1]]))}


def _rand_ssresp(rng):
    for _ in range(100):
        x = _rand_eig(rng)
        A = x["A"]
        c, _ = _faddeev(_frac(A))
        if max(complex(r).real for r in _roots([float(v) for v in c])) > 0.3:
            continue
        n = len(A)
        return {"A": A, "B": [float(rng.randint(-2, 3)) for _ in range(n)], "x0": [float(rng.randint(-3, 3)) for _ in range(n)],
                "u": round(rng.uniform(-3, 3), 2), "t": round(rng.uniform(0.2, 2), 2)}
    return {"A": [[0.0, 1.0], [-2.0, -3.0]], "B": [0.0, 1.0], "x0": [1.0, 0.0], "u": 1.0, "t": 1.0}


_add("ctrl_ss_response", name="State response to an initial state and a constant input", topic="state space / time response",
     purpose="x(t) = e^{At} x(0) + (integral of e^{A tau} d tau) B u for x' = A x + B u with a constant input u.",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector"), _v("x0", "initial state"), _n("u", "constant input"), _n("t", "time in seconds")],
     outputs=[_o("x", "state at time t")], primary="x", conditions=["|A| t <= 40", "t >= 0"],
     method="exponential of the augmented matrix [[A, B u], [0, 0]] t", latex=r"x(t) = e^{At}x_0 + \int_0^t e^{A\tau}d\tau\,Bu",
     run=ss_response, reference=ref_ss_response,
     wrong=lambda A, B, x0, u, t: {"x": _rnd(_snapm([a + b for a, b in zip(_mv(_expm(_mscale(A, t)), x0), [bb * u * t for bb in B])]))},
     rand=_rand_ssresp, phrases=["state response"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "B": [0, 1], "x0": [0, 0], "u": 2, "t": 3}, {"x": [9.0, 6.0]}),
               _ex({"A": [[-1]], "B": [1], "x0": [2], "u": 1, "t": 1}, {"x": [1.36787944117]})],
     extra_q=[_bad({"A": [[0, 1], [0, 0]], "B": [0, 1], "x0": [0, 0], "u": 2, "t": -3})], level="advanced")


def _zoh_ss(num, den, T):
    A, B, C, D = _ccf(num, den)
    n = len(A)
    M = [[A[i][j] * T for j in range(n)] + [B[i] * T] for i in range(n)] + [[0.0] * (n + 1)]
    E = _expm(M)
    return [r[:n] for r in E[:n]], [E[i][n] for i in range(n)], C, D


def c2d_zoh(num, den, T):
    num, den, T = _disc_tf_in(num, den, T)
    _need(max(abs(complex(p)) for p in _roots(den)) * T <= 20, "|pole| T is too large (limit 20)")
    Ad, Bd, C, D = _zoh_ss(num, den, T)
    nz, dz = _ss2tf(Ad, Bd, C, D)
    return _disc_out(nz, dz)


def _ref_mp_tf(Ad, Bd, Cd, Dd):
    """Transfer function of a discrete state-space model in high precision (sympy charpoly and adjugate)."""
    import sympy as sp
    z = sp.symbols("z")
    n = len(Ad)
    Am = sp.Matrix(n, n, lambda i, j: sp.Float(str(Ad[i][j]), 40))
    zI = sp.eye(n) * z - Am
    den = sp.Poly(zI.det(method="berkowitz"), z)
    adj = zI.adjugate(method="berkowitz")
    num = sp.expand((sp.Matrix([[sp.Float(str(c), 40) for c in Cd]]) * adj * sp.Matrix([sp.Float(str(b), 40) for b in Bd]))[0, 0]
                    + sp.Float(str(Dd), 40) * den.as_expr())
    dc = [sp.Float(c, 40) for c in den.all_coeffs()]
    nc = [sp.Float(c, 40) for c in sp.Poly(num, z).all_coeffs()] if num != 0 else [sp.Float(0)]
    nc = [sp.Float(0)] * (len(dc) - len(nc)) + nc
    big = max(abs(c) for c in nc) or 1
    return {"numz": [0.0 if abs(c) < 1e-25 * big else float(c / dc[0]) for c in nc], "denz": [float(c / dc[0]) for c in dc]}


def _ref_mp_expm(M):
    import mpmath
    mpmath.mp.dps = 40
    E = mpmath.expm(mpmath.matrix([[mpmath.mpf(str(x)) for x in r] for r in M]))
    return [[E[i, j] for j in range(len(M))] for i in range(len(M))]


def ref_c2d_zoh(num, den, T):
    from scipy.signal import tf2ss
    a, b, c, d = tf2ss(num, den)
    n = a.shape[0]
    M = [[float(a[i, j]) * T for j in range(n)] + [float(b[i, 0]) * T] for i in range(n)] + [[0.0] * (n + 1)]
    E = _ref_mp_expm(M)
    return _ref_mp_tf([r[:n] for r in E[:n]], [E[i][n] for i in range(n)], [float(x) for x in c[0]], float(d[0, 0]))


def _rand_c2d(rng):
    x = _rand_stable_tf(rng, 3)
    x["T"] = round(rng.uniform(0.02, 0.6), 3)
    return x


_add("ctrl_c2d_zoh", name="Zero-order-hold discretization of a transfer function", topic="discretization / ZOH",
     purpose="The step-invariant (ZOH) discrete equivalent G(z) = (1 - z^{-1}) Z{G(s)/s} of a proper G(s) sampled every T seconds.",
     inputs=[_v("num", "numerator in s"), _v("den", "denominator in s", 2), _n("T", "sampling period in seconds")],
     outputs=[_o("numz", "numerator in z (padded, normalised)"), _o("denz", "denominator in z (leading coefficient 1)")],
     primary="denz", conditions=["proper G(s)", "T > 0"], method="exponential of [[A, B], [0, 0]] T in state space, then Faddeev-LeVerrier",
     latex=r"A_d = e^{AT},\; B_d = \int_0^T e^{A\tau}d\tau\,B", run=c2d_zoh, reference=ref_c2d_zoh,
     wrong=lambda num, den, T: tustin(num, den, T), rand=_rand_c2d, phrases=["ZOH discretization", "zero-order hold discretization"],
     examples=[_ex({"num": [1], "den": [1, 1], "T": 0.5}, {"numz": [0.0, 0.393469340287], "denz": [1.0, -0.606530659713]}),
               _ex({"num": [1], "den": [1, 0, 0], "T": 1}, {"numz": [0.0, 0.5, 0.5], "denz": [1.0, -2.0, 1.0]})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "T": 0})], level="advanced")


def c2d_foh(num, den, T):
    num, den, T = _disc_tf_in(num, den, T)
    _need(max(abs(complex(p)) for p in _roots(den)) * T <= 20, "|pole| T is too large (limit 20)")
    A, B, C, D = _ccf(num, den)
    n = len(A)
    M = [[0.0] * (n + 2) for _ in range(n + 2)]
    for i in range(n):
        for j in range(n):
            M[i][j] = A[i][j] * T
        M[i][n] = B[i] * T
    M[n][n + 1] = 1.0
    E = _expm(M)
    Phi = [r[:n] for r in E[:n]]
    G1 = [E[i][n] for i in range(n)]
    G2 = [E[i][n + 1] for i in range(n)]
    PG2 = _mv(Phi, G2)
    Bd = [G1[i] - G2[i] + PG2[i] for i in range(n)]
    Dd = D + sum(C[i] * G2[i] for i in range(n))
    nz, dz = _ss2tf(Phi, Bd, C, Dd)
    return _disc_out(nz, dz)


def ref_c2d_foh(num, den, T):
    import mpmath
    from scipy.signal import tf2ss
    a, b, c, d = tf2ss(num, den)
    n = a.shape[0]
    M = [[0.0] * (n + 2) for _ in range(n + 2)]
    for i in range(n):
        for j in range(n):
            M[i][j] = float(a[i, j]) * T
        M[i][n] = float(b[i, 0]) * T
    M[n][n + 1] = 1.0
    E = _ref_mp_expm(M)
    Phi = mpmath.matrix([[E[i][j] for j in range(n)] for i in range(n)])
    G1 = mpmath.matrix([E[i][n] for i in range(n)])
    G2 = mpmath.matrix([E[i][n + 1] for i in range(n)])
    Bd = G1 - G2 + Phi * G2
    Dd = float(d[0, 0]) + sum(float(c[0, i]) * G2[i] for i in range(n))
    return _ref_mp_tf([[Phi[i, j] for j in range(n)] for i in range(n)], [Bd[i] for i in range(n)], [float(x) for x in c[0]], Dd)


_add("ctrl_c2d_foh", name="First-order-hold (triangle-hold) discretization", topic="discretization / FOH",
     purpose="The ramp-invariant (triangle-hold, non-causal first-order hold) discrete equivalent of a proper G(s) sampled every T seconds.",
     inputs=[_v("num", "numerator in s"), _v("den", "denominator in s", 2), _n("T", "sampling period in seconds")],
     outputs=[_o("numz", "numerator in z (padded, normalised)"), _o("denz", "denominator in z (leading coefficient 1)")],
     primary="numz", conditions=["proper G(s)", "T > 0"], method="exponential of the augmented matrix [[A T, B T, 0], [0, 0, 1], [0, 0, 0]]",
     latex=r"B_d = \Gamma_1 - \Gamma_2 + \Phi\Gamma_2,\quad D_d = D + C\Gamma_2", run=c2d_foh, reference=ref_c2d_foh,
     wrong=lambda num, den, T: c2d_zoh(num, den, T), rand=_rand_c2d, phrases=["FOH discretization", "first-order hold discretization"],
     examples=[_ex({"num": [1], "den": [1, 0], "T": 1}, {"numz": [0.5, 0.5], "denz": [1.0, -1.0]}),
               _ex({"num": [1], "den": [1, 1], "T": 1}, {"denz": [1.0, -0.367879441171], "numz": [0.367879441171, 0.264241117657]})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "T": -1})], level="advanced",
     weak="the same triangle-hold construction (scipy's) recomputed in 40-digit arithmetic")


def tustin_prewarp(num, den, T, w):
    num, den, T = _disc_tf_in(num, den, T)
    w = _pos(w, "w")
    _need(w * T < PI, "the prewarping frequency must be below the Nyquist frequency (w T < pi)")
    n = len(den) - 1
    c = w / math.tan(w * T / 2)
    return _disc_out(_subst(num, n, c, [1.0, -1.0], [1.0, 1.0]), _subst(den, n, c, [1.0, -1.0], [1.0, 1.0]))


def ref_tustin_prewarp(num, den, T, w):
    import mpmath
    import sympy as sp
    mpmath.mp.dps = 40
    s, z = sp.symbols("s z")
    c = sp.Float(mpmath.mpf(str(w)) / mpmath.tan(mpmath.mpf(str(w)) * mpmath.mpf(str(T)) / 2), 40)
    N = sum(sp.Rational(str(cc)) * s ** (len(num) - 1 - i) for i, cc in enumerate(num))
    D = sum(sp.Rational(str(cc)) * s ** (len(den) - 1 - i) for i, cc in enumerate(den))
    n = len(_ptrim(den)) - 1
    def sub(P):
        cs = sp.Poly(P, s).all_coeffs()
        deg = len(cs) - 1
        return sp.Poly(sp.expand(sum(a * c ** (deg - i) * (z - 1) ** (deg - i) * (z + 1) ** (n - deg + i)
                                     for i, a in enumerate(cs))), z)
    Nz, Dz = sub(N), sub(D)
    dc, nc = Dz.all_coeffs(), Nz.all_coeffs()
    nc = [0] * (len(dc) - len(nc)) + nc
    return {"numz": [float(x / dc[0]) for x in nc], "denz": [float(x / dc[0]) for x in dc]}


_add("ctrl_tustin_prewarp", name="Tustin discretization with frequency prewarping", topic="discretization / Tustin",
     purpose="The bilinear transform s = (w/tan(w T/2)) (z - 1)/(z + 1), which matches the continuous frequency response exactly at the prewarping frequency w.",
     inputs=[_v("num", "numerator in s"), _v("den", "denominator in s", 2), _n("T", "sampling period in seconds"),
             _n("w", "prewarping frequency in rad/s")],
     outputs=[_o("numz", "numerator in z (padded, normalised)"), _o("denz", "denominator in z (leading coefficient 1)")],
     primary="denz", conditions=["proper G(s)", "0 < w T < pi"], method="polynomial substitution with the prewarped constant",
     latex=r"s \leftarrow \frac{\omega}{\tan(\omega T/2)}\,\frac{z-1}{z+1}", run=tustin_prewarp, reference=ref_tustin_prewarp,
     wrong=lambda num, den, T, w: tustin(num, den, T), rand=lambda rng: dict(_rand_disc(rng), w=round(rng.uniform(0.5, 5), 2)),
     phrases=["prewarped Tustin discretization", "Tustin discretization with prewarping"],
     examples=[_ex({"num": [1], "den": [1, 1], "T": 1, "w": 1}, {"denz": [1.0, -0.293407993026], "numz": [0.353296003487, 0.353296003487]}),
               _ex({"num": [1], "den": [1, 0], "T": 0.5, "w": 2}, {"numz": [0.273151244922, 0.273151244922]})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "T": 1, "w": 4})], level="advanced")


def jury_test(coefficients):
    c = _ptrim(_vec(coefficients, "coefficients", 2, 12))
    _need(len(c) >= 2, "give a polynomial of degree 1 or more")
    f = [Fraction(x) for x in c]
    if f[0] < 0:
        f = [-x for x in f]
    n = len(f) - 1
    P1 = sum(f)
    Pm1 = sum(x * (-1) ** (n - i) for i, x in enumerate(f)) * (-1) ** n
    q = list(f)
    stable = True
    while len(q) >= 2:
        a0, an = q[0], q[-1]
        _need(abs(an) != abs(a0), "the Jury table is singular (|a_n| = |a_0| at a step): a root lies on the unit circle or the test is degenerate")
        if abs(an) > abs(a0):
            stable = False
            break
        k = an / a0
        rev = q[::-1]
        q = [q[i] - k * rev[i] for i in range(len(q))][:-1]
    return {"stable": stable, "P1": _r12(float(P1)), "Pm1": _r12(float(Pm1))}


def ref_jury_test(coefficients):
    import numpy as np
    c = _ptrim(coefficients)
    if c[0] < 0:
        c = [-x for x in c]
    n = len(c) - 1
    return {"stable": bool(np.max(np.abs(np.roots(c))) < 1), "P1": float(np.polyval(c, 1.0)), "Pm1": float((-1) ** n * np.polyval(c, -1.0))}


def _rand_jury(rng):
    for _ in range(100):
        n = rng.randint(1, 5)
        p = [1.0]
        k = 0
        while k < n:
            if n - k >= 2 and rng.random() < 0.4:
                r, th = rng.uniform(0.2, 1.4), rng.uniform(0.3, 2.8)
                if abs(r - 1) < 0.05:
                    continue
                p = _pmul(p, [1.0, -2 * r * math.cos(th), r * r])
                k += 2
            else:
                r = rng.uniform(-1.5, 1.5)
                if abs(abs(r) - 1) < 0.05:
                    continue
                p = _pmul(p, [1.0, -r])
                k += 1
        p = [round(x, 3) for x in p]
        r = _roots_raw(p)
        if min(abs(abs(complex(v)) - 1) for v in r) > 0.02:
            return {"coefficients": p}
    return {"coefficients": [1.0, -0.5]}


_add("ctrl_jury_test", name="Jury (Schur-Cohn) stability test for a discrete polynomial", topic="stability / discrete",
     purpose="Whether every root of a polynomial in z lies inside the unit circle (Jury/Schur-Cohn table in exact arithmetic), with the necessary-condition values P(1) and (-1)^n P(-1).",
     inputs=[_v("coefficients", "polynomial coefficients in z, highest power first", 2, 12)],
     outputs=[_o("stable", "true when every root is inside the unit circle", "boolean"), _o("P1", "P(1) (must be > 0)"),
              _o("Pm1", "(-1)^n P(-1) (must be > 0)")],
     primary="P1", conditions=["the table is not singular"], method="Schur-Cohn reduction q <- q - (a_n/a_0) q_reversed in exact fractions",
     latex=r"|a_n| < a_0,\; P(1) > 0,\; (-1)^nP(-1) > 0", run=jury_test, reference=ref_jury_test,
     wrong=lambda coefficients: (lambda r: dict(r, stable=not r["stable"], P1=-r["P1"]))(jury_test(coefficients)),
     rand=_rand_jury, phrases=["Jury test", "Jury stability test"],
     examples=[_ex({"coefficients": [1, -1.2, 0.35]}, {"stable": True, "P1": 0.15, "Pm1": 2.55}),
               _ex({"coefficients": [1, -2, 1.5]}, {"stable": False, "P1": 0.5})],
     extra_q=[_bad({"coefficients": [1, 0, 1]})], level="advanced")


def step_response(num, den, t, A=1.0):
    num, den = _tf_in(num, den)
    t, A = _nonneg(t, "t"), _num(A, "A")
    _need(max(abs(complex(p)) for p in _roots(den)) * t <= 40, "|pole| t is too large (limit 40)")
    Am, B, C, D = _ccf(num, den)
    n = len(Am)
    M = [[Am[i][j] * t for j in range(n)] + [B[i] * t] for i in range(n)] + [[0.0] * (n + 1)]
    E = _expm(M)
    y = sum(C[i] * E[i][n] for i in range(n)) + D
    return _rnd({"y": A * y})


def ref_step_response(num, den, t, A=1.0):
    from scipy.signal import tf2ss
    a, b, c, d = tf2ss(num, den)
    n = a.shape[0]
    M = [[float(a[i, j]) * t for j in range(n)] + [float(b[i, 0]) * t] for i in range(n)] + [[0.0] * (n + 1)]
    E = _ref_mp_expm(M)
    return {"y": float(A * (sum(float(c[0, i]) * E[i][n] for i in range(n)) + float(d[0, 0])))}


def _rand_step_general(rng):
    for _ in range(50):
        x = _rand_stable_tf(rng, 4)
        x["t"] = round(rng.uniform(0.2, 4), 2)
        x["A"] = round(rng.uniform(0.5, 5), 1)
        if abs(step_response(**x)["y"]) > 1e-3:
            return x
    return {"num": [1.0], "den": [1.0, 1.0], "t": 1.0}


_add("ctrl_step_response", name="Step response of a transfer function at a given time", topic="time response / general",
     purpose="y(t) of any proper G(s) after a step of size A, computed exactly through the state-space matrix exponential.",
     inputs=[_v("num", "numerator"), _v("den", "denominator", 2), _n("t", "time in seconds"), _n("A", "step size (default 1)", required=False)],
     outputs=[_o("y", "output at time t")], primary="y", conditions=["proper G(s)", "t >= 0"],
     method="controllable canonical form; exponential of [[A, B], [0, 0]] t", latex=r"y(t) = C\int_0^t e^{A\tau}d\tau\,B\,A_u + D A_u",
     run=step_response, reference=ref_step_response,
     wrong=lambda num, den, t, A=1.0: {"y": A * (1 - math.exp(-t)) * num[-1] / den[-1]},
     rand=_rand_step_general, phrases=["step response at time"],
     examples=[_ex({"num": [1], "den": [1, 1], "t": 1}, {"y": 0.632120558829}),
               _ex({"num": [4], "den": [1, 2, 4], "t": 1.0}, {"y": 0.849425634854})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "t": -1})], level="advanced")


def true_settling_time(zeta, wn, band=0.02):
    zeta, wn, band = _num(zeta, "zeta"), _pos(wn, "wn"), _num(band, "band")
    _need(0 < zeta < 1, "this capability is for an underdamped system: 0 < zeta < 1")
    _need(0 < band < 0.5, "the band must lie between 0 and 50 %")
    s = zeta * wn
    wd = wn * math.sqrt(1 - zeta * zeta)
    step = PI / wd
    k = int(math.floor(math.log(1.0 / band) / (s * step)))
    while k > 0 and math.exp(-s * k * step) <= band:
        k -= 1
    while math.exp(-s * (k + 1) * step) > band:
        k += 1
    tk = k * step
    zk = _bisect(lambda t: _y2(zeta, wn, t) - 1.0, tk, tk + step)
    ts = _bisect(lambda t: abs(_y2(zeta, wn, t) - 1.0) - band, tk, zk)
    return _rnd({"Ts": ts, "last_extremum": k})


def ref_true_settling_time(zeta, wn, band=0.02):
    import numpy as np
    from scipy.signal import step
    num, den = [wn * wn], [1.0, 2 * zeta * wn, wn * wn]
    env = math.log(1 / (band * math.sqrt(1 - zeta * zeta))) / (zeta * wn)
    wd = wn * math.sqrt(1 - zeta * zeta)
    N = int(1.2 * env / (math.pi / wd) * 400) + 2000
    tt = np.linspace(0, 1.2 * env, N)
    _, y = step((num, den), T=tt)
    out = np.abs(y - 1) > band
    i = int(np.nonzero(out)[0][-1])
    ts = _ref_crossing(lambda t: abs(_ref_step(num, den, t) - 1) - band, tt[i], tt[i + 1])
    return {"Ts": ts}


_add("ctrl_true_settling_time", name="Exact settling time of a second-order step response", topic="time response / second order",
     purpose="The last time at which the unit step response of wn^2/(s^2 + 2 zeta wn s + wn^2) leaves the band |y - 1| = band (not the envelope estimate), and which extremum precedes it.",
     inputs=[_n("zeta", "damping ratio (0 < zeta < 1)"), _n("wn", "natural frequency in rad/s"),
             _n("band", "settling band (default 2%)", required=False, pct=True)],
     outputs=[_o("Ts", "exact settling time", "time"), _o("last_extremum", "index k of the last extremum outside the band (0 = before the first peak)", "integer")],
     primary="Ts", conditions=["0 < zeta < 1", "0 < band < 50 %"],
     method="extrema at k pi/wd; the last band crossing lies between the last extremum outside the band and the next zero of y - 1 (bisection)",
     latex=r"T_s = \max\{t : |y(t) - 1| = \text{band}\}", run=true_settling_time, reference=ref_true_settling_time,
     wrong=lambda zeta, wn, band=0.02: {"Ts": -math.log(band * math.sqrt(1 - zeta * zeta)) / (zeta * wn), "last_extremum": 0},
     rand=lambda rng: {"zeta": round(rng.uniform(0.08, 0.95), 3), "wn": round(rng.uniform(0.3, 30), 2), "band": rng.choice([0.02, 0.05, 0.01])},
     phrases=["exact settling time", "true settling time"],
     examples=[_ex({"zeta": 0.5, "wn": 1}, {"Ts": 8.07634897393, "last_extremum": 2}),
               _ex({"zeta": 0.9, "wn": 1, "band": 0.02}, {"last_extremum": 0})],
     extra_q=[_bad({"zeta": 1.5, "wn": 1})], level="advanced")


# ================================================================================================ PID and compensator design (advanced)
def zn_from_plant(num, den, controller):
    num, den = _tf_in(num, den)
    _need(controller in ("P", "PI", "PID"), "controller must be P, PI or PID")
    pc = _phase_crossovers(num, den)
    _need(len(pc) == 1, "the plant needs exactly one phase crossover (a -180 degree frequency) for the ultimate-gain test"
          if pc else "the phase never reaches -180 degrees: there is no ultimate gain")
    g = _G(num, den, complex(0.0, pc[0]))
    Ku = 1.0 / math.hypot(g.real, g.imag)
    Pu = 2 * PI / pc[0]
    out = zn_ultimate(Ku, Pu, controller)
    out.update({"Ku": _r12(Ku), "Pu": _r12(Pu)})
    return out


def ref_zn_from_plant(num, den, controller):
    import control
    gm, _, _, wpc, _, _ = control.stability_margins(control.tf(num, den))
    r = ref_zn_ultimate(float(gm), 2 * math.pi / float(wpc), controller)
    r.update({"Ku": float(gm), "Pu": 2 * math.pi / float(wpc)})
    return r


def _rand_zn_plant(rng):
    a, b, c = sorted(rng.sample([0.5, 1.0, 2.0, 3.0, 4.0, 5.0, 8.0], 3))
    k = round(rng.uniform(0.5, 20), 2)
    if rng.random() < 0.5:
        den = _pmul(_pmul([1.0, a], [1.0, b]), [1.0, c])
    else:
        den = _pmul(_pmul([1.0, 0.0], [1.0, a]), [1.0, b])
    return {"num": [k], "den": [_r12(x) for x in den], "controller": rng.choice(["P", "PI", "PID"])}


_add("ctrl_zn_from_plant", name="Ziegler-Nichols tuning from a plant model", topic="PID tuning / Ziegler-Nichols",
     purpose="The ultimate gain Ku = 1/|G(j w180)| and period Pu = 2 pi/w180 of a plant model, then the Ziegler-Nichols P, PI or PID settings.",
     inputs=[_v("num", "plant numerator"), _v("den", "plant denominator", 2), _ch("controller", ["P", "PI", "PID"], "controller type")],
     outputs=_PID_OUT + [_o("Ku", "ultimate gain"), _o("Pu", "ultimate period", "time")], primary="Ku",
     conditions=["exactly one phase crossover"], method="phase crossover from Im(N(jw) conj D(jw)) = 0; Ziegler-Nichols table",
     latex=r"K_u = \frac{1}{|G(j\omega_{180})|},\quad P_u = \frac{2\pi}{\omega_{180}}", run=zn_from_plant, reference=ref_zn_from_plant,
     wrong=lambda num, den, controller: (lambda r: dict(r, Ku=r["Ku"] * 2, Pu=r["Pu"] / (2 * PI)))(zn_from_plant(num, den, controller)),
     rand=_rand_zn_plant, phrases=["Ziegler-Nichols from plant", "Ziegler Nichols from plant"],
     examples=[_ex({"num": [1], "den": [1, 3, 2, 0], "controller": "PID"}, {"Ku": 6.0, "Pu": 4.44288293816, "Kp": 3.6, "Ti": 2.22144146908}),
               _ex({"num": [1], "den": [1, 6, 11, 6], "controller": "P"}, {"Ku": 60.0, "Kp": 30.0})],
     extra_q=[_bad({"num": [1], "den": [1, 1], "controller": "PID"})], level="advanced",
     weak="the tuning table itself is restated; Ku and Pu are checked against python-control's stability margins")


def lead_design(phi_max_deg, w_m):
    phi, wm = _num(phi_max_deg, "phi_max_deg"), _pos(w_m, "w_m")
    _need(0 < phi < 90, "the phase lead must lie between 0 and 90 degrees")
    sn = math.sin(math.radians(phi))
    alpha = (1 - sn) / (1 + sn)
    T = 1.0 / (wm * math.sqrt(alpha))
    return _rnd({"alpha": alpha, "T": T, "zero": 1.0 / T, "pole": 1.0 / (alpha * T), "gain_db": -10 * math.log10(alpha)})


def ref_lead_design(phi_max_deg, w_m):
    import numpy as np
    from scipy.optimize import brentq

    def peak(alpha):                               # C(s) = (s + 1)/(alpha s + 1): frequency and value of its maximum phase
        num, den = np.array([1.0, 1.0]), np.array([alpha, 1.0])
        f = lambda w: float(np.real(np.polyval(np.polyder(num), 1j * w) / np.polyval(num, 1j * w)
                                    - np.polyval(np.polyder(den), 1j * w) / np.polyval(den, 1j * w)))
        w = brentq(f, 1e-3, 1e3 / alpha, xtol=1e-300, rtol=8.9e-16)
        return w, math.degrees(cmath.phase(_ref_freq([1.0, 1.0], [alpha, 1.0], w)))
    alpha = brentq(lambda a: peak(a)[1] - phi_max_deg, 1e-6, 1 - 1e-9, xtol=1e-300, rtol=8.9e-16)
    w1, _ = peak(alpha)
    T = w1 / w_m
    return {"alpha": alpha, "T": T, "zero": 1 / T, "pole": 1 / (alpha * T),
            "gain_db": 20 * math.log10(abs(_ref_freq([T, 1.0], [alpha * T, 1.0], w_m)))}


_add("ctrl_lead_design", name="Lead compensator design for a required phase lead", topic="compensators / lead",
     purpose="alpha = (1 - sin phi)/(1 + sin phi) and T = 1/(w_m sqrt(alpha)) so that C(s) = (T s + 1)/(alpha T s + 1) adds the phase lead phi at w_m; its zero, pole and gain at w_m.",
     inputs=[_n("phi_max_deg", "required maximum phase lead in degrees (0 to 90)"), _n("w_m", "frequency of the maximum lead in rad/s")],
     outputs=[_o("alpha", "alpha (pole/zero ratio z/p)"), _o("T", "time constant T", "time"), _o("zero", "zero corner 1/T (rad/s)"),
              _o("pole", "pole corner 1/(alpha T) (rad/s)"), _o("gain_db", "|C(j w_m)| in dB = -10 log10 alpha")],
     primary="alpha", conditions=["0 < phi < 90 degrees", "w_m > 0"], method="sin phi_max = (1 - alpha)/(1 + alpha), w_m = 1/(T sqrt(alpha))",
     latex=r"\alpha = \frac{1-\sin\phi_m}{1+\sin\phi_m},\quad T = \frac{1}{\omega_m\sqrt\alpha}", run=lead_design, reference=ref_lead_design,
     wrong=lambda phi_max_deg, w_m: (lambda a: {"alpha": a, "T": 1 / (w_m * a), "zero": w_m * a, "pole": w_m, "gain_db": -20 * math.log10(a)})(
         (1 - math.tan(math.radians(phi_max_deg) / 2)) / (1 + math.tan(math.radians(phi_max_deg) / 2))),
     rand=lambda rng: {"phi_max_deg": round(rng.uniform(5, 75), 1), "w_m": round(rng.uniform(0.2, 100), 2)},
     phrases=["lead compensator design"],
     examples=[_ex({"phi_max_deg": 30, "w_m": 1}, {"alpha": 0.333333333333, "T": 1.73205080757, "gain_db": 4.77121254720}),
               _ex({"phi_max_deg": 53.13010235415598, "w_m": 3}, {"alpha": 0.111111111111, "zero": 1.0, "pole": 9.0})],
     extra_q=[_bad({"phi_max_deg": 95, "w_m": 1})], level="advanced")


def lag_design(beta, w_c, ratio=10.0):
    beta, wc, ratio = _num(beta, "beta"), _pos(w_c, "w_c"), _num(ratio, "ratio")
    _need(beta > 1, "beta must exceed 1 (the factor by which the low-frequency gain is raised)")
    _need(ratio >= 2, "the zero should be at least a factor 2 below the crossover (ratio >= 2)")
    z = wc / ratio
    p = z / beta
    g = complex(0.0, wc)
    c = (g + z) / (g + p)
    return _rnd({"zero": z, "pole": p, "dc_gain": beta, "phase_deg": _deg(c), "magnitude_db": 20 * math.log10(math.hypot(c.real, c.imag))})


def ref_lag_design(beta, w_c, ratio=10.0):
    z = w_c / ratio
    p = z / beta
    h = _ref_freq([1.0, z], [1.0, p], w_c)
    return {"zero": z, "pole": p, "dc_gain": z / p, "phase_deg": math.degrees(cmath.phase(h)), "magnitude_db": 20 * math.log10(abs(h))}


_add("ctrl_lag_design", name="Lag compensator placement", topic="compensators / lag",
     purpose="C(s) = (s + z)/(s + p) with z = w_c/ratio (default a decade below the crossover) and p = z/beta: zero, pole, DC gain beta, and the phase lag and gain it adds at w_c.",
     inputs=[_n("beta", "required low-frequency gain increase (> 1)"), _n("w_c", "gain-crossover frequency in rad/s"),
             _n("ratio", "w_c / zero (default 10)", required=False)],
     outputs=[_o("zero", "zero (rad/s)"), _o("pole", "pole (rad/s)"), _o("dc_gain", "DC gain z/p = beta"),
              _o("phase_deg", "phase added at w_c in degrees (negative)"), _o("magnitude_db", "gain added at w_c in dB")],
     primary="phase_deg", conditions=["beta > 1", "ratio >= 2"], method="zero a decade below crossover, pole beta times lower",
     latex=r"C(s) = \frac{s + z}{s + z/\beta},\quad z = \omega_c/10", run=lag_design, reference=ref_lag_design,
     wrong=lambda beta, w_c, ratio=10.0: (lambda z: {"zero": z, "pole": z * beta, "dc_gain": 1 / beta,
                                                     "phase_deg": _deg(complex(z, w_c) / complex(z * beta, w_c)),
                                                     "magnitude_db": 0.0})(w_c / ratio),
     rand=lambda rng: dict({"beta": round(rng.uniform(1.5, 30), 2), "w_c": round(rng.uniform(0.1, 50), 2)},
                           **({"ratio": round(rng.uniform(3, 20), 1)} if rng.random() < 0.4 else {})),
     phrases=["lag compensator design"],
     examples=[_ex({"beta": 10, "w_c": 1}, {"zero": 0.1, "pole": 0.01, "dc_gain": 10.0, "phase_deg": -5.13765443982}),
               _ex({"beta": 5, "w_c": 2, "ratio": 4}, {"zero": 0.5, "pole": 0.1})],
     extra_q=[_bad({"beta": 0.5, "w_c": 1})], level="advanced", weak="the zero and pole placement is a restated rule; phase and gain are checked against scipy")


def pade(theta, n):
    theta = _pos(theta, "theta")
    n = int(n)
    _need(1 <= n <= 6, "the order must be between 1 and 6")
    c = []
    for k in range(n + 1):
        c.append(Fraction(math.factorial(2 * n - k) * math.factorial(n), math.factorial(2 * n) * math.factorial(k) * math.factorial(n - k)))
    den = [float(c[k]) * theta ** k for k in range(n, -1, -1)]
    num = [float(c[k]) * (-theta) ** k for k in range(n, -1, -1)]
    lead = den[0]
    return _rnd({"num": [x / lead for x in num], "den": [x / lead for x in den]})


def ref_pade(theta, n):
    import numpy as np
    from scipy.interpolate import pade as sp_pade
    an = [(-1) ** k / math.factorial(k) for k in range(2 * n + 1)]   # Taylor series of e^{-x}
    p, q = sp_pade(an, n, n)
    pc, qc = list(np.atleast_1d(p.coeffs)), list(np.atleast_1d(q.coeffs))   # in x = theta s, highest power first
    num = [float(pc[i]) * theta ** (n - i) for i in range(n + 1)]
    den = [float(qc[i]) * theta ** (n - i) for i in range(n + 1)]
    return {"num": [x / den[0] for x in num], "den": [x / den[0] for x in den]}


_add("ctrl_pade", name="Pade approximation of a time delay", topic="time delay / Pade",
     purpose="The (n, n) Pade approximation of e^{-theta s} as a rational transfer function (denominator's leading coefficient 1).",
     inputs=[_n("theta", "time delay in seconds"), _int("n", "approximation order (1 to 6)", 1, 6)],
     outputs=[_o("num", "numerator coefficients"), _o("den", "denominator coefficients (leading 1)")], primary="den",
     conditions=["theta > 0", "1 <= n <= 6"], method="Pade coefficients c_k = (2n-k)! n! / ((2n)! k! (n-k)!)",
     latex=r"e^{-\theta s} \approx \frac{\sum_k c_k(-\theta s)^k}{\sum_k c_k(\theta s)^k}", run=pade, reference=ref_pade,
     wrong=lambda theta, n: pade(theta * 2, n), rand=lambda rng: {"theta": round(rng.uniform(0.05, 10), 2), "n": rng.randint(1, 6)},
     phrases=["Pade approximation"],
     examples=[_ex({"theta": 2, "n": 1}, {"num": [-1.0, 1.0], "den": [1.0, 1.0]}),
               _ex({"theta": 1, "n": 2}, {"num": [1.0, -6.0, 12.0], "den": [1.0, 6.0, 12.0]})],
     extra_q=[_bad({"theta": 1, "n": 9})], level="advanced")


def scalar_lqr(a, b, q, r):
    a, b, q, r = _num(a, "a"), _num(b, "b"), _pos(q, "q"), _pos(r, "r")
    _need(b != 0, "b must not be zero")
    root = math.sqrt(a * a + b * b * q / r)
    P = q / (root - a)
    K = b * P / r
    return _rnd({"P": P, "K": K, "pole": -root})


def ref_scalar_lqr(a, b, q, r):
    from scipy.linalg import solve_continuous_are
    P = float(solve_continuous_are([[a]], [[b]], [[q]], [[r]])[0, 0])
    return {"P": P, "K": b * P / r, "pole": a - b * b * P / r}


_add("ctrl_scalar_lqr", name="LQR for a first-order system", topic="optimal control / LQR",
     purpose="For x' = a x + b u minimising the integral of q x^2 + r u^2: the Riccati solution P, the optimal gain K = b P/r and the closed-loop pole -sqrt(a^2 + b^2 q/r).",
     inputs=[_n("a", "system coefficient"), _n("b", "input coefficient"), _n("q", "state weight (> 0)"), _n("r", "input weight (> 0)")],
     outputs=[_o("P", "Riccati solution"), _o("K", "optimal feedback gain"), _o("pole", "closed-loop pole a - b K")], primary="K",
     conditions=["b != 0", "q, r > 0"], method="positive root of 2 a P - b^2 P^2 / r + q = 0",
     latex=r"2aP - \frac{b^2P^2}{r} + q = 0,\quad K = \frac{bP}{r}", run=scalar_lqr, reference=ref_scalar_lqr,
     wrong=lambda a, b, q, r: (lambda P: {"P": P, "K": b * P / r, "pole": a - b * b * P / r})(r * (a + math.sqrt(a * a + q / r)) / (b * b) if b else 0.0),
     rand=lambda rng: {"a": round(rng.uniform(-5, 5), 2), "b": round(rng.uniform(0.2, 5), 2) * rng.choice([1, -1]),
                       "q": round(rng.uniform(0.1, 20), 2), "r": round(rng.uniform(0.1, 10), 2)},
     phrases=["scalar LQR", "first-order LQR"],
     examples=[_ex({"a": 1, "b": 1, "q": 3, "r": 1}, {"P": 3.0, "K": 3.0, "pole": -2.0}),
               _ex({"a": 0, "b": 2, "q": 4, "r": 1}, {"P": 1.0, "K": 2.0})],
     extra_q=[_bad({"a": 1, "b": 0, "q": 3, "r": 1})], level="advanced")


def pi_pole_placement(K, tau, zeta, wn):
    K, tau, zeta, wn = _num(K, "K"), _pos(tau, "tau"), _pos(zeta, "zeta"), _pos(wn, "wn")
    _need(K != 0, "K must not be zero")
    return _rnd({"Kp": (2 * zeta * wn * tau - 1) / K, "Ki": wn * wn * tau / K})


def ref_pi_pole_placement(K, tau, zeta, wn):
    import control
    import numpy as np

    def char(kp, ki):
        cl = control.feedback(control.tf([kp, ki], [1.0, 0.0]) * control.tf([K], [tau, 1.0]), 1)
        d = np.array(cl.den[0][0], float)
        return d / d[0]
    c0, c1, c2 = char(1, 1), char(2, 1), char(1, 2)
    target = np.array([1.0, 2 * zeta * wn, wn * wn])
    Mx = np.column_stack([c1[1:] - c0[1:], c2[1:] - c0[1:]])
    sol = np.linalg.solve(Mx, target[1:] - c0[1:])
    return {"Kp": float(sol[0]) + 1.0, "Ki": float(sol[1]) + 1.0}


_add("ctrl_pi_pole_placement", name="PI gains placing the closed-loop poles of a first-order plant", topic="PID tuning / pole placement",
     purpose="Kp = (2 zeta wn tau - 1)/K and Ki = wn^2 tau/K, so that K/(tau s + 1) under PI control with unity feedback has the characteristic polynomial s^2 + 2 zeta wn s + wn^2.",
     inputs=[_n("K", "plant gain"), _n("tau", "plant time constant in seconds"), _n("zeta", "desired damping ratio"), _n("wn", "desired natural frequency in rad/s")],
     outputs=[_o("Kp", "proportional gain"), _o("Ki", "integral gain")], primary="Ki", conditions=["tau, zeta, wn > 0", "K != 0"],
     method="match tau s^2 + (1 + K Kp) s + K Ki to tau (s^2 + 2 zeta wn s + wn^2)",
     latex=r"K_p = \frac{2\zeta\omega_n\tau - 1}{K},\quad K_i = \frac{\omega_n^2\tau}{K}", run=pi_pole_placement, reference=ref_pi_pole_placement,
     wrong=lambda K, tau, zeta, wn: {"Kp": 2 * zeta * wn * tau / K, "Ki": wn * wn / K},
     rand=lambda rng: {"K": round(rng.uniform(0.2, 10), 2), "tau": round(rng.uniform(0.2, 20), 2), "zeta": round(rng.uniform(0.3, 1.5), 2),
                       "wn": round(rng.uniform(0.2, 10), 2)},
     phrases=["PI pole placement"],
     examples=[_ex({"K": 2, "tau": 5, "zeta": 0.7, "wn": 1}, {"Kp": 3.0, "Ki": 2.5}),
               _ex({"K": 1, "tau": 1, "zeta": 1, "wn": 2}, {"Kp": 3.0, "Ki": 4.0})],
     extra_q=[_bad({"K": 2, "tau": -5, "zeta": 0.7, "wn": 1})], level="advanced",
     weak="python-control closed-loop polynomials, solved as a linear system (the same coefficient matching)")


def kalman_scalar_steady(a, c, q, r):
    a, c, q, r = _num(a, "a"), _num(c, "c"), _pos(q, "q"), _pos(r, "r")
    _need(c != 0, "c must not be zero")
    Bq = r - a * a * r - q * c * c
    disc = math.sqrt(Bq * Bq + 4 * c * c * q * r)
    P = 2 * q * r / (Bq + disc) if Bq > 0 else (disc - Bq) / (2 * c * c)
    K = P * c / (c * c * P + r)
    return _rnd({"P_prior": P, "gain": K, "P_post": (1 - K * c) * P})


def ref_kalman_scalar_steady(a, c, q, r):
    from scipy.linalg import solve_discrete_are
    P = float(solve_discrete_are([[a]], [[c]], [[q]], [[r]])[0, 0])
    K = P * c / (c * c * P + r)
    return {"P_prior": P, "gain": K, "P_post": (1 - K * c) * P}


_add("ctrl_kalman_scalar_steady", name="Steady-state gain of a scalar Kalman filter", topic="estimation / Kalman filter",
     purpose="For x_k = a x_{k-1} + w (variance q) and z_k = c x_k + v (variance r): the steady prior variance P (discrete Riccati equation), the steady Kalman gain and the posterior variance.",
     inputs=[_n("a", "state transition coefficient"), _n("c", "measurement coefficient"), _n("q", "process noise variance"), _n("r", "measurement noise variance")],
     outputs=[_o("P_prior", "steady prior error variance"), _o("gain", "steady Kalman gain"), _o("P_post", "steady posterior error variance")],
     primary="gain", conditions=["c != 0", "q, r > 0"], method="positive root of c^2 P^2 + (r - a^2 r - q c^2) P - q r = 0",
     latex=r"P = a^2P - \frac{a^2c^2P^2}{c^2P + r} + q,\quad K = \frac{Pc}{c^2P + r}", run=kalman_scalar_steady, reference=ref_kalman_scalar_steady,
     wrong=lambda a, c, q, r: (lambda P: {"P_prior": P, "gain": P * c / (c * c * P + r), "P_post": (1 - P * c * c / (c * c * P + r)) * P})(q / max(1e-9, 1 - a * a) if abs(a) < 1 else q + r),
     rand=lambda rng: {"a": round(rng.uniform(-1.5, 1.5), 2), "c": round(rng.uniform(0.2, 3), 2), "q": round(rng.uniform(0.01, 5), 3), "r": round(rng.uniform(0.05, 5), 3)},
     phrases=["steady-state Kalman gain", "steady state Kalman gain"],
     examples=[_ex({"a": 1, "c": 1, "q": 1, "r": 1}, {"P_prior": 1.61803398875, "gain": 0.61803398875, "P_post": 0.61803398875}),
               _ex({"a": 0, "c": 1, "q": 2, "r": 2}, {"P_prior": 2.0, "gain": 0.5})],
     extra_q=[_bad({"a": 1, "c": 0, "q": 1, "r": 1})], level="advanced")


def fopdt_smith(t28, t63, dy, du):
    t28, t63, dy, du = _pos(t28, "t28"), _pos(t63, "t63"), _num(dy, "dy"), _num(du, "du")
    _need(t63 > t28, "t63 must be later than t28")
    _need(du != 0 and dy != 0, "the input and output changes must not be zero")
    tau = 1.5 * (t63 - t28)
    theta = t63 - tau
    _need(theta >= 0, "the times give a negative dead time: the response is not first order plus dead time")
    return _rnd({"K": dy / du, "tau": tau, "theta": theta})


def ref_fopdt_smith(t28, t63, dy, du):
    from scipy.optimize import fsolve
    f1, f2 = 1 - math.exp(-1 / 3), 1 - math.exp(-1)

    def eqs(v):
        tau, th = v
        return [1 - math.exp(-(t28 - th) / tau) - f1, 1 - math.exp(-(t63 - th) / tau) - f2]
    g = 1.6 * (t63 - t28)
    tau, th = fsolve(eqs, [g, t63 - g], xtol=1e-15)
    return {"K": dy / du, "tau": float(tau), "theta": float(th)}


_add("ctrl_fopdt_smith", name="First-order-plus-dead-time fit by Smith's two-point method", topic="identification / FOPDT",
     purpose="K = dy/du, tau = 1.5 (t63 - t28) and theta = t63 - tau from the times at which a step response reaches 28.3 % and 63.2 % of its change.",
     inputs=[_n("t28", "time to 28.3 % of the change (s)"), _n("t63", "time to 63.2 % of the change (s)"), _n("dy", "total output change"),
             _n("du", "input step size")],
     outputs=[_o("K", "process gain"), _o("tau", "time constant", "time"), _o("theta", "dead time", "time")], primary="tau",
     conditions=["0 < t28 < t63", "theta >= 0"], method="Smith's method", latex=r"\tau = 1.5(t_{63} - t_{28}),\quad \theta = t_{63} - \tau",
     run=fopdt_smith, reference=ref_fopdt_smith,
     wrong=lambda t28, t63, dy, du: {"K": dy / du, "tau": t63 - t28, "theta": t28},
     rand=lambda rng: (lambda tau, th: {"t28": round(th + tau / 3, 4), "t63": round(th + tau, 4), "dy": round(rng.uniform(0.5, 20), 2),
                                        "du": round(rng.uniform(0.5, 5), 2)})(rng.uniform(0.5, 30), rng.uniform(0.1, 10)),
     phrases=["Smith FOPDT fit", "Smith two-point method"],
     examples=[_ex({"t28": 4, "t63": 10, "dy": 6, "du": 2}, {"K": 3.0, "tau": 9.0, "theta": 1.0}),
               _ex({"t28": 2, "t63": 5, "dy": 1, "du": 1}, {"tau": 4.5, "theta": 0.5})],
     extra_q=[_bad({"t28": 1, "t63": 10, "dy": 6, "du": 2})], level="advanced",
     weak="the two defining equations re-solved numerically (fsolve)")


# ================================================================================================ Lyapunov and Riccati helpers
def _lyap(Acl, Q, exact=False):
    """P with Acl^T P + P Acl + Q = 0 (Kronecker linear system)."""
    n = len(Acl)
    zero = Fraction(0) if exact else 0.0
    M, rhs = [], []
    for i in range(n):
        for j in range(n):
            row = [zero] * (n * n)
            for k in range(n):
                row[k * n + j] = row[k * n + j] + Acl[k][i]
                row[i * n + k] = row[i * n + k] + Acl[k][j]
            M.append(row)
            rhs.append(-Q[i][j])
    x = _solve(M, rhs, exact=exact)
    P = [[x[i * n + j] for j in range(n)] for i in range(n)]
    return [[(P[i][j] + P[j][i]) / 2 for j in range(n)] for i in range(n)]


def _dlyap(Acl, Q, exact=False):
    """P with Acl^T P Acl - P + Q = 0."""
    n = len(Acl)
    zero = Fraction(0) if exact else 0.0
    M, rhs = [], []
    for i in range(n):
        for j in range(n):
            row = [zero] * (n * n)
            row[i * n + j] = row[i * n + j] + 1
            for k in range(n):
                for l in range(n):
                    row[k * n + l] = row[k * n + l] - Acl[k][i] * Acl[l][j]
            M.append(row)
            rhs.append(Q[i][j])
    x = _solve(M, rhs, exact=exact)
    P = [[x[i * n + j] for j in range(n)] for i in range(n)]
    return [[(P[i][j] + P[j][i]) / 2 for j in range(n)] for i in range(n)]


def _sym(Q, name):
    n = len(Q)
    for i in range(n):
        for j in range(n):
            _need(Q[i][j] == Q[j][i], f"{name} must be symmetric")
    return Q


def _posdef(Q):
    """Exact positive definiteness (leading principal minors of a matrix of Fractions)."""
    n = len(Q)
    return all(_det([r[:k] for r in Q[:k]]) > 0 for k in range(1, n + 1))


def _cl_poles(A, B, K):
    n = len(A)
    Acl = [[A[i][j] - B[i] * K[j] for j in range(n)] for i in range(n)]
    c, _ = _faddeev(Acl)
    return _roots([float(x) for x in c])


def _care(A, B, Q, R):
    """Continuous algebraic Riccati equation A^T P + P A - P B R^-1 B^T P + Q = 0 (single input) by Newton-Kleinman
    iteration from an exactly computed stabilising Ackermann gain."""
    n = len(A)
    _need(_rank(_ctrb(_frac(A), _frac(B))) == n, "(A, B) must be controllable")
    rho = 1.0 + max(sum(abs(x) for x in r) for r in A)
    alpha = [1.0]
    for i in range(n):
        alpha = _pmul(alpha, [1.0, float(math.ceil(rho)) + i])
    K = [float(x) for x in _acker(A, B, alpha)]
    P = None
    for _ in range(200):
        Acl = [[A[i][j] - B[i] * K[j] for j in range(n)] for i in range(n)]
        Qk = [[Q[i][j] + R * K[i] * K[j] for j in range(n)] for i in range(n)]
        P = _lyap(Acl, Qk)
        Kn = [sum(B[i] * P[i][j] for i in range(n)) / R for j in range(n)]
        d = max(abs(a - b) for a, b in zip(Kn, K))
        K = Kn
        if d <= 1e-15 * max(1.0, max(abs(x) for x in K)):
            break
    Acl = [[A[i][j] - B[i] * K[j] for j in range(n)] for i in range(n)]
    Qk = [[Q[i][j] + R * K[i] * K[j] for j in range(n)] for i in range(n)]
    P = _lyap(Acl, Qk)
    K = [sum(B[i] * P[i][j] for i in range(n)) / R for j in range(n)]
    return P, K


def _dare(A, B, Q, R):
    """Discrete algebraic Riccati equation P = A^T P A - A^T P B (R + B^T P B)^-1 B^T P A + Q (single input) by
    Hewer's iteration from the exact deadbeat gain."""
    n = len(A)
    _need(_rank(_ctrb(_frac(A), _frac(B))) == n, "(A, B) must be controllable")
    K = [float(x) for x in _acker(A, B, [1.0] + [0.0] * n)]
    for it in range(201):
        Acl = [[A[i][j] - B[i] * K[j] for j in range(n)] for i in range(n)]
        Qk = [[Q[i][j] + R * K[i] * K[j] for j in range(n)] for i in range(n)]
        P = _dlyap(Acl, Qk)
        PB = [sum(P[i][k] * B[k] for k in range(n)) for i in range(n)]
        s = R + sum(B[i] * PB[i] for i in range(n))
        Kn = [sum(PB[k] * A[k][j] for k in range(n)) / s for j in range(n)]
        d = max(abs(a - b) for a, b in zip(Kn, K))
        K = Kn
        if d <= 1e-15 * max(1.0, max(abs(x) for x in K)) and it > 0:
            break
    Acl = [[A[i][j] - B[i] * K[j] for j in range(n)] for i in range(n)]
    Qk = [[Q[i][j] + R * K[i] * K[j] for j in range(n)] for i in range(n)]
    P = _dlyap(Acl, Qk)
    PB = [sum(P[i][k] * B[k] for k in range(n)) for i in range(n)]
    s = R + sum(B[i] * PB[i] for i in range(n))
    K = [sum(PB[k] * A[k][j] for k in range(n)) / s for j in range(n)]
    return P, K


def _rand_spd(rng, n):
    for _ in range(100):
        L = [[float(rng.randint(-2, 2)) if j < i else (float(rng.randint(1, 3)) if j == i else 0.0) for j in range(n)] for i in range(n)]
        Q = _mmul(L, _mT(L))
        if _posdef(_frac(Q)):
            return Q
    return _eye(n)


def _rand_lqr(rng):
    for _ in range(100):
        n = rng.randint(2, 4)
        A = [[float(rng.randint(-3, 3)) for _ in range(n)] for _ in range(n)]
        B = [float(rng.randint(-2, 2)) for _ in range(n)]
        if _rank(_ctrb(_frac(A), _frac(B))) < n:
            continue
        return {"A": A, "B": B, "Q": _rand_spd(rng, n), "R": round(rng.uniform(0.2, 5), 2)}
    return {"A": [[0.0, 1.0], [0.0, 0.0]], "B": [0.0, 1.0], "Q": [[1.0, 0.0], [0.0, 1.0]], "R": 1.0}


def _well(rng, gen, fn, pkey):
    """Random Riccati problems that are well conditioned (a nearly uncontrollable pair gives a huge P whose digits
    no solver can resolve to 1e-9; such cases are kept out of the random checks, not out of the capability)."""
    for _ in range(200):
        x = gen(rng)
        try:
            r = fn(**x)
        except ValueError:
            continue
        P = r[pkey]
        if max(abs(v) for row in P for v in row) <= 50.0 * max(1.0, max(abs(v) for row in (x.get("Q") or x.get("W")) for v in row)):
            return x
    return gen(rng)


def _riccati_in(A, B, Q, R):
    A, B = _ss_in(A, B)
    Q = _square(Q, "Q")
    _need(len(Q) == len(A), "Q must be n x n")
    _sym(Q, "Q")
    _need(_posdef(_frac(Q)), "Q must be positive definite")
    R = _pos(R, "R")
    return A, B, Q, R


# ================================================================================================ optimal control and estimation
def lqr(A, B, Q, R):
    A, B, Q, R = _riccati_in(A, B, Q, R)
    P, K = _care(A, B, Q, R)
    return {"K": _rnd(K), "P": _rnd(P), "poles": _cl_poles(A, B, K)}


def ref_lqr(A, B, Q, R):
    import numpy as np
    from scipy.linalg import solve_continuous_are
    Am, Bm = np.array(A, float), np.array(B, float).reshape(-1, 1)
    P = solve_continuous_are(Am, Bm, np.array(Q, float), [[R]])
    K = (Bm.T @ P / R)[0]
    return {"K": [float(x) for x in K], "P": [[float(x) for x in r] for r in P],
            "poles": _ref_canon(np.linalg.eigvals(Am - Bm @ K.reshape(1, -1)))}


_add("ctrl_lqr", name="Continuous-time LQR gain (Riccati equation)", topic="optimal control / LQR",
     purpose="The optimal state feedback u = -K x minimising the integral of x^T Q x + R u^2 for x' = A x + B u: the Riccati solution P, K = R^{-1} B^T P and the closed-loop poles.",
     inputs=[_m("A", "state matrix (n x n)"), _v("B", "input vector"), _m("Q", "state weight (symmetric positive definite)"), _n("R", "input weight (> 0)")],
     outputs=[_o("K", "optimal gain"), _o("P", "Riccati solution"), _o("poles", "closed-loop poles")], primary="K",
     conditions=["(A, B) controllable", "Q symmetric positive definite", "R > 0"],
     method="Newton-Kleinman iteration (a Lyapunov equation per step) from a stabilising Ackermann gain",
     latex=r"A^TP + PA - PBR^{-1}B^TP + Q = 0,\quad K = R^{-1}B^TP", run=lqr, reference=ref_lqr,
     wrong=lambda A, B, Q, R: (lambda r: dict(r, K=[2 * k for k in r["K"]]))(lqr(A, B, Q, R)), rand=lambda rng: _well(rng, _rand_lqr, lqr, "P"), phrases=["LQR gain", "LQR"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "B": [0, 1], "Q": [[1, 0], [0, 1]], "R": 1}, {"K": [1.0, 1.73205080757], "P": [[1.73205080757, 1.0], [1.0, 1.73205080757]]}),
               _ex({"A": [[0, 1], [-1, 0]], "B": [0, 1], "Q": [[1, 0], [0, 0.5]], "R": 2}, {"K": [0.224744871392, 0.836355033932]})],
     extra_q=[_bad({"A": [[0, 1], [0, 0]], "B": [0, 1], "Q": [[1, 2], [0, 1]], "R": 1})], level="expert")


def dlqr(A, B, Q, R):
    A, B, Q, R = _riccati_in(A, B, Q, R)
    P, K = _dare(A, B, Q, R)
    return {"K": _rnd(K), "P": _rnd(P), "poles": _cl_poles(A, B, K)}


def ref_dlqr(A, B, Q, R):
    import numpy as np
    from scipy.linalg import solve_discrete_are
    Am, Bm = np.array(A, float), np.array(B, float).reshape(-1, 1)
    P = solve_discrete_are(Am, Bm, np.array(Q, float), [[R]])
    K = np.linalg.solve(R + Bm.T @ P @ Bm, Bm.T @ P @ Am)[0]
    return {"K": [float(x) for x in K], "P": [[float(x) for x in r] for r in P],
            "poles": _ref_canon(np.linalg.eigvals(Am - Bm @ K.reshape(1, -1)))}


def _rand_dlqr(rng):
    x = _rand_lqr(rng)
    x["A"] = [[v * 0.5 for v in r] for r in x["A"]]
    return x


_add("ctrl_dlqr", name="Discrete-time LQR gain (discrete Riccati equation)", topic="optimal control / LQR",
     purpose="The optimal state feedback u_k = -K x_k minimising the sum of x^T Q x + R u^2 for x_{k+1} = A x_k + B u_k: P from the discrete Riccati equation, K = (R + B^T P B)^{-1} B^T P A and the closed-loop poles.",
     inputs=[_m("A", "discrete state matrix (n x n)"), _v("B", "input vector"), _m("Q", "state weight (symmetric positive definite)"), _n("R", "input weight (> 0)")],
     outputs=[_o("K", "optimal gain"), _o("P", "Riccati solution"), _o("poles", "closed-loop poles")], primary="K",
     conditions=["(A, B) controllable", "Q symmetric positive definite", "R > 0"],
     method="Hewer's iteration (a discrete Lyapunov equation per step) from the deadbeat gain",
     latex=r"P = A^TPA - A^TPB(R + B^TPB)^{-1}B^TPA + Q", run=dlqr, reference=ref_dlqr,
     wrong=lambda A, B, Q, R: (lambda r: dict(r, K=[0.5 * k for k in r["K"]]))(dlqr(A, B, Q, R)), rand=lambda rng: _well(rng, _rand_dlqr, dlqr, "P"),
     phrases=["discrete LQR gain", "discrete LQR"],
     examples=[_ex({"A": [[1, 1], [0, 1]], "B": [0, 1], "Q": [[1, 0], [0, 1]], "R": 1}, {"K": [0.422082440385, 1.2439288539]}),
               _ex({"A": [[0.5, 0], [0, 0.8]], "B": [1, 1], "Q": [[1, 0], [0, 1]], "R": 1}, {"P": [[1.24129941786, -0.272125618133], [-0.272125618133, 1.7049507782]]})],
     extra_q=[_bad({"A": [[1, 0], [0, 2]], "B": [1, 0], "Q": [[1, 0], [0, 1]], "R": 1})], level="expert")


def kalman_bucy(A, C, W, V):
    A, C = _ss_in(A, None, C)
    W = _square(W, "W")
    _need(len(W) == len(A), "W must be n x n")
    _sym(W, "W")
    _need(_posdef(_frac(W)), "W must be positive definite")
    V = _pos(V, "V")
    _need(_rank(_ctrb(_frac(_mT(A)), _frac(C))) == len(A), "(A, C) must be observable")
    P, L = _care(_mT(A), C, W, V)
    n = len(A)
    return {"L": _rnd(L), "P": _rnd(P), "poles": _cl_poles(_mT(A), C, L)}


def ref_kalman_bucy(A, C, W, V):
    import numpy as np
    from scipy.linalg import solve_continuous_are
    Am, Cm = np.array(A, float), np.array(C, float).reshape(1, -1)
    P = solve_continuous_are(Am.T, Cm.T, np.array(W, float), [[V]])
    L = (P @ Cm.T / V)[:, 0]
    return {"L": [float(x) for x in L], "P": [[float(x) for x in r] for r in P],
            "poles": _ref_canon(np.linalg.eigvals(Am - L.reshape(-1, 1) @ Cm))}


def _rand_kb(rng):
    x = _rand_lqr(rng)
    return {"A": _mT(x["A"]), "C": x["B"], "W": x["Q"], "V": x["R"]}


_add("ctrl_kalman_bucy", name="Steady-state Kalman-Bucy filter gain", topic="estimation / Kalman filter",
     purpose="For x' = A x + w (covariance W), y = C x + v (variance V): the steady error covariance P from the filter Riccati equation A P + P A^T - P C^T V^{-1} C P + W = 0, the gain L = P C^T / V and the estimator poles (eigenvalues of A - L C).",
     inputs=[_m("A", "state matrix (n x n)"), _v("C", "output vector"), _m("W", "process noise covariance (symmetric positive definite)"),
             _n("V", "measurement noise variance (> 0)")],
     outputs=[_o("L", "Kalman gain (column)"), _o("P", "steady error covariance"), _o("poles", "estimator poles")], primary="L",
     conditions=["(A, C) observable", "W positive definite", "V > 0"], method="the dual LQR problem (A^T, C^T, W, V) by Newton-Kleinman iteration",
     latex=r"AP + PA^T - PC^TV^{-1}CP + W = 0,\quad L = PC^TV^{-1}", run=kalman_bucy, reference=ref_kalman_bucy,
     wrong=lambda A, C, W, V: (lambda r: dict(r, L=[x * V for x in r["L"]] if V != 1 else [2 * x for x in r["L"]]))(kalman_bucy(A, C, W, V)),
     rand=lambda rng: _well(rng, _rand_kb, kalman_bucy, "P"), phrases=["Kalman-Bucy gain", "continuous Kalman filter gain"],
     examples=[_ex({"A": [[0, 1], [0, 0]], "C": [1, 0], "W": [[1, 0], [0, 1]], "V": 1}, {"L": [1.73205080757, 1.0]}),
               _ex({"A": [[-1]], "C": [1], "W": [[3]], "V": 1}, {"L": [1.0], "P": [[1.0]]})],
     extra_q=[_bad({"A": [[0, 1], [0, 0]], "C": [0, 1], "W": [[1, 0], [0, 1]], "V": 1})], level="expert")


def discrete_kalman_gain(A, C, Q, R):
    A, C = _ss_in(A, None, C)
    Q = _square(Q, "Q")
    _need(len(Q) == len(A), "Q must be n x n")
    _sym(Q, "Q")
    _need(_posdef(_frac(Q)), "Q must be positive definite")
    R = _pos(R, "R")
    _need(_rank(_ctrb(_frac(_mT(A)), _frac(C))) == len(A), "(A, C) must be observable")
    P, _ = _dare(_mT(A), C, Q, R)
    n = len(A)
    PC = [sum(P[i][k] * C[k] for k in range(n)) for i in range(n)]
    s = R + sum(C[i] * PC[i] for i in range(n))
    return {"K": _rnd([x / s for x in PC]), "P_prior": _rnd(P)}


def ref_discrete_kalman_gain(A, C, Q, R):
    import numpy as np
    from scipy.linalg import solve_discrete_are
    Am, Cm = np.array(A, float), np.array(C, float).reshape(1, -1)
    P = solve_discrete_are(Am.T, Cm.T, np.array(Q, float), [[R]])
    K = (P @ Cm.T / (Cm @ P @ Cm.T + R))[:, 0]
    return {"K": [float(x) for x in K], "P_prior": [[float(x) for x in r] for r in P]}


def _rand_dkf(rng):
    x = _rand_dlqr(rng)
    return {"A": _mT(x["A"]), "C": x["B"], "Q": x["Q"], "R": x["R"]}


_add("ctrl_discrete_kalman_gain", name="Steady-state discrete Kalman filter gain", topic="estimation / Kalman filter",
     purpose="For x_{k+1} = A x_k + w (covariance Q), y_k = C x_k + v (variance R): the steady prior covariance P from the discrete filter Riccati equation and the measurement-update gain K = P C^T (C P C^T + R)^{-1}.",
     inputs=[_m("A", "discrete state matrix (n x n)"), _v("C", "output vector"), _m("Q", "process noise covariance (symmetric positive definite)"),
             _n("R", "measurement noise variance (> 0)")],
     outputs=[_o("K", "steady Kalman gain"), _o("P_prior", "steady prior error covariance")], primary="K",
     conditions=["(A, C) observable", "Q positive definite", "R > 0"], method="the dual discrete Riccati equation by Hewer's iteration",
     latex=r"P = APA^T - APC^T(CPC^T + R)^{-1}CPA^T + Q,\quad K = PC^T(CPC^T + R)^{-1}", run=discrete_kalman_gain,
     reference=ref_discrete_kalman_gain,
     wrong=lambda A, C, Q, R: (lambda r: dict(r, K=[2 * x for x in r["K"]]))(discrete_kalman_gain(A, C, Q, R)), rand=lambda rng: _well(rng, _rand_dkf, discrete_kalman_gain, "P_prior"),
     phrases=["discrete Kalman gain", "steady-state discrete Kalman gain"],
     examples=[_ex({"A": [[1]], "C": [1], "Q": [[1]], "R": 1}, {"K": [0.61803398875], "P_prior": [[1.61803398875]]}),
               _ex({"A": [[1, 1], [0, 1]], "C": [1, 0], "Q": [[1, 0], [0, 1]], "R": 1}, {"K": [0.821846413518, 0.422082440385]})],
     extra_q=[_bad({"A": [[1, 0], [0, 1]], "C": [1, 0], "Q": [[1, 0], [0, 1]], "R": 1})], level="expert")


# ================================================================================================ Lyapunov equations and Gramians
def lyapunov(A, Q):
    (A,) = _ss_in(A)
    Q = _square(Q, "Q")
    _need(len(Q) == len(A), "Q must be n x n")
    _sym(Q, "Q")
    _need(_is_hurwitz(A), "A must be stable (every eigenvalue in the left half plane)")
    P = _lyap(_frac(A), _frac(Q), exact=True)
    return {"P": [[_r12(float(x)) for x in r] for r in P], "positive_definite": _posdef(P)}


def ref_lyapunov(A, Q):
    import numpy as np
    from scipy.linalg import solve_continuous_lyapunov
    P = solve_continuous_lyapunov(np.array(A, float).T, -np.array(Q, float))
    return {"P": [[float(x) for x in r] for r in P], "positive_definite": _ref_pd(P)}


def _rand_hurwitz(rng, nlo=2, nhi=4):
    for _ in range(200):
        n = rng.randint(nlo, nhi)
        A = [[float(rng.randint(-4, 3)) for _ in range(n)] for _ in range(n)]
        for i in range(n):
            A[i][i] -= 2.0
        if _is_hurwitz(A):
            return A
    return [[-1.0, 0.0], [0.0, -2.0]]


def _rand_lyap(rng):
    A = _rand_hurwitz(rng)
    n = len(A)
    Q = _rand_spd(rng, n) if rng.random() < 0.8 else [[float(rng.randint(-2, 2)) if i != j else float(rng.randint(-1, 3)) for j in range(n)] for i in range(n)]
    Q = [[(Q[i][j] + Q[j][i]) / 2 for j in range(n)] for i in range(n)]
    return {"A": A, "Q": Q}


_add("ctrl_lyapunov", name="Continuous Lyapunov equation", topic="stability / Lyapunov",
     purpose="The solution P of A^T P + P A + Q = 0 for a stable A (exact), and whether P is positive definite (with Q > 0 that certifies stability).",
     inputs=[_m("A", "state matrix (stable, n x n)"), _m("Q", "symmetric matrix (n x n)")],
     outputs=[_o("P", "solution P"), _o("positive_definite", "true when P is positive definite", "boolean")], primary="P",
     conditions=["A stable", "Q symmetric"], method="Kronecker linear system in exact fractions",
     latex=r"A^TP + PA + Q = 0", run=lyapunov, reference=ref_lyapunov,
     wrong=lambda A, Q: lyapunov(_mT(A), Q) if _mT(A) != A else {"P": [[2 * x for x in r] for r in lyapunov(A, Q)["P"]], "positive_definite": False},
     rand=_rand_lyap, phrases=["Lyapunov equation"], decline=r"^(?!.*discrete).*lyapunov.*=",
     examples=[_ex({"A": [[-1, 0], [0, -2]], "Q": [[2, 0], [0, 4]]}, {"P": [[1.0, 0.0], [0.0, 1.0]], "positive_definite": True}),
               _ex({"A": [[0, 1], [-2, -3]], "Q": [[1, 0], [0, 1]]}, {"P": [[1.25, 0.25], [0.25, 0.25]]})],
     extra_q=[_bad({"A": [[1, 0], [0, -2]], "Q": [[1, 0], [0, 1]]}), {"q": "Solve the Lyapunov equation A^T P + P A + Q = 0 for P with A = [-1 0; 0 -2] and Q = [2 0; 0 4].", "outcome": "OUT_OF_SCOPE"}], level="expert")


def discrete_lyapunov(A, Q):
    (A,) = _ss_in(A)
    Q = _square(Q, "Q")
    _need(len(Q) == len(A), "Q must be n x n")
    _sym(Q, "Q")
    c, _ = _faddeev(_frac(A))
    _need(max(abs(complex(r)) for r in _roots([float(x) for x in c])) < 1, "A must be stable (every eigenvalue inside the unit circle)")
    P = _dlyap(_frac(A), _frac(Q), exact=True)
    return {"P": [[_r12(float(x)) for x in r] for r in P], "positive_definite": _posdef(P)}


def ref_discrete_lyapunov(A, Q):
    import numpy as np
    from scipy.linalg import solve_discrete_lyapunov
    P = solve_discrete_lyapunov(np.array(A, float).T, np.array(Q, float))
    return {"P": [[float(x) for x in r] for r in P], "positive_definite": _ref_pd(P)}


def _rand_dlyap(rng):
    for _ in range(200):
        n = rng.randint(2, 4)
        A = [[round(rng.uniform(-0.6, 0.6), 2) for _ in range(n)] for _ in range(n)]
        c, _ = _faddeev(_frac(A))
        if max(abs(complex(r)) for r in _roots([float(x) for x in c])) < 0.95:
            return {"A": A, "Q": _rand_spd(rng, n)}
    return {"A": [[0.5, 0.0], [0.0, 0.5]], "Q": [[1.0, 0.0], [0.0, 1.0]]}


_add("ctrl_discrete_lyapunov", name="Discrete Lyapunov equation", topic="stability / Lyapunov",
     purpose="The solution P of A^T P A - P + Q = 0 for a Schur-stable A (exact), and whether P is positive definite.",
     inputs=[_m("A", "discrete state matrix (eigenvalues inside the unit circle)"), _m("Q", "symmetric matrix (n x n)")],
     outputs=[_o("P", "solution P"), _o("positive_definite", "true when P is positive definite", "boolean")], primary="P",
     conditions=["A Schur stable", "Q symmetric"], method="Kronecker linear system in exact fractions",
     latex=r"A^TPA - P + Q = 0", run=discrete_lyapunov, reference=ref_discrete_lyapunov,
     wrong=lambda A, Q: lyapunov(_madd(A, _mscale(_eye(len(A)), -1.0)), Q) if _is_hurwitz(_madd(A, _mscale(_eye(len(A)), -1.0))) else {"P": Q, "positive_definite": True},
     rand=_rand_dlyap, phrases=["discrete Lyapunov equation"], decline=r"(?=.*discrete).*lyapunov.*=",
     examples=[_ex({"A": [[0.5, 0], [0, 0.5]], "Q": [[3, 0], [0, 3]]}, {"P": [[4.0, 0.0], [0.0, 4.0]], "positive_definite": True}),
               _ex({"A": [[0, 1], [0, 0]], "Q": [[1, 0], [0, 1]]}, {"P": [[1.0, 0.0], [0.0, 2.0]]})],
     extra_q=[_bad({"A": [[2, 0], [0, 0.5]], "Q": [[1, 0], [0, 1]]}), {"q": "Solve the discrete Lyapunov equation A^T P A - P + Q = 0 for P with A = [0.5 0; 0 0.5] and Q = [3 0; 0 3].", "outcome": "OUT_OF_SCOPE"}], level="expert")


def ctrb_gramian(A, B):
    A, B = _ss_in(A, B)
    _need(_is_hurwitz(A), "A must be stable for the infinite-horizon Gramian")
    Bf = _frac(B)
    W = _lyap(_mT(_frac(A)), [[Bf[i] * Bf[j] for j in range(len(B))] for i in range(len(B))], exact=True)
    d = _det(W)
    return {"Wc": [[_r12(float(x)) for x in r] for r in W], "controllable": d != 0}


def ref_ctrb_gramian(A, B):
    import numpy as np
    from scipy.linalg import solve_continuous_lyapunov
    Bm = np.array(B, float).reshape(-1, 1)
    W = solve_continuous_lyapunov(np.array(A, float), -Bm @ Bm.T)
    return {"Wc": [[float(x) for x in r] for r in W]}


def _rand_gram(rng):
    A = _rand_hurwitz(rng)
    return {"A": A, "B": [float(rng.randint(-2, 3)) or 1.0 for _ in range(len(A))]}


_add("ctrl_ctrb_gramian", name="Controllability Gramian", topic="state space / Gramians",
     purpose="The infinite-horizon controllability Gramian Wc solving A Wc + Wc A^T + B B^T = 0 for a stable A (exact), its determinant and whether (A, B) is controllable (Wc nonsingular).",
     inputs=[_m("A", "state matrix (stable, n x n)"), _v("B", "input vector")],
     outputs=[_o("Wc", "controllability Gramian"), _o("controllable", "true when Wc is nonsingular", "boolean")],
     primary="Wc", conditions=["A stable"], method="Lyapunov equation in exact fractions",
     latex=r"AW_c + W_cA^T + BB^T = 0", run=ctrb_gramian, reference=ref_ctrb_gramian,
     wrong=lambda A, B: ctrb_gramian(_mT(A), B) if _mT(A) != A else {"Wc": [[0.0]], "controllable": False},
     rand=_rand_gram, phrases=["controllability Gramian"],
     examples=[_ex({"A": [[-1, 0], [0, -2]], "B": [1, 1]}, {"Wc": [[0.5, 0.333333333333], [0.333333333333, 0.25]]}),
               _ex({"A": [[-1, 0], [0, -2]], "B": [1, 0]}, {"controllable": False})],
     extra_q=[_bad({"A": [[1, 0], [0, -2]], "B": [1, 1]})], level="expert",
     weak="the nonsingularity flag (exact determinant) is checked by recomputation only; the Gramian against scipy's Lyapunov solver")


def obsv_gramian(A, C):
    A, C = _ss_in(A, None, C)
    _need(_is_hurwitz(A), "A must be stable for the infinite-horizon Gramian")
    Cf = _frac(C)
    W = _lyap(_frac(A), [[Cf[i] * Cf[j] for j in range(len(C))] for i in range(len(C))], exact=True)
    d = _det(W)
    return {"Wo": [[_r12(float(x)) for x in r] for r in W], "observable": d != 0}


def ref_obsv_gramian(A, C):
    import numpy as np
    from scipy.linalg import solve_continuous_lyapunov
    Cm = np.array(C, float).reshape(1, -1)
    W = solve_continuous_lyapunov(np.array(A, float).T, -Cm.T @ Cm)
    return {"Wo": [[float(x) for x in r] for r in W]}


_add("ctrl_obsv_gramian", name="Observability Gramian", topic="state space / Gramians",
     purpose="The infinite-horizon observability Gramian Wo solving A^T Wo + Wo A + C^T C = 0 for a stable A (exact), its determinant and whether (A, C) is observable.",
     inputs=[_m("A", "state matrix (stable, n x n)"), _v("C", "output vector")],
     outputs=[_o("Wo", "observability Gramian"), _o("observable", "true when Wo is nonsingular", "boolean")],
     primary="Wo", conditions=["A stable"], method="Lyapunov equation in exact fractions",
     latex=r"A^TW_o + W_oA + C^TC = 0", run=obsv_gramian, reference=ref_obsv_gramian,
     wrong=lambda A, C: obsv_gramian(_mT(A), C) if _mT(A) != A else {"Wo": [[0.0]], "observable": False},
     rand=lambda rng: (lambda x: {"A": x["A"], "C": x["B"]})(_rand_gram(rng)), phrases=["observability Gramian"],
     examples=[_ex({"A": [[-1, 0], [0, -2]], "C": [1, 1]}, {"Wo": [[0.5, 0.333333333333], [0.333333333333, 0.25]]}),
               _ex({"A": [[0, 1], [-2, -3]], "C": [1, 0]}, {"Wo": [[0.916666666667, 0.25], [0.25, 0.0833333333333]]})],
     extra_q=[_bad({"A": [[1, 0], [0, -2]], "C": [1, 1]})], level="expert",
     weak="the nonsingularity flag (exact determinant) is checked by recomputation only; the Gramian against scipy's Lyapunov solver")


# ================================================================================================ norms
def h2_norm(num, den):
    num, den = _tf_in(num, den, strict=True)
    _need(_routh_ok(den), "the system must be stable")
    A, B, C, _ = _ccf(num, den)
    Af, Bf, Cf = _frac(A), _frac(B), _frac(C)
    n = len(A)
    Wo = _lyap(Af, [[Cf[i] * Cf[j] for j in range(n)] for i in range(n)], exact=True)
    h2sq = sum(Bf[i] * Wo[i][j] * Bf[j] for i in range(n) for j in range(n))
    return {"h2": _r12(math.sqrt(float(h2sq))), "h2_squared": _r12(float(h2sq))}


def ref_h2_norm(num, den):
    from scipy.integrate import quad
    f = lambda w: abs(_ref_freq(num, den, w)) ** 2
    v = quad(f, 0, 1, epsabs=0, epsrel=1e-13, limit=500)[0] + quad(f, 1, float("inf"), epsabs=0, epsrel=1e-13, limit=500)[0]
    return {"h2": math.sqrt(v / math.pi), "h2_squared": v / math.pi}


_add("ctrl_h2_norm", name="H2 norm of a stable transfer function", topic="frequency response / norms",
     purpose="||G||_2 = sqrt((1/(2 pi)) integral of |G(jw)|^2 dw) of a stable, strictly proper G(s), computed exactly as B^T Wo B from the observability Gramian.",
     inputs=[_v("num", "numerator"), _v("den", "denominator", 2)],
     outputs=[_o("h2", "H2 norm"), _o("h2_squared", "squared H2 norm")], primary="h2",
     conditions=["stable", "strictly proper"], method="Lyapunov equation of the controllable canonical form in exact fractions",
     latex=r"\|G\|_2^2 = \frac{1}{2\pi}\int_{-\infty}^{\infty}|G(j\omega)|^2d\omega = B^TW_oB", run=h2_norm, reference=ref_h2_norm,
     wrong=lambda num, den: (lambda r: {"h2": r["h2_squared"] * 2 if r["h2_squared"] != 0.5 else 1.0, "h2_squared": r["h2_squared"] * 2})(h2_norm(num, den)),
     rand=lambda rng: (lambda x: x if len(x["num"]) < len(x["den"]) else {"num": x["num"][1:] or [1.0], "den": x["den"]})(_rand_stable_tf(rng, 4)),
     phrases=["H2 norm"],
     examples=[_ex({"num": [1], "den": [1, 1]}, {"h2": 0.707106781187, "h2_squared": 0.5}),
               _ex({"num": [1], "den": [1, 2, 1]}, {"h2_squared": 0.25})],
     extra_q=[_bad({"num": [1], "den": [1, -1]}), _bad({"num": [1, 1], "den": [1, 2]})], level="expert")


def hinf_norm(num, den):
    num, den = _tf_in(num, den)
    _need(_routh_ok(den), "the system must be stable")
    f, g = _mag2(num), _mag2(den)
    P = _ptrim(_psub(_pmul(_pder(f), g), _pmul(f, _pder(g))))
    cands = [0.0]
    if len(P) >= 2 and any(x != 0 for x in P):
        cands += _real_pos_roots(P)
    best_w, best = 0.0, -1.0
    for w in cands:
        v = math.sqrt(_pval(f, w) / _pval(g, w))
        if v > best:
            best, best_w = v, w
    if len(num) == len(den):
        _need(abs(num[0] / den[0]) < best * (1 - 1e-12), "the peak gain is approached only as w -> infinity")
    return _rnd({"hinf": best, "hinf_db": 20 * math.log10(best), "w_peak": best_w})


def ref_hinf_norm(num, den):
    import numpy as np
    from scipy.optimize import minimize_scalar
    from scipy.signal import freqs
    ws = np.concatenate([[0.0], np.logspace(-4, 4, 40001)])
    _, h = freqs(num, den, worN=ws)
    i = int(np.argmax(np.abs(h)))
    if i == 0:
        val, w = abs(num[-1] / den[-1]), 0.0
    else:
        res = minimize_scalar(lambda w: -abs(_ref_freq(num, den, w)), bracket=(ws[i - 1], ws[i], ws[i + 1]), tol=1e-14)
        val = -res.fun
        # the stationary frequency itself: numpy roots of the numerator of d/dw (|N(jw)|^2 / |D(jw)|^2)
        cn = np.array([c * (1j) ** (len(num) - 1 - k) for k, c in enumerate(num)])
        cd = np.array([c * (1j) ** (len(den) - 1 - k) for k, c in enumerate(den)])
        F = np.poly1d(np.real(np.polymul(cn, np.conj(cn))))
        G = np.poly1d(np.real(np.polymul(cd, np.conj(cd))))
        Pd = F.deriv() * G - F * G.deriv()
        r = [x.real for x in Pd.roots if abs(x.imag) < 1e-7 * max(1, abs(x)) and x.real > 0]
        w = min(r, key=lambda x: abs(x - res.x))
    return {"hinf": val, "hinf_db": 20 * math.log10(val), "w_peak": w}


def _rand_hinf(rng):
    z = round(rng.uniform(0.05, 0.6), 3)
    wn = round(rng.uniform(0.5, 10), 2)
    den = _pmul([1.0, 2 * z * wn, wn * wn], [1.0, round(rng.uniform(0.5, 10), 1)]) if rng.random() < 0.5 else [1.0, 2 * z * wn, wn * wn]
    num = [round(rng.uniform(0.5, 20), 1)]
    if rng.random() < 0.3:
        num = _pmul(num, [1.0, round(rng.uniform(0.5, 5), 1)])
    return {"num": [_r12(x) for x in num], "den": [_r12(x) for x in den]}


_add("ctrl_hinf_norm", name="H-infinity norm (peak gain) of a stable transfer function", topic="frequency response / norms",
     purpose="The peak gain sup_w |G(jw)| of a stable proper G(s), in ratio and dB, and the frequency where it occurs (0 when the peak is the DC gain).",
     inputs=[_v("num", "numerator"), _v("den", "denominator", 2)],
     outputs=[_o("hinf", "H-infinity norm (peak gain)"), _o("hinf_db", "peak gain in dB"), _o("w_peak", "frequency of the peak", "angular_frequency")],
     primary="hinf", conditions=["stable", "proper", "peak at a finite frequency"],
     method="stationary points of |N(jw)|^2/|D(jw)|^2 (positive real roots of f' g - f g') and w = 0",
     latex=r"\|G\|_\infty = \sup_\omega |G(j\omega)|", run=hinf_norm, reference=ref_hinf_norm,
     wrong=lambda num, den: (lambda r: dict(r, hinf=abs(num[-1] / den[-1]), hinf_db=20 * math.log10(abs(num[-1] / den[-1])), w_peak=0.0))(hinf_norm(num, den)),
     rand=_rand_hinf, phrases=["H-infinity norm", "peak gain"],
     examples=[_ex({"num": [1], "den": [1, 1]}, {"hinf": 1.0, "w_peak": 0.0}),
               _ex({"num": [1], "den": [1, 0.2, 1]}, {"hinf": 5.02518907629, "w_peak": 0.989949493661})],
     extra_q=[_bad({"num": [1], "den": [1, -1]})], level="expert")


def nyquist_encirclements(num, den, K=1.0):
    num, den = _tf_in(num, den)
    K = _num(K, "K")
    _need(K != 0, "K must not be zero")
    op = _roots(den)
    _need(all(abs(complex(p).real) > 1e-9 * max(1.0, abs(complex(p))) for p in op),
          "the open loop has a pole on the imaginary axis (the contour needs an indentation; not handled here)")
    cl = _roots(_padd(den, _pscale(num, K)))
    _need(all(abs(complex(p).real) > 1e-9 * max(1.0, abs(complex(p))) for p in cl),
          "a closed-loop pole lies on the imaginary axis: the Nyquist curve passes through -1")
    P = sum(1 for p in op if complex(p).real > 0)
    Z = sum(1 for p in cl if complex(p).real > 0)
    return {"P": P, "Z": Z, "N": Z - P, "stable": Z == 0}


def ref_nyquist_encirclements(num, den, K=1.0):
    import control
    import numpy as np
    resp = control.nyquist_response(control.tf([K * x for x in num], den))
    N = int(resp.count)
    P = int(np.sum(np.roots(den).real > 0))
    Z = int(np.sum(control.feedback(control.tf([K * x for x in num], den), 1).poles().real > 0))
    return {"P": P, "Z": Z, "N": N, "stable": Z == 0}


def _rand_nyq(rng):
    for _ in range(100):
        n = rng.randint(1, 4)
        den = [1.0]
        for _ in range(n):
            r = round(rng.uniform(-4, 6), 1)
            if abs(r) < 0.3:
                continue
            den = _pmul(den, [1.0, r])
        if len(den) < 2:
            continue
        m = rng.randint(0, len(den) - 2)
        num = [1.0]
        for _ in range(m):
            num = _pmul(num, [1.0, round(rng.uniform(0.3, 6), 1)])
        x = {"num": num, "den": [_r12(v) for v in den], "K": round(10 ** rng.uniform(-0.5, 1.8), 2)}
        cl = _roots_raw(_padd(x["den"], _pscale(num, x["K"])))
        if min(abs(complex(p).real) for p in cl) > 0.05:
            return x
    return {"num": [1.0], "den": [1.0, 1.0], "K": 1.0}


_add("ctrl_nyquist", name="Nyquist criterion: encirclements of -1", topic="stability / Nyquist",
     purpose="For the open loop K G(s) under unity negative feedback: the open-loop right-half-plane poles P, the closed-loop right-half-plane poles Z and the net clockwise encirclements N = Z - P of the point -1 by the Nyquist curve.",
     inputs=[_v("num", "numerator of G(s)"), _v("den", "denominator of G(s)", 2), _n("K", "loop gain (default 1)", required=False)],
     outputs=[_o("P", "open-loop poles in the right half plane", "integer"), _o("Z", "closed-loop poles in the right half plane", "integer"),
              _o("N", "net clockwise encirclements of -1", "integer"), _o("stable", "true when Z = 0", "boolean")],
     primary="N", conditions=["no open-loop or closed-loop pole on the imaginary axis"], method="Z = N + P from the poles of G and of 1 + K G",
     latex=r"Z = N + P", run=nyquist_encirclements, reference=ref_nyquist_encirclements,
     wrong=lambda num, den, K=1.0: (lambda r: dict(r, N=r["P"] - r["Z"] if r["P"] != r["Z"] else 1))(nyquist_encirclements(num, den, K)),
     rand=_rand_nyq, phrases=["Nyquist encirclements", "Nyquist criterion"],
     examples=[_ex({"num": [1], "den": [1, -1], "K": 2}, {"P": 1, "Z": 0, "N": -1, "stable": True}),
               _ex({"num": [1], "den": [1, 6, 11, 6], "K": 100}, {"P": 0, "Z": 2, "N": 2, "stable": False})],
     extra_q=[_bad({"num": [1], "den": [1, 0, 1]})], level="expert")


def sensitivity_peak(num, den):
    num, den = _tf_in(num, den, strict=True)
    cl = _padd(den, num)
    _need(_routh_ok(cl), "the closed loop must be stable")
    f, g = _mag2(den), _mag2(cl)
    P = _ptrim(_psub(_pmul(_pder(f), g), _pmul(f, _pder(g))))
    cands = [0.0]
    if len(P) >= 2 and any(x != 0 for x in P):
        cands += _real_pos_roots(P)
    best_w, best = 0.0, -1.0
    for w in cands:
        v = math.sqrt(_pval(f, w) / _pval(g, w))
        if v > best:
            best, best_w = v, w
    _need(best > 1 + 1e-9, "|S(jw)| never exceeds 1: its supremum is approached only as w -> infinity")
    return _rnd({"Ms": best, "Ms_db": 20 * math.log10(best), "w_ms": best_w, "sm": 1.0 / best})


def ref_sensitivity_peak(num, den):
    import control
    _, _, sm, _, _, wms = control.stability_margins(control.tf(num, den))
    return {"Ms": 1 / float(sm), "Ms_db": -20 * math.log10(float(sm)), "w_ms": float(wms), "sm": float(sm)}


def _rand_sens(rng):
    for _ in range(100):
        x = _rand_loop(rng)
        if len(x["den"]) - len(x["num"]) >= 2 and _routh_ok(_padd(x["den"], x["num"])):
            return x
    return {"num": [2.0], "den": [1.0, 3.0, 2.0, 0.0]}


_add("ctrl_sensitivity_peak", name="Peak sensitivity Ms and modulus margin", topic="stability / robustness",
     purpose="Ms = max |1/(1 + L(jw))| of a stable unity-feedback loop, its frequency, and the modulus (stability) margin 1/Ms: the smallest distance from the Nyquist curve to -1.",
     inputs=[_v("num", "numerator of the open loop L(s)"), _v("den", "denominator of L(s)", 2)],
     outputs=[_o("Ms", "peak sensitivity"), _o("Ms_db", "peak sensitivity in dB"), _o("w_ms", "frequency of the peak", "angular_frequency"),
              _o("sm", "modulus margin 1/Ms")],
     primary="Ms", conditions=["closed loop stable", "strictly proper L(s)"],
     method="stationary points of |D(jw)|^2/|D(jw) + N(jw)|^2 (positive real roots of f' g - f g')",
     latex=r"M_s = \max_\omega \frac{1}{|1 + L(j\omega)|}", run=sensitivity_peak, reference=ref_sensitivity_peak,
     wrong=lambda num, den: (lambda r: dict(r, Ms=r["sm"], sm=r["Ms"], Ms_db=-r["Ms_db"]))(sensitivity_peak(num, den)),
     rand=_rand_sens, phrases=["peak sensitivity", "sensitivity peak"],
     examples=[_ex({"num": [2], "den": [1, 3, 2, 0]}, {"Ms": 2.31231401463}),
               _ex({"num": [1], "den": [1, 1, 0]}, {"Ms": 1.46788982501, "w_ms": 1.16877089448})],
     extra_q=[_bad({"num": [10], "den": [1, 3, 2, 0]})], level="expert")


# @@END@@

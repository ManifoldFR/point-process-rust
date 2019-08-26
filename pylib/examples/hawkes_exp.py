"""
This module illustrates how to 
"""
import numpy as np
import matplotlib.pyplot as plt
import pointprocesses as pp

plt.rcParams["figure.dpi"] = 120

alpha = 1.2  # jump size
tmax = 80.0
decay = 2.3  # intensity decay/mean reversion
lbda0 = 1.0  # background event intensity


def kernel(t: float, decay: float):
    return alpha * np.exp(-decay*t)


def intensity(t: float, lbda0: float, decay: float, evts):
    """
    The hawkes process intensity function.
    """
    timestamps = evts[0]
    delta_t = t - timestamps[timestamps < t]
    return lbda0 + (kernel(delta_t, decay)).sum()

# Vectorize the function for higher performance.
intensity = np.vectorize(intensity, excluded={3})

numsamples = 1000

processes = pp.temporal.batch_hawkes_exp(
    tmax, alpha, decay, lbda0, numsamples)

print("---- PLOT ----")

events = processes[0]

tarr = np.linspace(0, tmax, 500)
yarr = intensity(tarr, lbda0, decay, events)

stacked = np.stack([tarr, yarr]).T
points = np.stack([events[0], events[1]]).T
points = np.append(stacked, points, axis=0)
points = points[points[:,0].argsort()]

scatter_opts = {
    "s": 25.0,
    "alpha": 0.4,
    "linewidths": 0.5,
    "edgecolors": 'k',
    "zorder": 2
}

lineplot_opts = {
    "linewidth": 0.7,
    "zorder": 1
}

fig, (ax0, ax1) = plt.subplots(
    2, 1, sharex=True,
    gridspec_kw = {'height_ratios':[3, 0.5]},
    figsize=(8, 4))

ax0.plot(points[:,0], points[:,1], 'k',
    **lineplot_opts)
ax0.plot(points[:, 0], np.ones_like(points[:,0])*lbda0, 'r', 
    ls='--', lw=0.8, label="Background $\\lambda_0$")
ax0.legend()
ax0.set_ylabel(r"Intensity $\lambda(t)$")

ax1.scatter(events[0], [0. for _ in events[0]], **scatter_opts)
ax1.set_xlabel("Time $t$")
ax1.set_yticks([])

fig.tight_layout()
fig.savefig("hawkes.exp.png")
plt.show()

print("---- EVENT NUMBERS ----")
size_estimate = lbda0*tmax/(1-alpha/decay)
print("Theoretical evt. no. estimate: %f" % size_estimate)

sizes = np.array([p[0].shape[0] for p in processes]) # number of events in each process
print("Empirical evt. no. estimate: %f" % sizes.mean())

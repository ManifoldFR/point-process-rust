import itertools
import numpy as np
import matplotlib.pyplot as plt
import pointprocesses as pp

alpha = 0.7
jumps = itertools.repeat(alpha)
tmax = 20.0
beta = 1.0
lbda0 = 1.0


def kernel(t: float, decay: float):
    return np.exp(-decay*t)


def intensity(t: float, lbda0: float, decay: float, evts: np.ndarray):
    """
    The hawkes process intensity function.
    """
    prev_evts = evts[evts[:,0] < t]
    dts = t - prev_evts[:,0]
    marks = prev_evts[:,2]
    return lbda0 + (marks*kernel(dts, decay)).sum()


intensity = np.vectorize(intensity, excluded={3})


print("---- PLOT ----")

events = pp.hawkes_exp(tmax, beta, lbda0, jumps)
print(events)
tarr = np.linspace(0, tmax, 400)
yarr = intensity(tarr, lbda0, beta, events)

stacked = np.stack([tarr, yarr]).T
points = events[:,:2].copy()
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
    "linewidth": 1,
    "zorder": 1
}

fig, (ax0, ax1) = plt.subplots(2, 1, sharex=True,
                               gridspec_kw = {'height_ratios':[3, 0.5]},
                               figsize=(8,5))
ax0.plot(points[:,0], points[:,1], 'k', **lineplot_opts)
ax1.scatter(events[:,0], [0. for _ in events[:,0]], **scatter_opts)
ax0.set_ylabel(r"Intensity $\lambda(t)$")
ax1.set_xlabel("Time $t$")
fig.tight_layout()
plt.show()

print("---- EVENT NUMBERS ----")
size_estimate = lbda0*tmax/(1-alpha/beta)
print("Theoretical evt. no. estimate: %f" % size_estimate)
processes = [pp.hawkes_exp(tmax, beta, lbda0, jumps) for _ in range(1000)]
sizes = np.array([p.shape[0] for p in processes]) # number of events in each process
print("Empirical evt. no. estimate: %f" % sizes.mean())

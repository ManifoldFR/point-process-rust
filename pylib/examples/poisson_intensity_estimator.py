import numpy as np
import numba
import matplotlib.pyplot as plt
import pointprocesses as pp
from pointprocesses.temporal import variable_poisson


@numba.jit("double[:],double[:]", cache=True)
def count_events_by_(events, partition):
    m = len(partition)
    counts = np.zeros((m-1,))
    for i in range(m-1):
        low = partition[i]
        high = partition[i+1]
        counts[i] = np.sum((low < events) &(events < high))
    return counts


def intensity_estimator(data, partition) -> np.ndarray:
    """
    Inspired by Leemis (2001), "Nonparametric estimation and variate generation for a
    nonhomogeneous Poisson process from event count data"

    Args:
        data (list): set of simulated processes
        partition (list): partition of the overarching time interval
    """
    n = len(data)
    m = len(partition) - 1
    bandwidth = partition[1] - partition[0]
    estimates = np.zeros((n,m))
    for i in range(n):
        seq = data[i]  # i-th batch
        estimates[i,:] = count_events_by_(seq[0], partition) / bandwidth
    return estimates.mean(axis=0)


tmax = 8.0
trange = np.linspace(0, tmax, 201)
bandwidth = 0.1
partition = np.arange(0, tmax+bandwidth, bandwidth)

def intens(x):
    """Intensity function"""
    return 5.0*(1-0.9*np.exp(-x))*(1+0.2*np.sin(1.4*x)) + \
        1.0 * np.exp(0.2*x)

# max_lbda = np.max(1.01*intens(np.linspace(0, tmax, 200)))
max_lbda = 10.0
num_proc_samples = 500
# Simulated samples
data = [variable_poisson(tmax, intens, max_lbda) for _ in range(num_proc_samples)]
estimates = intensity_estimator(data, partition)

scatter_ops = {
    "s": 18.0,
    "color": "r",
    "linewidths": 0.5,
    "edgecolors": "k",
    "alpha": 0.7
}

plt.plot(trange, intens(trange),
    linestyle='--',
    label="actual intensity $\\lambda(t)$")
plt.scatter(0.5*(partition[1:]+partition[:-1]), estimates,
    label="estimate $\\hat{\\lambda}(t)$", **scatter_ops)

plt.xlabel("Time $t$")
plt.legend()
plt.tight_layout()
plt.savefig("estimate.png")
plt.show()
import numpy as np
import matplotlib.pyplot as plt
import pointprocesses as pp
from pointprocesses.temporal import variable_poisson


def count_events_by_(events, partition) -> list:
    m = len(partition)
    counts = np.zeros((m-1,), dtype=np.int)
    if events.ndim >= 2:
        events = events[:,0]
    for i in range(m-1):
        low = partition[i]
        high = partition[i+1]
        res = np.sum((events < high) & (events > low))
        counts[i] = res
    return counts


def intensity_estimator(processes, partition) -> np.ndarray:
    """
    Inspired by Leemis (2001), "Nonparametric estimation and variate generation for a
    nonhomogeneous Poisson process from event count data"

    Args:
        processes (list): set of simulated processes
        partition (list): partition of the overarching time interval
    """
    n = len(processes)
    m = len(partition) - 1
    bandwidth = partition[1] - partition[0]
    estimates = np.zeros((n,m))
    for i in range(n):
        events = processes[i]
        estimates[i,:] = count_events_by_(events, partition) / bandwidth
    return estimates.mean(axis=0)


tmax = 8.0
bandwidth = 0.2
partition = np.arange(0, tmax+bandwidth, bandwidth)

def intens(x):
    """Intensity function"""
    return 5.0*(1-np.exp(-x))*(1+0.2*np.sin(1.4*x))

max_lbda = 10.0
processes = [variable_poisson(tmax, intens, max_lbda) for _ in range(500)]
estimates = intensity_estimator(processes, partition)
# print("Partition:", partition)
# print("Intensity estimates:\n", estimates)

scatter_ops = {
    "s": 32.0,
    "linewidths": 0.8,
    "edgecolors": "k",
    "alpha": 0.7
}

plt.plot(partition, intens(partition),
    linestyle='--',
    label="actual intensity $\\lambda(t)$")
plt.scatter(0.5*(partition[1:]+partition[:-1]), estimates,
    label="estimate $\\hat{\\lambda}(t)$", **scatter_ops)
plt.xlabel("Time $t$")
plt.legend()
plt.tight_layout()
plt.savefig("estimate.png")
plt.show()
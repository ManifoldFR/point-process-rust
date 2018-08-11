import numpy as np
import matplotlib.pyplot as plt
import pointprocesses as pp

def count_events_by_(events, partition: list) -> list:
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


def intensity_estimator(processes, partition):
    """
    Inspired by Leemis (2001), "Nonparametric estimation and variate generation for a
    nonhomogeneous Poisson process from event count data"

    Args:
        processes (list): set of simulated processes
        partition (list): partition of the overarching time interval
    """
    n = len(processes)
    m = len(partition) - 1
    divisors = partition[1:] - partition[:-1]
    estimates = np.zeros((n,m))
    for i in range(n):
        events = processes[i]
        estimates[i,:] = count_events_by_(events, partition)
        estimates[i,:] = estimates[i,:]/divisors
    return estimates.mean(axis=0)


tmax = 8.0
partition = np.linspace(0, tmax, 100, endpoint=True)
intens = lambda x: 5.0*(1-np.exp(-x))*(1+0.2*np.sin(x))
max_lbda = 10.0
processes = [pp.variable_poisson(tmax, intens, max_lbda) for _ in range(500)]
estimates = intensity_estimator(processes, partition)
print("Partition:", partition)
print("Intensity estimates:\n", estimates)

scatter_ops = {
    "s": 32.0,
    "linewidths": 0.8,
    "edgecolors": "k",
    "alpha": 0.7
}

plt.plot(partition, intens(partition), label=r"actual intensity $\lambda(t)$")
plt.scatter(0.5*(partition[1:]+partition[:-1]), estimates, label=r"estimate $\hat{\lambda}(t)$", **scatter_ops)
plt.xlabel("Time $t$")
plt.legend()
plt.tight_layout()
plt.savefig("estimate.png")
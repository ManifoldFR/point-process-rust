import numpy as np
import matplotlib.pyplot as plt
import pointprocesses.pointprocesses as pp


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
    Inspired by Drazek (2013), Master's thesis at Leeds University

    Args:
        processes (list): set of simulated processes
        partition (list): partition of the overarching time interval
    """
    n = len(processes)
    m = len(partition) - 1
    divisors = partition[1:] - partition[:-1]
    print("Divisors:\n", divisors)
    estimates = np.zeros((n,m))
    for i in range(n):
        events = processes[i]
        estimates[i,:] = count_events_by_(events, partition)
        estimates[i,:] = estimates[i,:]/divisors
    return estimates.mean(axis=0)


tmax = 10.0
partition = np.linspace(0, tmax, 80, endpoint=True)
intens = lambda x: 5.0*(1+np.sin(2*x))
max_lbda = 10.0
processes = [pp.variable_poisson(tmax, intens, max_lbda) for _ in range(400)]
estimates = intensity_estimator(processes, partition)
print("Partition:", partition)
print("Intensity estimates:\n", estimates)

plt.figure(figsize=(8,6))
plt.plot(partition, intens(partition), label=r"actual intensity $\lambda(t)$")
plt.step(partition[:-1], estimates, where='post', label=r"estimate $\hat{\lambda}(t)$")
plt.legend()
plt.savefig("poisson-intensity-estimate.png")
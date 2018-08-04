from matplotlib import pyplot as plt
import numpy as np
import pointprocesses as pp


def intensity(x):
    return x*(1+0.2*np.cos(3*x))

max_lbda = 12.0

def max_likelihood_est(processes, tmax):
    return np.array([len(p) for p in processes]).mean()/tmax


tmax = 2.0
lbda = 2.1
res = pp.poisson_process(tmax, lbda)
print("Homogeneous process")
print(res)
print("number of events:", res.shape[0])

processes = [pp.poisson_process(tmax, lbda) for _ in range(100)]
print("Parameter estimation:", max_likelihood_est(processes, tmax))
print("============", end='\n')


tmax = 10.0
res = pp.variable_poisson(tmax, intensity, max_lbda)
print("Variable process")
print(res)
print("number of events:", res.shape[0])

plt.figure()
plt.xlabel("Time $t$")
plt.ylabel(r"Intensity $\lambda(t)$")
plt.scatter(res[:,0], res[:,1], s=4.0)
xarr = np.linspace(0, tmax, 100)
plt.plot(xarr, intensity(xarr))
plt.savefig("example.png", transparency=True)

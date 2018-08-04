import numpy as np
import pointprocesses as pp


def intensity(x):
    return x*(1+0.2*np.cos(3*x))

tmax = 10.0
max_lbda = 12.0

res = pp.poisson_process(tmax, 1.5)
print("Homogeneous process")
print(res)
print("number of events:", res.shape)
print("============")

res = pp.variable_poisson(tmax, intensity, max_lbda)
print("Variable process")
print(res)
print("number of events:", res.shape)

from matplotlib import pyplot as plt
plt.figure()
plt.xlabel("Time $t$")
plt.ylabel(r"Intensity $\lambda(t)$")
plt.scatter(res[:,0], res[:,1], s=4.0)
xarr = np.linspace(0, tmax, 100)
plt.plot(xarr, intensity(xarr))
plt.savefig("example.png", transparency=True)

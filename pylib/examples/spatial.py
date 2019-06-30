import numpy as np
import matplotlib.pyplot as plt
import pointprocesses as pp
from pointprocesses import spatial

close = np.array([0., 0.])
far = np.array([1., 1.])

scatter_opts = {
    "s": 18.0,
    "color": "r",
    "linewidths": 0.4,
    "edgecolors": 'k'
}

print("2D Poisson process")
lbda = 100.0
events = spatial.poisson_process(lbda, close, far)
print(events)


fig, (ax1, ax2) = plt.subplots(ncols=2, figsize=(10,5))
ax1.scatter(events[:,0], events[:,1], **scatter_opts)
ax1.set_title("2D Poisson process, $\\lambda={:.3f}$".format(lbda))
print("----------------")


print("2D variable Poisson process")
close = np.array([-1., -1.])
far = np.array([1., 1.])
def func(x, y):
    dist2 = x**2 + y**2
    return 160*(1-np.exp(-2*(np.power(dist2, 1.5))))
def intensity(a: np.ndarray):
    return func(a[0], a[1])
max_lbda = 160.0
events = spatial.variable_poisson(intensity, max_lbda, close, far)
# print(events)


xarr = np.linspace(-1, 1, 100)
yarr = xarr.copy()
X, Y = np.meshgrid(xarr, yarr)
Z1 = func(X, Y)
extent = [-1, 1, -1, 1]

cmap_name = "gray"
im = ax2.imshow(Z1, cmap=plt.get_cmap(cmap_name), extent=extent)
ax2.scatter(events[:,0], events[:,1], **scatter_opts)
ax2.set_title("2D Poisson process, variable intensity $\\lambda(x,y)$")
fig.subplots_adjust(right=0.8)
cbar_ax = fig.add_axes([0.85, 0.15, 0.05, 0.7])
cbar = fig.colorbar(im, cax=cbar_ax)
ax2.set_xlim((-1,1))
ax2.set_ylim((-1,1))
fig.savefig('2d_poisson.png')
plt.show()
print("----------------")
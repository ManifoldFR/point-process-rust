import numpy as np
import matplotlib.pyplot as plt
import pointprocesses as pyplot
from pointprocesses.temporal import poisson_process, variable_poisson

## Constant intensity process

tmax = 10.0
lbda = 2.0

simu_nums = 800
simuls = poisson_process(tmax, lbda)
timestamps = simuls
count = np.arange(0, timestamps.shape[0])

plt.step(timestamps, count,
         label='$N_t$')
plt.plot(timestamps, count, 'C0o', alpha=0.5)
plt.legend()
plt.tight_layout()
plt.show()

## Variable intensity process

def intensity(x):
    return 2 + 2 * np.tanh(0.8*(4-x))
trange = np.linspace(0, tmax, 301)
intens_t = intensity(trange)
max_lambda = 1.02 * np.max(intens_t)

simuls = variable_poisson(tmax, intensity, max_lambda)
timestamps, intens_ts = simuls
count = np.arange(0, timestamps.shape[0])

fig, ax1 = plt.subplots(1, 1)
ax1.plot(trange, intens_t,
         linewidth=1.0, linestyle='--',
         color='r',
         label='Intensity $\\lambda(t)$')
ax1.legend()
ax1.set_xlabel("$t$")
ax2 = ax1.twinx()
ax2.step(timestamps, count, label='N_t')
ax2.plot(timestamps, count, 'C0o', alpha=0.5)
ax2.legend()
fig.tight_layout()
plt.show()

from pointprocesses.temporal import batch_hawkes_exp
from pointprocesses.likelihood import hawkes_likelihood
import numpy as np
import matplotlib.pyplot as plt


tmax = 4000.
alpha = 0.4
beta = 1.0
lbda0 = 1.0

num_samples = 1

## Our data: one long sequence
data = batch_hawkes_exp(tmax, alpha, beta, lbda0, num_samples)

print("Number of tokens:", data[0][0].size)


## Illustrate likelihood as a function of (alpha, beta), lbda0 known
nx = 40
ny = 40


alpha_search = np.linspace(alpha - 0.2, alpha + 0.4, nx)
beta_search = np.linspace(beta - 0.4, beta + 0.7, ny)


lhood_func = np.vectorize(hawkes_likelihood, excluded={0})

alph_grid, beta_grid = np.meshgrid(alpha_search, beta_search)

z_lhood_vals = lhood_func(
    data[0][0], 
    lbda0, 
    alph_grid, 
    beta_grid,
    tmax)

print(z_lhood_vals)

idx_max = np.argmax(z_lhood_vals, axis=None)
idx_max = np.unravel_index(idx_max, z_lhood_vals.shape)

print("Grid search idx:", idx_max)
print("Grid search max loc:", (alpha_search[idx_max[0]], beta_search[idx_max[1]]))
print("Grid search max value:", z_lhood_vals[idx_max])


fig, ax = plt.subplots(1, 1, figsize=(6, 5))
ax: plt.Axes

cf_ = ax.contourf(
    alph_grid, beta_grid, z_lhood_vals,
    levels=50
)
cbar_ = fig.colorbar(cf_)

ax.contour(
    alph_grid, beta_grid, z_lhood_vals,
    levels=50,
    colors='k',
    linewidths=1.0
)

ax.scatter(alpha, beta, 
    marker='o',
    edgecolor='k')
ax.annotate("$(\\alpha,\\beta)_\\mathrm{real}$", (alpha, beta),
    xytext=(10,10), textcoords='offset pixels')

ax.set_xlabel("$\\alpha$")
ax.set_ylabel("$\\beta$")
ax.set_title("Log-likelihood of the data $L(\\Theta)$")

fig.tight_layout()
plt.show()

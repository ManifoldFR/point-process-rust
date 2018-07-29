import pointprocesses

result = pointprocesses.poisson_process(10.0, 4.0)
print(result)

print("Simulated %d events." % len(result))
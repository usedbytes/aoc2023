import sys
import matplotlib.pyplot as plt

xs = []
ys = []
zs = []

fig = plt.figure()
ax = fig.add_subplot(projection='3d')

with open(sys.argv[1]) as f:
    for line in f:
        line = line.strip()
        parts = [float(v) for v in line.split(",")]
        ax.plot3D(
                [parts[0], parts[3]],
                [parts[1], parts[4]],
                [parts[2], parts[5]],
        )
        ax.scatter(parts[0], parts[1], parts[2], marker='*')

plt.show()


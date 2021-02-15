import math
import matplotlib.pyplot as plt
import numpy as np

SCREEN_WIDTH = 640
SCREEN_HEIGHT= 480
spread = 1.0 / 36.7

x_sin = lambda time: (math.sin(time / 2) + math.cos(time / 7)) / 3
y_sin = lambda time: (math.sin(time / 3) + math.cos(time / 5)) / 3
shift_look_x = lambda time: SCREEN_WIDTH / 2 + SCREEN_WIDTH / 2 * x_sin(time)
shift_look_y = lambda time: SCREEN_HEIGHT / 2 + SCREEN_HEIGHT / 2 * y_sin(time)

t = np.arange(0, 400, spread)

fig, ax = plt.subplots(2)
fig.set_size_inches(12,6)

ax[0].plot(t, np.vectorize(shift_look_x)(t), 'g', label='x')
ax[0].plot(t, np.vectorize(shift_look_y)(t), 'r', label='y')
ax[0].set_ylabel('Added look-at direction')
ax[0].set_xlabel('time steps')
ax[0].legend(loc='best')

ax[1].plot(np.vectorize(shift_look_x)(t), np.vectorize(shift_look_y)(t), 'b')
ax[1].set_ylabel('y')
ax[1].set_xlabel('x')

plt.tight_layout()
plt.show()
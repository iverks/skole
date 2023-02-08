import matplotlib.pyplot as plt
import numpy as np

import smumerix

ys = np.linspace(-0.2, 0.2, 100)
angles = []

for y in ys:
    edg = smumerix.EventDrivenGas.new_for_test_4(y)
    edg.step()
    angle = edg.get_angle_off_x_axis(1)
    angles.append(angle)

angles = np.array(angles)

angles *= 180 / np.pi

plt.plot(ys, angles)
plt.xlabel("b")
plt.ylabel("degrees from x-axis")
plt.show()

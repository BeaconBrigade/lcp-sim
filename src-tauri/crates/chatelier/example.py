# Kevin Shao
# May 25, 2023
import matplotlib.pyplot as plt
import numpy as np

# Constants
kf = 1
kr = 0.2
x = 1
y = 1
v = 2
stop = 50
dt = 0.01
# Coefficients
a = 1
b = 1
c = 2
# Starting Conditions
A0 = 0.12
B0 = 0.3
C0 = 0
# Initialize
time = np.arange(0, stop, dt)
A = np.zeros(len(time))
B = np.zeros(len(time))
C = np.zeros(len(time))
A[0] = A0
B[0] = B0
C[0] = C0
for i in range(1, len(time)):  # Basically this is numerical integration
    rf = kf * (A[i - 1] ** x) * (B[i - 1] ** y)
    rr = kr * (C[i - 1] ** v)
    A[i] = A[i - 1] - a * rf * dt + a * rr * dt
    B[i] = B[i - 1] - b * rf * dt + b * rr * dt
    C[i] = C[i - 1] + c * rf * dt - c * rr * dt
plt.plot(time, A, label="$H_2$")
plt.plot(time, B, label="$I_2$")
plt.plot(time, C, label="$HI$")
plt.legend()
plt.title("Reaction Kinetics")
plt.xlabel("Time " + "$(s)$")
plt.ylabel("Concentration " + r"$(\frac{mol}{L})$")

plt.show()

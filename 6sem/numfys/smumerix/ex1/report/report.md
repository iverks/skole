# Report

## Problem 1

Running took ~90 seconds for 500_000 collisions of 5_000 particles. To collect more data i did 10_000 more steps 10 times and collected the speeds per step. That makes a total of 55_000 sampled speeds. This might not be independent enough data...

![histogram of speed distribution 500_000 steps](speed_dist_5000p_500000steps.png)

Running with 5_000_000 collisions and 5_000 particles took ~1420 seconds or about 24 minutes. To collect more data i did 500_000 more steps 10 times and collected the speeds per step.

![histogram of speed distribution 5_000_000 steps](speed_dist_5000p_5000000steps.png)

## Problem 2

Took 1745 seconds or about 30 minutes with 5_000_000 collisions and 5_000 particles. Did same data collection as P1. 
Average speed of light particles: 0.056
Average speed of heavy particles: 0.028

![histograms of the speed distributions of both masses](2_masses_5000p_5000000steps.png)

## Problem 3

With 5000 particles and 1500 samples * 625 steps per sample it took about 3 minutes per xi. For xi=1 we got this graph:

![energy development with xi 1](eq_5000particles_1_xi.png)

For xi=0.9 we got this graph:

![energy development with xi 0.9](eq_5000particles_0.9_xi.png)

We can see that for 1 the energy stabilises while for 0.9 the energy falls.

For xi = 0.8 i am getting errors telling me that the timestep is negative 🙄.



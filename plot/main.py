import matplotlib.pyplot as plt
from matplotlib import use
use("QtAgg");
y = [
	0.00009672407029360853,
	0.0003868843602745721,
	0.0008704450544407522,
	0.0015473462843615586,
	0.002417503865118067,
	0.003480808926972014,
	0.004737127442915807,
	0.006186299652951838,
	0.00782813938612121,
]
x = [
	-0.999999995322227,
	0.999999925160243,
	-0.9999996211626319,
	0.9999988028590215,
	-0.9999970778332615,
	0.999993941966257,
	-0.9999887797488478,
	0.9999808646652214,
	-0.9999693596474601,
]
plt.plot(x, y)
plt.ylabel('some numbers')
plt.show()

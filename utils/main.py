from evaluatee import Evaluatee
from plot import plot


ric3 = Evaluatee("../rIC3-CAV25/result/rIC3-ic3.txt")
abc = Evaluatee("../rIC3-CAV25/result/ABC-pdr.txt")
ic3ref = Evaluatee("../rIC3-CAV25/result/IC3ref.txt")
nuxmv_ic3 = Evaluatee("../rIC3-CAV25/result/nuXmv-ic3.txt")

evaluatee = [ric3, abc, ic3ref, nuxmv_ic3]
for e in evaluatee:
    e.summary()

plot(evaluatee)


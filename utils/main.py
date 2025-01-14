from evaluatee import Evaluatee
from plot import plot
from scatter import scatter
from analyze import analyze


# ric3 = Evaluatee("../rIC3-CAV25/result/rIC3-ic3.txt")
# abc = Evaluatee("../rIC3-CAV25/result/ABC-pdr.txt")
# ic3ref = Evaluatee("../rIC3-CAV25/result/IC3ref.txt")
# nuxmv_ic3 = Evaluatee("../rIC3-CAV25/result/nuXmv-ic3.txt")
# avr = Evaluatee("../rIC3-CAV25/result/AVR-ic3sa.txt")
# pono_ic3sa = Evaluatee("../rIC3-CAV25/result/Pono-ic3sa.txt")
# pono_ic3bits = Evaluatee("../rIC3-CAV25/result/Pono-ic3bits.txt")

# evaluatee = [ric3, abc, ic3ref, nuxmv_ic3, avr, pono_ic3sa, pono_ic3bits]
# analyze(evaluatee)
# plot(evaluatee)
# scatter(evaluatee)

ric3_portfolio = Evaluatee("../rIC3-CAV25/result/rIC3-portfolio.txt")
abc_superprove = Evaluatee("../rIC3-CAV25/result/ABC-superprove.txt")

evaluatee = [ric3_portfolio, abc_superprove]
analyze(evaluatee)
plot(evaluatee)
scatter(evaluatee)

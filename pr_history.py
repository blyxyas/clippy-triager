import matplotlib.pyplot as plt
import numpy as np
import math
from datetime import datetime

def days_since_newyear(date):
    dt = datetime.strptime(date, '%Y-%m-%d')
    diff = dt - datetime(2018, 1, 1)
    return diff.days / 7

plt.style.use('_mpl-gallery')

x = 0 + np.arange(8)

# make data:
datapoints = {}
current_week = {}
with open("pr_history_general") as file:
    for idx, line in enumerate(file):
        c = 8
        if idx >= 3020:
            c -= 1
        pr_num = math.floor(days_since_newyear(line[c:-1].rstrip('\x00')))
        datapoints[pr_num] = 0

with open("pr_history_general") as file:
    for idx, line in enumerate(file):
        c = 8
        if idx >= 3020:
            c -= 1
        pr_num = math.floor(days_since_newyear(line[c:-1].rstrip('\x00')))
        datapoints[pr_num] += 1
        

# plot
fig, ax = plt.subplots()

plt.axvline(x = 391, color = 'red', label = 'since feature freeze started')
plt.axvline(x = 403, color = 'red', label = 'feature freeeze ended')


for (key, value) in datapoints.items():
    ax.bar(key, value, width=1, edgecolor="white", linewidth=0.7)

# ax.set(xlim=(0, 337), xticks=np.arange(1, 337),
#        ylim=(0, 337), yticks=np.arange(1, 337))

plt.show()
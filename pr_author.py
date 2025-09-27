import matplotlib.pyplot as plt
import numpy as np
import math

plt.style.use('_mpl-gallery')

x = 0 + np.arange(8)

# make data:
new_contributors = {}
known_contributors = {}
datapoints = {}
with open("pr_by_author") as file:
    for idx, line in enumerate(file):
        c = 8
        if idx >= 3020:
            c -= 1

        if line[0] != "1":
            continue

        login = line.split(" - ")[1].strip()
        known_contributors[login] = [0, []]

with open("pr_by_author") as file:
    for idx, line in enumerate(file):
        c = 8
        if idx >= 3020:
            c -= 1

        if line[0] != "1":
            continue

        split = line.split(" - ")
        login = split[1].strip()
        pr_num = split[0].strip()
        known_contributors[login][0] += 1
        known_contributors[login][1].append(pr_num)

new_contributors = {}
num_new_contributors = 0
for (author, pr_count_list) in known_contributors.items():
    old_prs = 0
    new_prs = 0
    # print(pr_count_list)
    for pr in pr_count_list[1]:
        # print/(pr)
        if int(pr) < 15144:
            old_prs += 1
        else:
            new_prs += 1
            old_prs -= 1
    
    if old_prs >= 2:
        continue
    if old_prs < 0:
        new_contributors[author] = pr_count_list
        num_new_contributors += 1

how_many_with_each = {}
for (key, value) in new_contributors.items():
    how_many_with_each[value[0]] = 0
    print(key, value)

for (key, value) in new_contributors.items():
    how_many_with_each[value[0]] += 1

print(sorted(how_many_with_each.items()))
print(num_new_contributors)
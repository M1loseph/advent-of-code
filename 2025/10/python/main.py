import numpy as np
from scipy.optimize import linprog

with open("input.txt") as f:
    data = f.readlines()

machines = []

for line in data:
    segments = line.strip().split(" ")
    buttons = segments[1:len(segments) - 1]
    buttons = [b.removeprefix("(").removesuffix(")").split(",") for b in buttons]
    buttons = [[int(d) for d in b] for b in buttons]

    joltage = segments[-1].removeprefix("{").removesuffix("}").split(",")
    joltage = [int(j) for j in joltage]

    machines.append({"buttons": buttons, "joltage": joltage})

total = 0

for machine in machines:
    objective_coeffs = np.ones(len(machine["buttons"]))

    columns = []
    for button in machine["buttons"]:
        initial = [0] * len(machine["joltage"])
        for index in button:
            initial[index] = 1
        columns.append(initial)

    A_eq = np.array(columns).T

    b_eq = np.array(machine["joltage"])

    bounds = [(0, float("inf"))] * len(machine["buttons"])

    result = linprog(c=objective_coeffs,
                     A_eq=A_eq,
                     b_eq=b_eq,
                     bounds=bounds,
                     integrality=[1] * len(machine["buttons"])
                     )
    total += int(sum(result.x))

print("Puzzle 2 result:", int(total))

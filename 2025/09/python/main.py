import matplotlib.pyplot as plt

with open("input.txt") as file:
    lines = [ line.split(",") for line in file ]
    points = [ (int(point[0]), int(point[1])) for point in lines ]

# Visualize the problem
x, y = zip(*points)

plt.figure(figsize=(6, 4))
plt.plot(x, y, marker='o', linestyle='-', color='b')
plt.show()

def versor(segment):
    (a, b) = segment
    (ax, ay) = a
    (bx, by) = b
    return ((ax - bx) ** 0, (ay - by) ** 0)

def are_parallel(first, second):
    (vax, vay) = versor(first)
    (vbx, vby) = versor(second)
    return abs(vax) == abs(vbx) and abs(vay) == abs(vby)
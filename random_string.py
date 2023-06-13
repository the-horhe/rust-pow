import random

with open("rs.txt", "w") as fl:
    for item in range(1000000):
        fl.write(f"{random.randint(0, 32000)} ")

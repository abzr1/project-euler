sum_of_sq = sum(i**2 for i in range(1, 101))
sq_of_sum = sum(list(range(1, 101))) ** 2
print(sq_of_sum - sum_of_sq)

def is_palindrome(n):
    return n == int(str(n)[::-1])


largest = 0

for i in range(100, 1000):
    for x in range(100, 1000):
        if is_palindrome(i * x) and i * x > largest:
            largest = i * x

print(largest)

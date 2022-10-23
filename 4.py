def is_palindrome(n):
    if n == int(str(n)[::-1]):
        return True
    else:
        return False


a = 0
b = 0
largest_palindrome = 0

for i in range(100, 1000):
    for x in range(100, 1000):
        if is_palindrome(i * x) and i * x > largest_palindrome:
            largest_palindrome = i * x
            a = i
            b = x

print(f"{a} * {b} = {largest_palindrome} is the largest palindrome")

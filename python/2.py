a = 1
b = 2
c = 0

while b < 4000000:
    if b % 2 == 0:
        c += b

    temp = a
    a = b
    b += temp

print(c)

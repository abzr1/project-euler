x = 20

while True:
    # Some factors can be removed because e.g. if x % 20 == 0 then x % 2 == 0 and x % 10 == 0.
    if (
        x % 11 == 0
        and x % 13 == 0
        and x % 14 == 0
        and x % 16 == 0
        and x % 17 == 0
        and x % 18 == 0
        and x % 19 == 0
        and x % 20 == 0
    ):
        break
    else:
        x += 20  # 20 is the largest factor

print(x)
